use alloy::primitives::{Address, U256};

use crate::contract_payloads::angstrom::AngstromBundle;

/// represents the price settled on angstrom between two tokens
pub struct PairsWithPrice {
    pub token0:         Address,
    pub token1:         Address,
    pub price_1_over_0: U256
}

impl PairsWithPrice {
    /// Decodes the AngstromPayload bundle and allows us to checkout
    /// the prices that the pools settled at. We then can use this for things
    /// such as our eth -> erc-20 gas price calculator
    pub fn from_angstrom_bundle(bundle: &AngstromBundle) -> Vec<Self> {
        bundle
            .pairs
            .iter()
            .map(|pair| Self {
                token0:         bundle.assets[pair.index0 as usize].addr,
                token1:         bundle.assets[pair.index1 as usize].addr,
                price_1_over_0: pair.price_1over0
            })
            .collect::<Vec<_>>()
    }
}
