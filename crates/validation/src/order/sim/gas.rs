use std::{collections::HashMap, sync::Arc};

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, U256},
    providers::{
        builder,
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, IpcConnect, RootProvider
    },
    pubsub::PubSubFrontend,
    signers::local::PrivateKeySigner,
    sol_types::{SolCall, SolValue}
};
use angstrom_types::{
    contract_bindings::angstrom::Angstrom::Overflow,
    contract_payloads::angstrom::AngstromBundle,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder,
        RawPoolOrder
    }
};
use eyre::eyre;
use pade::PadeEncode;
use reth_errors::RethError;
use reth_primitives::{keccak256, transaction::FillTxEnv, TxKind};
use revm::{
    db::{CacheDB, WrapDatabaseRef},
    handler::register::{EvmHandler, HandleRegister},
    inspector_handle_register,
    interpreter::Gas,
    primitives::{AccountInfo, EnvWithHandlerCfg, ResultAndState, TxEnv},
    DatabaseRef, Evm
};

use super::gas_inspector::{GasSimulationInspector, GasUsed};
use crate::BlockStateProviderFactory;

/// A address we can use to deploy contracts
const DEFAULT_FROM: Address =
    alloy::primitives::address!("aa250d5630b4cf539739df2c5dacb4c659f2488d");

/// deals with the calculation of gas for a given type of order.
/// user orders and tob orders take different paths and are different size and
/// as such, pay different amount of gas in order to execute.
/// The calculation is done by this pc offset inspector which captures the
/// specific PC offsets of the code we want the user to pay for specifically.
/// Once the bundle has been built. We simulate the bundle and then calculate
/// the shared gas by using the simple formula:
/// (Bundle execution cost - Sum(Orders Gas payed)) / len(Orders)
#[derive(Clone)]
pub struct OrderGasCalculations<DB> {
    db:               CacheDB<Arc<DB>>,
    // the deployed addresses in cache_db
    angstrom_address: Address,
    uniswap_address:  Address
}

