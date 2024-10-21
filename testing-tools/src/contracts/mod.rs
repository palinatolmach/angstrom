use alloy::{
    contract::{CallBuilder, RawCallBuilder},
    primitives::{address, keccak256, Address, Bytes, B256, U160, U256},
    sol_types::SolValue
};
use angstrom_types::contract_bindings::mock_rewards_manager::MockRewardsManager;
use eyre::eyre;

use crate::contracts::deploy::mine_address;

pub mod anvil;
pub mod deploy;
pub mod environment;
//mod reward;
//pub use reward::RewardTestEnv;

/// This trait is used to provide safe run and potentially debug capabilities
/// for our local contract runs.
pub trait DebugTransaction {
    #[allow(async_fn_in_trait)] // OK because this is not for public consumption
    async fn run_safe(self) -> eyre::Result<()>;
}

impl<T, P, D> DebugTransaction for CallBuilder<T, P, D>
where
    T: Clone + Send + Sync + alloy::transports::Transport,
    P: alloy::providers::Provider<T>,
    D: alloy::contract::CallDecoder
{
    async fn run_safe(self) -> eyre::Result<()> {
        let receipt = self
            .gas(30_000_000_u128)
            .send()
            .await?
            .get_receipt()
            .await?;
        if receipt.inner.status() {
            Ok(())
        } else {
            // We can make this do a cool backtrace later
            Err(eyre!("Transaction with hash {} failed", receipt.transaction_hash))
        }
    }
}

// pub fn mine_address_with_factory(
//     factory: Option<Address>,
//     flags: U160,
//     mask: U160,
//
//     initcode: &Bytes
// ) -> (Address, U256) {
//     let init_code_hash = keccak256(initcode);
//     let mut salt = U256::ZERO;
//     let create2_factory = factory.unwrap_or(CREATE2_FACTORY);
//     let mut counter: u128 = 0;
//     loop {
//         let target_address: Address =
// create2_factory.create2(B256::from(salt), init_code_hash);         let
// u_address: U160 = target_address.into();         if (u_address & mask) ==
// flags {             break
//         }
//         salt += U256::from(1_u8);
//         counter += 1;
//         if counter > 100_000 {
//             panic!("We tried this too many times!")
//         }
//     }
//     let final_address = create2_factory.create2(B256::from(salt),
// init_code_hash);     (final_address, salt)
// }

pub async fn deploy_mock_rewards_manager<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    pool_manager: Address
) -> Address
where {
    // Setup our flags and mask
    // Flags for our MockRewardsManager address
    let before_swap = U160::from(1_u8) << 7;
    let before_initialize = U160::from(1_u8) << 13;
    let before_add_liquidity = U160::from(1_u8) << 11;
    let after_remove_liquidity = U160::from(1_u8) << 8;

    let flags = before_swap | before_initialize | before_add_liquidity | after_remove_liquidity;
    let mask: U160 = (U160::from(1_u8) << 14) - U160::from(1_u8);

    let mock_builder =
        MockRewardsManager::deploy_builder(&provider, pool_manager, Address::default());
    let (mock_tob, salt) = mine_address(flags, mask, mock_builder.calldata());
    let final_mock_initcode = [salt.abi_encode(), mock_builder.calldata().to_vec()].concat();
    let raw_deploy = RawCallBuilder::new_raw_deploy(&provider, final_mock_initcode.into());
    raw_deploy.call_raw().await.unwrap();
    mock_tob
}
