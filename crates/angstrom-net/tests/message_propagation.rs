use std::time::Duration;

use rand::thread_rng;
use reth_provider::test_utils::NoopProvider;
use testing_tools::{
    network::AngstromTestnet,
    type_generator::consensus::{
        generate_random_commit, preproposal::PreproposalBuilder, proposal::ProposalBuilder
    }
};

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_broadcast_order_propagation() {
    reth_tracing::init_test_tracing();
    let noop = NoopProvider::default();
    let mut testnet = AngstromTestnet::new(3, noop).await;

    // connect all peers
    let res = tokio::time::timeout(Duration::from_secs(3), testnet.connect_all_peers()).await;
    assert!(res.is_ok(), "failed to connect all peers within 3 seconds");

    // let orders = (0..3)
    //     .map(|_| generate_random_valid_order())
    //     .collect::<Vec<_>>();
    let orders = vec![];

    let delay_seconds = 4;
    let res = tokio::time::timeout(
        Duration::from_secs(delay_seconds),
        testnet.broadcast_message_orders(angstrom_network::StromMessage::PropagatePooledOrders(
            orders.clone()
        ))
    )
    .await;

    assert_eq!(
        res,
        Ok(true),
        "failed to receive and react to order within {} seconds",
        delay_seconds
    );

    let res = tokio::time::timeout(
        Duration::from_secs(4),
        testnet.broadcast_message_orders(angstrom_network::StromMessage::PropagatePooledOrders(
            orders
        ))
    )
    .await;

    assert_eq!(res, Ok(true), "failed to receive and react to order within 4 seconds");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_singular_order_propagation() {
    reth_tracing::init_test_tracing();
    let noop = NoopProvider::default();
    let mut testnet = AngstromTestnet::new(3, noop).await;

    // connect all peers
    //
    let res = tokio::time::timeout(Duration::from_secs(3), testnet.connect_all_peers()).await;
    assert!(res.is_ok(), "failed to connect all peers within 3 seconds");

    // let orders = (0..3)
    //     .map(|_| generate_random_valid_order())
    //     .collect::<Vec<_>>();
    let orders = vec![];

    let delay_seconds = 8;

    let res = tokio::time::timeout(
        Duration::from_secs(delay_seconds),
        testnet.send_order_message(angstrom_network::StromMessage::PropagatePooledOrders(
            orders.clone()
        ))
    )
    .await;

    assert_eq!(
        res,
        Ok(true),
        "failed to receive and react to order within {} seconds",
        delay_seconds
    );

    let res = tokio::time::timeout(
        Duration::from_secs(4),
        testnet.send_order_message(angstrom_network::StromMessage::PropagatePooledOrders(
            orders.clone()
        ))
    )
    .await;

    assert_eq!(res, Ok(true), "failed to receive and react to order within 4 seconds");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_broadcast_consensus_propagation() {
    reth_tracing::init_test_tracing();
    let noop = NoopProvider::default();
    let mut testnet = AngstromTestnet::new(4, noop).await;

    // connect all peers
    let res = tokio::time::timeout(Duration::from_secs(3), testnet.connect_all_peers()).await;
    assert!(res.is_ok(), "failed to connect all peers within 3 seconds");
    let sk = blsful::SecretKey::random(&mut thread_rng());

    for _ in 0..3 {
        // commits
        let commit = generate_random_commit(&sk);
        let delay_seconds = 6;
        let res = tokio::time::timeout(
            Duration::from_secs(delay_seconds),
            testnet.send_consensus_broadcast(angstrom_network::StromMessage::Commit(commit))
        )
        .await;
        assert_eq!(
            res,
            Ok(true),
            "failed to receive and react to commit within {} second{}",
            delay_seconds,
            if delay_seconds == 1 { "" } else { "s" }
        );

        // preposals
        let preposal = PreproposalBuilder::new()
            .order_count(10)
            .for_random_pools(1)
            .for_block(100)
            .build();
        let res = tokio::time::timeout(
            Duration::from_secs(1),
            testnet.send_consensus_broadcast(angstrom_network::StromMessage::PrePropose(preposal))
        )
        .await;
        assert_eq!(res, Ok(true), "failed to receive and react to preposal within 1 second");

        // proposals
        let proposal = ProposalBuilder::new()
            .order_count(10)
            .for_random_pools(1)
            .for_block(0)
            .build();
        let res = tokio::time::timeout(
            Duration::from_secs(1),
            testnet.send_consensus_broadcast(angstrom_network::StromMessage::Propose(proposal))
        )
        .await;
        assert_eq!(res, Ok(true), "failed to receive and react to proposal within 1 second");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_consensus_propagation() {
    reth_tracing::init_test_tracing();
    let noop = NoopProvider::default();
    let mut testnet = AngstromTestnet::new(4, noop).await;

    // connect all peers
    let res = tokio::time::timeout(Duration::from_secs(3), testnet.connect_all_peers()).await;
    assert!(res.is_ok(), "failed to connect all peers within 3 seconds");
    let sk = blsful::SecretKey::random(&mut thread_rng());

    for _ in 0..3 {
        // commits
        let commit = generate_random_commit(&sk);
        let res = tokio::time::timeout(
            Duration::from_secs(1),
            testnet.send_consensus_message(angstrom_network::StromMessage::Commit(commit))
        )
        .await;
        assert_eq!(res, Ok(true), "failed to receive and react to commit within 1 second");

        // preposals
        let preposal = PreproposalBuilder::new()
            .order_count(10)
            .for_random_pools(1)
            .for_block(100)
            .build();
        let res = tokio::time::timeout(
            Duration::from_secs(1),
            testnet.send_consensus_message(angstrom_network::StromMessage::PrePropose(preposal))
        )
        .await;
        assert_eq!(res, Ok(true), "failed to receive and react to preposal within 1 second");

        // proposals
        let proposal = ProposalBuilder::new()
            .order_count(10)
            .for_random_pools(1)
            .for_block(0)
            .build();
        let res = tokio::time::timeout(
            Duration::from_secs(1),
            testnet.send_consensus_message(angstrom_network::StromMessage::Propose(proposal))
        )
        .await;
        assert_eq!(res, Ok(true), "failed to receive and react to proposal within 1 second");
    }
}