impl<DB> OrderGasCalculations<DB>
where
    DB: Unpin + Clone + 'static + revm::DatabaseRef,
    <DB as revm::DatabaseRef>::Error: Send + Sync
{
    pub fn new(db: Arc<DB>) -> eyre::Result<Self> {
        let ConfiguredRevm { db, uni_swap, angstrom } =
            Self::setup_revm_cache_database_for_simulation(db)?;

        Ok(Self { db, uniswap_address: uni_swap, angstrom_address: angstrom })
    }

    pub fn gas_of_tob_order(
        &self,
        tob: &OrderWithStorageData<TopOfBlockOrder>
    ) -> eyre::Result<GasUsed> {
        self.execute_on_revm(
            &HashMap::default(),
            OverridesForTestAngstrom {
                amount_in:    U256::from(tob.amount_in()),
                amount_out:   U256::from(tob.amount_out_min()),
                token_out:    tob.token_out(),
                token_in:     tob.token_in(),
                user_address: tob.from()
            },
            |execution_env| {
                let bundle = AngstromBundle::build_dummy_for_tob_gas(tob)
                    .unwrap()
                    .pade_encode();

                let tx = &mut execution_env.tx;
                tx.caller = DEFAULT_FROM;
                tx.transact_to = TxKind::Call(self.angstrom_address);
                tx.data = angstrom_types::contract_bindings::angstrom::Angstrom::executeCall::new(
                    (bundle.into(),)
                )
                .abi_encode()
                .into();
                tx.value = U256::from(0);
                tx.nonce = None;
            }
        )
    }

    pub fn gas_of_book_order(
        &self,
        order: &OrderWithStorageData<GroupedVanillaOrder>
    ) -> eyre::Result<GasUsed> {
        self.execute_on_revm(
            &HashMap::default(),
            OverridesForTestAngstrom {
                amount_in:    U256::from(order.amount_in()),
                amount_out:   U256::from(order.amount_out_min()),
                token_out:    order.token_out(),
                token_in:     order.token_in(),
                user_address: order.from()
            },
            |execution_env| {
                let bundle = AngstromBundle::build_dummy_for_user_gas(order)
                    .unwrap()
                    .pade_encode();

                let tx = &mut execution_env.tx;
                tx.caller = DEFAULT_FROM;
                tx.transact_to = TxKind::Call(self.angstrom_address);
                tx.data = angstrom_types::contract_bindings::angstrom::Angstrom::executeCall::new(
                    (bundle.into(),)
                )
                .abi_encode()
                .into();
                tx.value = U256::from(0);
                tx.nonce = None;
            }
        )
    }

    fn execute_with_db<D: DatabaseRef, F>(db: D, f: F) -> eyre::Result<(ResultAndState, D)>
    where
        F: FnOnce(&mut TxEnv),
        <D as revm::DatabaseRef>::Error: Send + Sync
    {
        let evm_handler = EnvWithHandlerCfg::default();
        let mut revm_sim = revm::Evm::builder()
            .with_ref_db(db)
            .with_env_with_handler_cfg(evm_handler)
            .modify_env(|env| {
                env.cfg.disable_balance_check = true;
                env.cfg.disable_block_gas_limit = true;
            })
            .modify_tx_env(f)
            .build();

        let Ok(out) = revm_sim.transact() else {
            return Err(eyre!("failed to transact transaction"))
        };
        let (cache_db, _) = revm_sim.into_db_and_env_with_handler_cfg();
        Ok((out, cache_db.0))
    }

    /// deploys angstrom + univ4 and then sets DEFAULT_FROM address as a node in
    /// the network.
    fn setup_revm_cache_database_for_simulation(db: Arc<DB>) -> eyre::Result<ConfiguredRevm<DB>> {
        let mut cache_db = CacheDB::new(db.clone());

        let (out, cache_db) = Self::execute_with_db(cache_db, |tx| {
            tx.transact_to = TxKind::Create;
            tx.caller = DEFAULT_FROM;
            tx.data =
                angstrom_types::contract_bindings::pool_manager::PoolManager::BYTECODE.clone();
            tx.value = U256::from(0);
        })?;

        if !out.result.is_success() {
            eyre::bail!("failed to deploy uniswap v4 pool manager");
        }
        let v4_address = Address::from_slice(out.result.output().unwrap());

        // deploy angstrom.

        let mut angstrom_raw_bytecode =
            angstrom_types::contract_bindings::angstrom::Angstrom::BYTECODE.clone();

        // in solidity when deploying. constructor args are appended to the end of the
        // bytecode.
        let constructor_args = (v4_address, DEFAULT_FROM, DEFAULT_FROM).abi_encode().into();
        let data = [angstrom_raw_bytecode, constructor_args].concat();

        let (out, cache_db) = Self::execute_with_db(cache_db, |tx| {
            tx.transact_to = TxKind::Create;
            tx.caller = DEFAULT_FROM;
            tx.data = data.into();
            tx.value = U256::from(0);
        })?;

        if !out.result.is_success() {
            eyre::bail!("failed to deploy angstrom");
        }
        let angstrom_address = Address::from_slice(out.result.output().unwrap());

        // enable default from to call the angstrom contract.
        let (out, mut cache_db) = Self::execute_with_db(cache_db, |tx| {
            tx.transact_to = TxKind::Call(angstrom_address);
            tx.caller = DEFAULT_FROM;
            tx.data = angstrom_types::contract_bindings::angstrom::Angstrom::toggleNodesCall::new(
                (vec![DEFAULT_FROM],)
            )
            .abi_encode()
            .into();

            tx.value = U256::from(0);
        })?;

        if !out.result.is_success() {
            eyre::bail!("failed to set default from address as node on angstrom");
        }
        Ok(ConfiguredRevm { db: cache_db, angstrom: angstrom_address, uni_swap: v4_address })
    }

    fn fetch_db_with_overrides(
        &self,
        overrides: OverridesForTestAngstrom
    ) -> eyre::Result<CacheDB<Arc<DB>>> {
        // fork db
        let mut cache_db = self.db.clone();

        // change approval of token in and then balance of token out
        let OverridesForTestAngstrom { user_address, amount_in, amount_out, token_in, token_out } =
            overrides;
        // for the first 10 slots, we just force override everything to balance. because
        // of the way storage slots work in solidity. this shouldn't effect
        // anything
        // https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html
        for i in 0..10 {
            let balance_amount_out_slot = keccak256((self.angstrom_address, i).abi_encode());

            //keccak256(angstrom . keccak256(user . idx)))
            let approval_slot = keccak256(
                (self.angstrom_address, keccak256((user_address, i).abi_encode())).abi_encode()
            );

            cache_db
                .insert_account_storage(token_out, balance_amount_out_slot.into(), amount_out)
                .map_err(|_| eyre!("failed to insert account into storage"))?;
            cache_db
                .insert_account_storage(token_in, approval_slot.into(), amount_in)
                .map_err(|_| eyre!("failed to insert account into storage"))?;
        }

        Ok(cache_db)
    }

    fn execute_on_revm<F>(
        &self,
        offsets: &HashMap<usize, usize>,
        overrides: OverridesForTestAngstrom,
        f: F
    ) -> eyre::Result<GasUsed>
    where
        F: FnOnce(&mut EnvWithHandlerCfg)
    {
        let mut inspector = GasSimulationInspector::new(self.angstrom_address, offsets);
        let mut evm_handler = EnvWithHandlerCfg::default();

        f(&mut evm_handler);

        {
            let mut evm = revm::Evm::builder()
                .with_ref_db(self.fetch_db_with_overrides(overrides)?)
                .with_external_context(&mut inspector)
                .with_env_with_handler_cfg(evm_handler)
                .append_handler_register(inspector_handle_register)
                .modify_env(|env| {
                    env.cfg.disable_balance_check = true;
                })
                .build();

            let result = evm
                .transact()
                .map_err(|_| eyre!("failed to transact with revm"))?;

            if !result.result.is_success() {
                return Err(eyre::eyre!(
                    "gas simulation had a revert. cannot guarantee the proper gas was estimated"
                ))
            }
        }

        Ok(inspector.into_gas_used())
    }
}

struct ConfiguredRevm<DB> {
    pub uni_swap: Address,
    pub angstrom: Address,
    pub db:       CacheDB<Arc<DB>>
}

struct OverridesForTestAngstrom {
    pub user_address: Address,
    pub amount_in:    U256,
    pub amount_out:   U256,
    pub token_in:     Address,
    pub token_out:    Address
}
