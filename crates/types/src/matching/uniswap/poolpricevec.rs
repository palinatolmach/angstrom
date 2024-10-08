use std::cmp::Ordering;

use alloy::primitives::{Uint, U256};
use eyre::OptionExt;
use uniswap_v3_math::sqrt_price_math::{
    _get_amount_0_delta, _get_amount_1_delta, get_next_sqrt_price_from_input,
    get_next_sqrt_price_from_output
};

use super::{poolprice::PoolPrice, Direction};
use crate::{
    matching::{Ray, SqrtPriceX96},
    orders::OrderPrice
};

#[derive(Clone, Debug)]
pub struct PoolPriceVec<'a> {
    pub start_bound: PoolPrice<'a>,
    pub end_bound:   PoolPrice<'a>,
    pub d_t0:        U256,
    pub d_t1:        U256
}

impl<'a> PoolPriceVec<'a> {
    pub fn new(start_bound: PoolPrice<'a>, end_bound: PoolPrice<'a>) -> Self {
        let (d_t0, d_t1) =
            Self::delta_to_price(start_bound.price, end_bound.price, start_bound.liquidity());
        Self { start_bound, end_bound, d_t0, d_t1 }
    }

    fn delta(start: PoolPrice<'a>, end: PoolPrice<'a>) -> eyre::Result<(U256, U256)> {
        let mut cur_price = start.price;
        let mut cur_range = start.market_pool;
        let end_price = end.price;
        let mut total_dt0 = U256::ZERO;
        let mut total_dt1 = U256::ZERO;
        while cur_price != end_price {
            let range_bound = SqrtPriceX96::at_tick(cur_range.upper_tick)?;
            let target_price = if range_bound < end_price {
                cur_range = cur_range
                    .next(Direction::BuyingT0)
                    .ok_or_eyre("Unable to get next range")?;
                range_bound
            } else {
                end_price
            };
            let (d_t0, d_t1) = Self::delta_to_price(cur_price, target_price, cur_range.liquidity);
            total_dt0 += d_t0;
            total_dt1 += d_t1;
            cur_price = target_price;
        }
        Ok((total_dt0, total_dt1))
    }

    pub fn to_price(&self, target: SqrtPriceX96) -> Option<Self> {
        let (start_in_bounds, end_in_bounds) = if self.is_buy() {
            (Ordering::Greater, Ordering::Less)
        } else {
            (Ordering::Less, Ordering::Greater)
        };
        if self.start_bound.price.cmp(&target) == start_in_bounds {
            if self.end_bound.price.cmp(&target) == end_in_bounds {
                // If the target price is between the start and end bounds, make a subvec
                let new_upper = self.start_bound.market_pool.market.at_price(target).ok()?;
                Some(Self::new(self.start_bound.clone(), new_upper))
            } else {
                // If the target price is beyond the end bound in the appropriate direction,
                // return a copy of this existing vector
                Some(self.clone())
            }
        } else {
            // If the target price is equal to or beyond the start price in an inappropriate
            // direction, there is no vector to be made
            None
        }
    }

    fn delta_to_price(
        start_price: SqrtPriceX96,
        end_price: SqrtPriceX96,
        liquidity: u128
    ) -> (U256, U256) {
        let sqrt_ratio_a_x_96 = start_price.into();
        let sqrt_ratio_b_x_96 = end_price.into();
        let d_t0 = _get_amount_0_delta(sqrt_ratio_a_x_96, sqrt_ratio_b_x_96, liquidity, false)
            .unwrap_or(Uint::from(0));
        let d_t1 = _get_amount_1_delta(sqrt_ratio_a_x_96, sqrt_ratio_b_x_96, liquidity, false)
            .unwrap_or(Uint::from(0));
        (d_t0, d_t1)
    }

    pub fn is_buy(&self) -> bool {
        self.start_bound.price < self.end_bound.price
    }

    /// Returns `(quantity, price)`
    pub fn quantity(&self, target_price: OrderPrice) -> (U256, U256) {
        let t: SqrtPriceX96 = Ray::from(*target_price).into();

        // If our target price is past our end bound, our quantity is the entire range
        if self.is_buy() && t > self.end_bound.price {
            return (self.d_t0, self.d_t1);
        } else if t < self.end_bound.price {
            return (self.d_t0, self.d_t1);
        }

        let (quantity, price) =
            Self::delta_to_price(self.start_bound.price, t, self.start_bound.liquidity());
        (quantity, price)
    }

    // Maybe it's OK that I don't check the price again here because in the matching
    // algo I've only offered a quantity bounded by the price, so we should
    // always be OK?
    pub fn fill(&self, quantity: U256) -> Self {
        let liquidity = self.start_bound.liquidity();
        let end_sqrt_price = if self.is_buy() {
            get_next_sqrt_price_from_output(
                self.start_bound.price.into(),
                liquidity,
                quantity,
                true
            )
            .map(SqrtPriceX96::from)
            .unwrap()
        } else {
            get_next_sqrt_price_from_input(self.start_bound.price.into(), liquidity, quantity, true)
                .map(SqrtPriceX96::from)
                .unwrap()
        };
        let (d_t0, d_t1) = Self::delta_to_price(self.start_bound.price, end_sqrt_price, liquidity);
        let mut end_bound = self.start_bound.clone();
        end_bound.price = end_sqrt_price;
        Self { end_bound, start_bound: self.start_bound.clone(), d_t0, d_t1 }
    }
}
