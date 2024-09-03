use std::collections::HashMap;

use alloy::primitives::{address, I256, U256};
use angstrom_types::{
    matching::{Ray, SqrtPriceX96},
    sol_bindings::{
        grouped_orders::OrderWithStorageData,
        sol::{SolDonate, SolPoolRewardsUpdate, SolRewardsUpdate, TopOfBlockOrder}
    }
};
use eyre::{eyre, Context, OptionExt};
use uniswap_v3_math::{swap_math::compute_swap_step, tick_math::get_sqrt_ratio_at_tick};

use super::{MarketSnapshot, Tick};

#[derive(Debug)]
pub struct ToBOutcome {
    pub start_tick:      i32,
    pub start_liquidity: u128,
    pub tribute:         U256,
    pub total_cost:      U256,
    pub tick_donations:  HashMap<Tick, U256>
}

impl ToBOutcome {
    /// Sum of the donations across all ticks
    pub fn total_donations(&self) -> U256 {
        self.tick_donations
            .iter()
            .fold(U256::ZERO, |acc, (_tick, donation)| acc + donation)
    }

    /// Tick donations plus tribute to determine total value of this outcome
    pub fn total_value(&self) -> U256 {
        self.total_donations() + self.tribute
    }

    pub fn to_donate(&self, a0_idx: u16, a1_idx: u16) -> SolPoolRewardsUpdate {
        let mut donations = self.tick_donations.iter().collect::<Vec<_>>();
        // Will sort from lowest to highest (donations[0] will be the lowest tick
        // number)
        donations.sort_by_key(|f| f.0);
        // Each reward value is the cumulative sum of the rewards before it
        let quantities = donations
            .iter()
            .scan(U256::ZERO, |state, (_tick, q)| {
                *state += **q;
                Some(u128::try_from(*state).unwrap())
            })
            .collect::<Vec<_>>();
        let update = SolRewardsUpdate {
            startTick: *donations[0].0 + 1,
            startLiquidity: self.start_liquidity,
            quantities
        };
        SolPoolRewardsUpdate { asset0: a0_idx, asset1: a1_idx, update }
    }
}

