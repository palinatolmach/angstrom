use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};

use common::{ConsensusState, IsLeader, PRE_PROPOSE};
use futures::FutureExt;
use guard_types::on_chain::SimmedBundle;

use super::{
    pre_propose::PreProposeState, GlobalStateContext, RoundAction, RoundStateMessage,
    StateTransition, Timeout
};

pub struct OrderAccumulationState {
    timeout:     Timeout,
    best_bundle: Option<SimmedBundle>,
    is_leader:   IsLeader
}
impl OrderAccumulationState {
    pub fn new(timeout: Timeout, is_leader: IsLeader) -> Self {
        Self { timeout, best_bundle: None, is_leader }
    }

    pub fn new_bundle(&mut self, bundle: SimmedBundle) {
        if self
            .best_bundle
            .as_ref()
            .map(|cur_best| bundle.get_cumulative_lp_bribe() > cur_best.get_cumulative_lp_bribe())
            .filter(|f| *f)
            .is_some()
        {
            self.best_bundle.replace(bundle);
        }
    }
}

impl StateTransition for OrderAccumulationState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _: GlobalStateContext
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        self.timeout.poll_unpin(cx).map(|_| {
            (
                RoundAction::PrePropose(PreProposeState::new(
                    // TODO: placeholder
                    Timeout::new(Duration::from_secs(6)),
                    // todo don't unwrap
                    self.best_bundle.take().unwrap(),
                    self.is_leader.clone()
                )),
                PRE_PROPOSE,
                None
            )
        })
    }
}
