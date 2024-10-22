use std::{
    collections::{HashMap, VecDeque},
    sync::Arc
};

use alloy::primitives::Address;
use angstrom_types::{pair_with_price::PairsWithPrice, primitive::PoolId};
use matching_engine::cfmm::uniswap::pool_manager::UniswapPoolManager;

/// The token price generator gives us the avg instantaneous price of the last 5
/// blocks of the underlying V4 pool. This is then used in order to convert the
/// gas used from eth to token0 of the pool the user is swapping over.
pub struct TokenPriceGenerator<Provider> {
    /// stores the last N amount of prices. TODO: (Address, Address) -> PoolKey
    /// once plamen updates.
    prev_prices: VecDeque<HashMap<(Address, Address), PairsWithPrice>>,
    uni:         Arc<UniswapPoolManager<Provider>>
}

impl<Provider> TokenPriceGenerator<Provider> {
    /// is a bit of a pain as we need todo a look-back in-order to grab last 5
    /// blocks.
    pub async fn new(
        active_pairs: Vec<PoolId>,
        uni: &UniswapPoolManager<Provider>
    ) -> eyre::Result<Self> {
        todo!()
    }

    fn get_best_eth_pair(&self, token0: Address, token1: Address) -> PairsWithPrice {
        todo!()
    }
}
