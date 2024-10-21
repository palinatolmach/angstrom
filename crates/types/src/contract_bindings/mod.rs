#[rustfmt::skip]
pub mod pool_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolManager,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/PoolManager.sol/PoolManager.json"
    );
}

#[rustfmt::skip]
pub mod mock_rewards_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockRewardsManager,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/MockRewardsManager.sol/MockRewardsManager.json"
    );
}

#[rustfmt::skip]
pub mod angstrom {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        Angstrom,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/Angstrom.sol/Angstrom.json"
    );
}

#[rustfmt::skip]
pub mod pool_gate {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolGate,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/PoolGate.sol/PoolGate.json"
    );
}

#[rustfmt::skip]
pub mod mintable_mock_erc_20 {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MintableMockERC20,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/MintableMockERC20.sol/MintableMockERC20.json"
    );
}

