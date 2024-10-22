use std::collections::{HashMap, VecDeque};

use alloy::primitives::Address;
use angstrom_types::pair_with_price::PairsWithPrice;

/// The token price generator gives us the avg instantaneous price of the last 5
/// blocks of the underlying V4 pool. This is then used in order to convert the
/// gas used from eth to token0 of the pool the user is swapping over.
pub struct TokenPriceGenerator {
    /// stores the last N amount of prices
    prev_prices: VecDeque<HashMap<(Address, Address), PairsWithPrice>>
}

impl TokenPriceGenerator {
    /// is a bit of a pain as we need todo a look-back in-order to grab last 5
    /// blocks.
    pub async fn new() -> eyre::Result<Self> {
        todo!()
    }
}
