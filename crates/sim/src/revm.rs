use std::{
    collections::HashMap,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_core::{abi::Bytes, types::Address};
use futures_util::{stream::FuturesUnordered, Future, FutureExt, StreamExt};
use revm_primitives::{Bytecode, B160};
use tokio::{runtime::Handle, sync::mpsc::UnboundedReceiver, task::JoinHandle};

use crate::{
    errors::{SimError, SimResult},
    executor::{TaskKind, ThreadPool},
    lru_db::RevmLRU,
    slot_keeper::SlotKeeper,
    state::{AddressSlots, RevmState},
    SimEvent
};

///TODO: replace once settled
const V4_BYTE_CODE: Bytes = vec![];
///TODO: replace once settled
const ANGSTROM_ADDRESS: B160 = B160::zero();

/// revm state handler
pub struct Revm {
    transaction_rx: UnboundedReceiver<SimEvent>,
    threadpool:     ThreadPool,
    state:          Arc<RevmState>,
    slot_changes:   AddressSlots,
    slot_keeper:    SlotKeeper,
    futures:        FuturesUnordered<JoinHandle<Option<AddressSlots>>>
}

impl Revm {
    pub fn new(transaction_rx: UnboundedReceiver<SimEvent>, db: RevmLRU) -> Result<Self, SimError> {
        let threadpool = ThreadPool::new()?;
        Ok(Self {
            slot_keeper: SlotKeeper::new(db.clone(), vec![], threadpool.runtime.handle().clone()),
            transaction_rx,
            threadpool,
            state: Arc::new(RevmState::new(db)),
            slot_changes: HashMap::new(),
            futures: FuturesUnordered::new()
        })
    }

    pub fn get_threadpool_handle(&self) -> Handle {
        self.threadpool.runtime.handle().clone()
    }

    fn update_slots(&mut self, touched_slots: AddressSlots) {
        for (addr, t_slots) in touched_slots.into_iter() {
            let slot = self
                .slot_changes
                .entry(addr)
                .or_insert_with(|| HashMap::new());
            for (key, val) in t_slots.into_iter() {
                slot.insert(key, val);
            }
        }
    }

    /// handles incoming transactions from clients
    fn handle_incoming_event(&mut self, tx_type: SimEvent) {
        let state = self.state.clone();

        match tx_type {
            SimEvent::Hook(data, overrides, sender) => {
                let slots = self.slot_keeper.get_current_slots().clone();
                let fut = async move {
                    let res = state.simulate_hooks(data, overrides, slots);

                    match res {
                        Ok((sim_res, slots)) => {
                            let _ = sender.send(sim_res);
                            Some(slots)
                        }
                        Err(e) => {
                            let _ = sender.send(SimResult::SimError(e));
                            None
                        }
                    }
                };

                self.futures.push(self.threadpool.spawn_return_task_as(fut));
            }
            SimEvent::UniswapV4(tx, sender) => {
                let fut = async move {
                    let mut map = HashMap::new();

                    let bytecode = Bytecode {
                        bytecode: Bytes::from(V4_BYTE_CODE).into(),
                        ..Default::default()
                    };

                    map.insert(ANGSTROM_ADDRESS, bytecode);
                    let _ = match state.simulate_v4_tx(tx, map) {
                        Ok(res) => sender.send(res),
                        Err(err) => sender.send(SimResult::SimError(err))
                    };
                };

                let _ = self.threadpool.spawn_task_as(fut, TaskKind::Blocking);
            }
            SimEvent::BundleTx(tx, caller_info, sender) => {
                let fut = async move {
                    let res = state.simulate_bundle(tx, caller_info);
                    let _ = if let Err(e) = res {
                        sender.send(SimResult::SimError(e))
                    } else {
                        sender.send(res.unwrap())
                    };
                };
                let _ = self.threadpool.spawn_task_as(fut, TaskKind::Blocking);
            }
            SimEvent::NewBlock(sender) => {
                let slot_changes = self.slot_changes.clone();
                let fut = async move {
                    let res = RevmState::update_evm_state(state, &slot_changes);
                    let _ = if let Err(e) = res {
                        sender.send(SimResult::SimError(e))
                    } else {
                        sender.send(SimResult::SuccessfulRevmBlockUpdate)
                    };
                };
                let _ = self.threadpool.block_on_rt(fut);
                self.slot_changes.clear();
            }
        };
    }

    // this will be wired into new block
    #[allow(unused)]
    fn handle_new_pools(&mut self, pools: Vec<Address>, cx: &mut Context<'_>) {
        self.slot_keeper.new_addresses(pools);
        let _ = self.slot_keeper.poll_unpin(cx);
    }
}

impl Future for Revm {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        while let Poll::Ready(poll_tx) = this.transaction_rx.poll_recv(cx) {
            match poll_tx {
                Some(tx) => this.handle_incoming_event(tx),
                None => return Poll::Ready(())
            }
        }

        while let Poll::Ready(Some(Ok(poll_slot))) = this.futures.poll_next_unpin(cx) {
            match poll_slot {
                Some(slot) => this.update_slots(slot),
                None => ()
            }
        }

        return Poll::Pending
    }
}
