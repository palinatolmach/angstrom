use crate::types::subscriptions::ConsensusSubscriptionKind;
use consensus::ConsensusState;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "angstrom_consensus"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "angstrom_consensus"))]
#[async_trait::async_trait]
pub trait ConsensusApi {
    #[method(name = "current_state")]
    async fn consensus_state(&self) -> RpcResult<ConsensusState>;

    #[subscription(
        name = "consensus_state",
        unsubscribe = "unsubscribe_consensus_state",
        item = crate::types::subscriptions::ConsensusSubscriptionResult
    )]
    async fn subscribe_consensus_state(
        &self,
        kind: ConsensusSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult;
}
