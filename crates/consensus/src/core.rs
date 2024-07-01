use std::{collections::VecDeque, sync::Arc};

use angstrom_types::consensus::{Commit, EvidenceError, PreProposal, Proposal};
use reth_primitives::Block;
use thiserror::Error;
use tracing::error;

use crate::{
    evidence::EvidenceCollector, round::RoundState, round_robin_algo::RoundRobinAlgo,
    signer::Signer
};

#[derive(Debug, Clone)]
pub enum ConsensusMessage {
    /// Start/Cycle the consensus process as a new block has begun
    NewBlock(u64),
    /// All angstrom nodes broadcast their signed order pools to the network
    PrePropose(PreProposal),
    /// The Proposer broadcasts its signed proposal for validation.  This might
    /// be after execution-time but all nodes need to review this information
    Proposal(Proposal),
    /// Commit or nil vote on whether the proposal was properly executed
    Commit(Box<Commit>)
}

#[derive(Debug, Error)]
pub enum ConsensusError {
    #[error("Evidence Module had an Error: {0:#?}")]
    EvidenceError(#[from] EvidenceError)
}

/// The ConsensusCore module handles everything related to consensus.
/// This includes tracking slashable events, other angstroms commits and votes
/// and submitting to consensus.
/// # Design Principles
/// The main interfacing idea for the ConsensusCore is that this module
/// only operates on truths. What this means is this module avoids doing
/// any comparison, building or evaluation in order to keep it as simple as
/// possible (Of course we cannot rid all of this, however there is always a
/// focus to minimize this). all values that are handed to this module are true.
/// for example, this means that the consensus module doesn't know of any other
/// bundles that this angstrom has built except for the most profitable one. Nor
/// does it know what the proper pricing for a given storage slot is. We
/// abstract all of this out in order to keep this module as clean as possible
/// as proper functionality is critical here to ensure that angstrom works
/// properly.
#[allow(dead_code)]
pub struct ConsensusCore {
    /// keeps track of the current round state
    round_state:        RoundState,
    /// leader selection algo
    leader_selection:   RoundRobinAlgo,
    /// collects + formulates evidence of byzantine angstroms
    evidence_collector: EvidenceCollector,
    /// deals with all signing and signature verification
    signer:             Signer,
    /// messages to share with others
    outbound:           VecDeque<ConsensusMessage>
}

impl ConsensusCore {
    /// returns self but also returns the block that the round robin algo
    /// has historic state up until
    pub fn new() -> (Self, u64) {
        todo!()
    }

    pub fn new_block(&mut self, block: Arc<Block>) {
        // need to make sure that this is sequential
        if self.round_state.current_height() + 1 == block.number {
            // TODO: wire in angstrom selection stuff
            // let new_leader =
            // self.leader_selection.on_new_block(block.clone());

            // self.round_state.new_height(block.number, new_leader);
        } else {
            panic!("have a gap in blocks which will break the round robin algo");
        }
    }

    #[allow(dead_code)]
    pub fn new_pre_propose(&mut self, _commit: PreProposal) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn proposal(&mut self, _proposal: Proposal) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn proposal_commit(&mut self, _commit: Commit) {
        todo!()
    }
}

// impl Stream for ConsensusCore {
//     type Item = Result<ConsensusMessage, ConsensusError>;

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) ->
// Poll<Option<Self::Item>> {         let _ =
// self.round_state.poll_next_unpin(cx);         todo!()
//     }
// }
