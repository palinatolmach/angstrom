use std::{
    cmp::{max, min},
    collections::HashMap
};

use alloy::primitives::U256;
use eyre::OptionExt;
use uniswap_v3_math::tick_math::{get_sqrt_ratio_at_tick, get_tick_at_sqrt_ratio};

use super::{liqrange::LiqRangeRef, poolpricevec::PoolPriceVec, Tick};
use crate::matching::SqrtPriceX96;

/// A PoolPrice represents a price based on the market state preserved in a
/// parent MarketSnapshot.  The PoolPrice can be moved and operated on to
/// simulate the behavior of the price if the underlying assets are bought and
/// sold.  This price will always depend on a specific PoolSnapshot so if
/// underlying parameters such as Liquidity or the decimal representation of the
/// assets were to change, you would need to procure a new PoolSnapshot and
/// new PoolPrices dependent on that.
#[derive(Clone, Debug)]
pub struct PoolPrice<'a> {
    /// Current PoolRange that the price is in
    pub(crate) market_pool: LiqRangeRef<'a>,
    /// Tick number within the current PoolRange that we're working with
    pub(crate) tick:        Tick,
    /// The ratio between Token0 and Token1
    pub(crate) price:       SqrtPriceX96
}

impl<'a> PoolPrice<'a> {
    /// Build a PriceRange from the current price to the closer of either the
    /// target price or the bound of the current liquidity pool.  Will jump
    /// to the next liquidity pool if we're currently on the edge of one
    pub fn buy_to_price(&self, target_price: SqrtPriceX96) -> Option<PoolPriceVec<'a>> {
        self.order_to_target(Some(target_price), true)
    }

    /// Build a PriceRange from the current price to the bound of the current
    /// liquidity pool.  Will jump to the next liquidity pool if we're
    /// currently on the edge of one.
    pub fn buy_to_next_bound(&self) -> Option<PoolPriceVec<'a>> {
        self.order_to_target(None, true)
    }

    /// Buy from the AMM with a quantity of the input token that exceeds the
    /// amount required to purchase the requested quantity of the output token.
    /// The excess quantity is distributed as our "bribe" to the LPs present in
    /// each tick based on our ToB distribution algorithm.
    pub fn buy_and_bribe(
        &self,
        _input: U256,
        _output: U256
    ) -> Result<(Self, HashMap<Tick, U256>), String> {
        Ok((self.clone(), HashMap::new()))
    }

    pub fn sell_to_price(&self, target_price: SqrtPriceX96) -> Option<PoolPriceVec<'a>> {
        self.order_to_target(Some(target_price), false)
    }

    pub fn sell_to_next_bound(&self) -> Option<PoolPriceVec<'a>> {
        self.order_to_target(None, false)
    }

    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn liquidity_range(&self) -> LiqRangeRef<'a> {
        self.market_pool
    }

    pub fn liquidity(&self) -> u128 {
        self.market_pool.liquidity
    }

    pub fn range_to(&self, price: SqrtPriceX96) -> eyre::Result<PoolPriceVec<'a>> {
        let tick = price.to_tick()?;
        let market_pool = self
            .market_pool
            .market
            .get_range_for_tick(tick)
            .ok_or_eyre("Unable to find pool")?;
        let target = Self { market_pool, tick, price };
        Ok(PoolPriceVec::new(self.clone(), target))
    }

    /// This will produce a Uniswap Price Range that spans from the current
    /// price to the CLOSER of the target price or the nearest liquidity
    /// pool boundary
    pub fn order_to_target(
        &self,
        target_price: Option<SqrtPriceX96>,
        buy: bool
    ) -> Option<PoolPriceVec<'a>> {
        // Bounds check our target price if provided
        if let Some(p) = target_price {
            if buy {
                // Buying from the market will raise the price, so if our target price is on the
                // wrong side of our current price, we can't do this.
                if p <= self.price {
                    return None;
                }
            } else {
                // Selling to the market will lower the price, so the same applies here
                if p >= self.price {
                    return None;
                }
            }
        }
        let bound_price = if buy {
            SqrtPriceX96::at_tick(self.market_pool.upper_tick).ok()?
        } else {
            SqrtPriceX96::at_tick(self.market_pool.lower_tick).ok()?
        };

        let mut new_range_idx = self.market_pool.range_idx;
        let mut pool = self.market_pool.range;
        let (mut tick_bound_price, next_tick) = if buy {
            let upper_tick_price = get_sqrt_ratio_at_tick(pool.upper_tick)
                .ok()
                .map(SqrtPriceX96::from)?;
            let next_tick = self.market_pool.range_idx.checked_sub(1);
            (upper_tick_price, next_tick)
        } else {
            let lower_tick_price = get_sqrt_ratio_at_tick(pool.lower_tick)
                .ok()
                .map(SqrtPriceX96::from)?;
            let next_tick = self.market_pool.range_idx.checked_add(1);
            (lower_tick_price, next_tick)
        };
        if self.price == tick_bound_price {
            // We're at the tick bound, we need to look at the next pool!
            new_range_idx = next_tick?;
            pool = self.market_pool.market.ranges.get(new_range_idx)?;
            tick_bound_price = if buy {
                get_sqrt_ratio_at_tick(pool.upper_tick)
                    .ok()
                    .map(SqrtPriceX96::from)?
            } else {
                get_sqrt_ratio_at_tick(pool.lower_tick)
                    .ok()
                    .map(SqrtPriceX96::from)?
            };
        }
        let closest_price = if let Some(p) = target_price {
            if buy {
                min(p, tick_bound_price)
            } else {
                max(p, tick_bound_price)
            }
        } else {
            tick_bound_price
        };
        let end_bound = Self {
            market_pool: LiqRangeRef { range: pool, range_idx: new_range_idx, ..self.market_pool },
            price:       closest_price,
            tick:        get_tick_at_sqrt_ratio(closest_price.into()).ok()?
        };
        Some(PoolPriceVec::new(self.clone(), end_bound))
    }

    pub fn price(&self) -> &SqrtPriceX96 {
        &self.price
    }

    pub fn as_sqrtpricex96(&self) -> SqrtPriceX96 {
        self.price
    }

    /// Return the current price as a float - we need to figure out what our
    /// price representation is going to look like overall
    pub fn as_float(&self) -> f64 {
        self.price.as_f64()
    }

    pub fn as_u256(&self) -> U256 {
        self.price.into()
    }
}
