mod origin;
use std::fmt;

mod validation;
use std::fmt::Debug;

use alloy_primitives::{Address, Bytes, TxHash, U128, U256};
pub use origin::*;
pub use validation::*;

pub trait PooledOrder: fmt::Debug + Send + Sync + Clone + Unpin + 'static {
    type ValidationData: Send + Debug + Sync + Clone + Unpin + 'static;

    /// Hash of the order
    fn hash(&self) -> TxHash;

    fn order_id(&self) -> OrderId;

    /// The order signer
    fn from(&self) -> Address;

    /// Transaction nonce
    fn nonce(&self) -> U256;

    /// Amount of tokens to sell
    fn amount_in(&self) -> u128;

    /// Min amount of tokens to buy
    fn amount_out_min(&self) -> u128;

    /// Limit Price
    fn limit_price(&self) -> u128;

    /// Order deadline
    fn deadline(&self) -> U256;

    /// Returns a measurement of the heap usage of this type and all its
    /// internals.
    fn size(&self) -> usize;

    /// Returns the length of the rlp encoded transaction object
    fn encoded_length(&self) -> usize;

    /// Returns chain_id
    fn chain_id(&self) -> Option<u64>;

    /// Returns if the order should be pending or parked
    fn is_valid(&self) -> bool;

    /// Returns the direction of the pool defined by ordering
    fn is_bid(&self) -> bool;
}

pub trait PooledComposableOrder: PooledOrder {
    fn pre_hook(&self) -> Option<Bytes>;

    fn post_hook(&self) -> Option<Bytes>;
}
