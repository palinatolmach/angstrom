use alloy::{primitives::Signed, sol};
use pade_macro::{PadeDecode, PadeEncode};

sol! {
    type PoolId is bytes32;

    #[derive(PadeEncode, Debug)]
    struct Asset {
        address addr;
        uint128 borrow;
        uint128 save;
        uint128 settle;
    }

    #[derive(PadeEncode, Debug)]
    struct Pair {
        uint16 index0;
        uint16 index1;
        uint16 store_index;
        uint256 price_1over0;
    }
}

#[derive(Debug, PadeEncode, PadeDecode)]
pub enum RewardsUpdate {
    MultiTick { start_tick: Signed<24, 1>, start_liquidity: u128, quantities: Vec<u128> },
    CurrentOnly { amount: u128 }
}

#[derive(Debug, PadeEncode)]
pub struct PoolUpdate {
    pub zero_for_one:     bool,
    pub pair_index:       u16,
    pub swap_in_quantity: u128,
    pub rewards_update:   RewardsUpdate
}

#[derive(PadeEncode, Debug)]
pub struct MockContractMessage {
    pub assets: Vec<Asset>,
    pub pairs:  Vec<Pair>,
    pub update: PoolUpdate
}
