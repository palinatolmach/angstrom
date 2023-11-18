use std::collections::HashMap;

use super::{pending::PendingPool, LimitOrderLocation, LimitPoolError};
use crate::{
    common::{BidAndAsks, OrderId, PoolId},
    PooledComposableOrder, PooledLimitOrder
};

pub struct ComposableLimitPool<T: PooledComposableOrder + PooledLimitOrder>(
    HashMap<PoolId, PendingPool<T>>
);

impl<T: PooledComposableOrder + PooledLimitOrder> ComposableLimitPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        let id = order.order_id();
        self.0
            .get_mut(&id.pool_id)
            .map(|pool| pool.new_order(order))
            .ok_or_else(|| LimitPoolError::NoPool(id.pool_id))
    }

    pub fn remove_order(&mut self, tx_id: &OrderId) -> Option<T> {
        self.0.get_mut(&tx_id.pool_id)?.remove_order(tx_id.hash)
    }

    pub fn fetch_all_pool_orders(&self, id: &PoolId) -> Vec<&T> {
        self.0
            .get(id)
            .map(|inner| inner.fetch_all_orders())
            .unwrap()
    }

    pub fn fetch_all_pool_bids(&self, id: &PoolId) -> Vec<&T> {
        self.0.get(id).map(|inner| inner.fetch_all_bids()).unwrap()
    }

    pub fn fetch_all_pool_asks(&self, id: &PoolId) -> Vec<&T> {
        self.0.get(id).map(|inner| inner.fetch_all_asks()).unwrap()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_pool_intersection(&self, id: &PoolId) -> BidAndAsks<T> {
        self.0
            .get(id)
            .map(|inner| inner.fetch_intersection())
            .unwrap()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_pool_intersection_with_buffer(&self, _buffer: u8) -> BidAndAsks<T> {
        todo!("Blocked until added tick impl")
    }

    pub fn fetch_all_orders(&self) -> Vec<Vec<&T>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_orders())
            .collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<Vec<&T>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_bids())
            .collect()
    }

    pub fn fetch_all_asks(&self) -> Vec<Vec<&T>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_asks())
            .collect()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> Vec<BidAndAsks<T>> {
        self.0
            .values()
            .map(|inner| inner.fetch_intersection())
            .collect()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(&self, _buffer: u8) -> Vec<BidAndAsks<T>> {
        todo!("Blocked until added tick impl")
    }
}
