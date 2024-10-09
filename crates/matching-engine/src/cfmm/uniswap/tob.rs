use std::collections::HashMap;

use alloy::primitives::{aliases::I24, I256, U256};
use angstrom_types::{
    contract_payloads::rewards::RewardsUpdate,
    matching::{
        uniswap::{Direction, PoolSnapshot, Quantity, Tick},
        Ray, SqrtPriceX96
    },
    sol_bindings::{grouped_orders::OrderWithStorageData, rpc_orders::TopOfBlockOrder}
};
use eyre::{eyre, Context, OptionExt};
use uniswap_v3_math::swap_math::compute_swap_step;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ToBOutcome {
    pub start_tick:      i32,
    pub start_liquidity: u128,
    pub tribute:         U256,
    pub total_cost:      U256,
    pub total_reward:    U256,
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

    pub fn to_rewards_update(&self) -> RewardsUpdate {
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
        let start_tick = I24::try_from(donations.first().map(|(a, _)| *a + 1).unwrap_or_default())
            .unwrap_or_default();
        match quantities.len() {
            0 | 1 => RewardsUpdate::CurrentOnly {
                amount: quantities.first().copied().unwrap_or_default()
            },
            _ => RewardsUpdate::MultiTick {
                start_tick,
                start_liquidity: self.start_liquidity,
                quantities
            }
        }
    }
}

pub fn new_reward(
    tob: &OrderWithStorageData<TopOfBlockOrder>,
    amm: &PoolSnapshot
) -> eyre::Result<ToBOutcome> {
    let output = match tob.is_bid {
        true => Quantity::Token0(tob.quantityOut),
        false => Quantity::Token1(tob.quantityOut)
    };
    let pricevec = (amm.current_price() - output)?;
    println!("Total cost: {}\tquantityIn: {}", pricevec.input(), tob.quantityIn);
    let total_cost: u128 = pricevec.input().saturating_to();
    if total_cost > tob.quantityIn {
        return Err(eyre!("Not enough input to cover the transaction"));
    }
    let leftover = tob.quantityIn - total_cost;
    let donation = pricevec.donation(leftover);
    Ok(ToBOutcome {
        start_tick:      amm.current_price().tick(),
        start_liquidity: amm.current_price().liquidity(),
        tribute:         U256::from(donation.tribute),
        total_cost:      pricevec.input(),
        total_reward:    U256::from(donation.total_donated),
        tick_donations:  donation.tick_donations
    })
}

