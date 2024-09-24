use alloy::primitives::{FixedBytes, Log};
use reth_primitives::Address;

use crate::contract_bindings::poolmanager::PoolManager::Initialize;

pub type PoolId = FixedBytes<32>;

pub type PoolIdWithDirection = (bool, PoolId);

/// just a placeholder type so i can implement the general architecture
#[derive(Debug, Clone, Copy)]
pub struct NewInitializedPool {
    pub currency_in:  Address,
    pub currency_out: Address,
    pub id:           PoolIdWithDirection
}

impl From<Log<Initialize>> for NewInitializedPool {
    fn from(value: Log<Initialize>) -> Self {
        // tf do the indexes come from
        Self {
            currency_in:  value.currency0,
            currency_out: value.currency1,
            // where is this generated
            id:           (true, value.id)
        }
    }
}
