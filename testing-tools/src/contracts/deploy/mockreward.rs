use alloy::{
    contract::RawCallBuilder,
    primitives::{Address, U160},
    sol_types::SolValue
};
use angstrom_types::contract_bindings::mockrewardsmanager::MockRewardsManager;

use super::{mine_address, DEFAULT_CREATE2_FACTORY};

pub async fn deploy_mock_rewards_manager<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    pool_manager: Address,
    controller: Address
) -> Address {
    deploy_mock_rewards_manager_with_factory(
        provider,
        pool_manager,
        DEFAULT_CREATE2_FACTORY,
        controller
    )
    .await
}

pub async fn deploy_mock_rewards_manager_with_factory<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    pool_manager: Address,
    factory: Address,
    controller: Address
) -> Address {
    // Setup our flags and mask
    // Flags for our MockRewardsManager address
    let before_swap = U160::from(1_u8) << 7;
    let before_initialize = U160::from(1_u8) << 13;
    let before_add_liquidity = U160::from(1_u8) << 11;
    let before_remove_liquidity = U160::from(1_u8) << 9;

    let flags = before_swap | before_initialize | before_add_liquidity | before_remove_liquidity;
    let mask: U160 = (U160::from(1_u8) << 14) - U160::from(1_u8);

    let mock_builder = MockRewardsManager::deploy_builder(&provider, pool_manager, controller);
    let (mock_tob_address, salt) = mine_address(flags, mask, mock_builder.calldata());
    let final_mock_initcode = [salt.abi_encode(), mock_builder.calldata().to_vec()].concat();
    RawCallBuilder::new_raw(&provider, final_mock_initcode.into())
        .to(factory)
        .gas(10_000_000_u128)
        .send()
        .await
        .unwrap()
        .watch()
        .await
        .unwrap();
    mock_tob_address
}
