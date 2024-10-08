mod liqrange;
mod poolprice;
mod poolpricevec;
mod poolsnapshot;

pub use liqrange::{LiqRange, LiqRangeRef};
pub use poolprice::PoolPrice;
pub use poolpricevec::PoolPriceVec;
pub use poolsnapshot::PoolSnapshot;

pub type Tick = i32;

pub enum Quantity {
    Token0(u128),
    Token1(u128)
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    /// When buying T0, the price will go up and the tick number will increase
    BuyingT0,
    /// When selling T0, the price will go down and the tick number will
    /// decrease
    SellingT0
}

impl Direction {
    pub fn from_is_bid(is_bid: bool) -> Self {
        match is_bid {
            true => Self::BuyingT0,
            false => Self::SellingT0
        }
    }

    pub fn is_bid(&self) -> bool {
        matches!(self, Self::BuyingT0)
    }
}

/*
A Uniswap pool is a relation between Token0 and Token1.  The price is defined as Token1/Token0.  We might use terms like
'buy' and 'sell' in here, those terms are in the context of Token0 where "buy" is input Token1 to get Token0
out and "sell" is input Token0 to get Token1 out
 */
