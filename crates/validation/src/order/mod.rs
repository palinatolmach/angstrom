use std::{fmt::Debug, future::Future, pin::Pin};

use alloy::primitives::Address;
use angstrom_types::{
    orders::{OrderId, OrderOrigin},
    sol_bindings::{
        ext::RawPoolOrder,
        grouped_orders::{
            AllOrders, GroupedComposableOrder, GroupedVanillaOrder, OrderWithStorageData
        },
        rpc_orders::TopOfBlockOrder
    }
};
use angstrom_utils::GenericExt;
use reth_primitives::B256;
use sim::SimValidation;
use state::account::user::UserAddress;
use tokio::sync::oneshot::{channel, Sender};
use crate::BlockStateProviderFactory;
use crate::{validator::ValidationRequest, BlockStateProviderFactory};

pub mod order_validator;
pub mod sim;
pub mod state;

use crate::validator::ValidationClient;

pub type ValidationFuture<'a> =
    Pin<Box<dyn Future<Output = OrderValidationResults> + Send + Sync + 'a>>;

pub type ValidationsFuture<'a> =
    Pin<Box<dyn Future<Output = Vec<OrderValidationResults>> + Send + Sync + 'a>>;

pub enum OrderValidationRequest {
    ValidateOrder(Sender<OrderValidationResults>, AllOrders, OrderOrigin)
}

/// TODO: not a fan of all the conversions. can def simplify
impl From<OrderValidationRequest> for OrderValidation {
    fn from(value: OrderValidationRequest) -> Self {
        match value {
            OrderValidationRequest::ValidateOrder(tx, order, orign) => match order {
                AllOrders::Standing(p) => {
                    // TODO: check hook data and deal with composable
                    // if p.hook_data.is_empty() {
                    OrderValidation::Limit(tx, GroupedVanillaOrder::Standing(p), orign)
                    // } else {
                    //
                    //     OrderValidation::LimitComposable(
                    //         tx,
                    //         GroupedComposableOrder::Partial(p),
                    //         orign
                    //     )
                    // }
                }
                AllOrders::Flash(kof) => {
                    // TODO: check hook data and deal with composable
                    // if kof.hook_data.is_empty() {
                    OrderValidation::Limit(tx, GroupedVanillaOrder::KillOrFill(kof), orign)
                    // } else {
                    //     OrderValidation::LimitComposable(
                    //         tx,
                    //         GroupedComposableOrder::KillOrFill(kof),
                    //         orign
                    //     )
                    // }
                }
                AllOrders::TOB(tob) => OrderValidation::Searcher(tx, tob, orign)
            }
        }
    }
}

pub enum ValidationMessage {
    ValidationResults(OrderValidationResults)
}

#[derive(Debug, Clone)]
pub enum OrderValidationResults {
    Valid(OrderWithStorageData<AllOrders>),
    // the raw hash to be removed
    Invalid(B256),
    TransitionedToBlock
}

impl OrderValidationResults {
    pub fn add_gas_cost_or_invalidate<DB>(&mut self, sim: &SimValidation<DB>, is_limit: bool)
    where
        DB: BlockStateProviderFactory + Unpin + Clone + 'static + revm::DatabaseRef
    {
        if let Self::Valid(order) = self {
            if is_limit {
                let mut order = order
                    .try_map_inner(|order| match order {
                        AllOrders::Standing(s) => Ok(GroupedVanillaOrder::Standing(s)),
                        AllOrders::Flash(f) => Ok(GroupedVanillaOrder::KillOrFill(f)),
                        _ => unreachable!()
                    })
                    .unwrap();

                if let Ok(gas_used) = sim.calculate_user_gas(&order) {
                    order.priority_data.gas += gas_used as u128;
                } else {
                    let order_hash = order.order_hash();
                    *self = OrderValidationResults::Invalid(order_hash);
                }
            } else {
                let mut order = order
                    .try_map_inner(|order| match order {
                        AllOrders::TOB(s) => Ok(s),
                        _ => unreachable!()
                    })
                    .unwrap();
                if let Ok(gas_used) = sim.calculate_tob_gas(&order) {
                    order.priority_data.gas += gas_used as u128;
                } else {
                    let order_hash = order.order_hash();
                    *self = OrderValidationResults::Invalid(order_hash);
                }
            }
        }
    }
}

pub enum OrderValidation {
    Limit(Sender<OrderValidationResults>, GroupedVanillaOrder, OrderOrigin),
    LimitComposable(Sender<OrderValidationResults>, GroupedComposableOrder, OrderOrigin),
    Searcher(Sender<OrderValidationResults>, TopOfBlockOrder, OrderOrigin)
}
impl OrderValidation {
    pub fn user(&self) -> Address {
        match &self {
            Self::Searcher(_, u, _) => u.from(),
            Self::LimitComposable(_, u, _) => u.from(),
            Self::Limit(_, u, _) => u.from()
        }
    }
}

/// Provides support for validating transaction at any given state of the chain
pub trait OrderValidatorHandle: Send + Sync + Clone + Debug + Unpin + 'static {
    /// The order type of the limit order pool
    type Order: Send + Sync;

    fn validate_order(&self, origin: OrderOrigin, transaction: Self::Order) -> ValidationFuture;

    /// Validates a batch of orders.
    ///
    /// Must return all outcomes for the given orders in the same order.
    fn validate_orders(&self, transactions: Vec<(OrderOrigin, Self::Order)>) -> ValidationsFuture {
        Box::pin(futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_order(origin, tx))
        ))
    }

    /// orders that are either expired or have been filled.
    fn new_block(
        &self,
        block_number: u64,
        completed_orders: Vec<B256>,
        addresses: Vec<Address>
    ) -> ValidationFuture;
}

impl OrderValidatorHandle for ValidationClient {
    type Order = AllOrders;

    fn new_block(
        &self,
        block_number: u64,
        orders: Vec<B256>,
        addresses: Vec<Address>
    ) -> ValidationFuture {
        Box::pin(async move {
            let (tx, rx) = channel();
            let _ = self.0.send(ValidationRequest::NewBlock {
                sender: tx,
                block_number,
                orders,
                addresses
            });

            rx.await.unwrap()
        })
    }

    fn validate_order(&self, origin: OrderOrigin, transaction: Self::Order) -> ValidationFuture {
        Box::pin(async move {
            let (tx, rx) = channel();
            let _ = self
                .0
                .send(ValidationRequest::Order(OrderValidationRequest::ValidateOrder(
                    tx,
                    transaction,
                    origin
                )));

            rx.await.unwrap()
        })
    }
}
