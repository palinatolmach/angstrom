use alloy_primitives::{BlockNumber, FixedBytes, B512};
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::PoolSolution,
    primitive::{BLSValidatorID, PeerId}
};
use blsful::{Bls12381G1Impl, SecretKey as BlsSecretKey};
use secp256k1::{rand::thread_rng, SecretKey};

/// The Signer deals with verifying external signatures as well as
/// signing our payloads.  Pub fields for now.
#[derive(Clone)]
pub struct Signer {
    pub my_id:        PeerId,
    pub validator_id: BLSValidatorID,
    pub key:          SecretKey,
    pub bls_key:      BlsSecretKey<Bls12381G1Impl>
}

impl Default for Signer {
    fn default() -> Self {
        let rng = thread_rng();
        let key = SecretKey::new(&mut secp256k1::rand::thread_rng());
        Signer {
            my_id: FixedBytes::random(),
            validator_id: BLSValidatorID::default(),
            key,
            bls_key: BlsSecretKey::random(rng)
        }
    }
}

impl Signer {
    pub fn new(secret_key: SecretKey) -> Self {
        // Our BLS key will be derived from our public key, I don't know if that's cool
        // or good but it should at least keep the key constant for the moment
        // and ensure that we only have to manage a single secret
        let bls_key = BlsSecretKey::from_hash(secret_key.secret_bytes());
        Self { key: secret_key, bls_key, ..Default::default() }
    }

    pub fn sign_proposal(
        &self,
        ethereum_block: BlockNumber,
        preproposals: Vec<PreProposal>,
        solutions: Vec<PoolSolution>
    ) -> Proposal {
        Proposal::generate_proposal(ethereum_block, self.my_id, preproposals, solutions, &self.key)
    }

    pub fn sign_commit(&self, proposal: &Proposal) -> Commit {
        Commit::from_proposal(proposal, &self.bls_key)
    }

    pub fn is_us(&self, _addr: &B512) -> bool {
        todo!("change key to proper")
    }
}
