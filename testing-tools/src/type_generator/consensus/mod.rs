pub mod preproposal;
pub mod proposal;

use angstrom_types::{
    consensus::Commit,
    primitive::PoolId,
    sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData}
};
use blsful::{Bls12381G1Impl, SecretKey};
use proposal::ProposalBuilder;
use rand::rngs::ThreadRng;

use super::orders::DistributionParameters;
use crate::type_generator::orders::{generate_limit_order, generate_order_distribution};

pub fn generate_limit_order_set(
    rng: &mut ThreadRng,
    count: usize,
    is_bid: bool,
    block: u64
) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
    (0..count)
        .map(|_| generate_limit_order(rng, true, is_bid, None, Some(block), None, None, None, None))
        .collect()
}

pub fn generate_limit_order_distribution(
    count: usize,
    pool_id: PoolId,
    block: u64
) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
    let mut res = Vec::with_capacity(count * 2);
    let (bidprice, askprice) = DistributionParameters::crossed_at(100_000_000.0);
    let (bidquant, askquant) = DistributionParameters::fixed_at(100.0);
    res.extend(
        generate_order_distribution(true, count, bidprice, bidquant, pool_id, block).unwrap()
    );
    res.extend(
        generate_order_distribution(false, count, askprice, askquant, pool_id, block).unwrap()
    );
    res
}

pub fn generate_random_commit(sk: &SecretKey<Bls12381G1Impl>) -> Commit {
    let proposal = ProposalBuilder::new()
        .order_count(100)
        .for_random_pools(10)
        .for_block(10)
        .build();
    Commit::from_proposal(&proposal, sk)
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::generate_random_commit;
    use crate::type_generator::consensus::{
        preproposal::PreproposalBuilder, proposal::ProposalBuilder
    };

    #[test]
    fn random_preproposal_is_valid() {
        let preproposal = PreproposalBuilder::new()
            .order_count(100)
            .for_random_pools(1)
            .for_block(10)
            .build();
        assert!(preproposal.is_valid(), "Preproposal cannot validate itself");
    }

    #[test]
    fn random_proposal_is_valid() {
        let proposal = ProposalBuilder::new()
            .order_count(100)
            .for_random_pools(1)
            .for_block(10)
            .build();
        assert!(proposal.is_valid(), "Proposal cannot validate itself");
    }

    #[test]
    fn random_commit_is_valid() {
        // We generate our secret key first
        let sk = blsful::SecretKey::random(&mut thread_rng());
        let commit = generate_random_commit(&sk);
        assert!(commit.is_valid(&[sk.public_key()]));
    }
}
