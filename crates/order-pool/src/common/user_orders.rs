use alloy_primitives::{Address, B160, U256};
use guard_types::orders::{OrderId, PooledOrder, ValidatedOrder};
use revm::primitives::HashMap;

use crate::inner::PoolError;

/// the sum of all pending orders for a given user. This is done
/// so that validation of specific orders is not dependant on all other orders.
pub struct PendingState {
    token_balances:  HashMap<B160, U256>,
    token_approvals: HashMap<B160, U256>
}

pub struct UserOrders(HashMap<Address, (PendingState, Vec<OrderId>)>);

impl UserOrders {
    pub fn new_order<O: PooledOrder, Data>(
        &mut self,
        order: ValidatedOrder<O, Data>
    ) -> Result<(), PoolError> {
        let id: OrderId = order.order_id();
        let _ = self.check_for_nonce_overlap(&id)?;

        let user = id.address;


        Ok(())
    }

    fn apply_new_order_deltas(
        &mut self,
        token_in: B160,
        amount_in: B160,
        state: &mut PendingState
    ) -> Result<(), ()> {
        Ok(())
    }

    /// Helper function for checking for duplicates when adding orders
    fn check_for_nonce_overlap(&self, id: &OrderId) -> Result<(), PoolError> {
        if self
            .0
            .get(&id.address)
            .map(|inner| inner.1.iter().any(|other_id| other_id.nonce == id.nonce))
            .unwrap_or(false)
        {
            return Err(PoolError::DuplicateNonce(id.clone()))
        }

        Ok(())
    }
}