pub fn calculate_reward(
    tob: &OrderWithStorageData<TopOfBlockOrder>,
    amm: PoolSnapshot
) -> eyre::Result<ToBOutcome> {
    // This implies that a bid will be purchasing T0 out of the pool, therefore
    // increasing the price while an ask will be selling T0 to the pool, decreasing
    // the price
    let direction = Direction::from_is_bid(tob.is_bid);

    // We start out at the tick and price that the AMM begins at
    let pool_price = amm.current_price();
    let mut current_liq_range = Some(pool_price.liquidity_range());
    let mut current_price = *pool_price.price();
    // Our fee is nothing
    let fee_pips = 0;

    // Turn our output into a negative number so compute_swap_step knows we're
    // looking to get an exact amount out
    let mut expected_out = I256::try_from(tob.order.quantityOut).wrap_err_with(|| {
        format!(
            // This should be impossible
            "Expected ToB order output too large to convert u128 -> I256: {}",
            tob.order.quantityOut
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
        // Update our current liquidiy range
        let liq_range =
            current_liq_range.ok_or_else(|| eyre!("Unable to find next liquidity range"))?;
        println!("Operating on liq range [{}..{})", liq_range.lower_tick(), liq_range.upper_tick());
        // Compute our swap towards the appropriate end of our current liquidity bound
        let target_tick = liq_range.end_bound(direction);
        let target_price = SqrtPriceX96::at_tick(target_tick)?;
        let (fin_price, amount_in, amount_out, amount_fee) = compute_swap_step(
            current_price.into(),
            target_price.into(),
            liq_range.liquidity(),
            expected_out,
            fee_pips
        )
        .wrap_err_with(|| {
            format!(
                "Unable to compute swap step from tick {:?} to {}",
                current_price.to_tick(),
                target_tick
            )
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

        println!("S price: {}\nT price: {}\nE price: {}", *current_price, *target_price, fin_price);

        // This seems to work properly, so let's run with it
        let avg_price = Ray::calc_price(amount_in, amount_out);

        // Push this stake onto our list of stakes to resolve
        stakes.push((avg_price, end_price, amount_out, liq_range));

        // If we're going to be continuing, move on to the next liquidity range
        current_liq_range = liq_range.next(direction);
        current_price = SqrtPriceX96::from(fin_price);
        println!("C: {}\nF: {}", *current_price, fin_price);
    }

    // Determine how much extra quantityIn we have that will be used as tribute to
    // the LPs
    let bribe = U256::from(tob.quantityIn)
        .checked_sub(total_cost)
        .ok_or_else(|| {
            eyre!("Total cost greater than amount offered: {} > {}", total_cost, tob.quantityIn)
        })?;

    if stakes.is_empty() {
        // TODO: Maybe this should just be a big donation to the current tick?
        return Err(eyre!("No actual purchases could be made with this TOB order"))
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

        println!("Rem: {}\tCost: {}", rem_bribe, step_cost);
        if rem_bribe >= step_cost {
            // If we have enough bribe to pay the whole cost, allocate that and step forward
            // to the next price gap
            cur_q += stake.2;
            filled_price = target_price;
            rem_bribe -= step_cost;
        } else {
            // If we don't have enough bribe to pay the whole cost, figure out where the
            // target price winds up based on what we do have and end this iteration
            if rem_bribe > U256::ZERO {
                let partial_dprice = Ray::calc_price(rem_bribe, q_step);
                filled_price += partial_dprice;
            }
            break
        }
    }

    // We've now found our filled price, we can allocate our reward to each tick
    // based on how much it costs to bring them up to that price.
    let mut reward_t = U256::ZERO;
    let tick_donations: HashMap<Tick, U256> = stakes
        .iter()
        .filter_map(|(p_avg, _p_end, q_out, liq)| {
            // We always donate to the lower tick of our liquidity range as that is the
            // appropriate initialized tick to target
            let tick_num = liq.lower_tick();
            if filled_price > *p_avg {
                let total_dprice = filled_price - *p_avg;
                let total_reward = total_dprice.mul_quantity(*q_out);
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
        start_tick: pool_price.tick(),
        start_liquidity: pool_price.liquidity(),
        tribute,
        total_cost,
        total_reward: reward_t,
        tick_donations
    })
}

#[cfg(test)]
mod test {
    use alloy::primitives::Uint;
    use angstrom_types::matching::{
        uniswap::{LiqRange, PoolSnapshot, Quantity},
        SqrtPriceX96
    };
    use rand::thread_rng;
    use testing_tools::type_generator::orders::generate_top_of_block_order;
    use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

    use super::calculate_reward;
    use crate::cfmm::uniswap::tob::new_reward;

    fn generate_amm_market(target_tick: i32) -> PoolSnapshot {
        let range =
            LiqRange::new(target_tick - 1000, target_tick + 1000, 100_000_000_000_000).unwrap();
        let ranges = vec![range];
        let sqrt_price_x96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(target_tick).unwrap());
        PoolSnapshot::new(ranges, sqrt_price_x96).unwrap()
    }

    #[test]
    fn calculates_reward() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let total_payment = 10_000_000_000_000_u128;
        let order = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let newresult = new_reward(&order, &amm).unwrap();
        let result = calculate_reward(&order, amm).expect("Error calculating tick donations");
        let total_donations = result.total_donations();
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            Uint::from(total_payment),
            "Total allocations do not add up to input payment"
        );
        assert_eq!(result, newresult, "New result not equal");
    }

    #[test]
    fn handles_insufficient_funds() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let order = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(10_000_000_u128),
            Some(100000000_u128)
        );
        let newresult = new_reward(&order, &amm);
        let result = calculate_reward(&order, amm);
        assert!(result.is_err_and(|e| {
            e.to_string()
                .starts_with("Total cost greater than amount offered")
        }));
        assert!(newresult.is_err());
    }

    #[test]
    fn handles_precisely_zero_donation() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        // Hand-calculated that this is the correct payment for this starting price and
        // liquidity
        let total_payment = 2_201_872_310_000_u128;
        let order = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let newresult = new_reward(&order, &amm).unwrap();
        let result = calculate_reward(&order, amm).expect("Error calculating tick donations");
        let total_donations = result.total_donations();
        assert!(
            result.tick_donations.is_empty(),
            "Donations are being offered when we shouldn't have any"
        );
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            Uint::from(total_payment),
            "Total allocations do not add up to input payment"
        );
        assert_eq!(result, newresult, "New result not equal");
    }

    #[test]
    fn handles_partial_donation() {
        let mut rng = thread_rng();
        let price = SqrtPriceX96::from(get_sqrt_ratio_at_tick(100000).unwrap());
        let liquidity = 100_000_000_000_000;
        let ranges = vec![
            LiqRange::new(100000, 100001, liquidity).unwrap(),
            LiqRange::new(100001, 100002, liquidity).unwrap(),
            LiqRange::new(100002, 100003, liquidity).unwrap(),
        ];
        let amm = PoolSnapshot::new(ranges, price).unwrap();
        let total_payment = 2_201_872_310_000_u128; // + 20_000_000_u128;
                                                    //let total_payment = 2_202_072_310_000_u128;
        let order = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        //let newresult = new_reward(&order, &amm).unwrap();
        let result = calculate_reward(&order, amm).expect("Error calculating tick donations");
        println!("Result: {:?}", result);
        let total_donations = result.total_donations();
        assert!(result.tick_donations.contains_key(&100000), "Donation to first tick missing");
        assert!(result.tick_donations.contains_key(&100001), "Donation to second tick missing");
        assert!(
            !result.tick_donations.contains_key(&100002),
            "Donation to third tick present when it shouldn't be"
        );
        assert_eq!(
            total_donations + result.total_cost + result.tribute,
            Uint::from(total_payment),
            "Total allocations do not add up to input payment"
        );
        //assert_eq!(result, newresult, "New result not equal");
    }

    #[test]
    fn handles_bid_order() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let order = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(10_000_000_000_000_u128),
            Some(100000000_u128)
        );
        let newresult = new_reward(&order, &amm);
        let result = calculate_reward(&order, amm);
        assert!(result.is_ok());
        assert!(newresult.is_ok());
    }

    #[test]
    fn handles_ask_order() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let order = generate_top_of_block_order(
            &mut rng,
            false,
            None,
            None,
            Some(10_000_000_000_000_u128),
            Some(800000000_u128)
        );
        let newresult = new_reward(&order, &amm);
        let result = calculate_reward(&order, amm);
        assert!(result.is_ok());
        assert!(newresult.is_ok());
    }

    #[test]
    fn only_rewards_initialized_ticks() {
        let mut rng = thread_rng();
        let amm = generate_amm_market(100000);
        let total_payment = 2_203_371_417_593_u128;
        let order = generate_top_of_block_order(
            &mut rng,
            true,
            None,
            None,
            Some(total_payment),
            Some(100000000_u128)
        );
        let first_tick = 100000 - 1000;
        let newresult = new_reward(&order, &amm).unwrap();
        let result = calculate_reward(&order, amm).expect("Error calculating tick donations");
        assert!(
            result.tick_donations.len() == 1,
            "Too many donations - only one initialized tick in this market"
        );
        assert!(
            result.tick_donations.contains_key(&first_tick),
            "Donation not made to only initialized tick"
        );
        assert_eq!(result, newresult, "New result not equal");
    }

    #[test]
    fn use_new_constructs() {
        let price = SqrtPriceX96::from(get_sqrt_ratio_at_tick(100000).unwrap());
        let liquidity = 100_000_000_000_000;
        let ranges = vec![
            LiqRange::new(100000, 100001, liquidity).unwrap(),
            LiqRange::new(100001, 100002, liquidity).unwrap(),
            LiqRange::new(100002, 100003, liquidity).unwrap(),
        ];
        let amm = PoolSnapshot::new(ranges, price).unwrap();
        // let amm = generate_amm_market(100000);
        let pricevec = (amm.current_price() - Quantity::Token0(100_000_000_u128)).unwrap();
        let steps = pricevec.steps();
        println!("Steps: {:?}", steps);
        let donate = pricevec.donation(50002);
        println!("Donation: {:?}", donate);
    }
}
