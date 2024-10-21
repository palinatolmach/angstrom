use std::hash::Hash;

use alloy::primitives::{keccak256, FixedBytes, Keccak256, B256};
use bitmaps::Bitmap;
use blsful::{Bls12381G1Impl, PublicKey, SecretKey};
use reth_network_peers::PeerId;
use serde::{Deserialize, Serialize};

use super::Proposal;
use crate::primitive::{BLSSignature, BLSValidatorID};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Commit {
    pub block_height:     u64,
    pub source:           PeerId,
    pub preproposal_hash: B256,
    pub solution_hash:    B256,
    // This signature is (block_height | vanilla_bundle_hash |
    // lower_bound_hash | order_buffer_hash)
    pub signature:        BLSSignature
}

impl Commit {
    pub fn generate_commit_all(
        block_height: u64,
        source: PeerId,
        preproposal_hash: B256,
        solution_hash: B256,
        sk: &SecretKey<Bls12381G1Impl>
    ) -> Self {
        let mut hasher: Keccak256 = Keccak256::new();
        hasher.update(block_height.to_be_bytes());
        hasher.update(preproposal_hash);
        hasher.update(solution_hash);
        let message = hasher.finalize();

        // TODO: where do we get the validator idx from
        let signature = BLSSignature::sign(0, sk, message.as_slice());

        Self { block_height, source, preproposal_hash, solution_hash, signature }
    }

    /// Get a reference to the validator bitmap for this Commit.  All the
    /// validator maps should always be the same so we return the one from
    /// `message_sig`
    pub fn validator_map(&self) -> &Bitmap<128> {
        self.signature.validator_map()
    }

    /// Returns the number of validators that have signed this Commit message
    pub fn num_signed(&self) -> usize {
        self.signature.validator_map().len()
    }

    fn hash_message(&self) -> FixedBytes<32> {
        let mut hasher = Keccak256::new();
        hasher.update(self.block_height.to_be_bytes());
        hasher.update(self.preproposal_hash);
        hasher.update(self.solution_hash);
        hasher.finalize()
    }

    pub fn add_signature(
        &mut self,
        validator_id: BLSValidatorID,
        sk: &SecretKey<Bls12381G1Impl>
    ) -> bool {
        // These can only fail if the SK is zero in which case they'll all fail
        // so no need to return early
        self.signature
            .sign_into(validator_id, sk, self.hash_message().as_slice())
    }

    pub fn is_valid(&self, public_key_library: &[PublicKey<Bls12381G1Impl>]) -> bool {
        self.signature
            .validate(public_key_library, self.hash_message().as_slice())
    }

    /// Validate that this commit message is associated with a specific Proposal
    /// - incomplete
    pub fn is_for(&self, proposal: &Proposal) -> bool {
        self.block_height == proposal.block_height
        // Also check to make sure our hashes match the proposal data
    }

    /// Returns true if this Commit claims to have been signed by the specified
    /// validator.  This does not inherently validate the Commit so make
    /// sure to do that as well!
    pub fn signed_by(&self, validator_id: BLSValidatorID) -> bool {
        self.signature.signed_by(validator_id)
    }

    pub fn from_proposal(proposal: &Proposal, sk: &SecretKey<Bls12381G1Impl>) -> Self {
        let block_height = proposal.block_height;
        let mut buf = Vec::new();
        buf.extend(bincode::serialize(&proposal.preproposals).unwrap());
        let preproposal_hash = keccak256(buf);
        let mut buf = Vec::new();
        buf.extend(bincode::serialize(&proposal.solutions).unwrap());
        let solution_hash = keccak256(buf);

        Self::generate_commit_all(
            block_height,
            proposal.source,
            preproposal_hash,
            solution_hash,
            sk
        )
    }
}
