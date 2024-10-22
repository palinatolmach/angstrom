use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::{manager::StromConsensusEvent, StromMessage};
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::{OrderSet, PoolSolution},
    primitive::PeerId,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};
use futures::{future::BoxFuture, Future, Stream};
use matching_engine::MatchingManager;
use order_pool::order_storage::OrderStorage;
use serde::{Deserialize, Serialize};
use tokio::time::{self, Instant};

use crate::{AngstromValidator, Signer};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BidSubmission {
    pub block_height:  BlockNumber,
    // this is used mostly for early messages
    pub pre_proposals: HashSet<PreProposal>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BidAggregation {
    pub block_height:  BlockNumber,
    pub pre_proposals: HashSet<PreProposal>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Finalization {
    pub block_height:  BlockNumber,
    pub pre_proposals: HashSet<PreProposal>,
    pub proposal:      Option<Proposal>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusState {
    BidSubmission(BidSubmission),
    BidAggregation(BidAggregation),
    Finalization(Finalization)
}

impl ConsensusState {
    fn name(&self) -> &'static str {
        match self {
            Self::BidSubmission(_) => "BidSubmission",
            Self::BidAggregation(_) => "BidAggregation",
            Self::Finalization(_) => "Finalization"
        }
    }
}

async fn build_proposal(pre_proposals: Vec<PreProposal>) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(pre_proposals).await
}

pub struct RoundStateMachine {
    current_state:          ConsensusState,
    signer:                 Signer,
    round_leader:           PeerId,
    validators:             Vec<AngstromValidator>,
    order_storage:          Arc<OrderStorage>,
    initial_state_duration: Duration,
    metrics:                ConsensusMetricsWrapper,
    transition_future:      Option<BoxFuture<'static, ConsensusState>>,
    initial_state_timer:    Option<Pin<Box<time::Sleep>>>,
    waker:                  Option<Waker>
}

impl RoundStateMachine {
    pub fn new(
        block_height: BlockNumber,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        round_leader: PeerId,
        validators: Vec<AngstromValidator>,
        metrics: ConsensusMetricsWrapper,
        initial_state_duration: Option<Duration>
    ) -> Self {
        let initial_state_duration =
            initial_state_duration.unwrap_or_else(|| Duration::from_secs(3));
        let timer = Box::pin(time::sleep(initial_state_duration));

        Self {
            current_state: Self::initial_state(block_height),
            round_leader,
            validators,
            initial_state_duration,
            order_storage,
            signer,
            metrics,
            transition_future: None,
            initial_state_timer: Some(timer),
            waker: None
        }
    }

    pub fn my_id(&self) -> PeerId {
        self.signer.my_id
    }

    pub fn is_leader(&self, node: PeerId) -> bool {
        self.round_leader == node
    }

    pub fn i_am_leader(&self) -> bool {
        self.is_leader(self.my_id())
    }

    pub fn has_quorum(&self, voters: usize) -> bool {
        voters >= (self.validators.len() * 2) / 3 + 1
    }

    pub fn reset_round(&mut self, block: BlockNumber, leader: PeerId) {
        self.round_leader = leader;
        self.current_state = Self::initial_state(block);
        self.initial_state_timer = Some(Box::pin(time::sleep(self.initial_state_duration)));
        self.transition_future = None;
    }

    pub fn initial_state(block_height: BlockNumber) -> ConsensusState {
        ConsensusState::BidSubmission(BidSubmission { block_height, ..Default::default() })
    }

    pub fn my_pre_proposal(&self, pre_proposals: &HashSet<PreProposal>) -> Option<StromMessage> {
        pre_proposals
            .iter()
            .find(|p| p.source == self.my_id())
            .cloned()
            .map(StromMessage::PrePropose)
    }

    /// Invariants:
    ///  * won't be called with messages with payload from ourselves
    ///  * won't be called with messages for non-current block height
    ///
    /// Returns:
    ///   In general we do not want to return an updated message for broadcast.
    ///   However, during bid aggregation, we need to tell the rest of the
    ///   network what our updated pre_proposal is, so when it's time for the
    ///   leader to propose, it needs to see an agreement on the ToB and RoB
    pub fn on_strom_message(&mut self, strom_msg: StromConsensusEvent) -> Option<StromMessage> {
        let current_state = self.current_state.clone();
        let new_state = match current_state {
            ConsensusState::BidSubmission(bid_submission) => {
                let (updated_submission, new_state) =
                    self.handle_strom_msg_in_submission(bid_submission, strom_msg);
                self.current_state = ConsensusState::BidSubmission(updated_submission);
                new_state
            }
            ConsensusState::BidAggregation(bid_aggregation) => {
                let (updated_aggregation, new_state) =
                    self.handle_strom_msg_in_aggregation(bid_aggregation, strom_msg);
                self.current_state = ConsensusState::BidAggregation(updated_aggregation);
                new_state
            }
            ConsensusState::Finalization(finalization) => {
                let (updated_finalization, new_state) =
                    self.handle_strom_msg_in_finalization(finalization, strom_msg);
                self.current_state = ConsensusState::Finalization(updated_finalization);
                new_state
            }
        };

        if let Some(new_state) = new_state {
            self.force_transition(new_state);
        }

        // send the updated pre-proposal with the new orders
        if let ConsensusState::BidAggregation(BidAggregation { pre_proposals, .. }) =
            &self.current_state
        {
            return self.my_pre_proposal(pre_proposals);
        }

        None
    }

    fn handle_strom_msg_in_submission(
        &self,
        mut bid_submission: BidSubmission,
        strom_msg: StromConsensusEvent
    ) -> (BidSubmission, Option<ConsensusState>) {
        match strom_msg {
            // we are lagging, we should transition to bid aggregation
            StromConsensusEvent::PreProposal(msg_sender, pre_proposal) => {
                let PreProposal {
                    block_height: pre_proposal_height,
                    source: pre_proposal_sender,
                    ..
                } = pre_proposal;
                if !pre_proposal.is_valid() {
                    tracing::warn!(
                        %msg_sender,
                        %bid_submission.block_height,
                        %pre_proposal_sender,
                        %pre_proposal_height,
                        "received pre_proposal with invalid signature",
                    );
                    return (bid_submission, None);
                }
                bid_submission.pre_proposals.insert(pre_proposal.clone());

                let bid_aggregation = self.generate_bid_aggregation(bid_submission.clone());

                bid_submission.pre_proposals = bid_aggregation.pre_proposals.clone();

                // TODO: do we want to skip bid aggregation if we have quorum here ?
                (bid_submission, Some(ConsensusState::BidAggregation(bid_aggregation)))
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                let Proposal {
                    block_height: proposal_block_height, source: proposal_sender, ..
                } = proposal;
                // TODO: we got an invalid proposal; start slashing ?
                if !proposal.is_valid() || !self.is_leader(proposal_sender) {
                    return (bid_submission, None);
                }

                // we are following and lagging a lot
                if !self.i_am_leader() {
                    let pre_proposals = bid_submission.pre_proposals.clone();
                    return (
                        bid_submission,
                        Some(ConsensusState::Finalization(Finalization {
                            block_height: proposal_block_height,
                            proposal: Some(proposal),
                            pre_proposals
                        }))
                    );
                }

                // that should not be possible. it means we got our own proposal from another
                // node, while we are in bid submission state
                panic!(
                    "Message sender: {}, Proposal block height: {}, Proposal source: {}, Current \
                     state {}, something is terribly wrong",
                    msg_sender,
                    proposal_block_height,
                    proposal_sender,
                    self.current_state.name()
                );
            }
            // TODO: this could be used  by the leader to gossip the 2/3 + 1 pre_proposals that were
            // used for the bundle
            StromConsensusEvent::Commit(..) => (bid_submission, None)
        }
    }

    fn generate_bid_aggregation(&self, bid_submission: BidSubmission) -> BidAggregation {
        let BidSubmission { block_height, pre_proposals } = bid_submission;
        let OrderSet { limit, searcher } = self.order_storage.get_all_orders();
        let mut pre_proposals = pre_proposals.clone();

        let pre_proposal =
            self.generate_our_merged_pre_proposal(block_height, limit, searcher, &pre_proposals);
        pre_proposals.insert(pre_proposal);

        BidAggregation { block_height, pre_proposals }
    }

    fn generate_our_merged_pre_proposal(
        &self,
        block_height: BlockNumber,
        limit: Vec<OrderWithStorageData<GroupedVanillaOrder>>,
        searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>,
        pre_proposals: &HashSet<PreProposal>
    ) -> PreProposal {
        let merged_limit_orders = self.merge_limit_orders(
            pre_proposals
                .iter()
                .flat_map(|p| p.limit.clone())
                .chain(limit)
                .collect()
        );

        let merged_searcher_orders = self.merge_searcher_orders(
            pre_proposals
                .iter()
                .flat_map(|p| p.searcher.clone())
                .chain(searcher)
                .collect()
        );

        PreProposal::generate_pre_proposal(
            block_height,
            self.my_id(),
            merged_limit_orders,
            merged_searcher_orders,
            &self.signer.key
        )
    }

    fn handle_strom_msg_in_aggregation(
        &self,
        mut bid_aggregation: BidAggregation,
        strom_msg: StromConsensusEvent
    ) -> (BidAggregation, Option<ConsensusState>) {
        match strom_msg {
            StromConsensusEvent::PreProposal(_, pre_proposal) => {
                let PreProposal { block_height, .. } = pre_proposal;

                if !pre_proposal.is_valid() {
                    return (bid_aggregation, None);
                }

                bid_aggregation.pre_proposals.insert(pre_proposal.clone());

                // TODO: make it prettier
                let self_proposal = self.generate_our_merged_pre_proposal(
                    block_height,
                    Vec::new(),
                    Vec::new(),
                    &bid_aggregation.pre_proposals
                );

                bid_aggregation.pre_proposals.insert(self_proposal);

                if self.has_quorum(bid_aggregation.pre_proposals.len()) {
                    return (
                        bid_aggregation.clone(),
                        Some(ConsensusState::Finalization(Finalization {
                            block_height,
                            proposal: None,
                            pre_proposals: bid_aggregation.pre_proposals
                        }))
                    );
                }

                (bid_aggregation, None)
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                let Proposal {
                    block_height: proposal_block_height, source: proposal_sender, ..
                } = proposal;
                // TODO: we got an invalid proposal; start slashing ?
                if !proposal.is_valid() || !self.is_leader(proposal_sender) {
                    return (bid_aggregation, None);
                }

                // we are following and lagging a bit
                if !self.i_am_leader() {
                    let pre_proposals = bid_aggregation.pre_proposals.clone();
                    return (
                        bid_aggregation,
                        Some(ConsensusState::Finalization(Finalization {
                            block_height: proposal_block_height,
                            proposal: Some(proposal),
                            pre_proposals
                        }))
                    );
                }

                // that should not be possible. it means we got our own proposal from another
                // node, while we are in bid aggregation state
                panic!(
                    "Message sender: {}, Proposal block height: {}, Proposal source: {}, Current \
                     state {}, something is terribly wrong",
                    msg_sender,
                    proposal_block_height,
                    proposal_sender,
                    self.current_state.name()
                );
            }
            // TODO: used to broadcast after the bundle submission
            StromConsensusEvent::Commit(..) => (bid_aggregation, None),
            _ => (bid_aggregation, None)
        }
    }

    fn handle_strom_msg_in_finalization(
        &self,
        mut finalization: Finalization,
        strom_msg: StromConsensusEvent
    ) -> (Finalization, Option<ConsensusState>) {
        match strom_msg {
            StromConsensusEvent::PreProposal(msg_sender, pre_proposal) => {
                let PreProposal {
                    block_height: pre_proposal_height,
                    source: pre_proposal_sender,
                    ..
                } = pre_proposal;
                if !pre_proposal.is_valid() {
                    tracing::warn!(
                        %msg_sender,
                        %finalization.block_height,
                        %pre_proposal_sender,
                        %pre_proposal_height,
                        "received pre_proposal with invalid signature",
                    );
                    return (finalization, None);
                }

                finalization.pre_proposals.insert(pre_proposal);

                if !self.i_am_leader() {
                    // if we are not leader wait for Ethereum to restart us
                    return (finalization, None);
                }

                if !self.has_quorum(finalization.pre_proposals.len()) {
                    return (finalization, None);
                }

                (finalization, None)
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                let Proposal {
                    block_height: proposal_block_height, source: proposal_sender, ..
                } = proposal;
                // TODO: we got an invalid proposal; start slashing ?
                if !proposal.is_valid() || !self.is_leader(proposal_sender) {
                    return (finalization, None);
                }

                // we are following and lagging a bit
                if !self.i_am_leader() {
                    let pre_proposals = finalization.pre_proposals.clone();
                    return (
                        finalization,
                        Some(ConsensusState::Finalization(Finalization {
                            block_height: proposal_block_height,
                            proposal: Some(proposal),
                            pre_proposals
                        }))
                    );
                }

                // that should not be possible. it means we got our own proposal from another
                // node, while we are in bid aggregation state
                panic!(
                    "Message sender: {}, Proposal block height: {}, Proposal source: {}, Current \
                     state {}, something is terribly wrong",
                    msg_sender,
                    proposal_block_height,
                    proposal_sender,
                    self.current_state.name()
                );
            }
            // TODO: this could be used  by the leader to gossip the 2/3 + 1 pre_proposals that were
            // used for the bundle
            StromConsensusEvent::Commit(..) => (finalization, None)
        }
    }

    fn merge_limit_orders(
        &self,
        left_limit: Vec<OrderWithStorageData<GroupedVanillaOrder>>
    ) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
        let mut unique_limit_per_pool: HashMap<_, HashSet<GroupedVanillaOrder>> = HashMap::new();

        for order_with_data in left_limit {
            unique_limit_per_pool
                .entry(order_with_data.pool_id)
                .or_insert_with(HashSet::new)
                .insert(order_with_data.order.clone());
        }

        unique_limit_per_pool
            .into_iter()
            .flat_map(|(pool_id, orders)| {
                orders.into_iter().map(move |order| OrderWithStorageData {
                    pool_id,
                    order,
                    ..Default::default()
                })
            })
            .collect()
    }

    fn merge_searcher_orders(
        &self,
        left_searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>
    ) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        let mut top_searcher_per_pool: HashMap<_, OrderWithStorageData<TopOfBlockOrder>> =
            HashMap::new();
        for order_with_data in left_searcher {
            let pool_id = order_with_data.pool_id;
            top_searcher_per_pool
                .entry(pool_id)
                .and_modify(|existing_order| {
                    if order_with_data.tob_reward > existing_order.tob_reward {
                        *existing_order = order_with_data.clone();
                    }
                })
                .or_insert(order_with_data);
        }
        top_searcher_per_pool.into_values().collect()
    }

    fn force_transition(&mut self, mut new_state: ConsensusState) {
        let from_state = self.current_state.clone();
        let signer = self.signer.clone();

        self.transition_future = Some(Box::pin(async move {
            if let (
                ConsensusState::BidAggregation(BidAggregation {
                    block_height: pre_proposal_height,
                    pre_proposals
                }),
                ConsensusState::Finalization(ref mut finalization)
            ) = (from_state, &mut new_state)
            {
                let pre_proposals: Vec<PreProposal> = pre_proposals.iter().cloned().collect();
                // TODO: maybe spawn a new thread here, since proposal building might be comp
                // expensive
                let solutions = build_proposal(pre_proposals.clone()).await.unwrap();
                let proposal = signer.sign_proposal(pre_proposal_height, pre_proposals, solutions);

                finalization.proposal = Some(proposal);

                // TODO: submit the proposal/bundle to the chain
                // Ethereum.submit(proposal).await
            }
            new_state
        }));

        // wake up the task to ensure poll_next is called
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }
}

impl Stream for RoundStateMachine {
    type Item = ConsensusState;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        this.waker = Some(cx.waker().clone());

        if let Some(ref mut future) = this.transition_future {
            return match future.as_mut().poll(cx) {
                Poll::Ready(new_state) => Poll::Ready(Some(new_state)),
                Poll::Pending => Poll::Pending
            };
        }

        if let Some(ref mut timer) = this.initial_state_timer {
            if timer.as_mut().poll(cx).is_ready() {
                // if the timer ran out and we are not in bid submission, something curious
                // happened
                if let ConsensusState::BidSubmission(ref bid_submission) = this.current_state {
                    let bid_aggregation = this.generate_bid_aggregation(bid_submission.clone());
                    this.transition_future =
                        Some(Box::pin(async { ConsensusState::BidAggregation(bid_aggregation) }));
                    this.initial_state_timer = None;
                }
            }
        }

        Poll::Pending
    }
}