pub fn calculate_reward(
    tob: OrderWithStorageData<TopOfBlockOrder>,
    amm: MarketSnapshot
) -> eyre::Result<ToBOutcome> {
    // This implies that a bid will be purchasing T0 out of the pool, therefore
    // increasing the price while an ask will be selling T0 to the pool, decreasing
    // the price
    let tick_motion = if tob.is_bid { 1 } else { -1 };

    // We start out at the tick and price that the AMM begins at
    let mut current_tick = amm.current_tick;
    let mut current_price = amm.sqrt_price_x96;
    // TODO:  Figure out how fee pips factor into this
    let fee_pips = 600;

    // Turn our output into a negative number so compute_swap_step knows we're
    // looking to get an exact amount out
    let mut expected_out = I256::try_from(tob.order.amountOut).wrap_err_with(|| {
        format!(
            "Expected ToB order output too large to convert U256 -> I256: {}",
            tob.order.amountOut
        )
    })? * I256::MINUS_ONE;

    // Initialize some things we're going to do to track
    let mut total_cost = U256::ZERO;
    let mut stakes = Vec::new();

    // The bid/ask direction determines what we're trading.  In all cases, our
    // amountIn is what we have to give and our amountOut is what we expect to get.
    // So our bribe is always housed in amountIn no matter what the directionality
    // of the order is, and we always want to count down our amountOut to find out
    // where we stop selling to the AMM and start taking a bribe
    while expected_out < I256::ZERO {
        let next_tick = current_tick + tick_motion;
        let next_price = SqrtPriceX96::from(
            get_sqrt_ratio_at_tick(next_tick)
                .wrap_err_with(|| format!("Unable to get SqrtPrice at tick {}", next_tick))?
        );
        let liquidity = amm
            .liquidity_at_tick(current_tick)
            .ok_or_else(|| eyre!("Unable to find liquidity for tick {}", current_tick))?;
        let (fin_price, amount_in, amount_out, amount_fee) = compute_swap_step(
            current_price.into(),
            next_price.into(),
            liquidity,
            expected_out,
            fee_pips
        )
        .wrap_err_with(|| {
            format!("Unable to compute swap step from tick {} to {}", current_tick, next_tick)
        })?;

        // See how much output we have yet to go
        let signed_out = I256::try_from(amount_out)
            .wrap_err("Output of step too large to convert U256 -> I256")?;
        expected_out = expected_out
            .checked_add(signed_out)
            .ok_or_eyre("Unable to add signed_out to expected_out")?;

        // Add the amount in and our total fee to our cost
        total_cost += amount_in;
        total_cost += amount_fee;

        // How much should this have cost if it was done by the raw price
        let end_price = Ray::from(SqrtPriceX96::from(fin_price));

        // This seems to work properly, so let's run with it
        let avg_price = Ray::calc_price(amount_in, amount_out);

        // See if we have enough bribe left over to cover the total amount so far (can
        // we do this)?
        stakes.push((avg_price, end_price, amount_out));

        // Iterate!
        current_tick += tick_motion;
        current_price = SqrtPriceX96::from(fin_price);
    }

    // Determine how much extra amountIn we have that will be used as tribute to the
    // LPs
    let bribe = tob.amountIn.checked_sub(total_cost).ok_or_else(|| {
        eyre!("Total cost greater than amount offered: {} > {}", total_cost, tob.amountIn)
    })?;

    if stakes.is_empty() {
        // TODO: Maybe this should just be a big donation to the current tick?
        return Err(eyre!("No actual purchases could be made with this TOB order"));
    }

    let mut rem_bribe = bribe;
    let mut cur_q = U256::ZERO;
    let mut filled_price = stakes[0].0;

    let mut stake_iter = stakes.iter().peekable();
    while let Some(stake) = stake_iter.next() {
        let q_step = cur_q + stake.2;
        // Our target price is either the average price of the next stake or the end
        // price of the current stake if there's no next stake to deal with
        let target_price = stake_iter
            .peek()
            .map(|next_stake| next_stake.0)
            .unwrap_or_else(|| stake.1);
        // The difference between this tick's average price and our target price
        let d_price = target_price - stake.0;

        // The step cost is the total cost in needed to ensure that all sold quantities
        // were sold at our target price
        let step_cost = d_price.mul_quantity(q_step);

        if rem_bribe >= step_cost {
            // If we have enough bribe to pay the whole cost, allocate that and step forward
            // to the next price gap
            cur_q += stake.2;
            filled_price = target_price;
            rem_bribe -= step_cost;
        } else {
            // If we don't have enough bribe to pay the whole cost, figure out where the
            // target price winds up based on what we do have and end this iteration
            let partial_dprice = Ray::calc_price(rem_bribe, q_step);
            filled_price += partial_dprice;
            break;
        }
    }

    // We've now found our filled price, we can allocate our reward to each tick
    // based on how much it costs to bring them up to that price.
    let mut reward_t = U256::ZERO;
    let tick_donations: HashMap<Tick, U256> = stakes
        .iter()
        .enumerate()
        .filter_map(|(i, stake)| {
            let tick_num = amm.current_tick + (i as i32 * tick_motion);
            if filled_price > stake.0 {
                let total_dprice = filled_price - stake.0;
                let total_reward = total_dprice.mul_quantity(stake.2);
                if total_reward > U256::ZERO {
                    reward_t += total_reward;
                    Some((tick_num, total_reward))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let tribute = bribe - reward_t;
    // Both our tribute and our tick_donations are done in the same currency as
    // amountIn
    Ok(ToBOutcome {
        start_tick: amm.current_tick,
        start_liquidity: amm.current_position().liquidity(),
        tribute,
        total_cost,
        tick_donations
    })
}

#[cfg(test)]
mod test {
    use alloy::{
        contract::RawCallBuilder,
        network::EthereumWallet,
        primitives::{address, keccak256, Address, Bytes, Uint, B256, U160, U256},
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
        sol,
        sol_types::SolValue
    };
    use angstrom_types::{
        contract_bindings::{
            hookdeployer::HookDeployer::{self, HookDeployerInstance},
            mockrewardsmanager::MockRewardsManager,
            poolgate::PoolGate,
            poolmanager::PoolManager
        },
        matching::SqrtPriceX96
    };
    use pade::PadeEncode;
    use pade_macro::PadeEncode;
    use rand::thread_rng;
    use testing_tools::type_generator::orders::generate_top_of_block_order;
    use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

    use super::calculate_reward;
    use crate::cfmm::uniswap::{MarketSnapshot, PoolRange};

    fn generate_amm_market(target_tick: i32) -> MarketSnapshot {
        let range =
            PoolRange::new(target_tick - 1000, target_tick + 1000, 100_000_000_000_000).unwrap();
        let ranges = vec![range];
        let sqrt_price_x96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(target_tick).unwrap());
        MarketSnapshot::new(ranges, sqrt_price_x96).unwrap()
    }

    #[test]
    fn calculates_reward() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        let total_payment = Uint::from(10_000_000_000_000_u128);
        order.order.amountIn = total_payment;
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm).expect("Error calculating tick donations");
        let total_donations = result.total_donations();
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            total_payment,
            "Total allocations do not add up to input payment"
        );
    }

    #[test]
    fn handles_insufficient_funds() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(-100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        order.is_bid = true;
        order.order.amountOut = Uint::from(10_000_000_000_000_u128);
        order.order.amountIn = Uint::from(100000000);
        let result = calculate_reward(order, amm);
        assert!(result.is_err_and(|e| {
            e.to_string()
                .starts_with("Total cost greater than amount offered")
        }))
    }

    #[test]
    fn handles_precisely_zero_donation() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        let total_payment = Uint::from(2_203_194_246_001_u128);
        order.order.amountIn = total_payment;
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm).expect("Error calculating tick donations");
        let total_donations = result.total_donations();
        assert!(
            result.tick_donations.is_empty(),
            "Donations are being offered when we shouldn't have any"
        );
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            total_payment,
            "Total allocations do not add up to input payment"
        );
    }

    #[test]
    fn handles_partial_donation() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        let total_payment = Uint::from(2_203_371_417_593_u128);
        order.order.amountIn = total_payment;
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm).expect("Error calculating tick donations");
        let total_donations = result.total_donations();
        assert!(result.tick_donations.contains_key(&100000), "Donation to first tick missing");
        assert!(result.tick_donations.contains_key(&100001), "Donation to second tick missing");
        assert!(
            !result.tick_donations.contains_key(&100002),
            "Donation to third tick present when it shouldn't be"
        );
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            total_payment,
            "Total allocations do not add up to input payment"
        );
    }

    #[test]
    fn handles_bid_order() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        order.is_bid = true;
        order.order.amountIn = Uint::from(10_000_000_000_000_u128);
        order.order.amountOut = Uint::from(100000000);
        let result = calculate_reward(order, amm);
        assert!(result.is_ok());
    }

    #[test]
    fn handles_ask_order() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        order.is_bid = false;
        order.order.amountOut = Uint::from(10_000_000_000_000_u128);
        order.order.amountIn = Uint::from(800000000);
        let result = calculate_reward(order, amm);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn deploys_uniswap_contract() {
        let anvil = alloy::node_bindings::Anvil::new().try_spawn().unwrap();
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        let rpc_url = anvil.endpoint().parse().unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);
        println!("Anvil running at '{}'", anvil.endpoint());

        let pool_manager = PoolManager::deploy(&provider, U256::from(50_000_u32))
            .await
            .unwrap();
        println!("PoolManager deployed at address: {}", pool_manager.address());

        let pool_gate = PoolGate::deploy(&provider, *pool_manager.address())
            .await
            .unwrap();
        println!("PoolGate deployed at address: {}", pool_gate.address());

        // Flags for our MockRewardsManager address
        let before_swap = U160::from(1_u8) << 7;
        let before_initialize = U160::from(1_u8) << 13;
        let before_add_liquidity = U160::from(1_u8) << 11;
        let after_remove_liquidity = U160::from(1_u8) << 8;

        let flags = before_swap | before_initialize | before_add_liquidity | after_remove_liquidity;
        let all_hook_mask: U160 = (U160::from(1_u8) << 14) - U160::from(1_u8);

        let builder = MockRewardsManager::deploy_builder(&provider, *pool_manager.address());
        let full_initcode = builder.calldata();

        //    [MockRewardsManager::BYTECODE.to_vec(),
        // pool_manager.address().abi_encode_packed()]        .concat();
        let init_code_hash = keccak256(full_initcode);
        let mut salt = U256::ZERO;
        let create2_factory = address!("4e59b44847b379578588920cA78FbF26c0B4956C");
        let mut counter: u128 = 0;
        loop {
            let target_address: Address = create2_factory.create2(B256::from(salt), init_code_hash);
            let u_address: U160 = target_address.into();
            if (u_address & all_hook_mask) == flags {
                break;
            }
            salt += U256::from(1_u8);
            counter += 1;
            if counter > 100_000 {
                panic!("We tried this too many times!")
            }
        }
        let final_address = create2_factory.create2(B256::from(salt), init_code_hash);
        println!("I found my address and it's {}", final_address);
        let final_initcode = [salt.abi_encode(), full_initcode.to_vec()].concat();
        let raw_deploy = RawCallBuilder::new_raw_deploy(&provider, final_initcode.into());
        //let raw_address = raw_deploy.calculate_create_address().unwrap();
        //println!("My raw address is:          {}", raw_address);
        raw_deploy.call_raw().await.unwrap();
        println!("MockRewardsManager deployed at address: {}", final_address);
        // }

        // #[tokio::test]
        // async fn talks_to_contract() {
        // Define the contract and types
        sol! {
            #[derive(PadeEncode)]
            struct Asset {
                address addr;
                uint128 borrow;
                uint128 save;
                uint128 settle;
            }

            #[derive(PadeEncode)]
            struct RewardsUpdate {
                int24 startTick;
                uint128 startLiquidity;
                uint128[] quantities;
            }

            #[derive(PadeEncode)]
            struct PoolRewardsUpdate {
                uint16 asset0;
                uint16 asset1;
                RewardsUpdate update;
            }

            #[derive(PadeEncode)]
            struct MockContractMessage {
                Asset[] addressList;
                PoolRewardsUpdate update;
            }
        }

        // These are TEMPROARY LOCAL ADDRESSES from Dave's Testnet - if you are seeing
        // these used in prod code they are No Bueno
        let asset1 = address!("76ca03a67C049477FfB09694dFeF00416dB69746");
        let asset0 = address!("1696C7203769A71c97Ca725d42b13270ee493526");

        // Build a ToB outcome that we care about
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let mut order = generate_top_of_block_order(&mut rng, true, None, None);
        let total_payment = Uint::from(10_000_000_000_000_u128);
        order.order.amountIn = total_payment;
        order.order.amountOut = Uint::from(100000000);
        let tob_outcome = calculate_reward(order, amm).expect("Error calculating tick donations");
        // ---- Manually do to_donate to be in our new structs
        let mut donations = tob_outcome.tick_donations.iter().collect::<Vec<_>>();
        // Will sort from lowest to highest (donations[0] will be the lowest tick
        // number)
        donations.sort_by_key(|f| f.0);
        // Each reward value is the cumulative sum of the rewards before it
        let quantities = donations
            .iter()
            .scan(U256::ZERO, |state, (_tick, q)| {
                *state += **q;
                Some(u128::try_from(*state).unwrap())
            })
            .collect::<Vec<_>>();
        let update = RewardsUpdate {
            startTick: *donations[0].0 + 1,
            startLiquidity: tob_outcome.start_liquidity,
            quantities
        };
        let update = PoolRewardsUpdate { asset0: 0, asset1: 1, update };
        println!("Encoded u16: {:?}", 1_u16.pade_encode());
        // ---- End of all that

        // Connect to our contract and send the ToBoutcome over
        //let signer = PrivateKeySigner::from_signing_key()
        //let wallet = EthereumWallet::from(signer);
        // let provider = ProviderBuilder::new().on_http("http://localhost:8545".parse().unwrap());
        // Currently the address of a local deploy that I'm running, not the right
        // address, should configure this to stand up on its own
        let contract_address = address!("4026bA349706b18b9dA081233cc20B3C5B4bE980");
        // let new_test =
        // contract.getGrowthInsideTick(FixedBytes::<32>::default(),12345);
        // let new_test_res = new_test.send().await.unwrap().;
        // println!("New test: {:?}", new_test_res);
        let address_list = [asset0, asset1]
            .into_iter()
            .map(|addr| Asset { addr, borrow: 0, save: 0, settle: 0 })
            .collect();
        let tob_mock_message = MockContractMessage { addressList: address_list, update };
        let tob_bytes = Bytes::from(pade::PadeEncode::pade_encode(&tob_mock_message));
        println!("Full encoded message: {:?}", tob_mock_message.pade_encode());
        let contract =
            angstrom_types::contract_bindings::mockrewardsmanager::MockRewardsManager::new(
                final_address,
                &provider
            );
        let call = contract.reward(tob_bytes);
        let call_return = call.call().await.unwrap();
    }
}
