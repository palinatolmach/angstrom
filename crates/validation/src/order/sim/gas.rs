use std::{collections::HashMap, sync::Arc};

use alloy::primitives::Address;
use angstrom_types::sol_bindings::{
    grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
    rpc_orders::TopOfBlockOrder
};
use reth_primitives::transaction::FillTxEnv;
use revm::{
    db::WrapDatabaseRef, handler::register::EvmHandler, inspector_handle_register,
    interpreter::Gas, primitives::{EnvWithHandlerCfg, TxEnv}, Evm
};

use super::gas_inspector::{GasSimulationInspector, GasUsed};
use crate::{BlockStateProviderFactory, RevmLRU};

/// deals with the calculation of gas for a given type of order.
/// user orders and tob orders take different paths and are different size and
/// as such, pay different amount of gas in order to execute.
/// The calculation is done by this pc offset inspector which captures the
/// specific PC offsets of the code we want the user to pay for specifically.
/// Once the bundle has been built. We simulate the bundle and then calculate
/// the shared gas by using the simple formula:
/// (Bundle execution cost - Sum(Orders Gas payed)) / len(Orders)
pub struct OrderGasCalculations<DB> {
    db:               Arc<RevmLRU<DB>>,
    angstrom_address: Address,
}

impl<DB> OrderGasCalculations<DB>
where
    DB: BlockStateProviderFactory + Unpin + Clone + 'static
{
    pub fn new(db: Arc<RevmLRU<DB>>, angstrom_address: Address) -> Self {
        Self { db, angstrom_address }
    }


    fn execute_on_revm<F>(
        &self,
        offsets: &HashMap<usize, usize>,
        f: F
    ) -> Result<GasUsed, GasSimulationError>
    where
        F: FnOnce(&mut EnvWithHandlerCfg)
    {
        let mut inspector = GasSimulationInspector::new(self.angstrom_address, offsets);
        let mut evm_handler = EnvWithHandlerCfg::default();

        f(&mut evm_handler);

        let mut evm = revm::Evm::builder()
            .with_ref_db(self.db.clone())
            .with_external_context(&mut inspector)
            .with_env_with_handler_cfg(evm_handler)
            .append_handler_register(inspector_handle_register)
            .modify_env(|env| {
                env.cfg.disable_balance_check = true;
            })
            .build();

        let result = evm.transact()?;

        if !result.result.is_success() {
            return Err(eyre::eyre!(
                "gas simulation had a revert. cannot guarantee the proper gas was estimated"
            ))
        }

        Ok(inspector.into_gas_used())
    }

    pub fn gas_of_tob_order(
        &self,
        tob: &OrderWithStorageData<TopOfBlockOrder>
    ) -> Result<GasUsed, GasSimulationError> {
        self.execute_on_revm(&HashMap::default(), |execution_env| {
        })
    }

    pub fn gas_of_book_order(
        &self,
        order: &OrderWithStorageData<GroupedVanillaOrder>
    ) -> Result<GasUsed, GasSimulationError> {
        self.execute_on_revm(&HashMap::default(), |execution_env| {
            execution_env.env.tx.data = 
            // execution_env.env.
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GasSimulationError {
    #[error("Transaction Reverted")]
    TransactionReverted
}
