// Allows us to impl revm::DatabaseRef on the default provider type.
use reth_provider::{
    AccountReader, BlockHashReader, StateProofProvider, StateProvider, StateProviderFactory
};
use reth_storage_api::{StateRootProvider, StorageRootProvider};
use reth_trie::{
    updates::TrieUpdates, AccountProof, HashedPostState, HashedStorage, MultiProof, TrieInput
};

pub struct RethDbWrapper<DB: StateProviderFactory + Unpin + Clone + 'static>(DB);

impl<DB> RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn new(db: DB) -> Self {
        Self(db)
    }
}

impl<DB> StateProvider for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn storage(
        &self,
        account: reth_primitives::Address,
        storage_key: reth_primitives::StorageKey
    ) -> reth_provider::ProviderResult<Option<reth_primitives::StorageValue>> {
        self.0.latest()?.storage(account, storage_key)
    }

    fn account_code(
        &self,
        addr: reth_primitives::Address
    ) -> reth_provider::ProviderResult<Option<reth_primitives::Bytecode>> {
        self.0.latest()?.account_code(addr)
    }

    fn account_nonce(
        &self,
        addr: reth_primitives::Address
    ) -> reth_provider::ProviderResult<Option<u64>> {
        self.0.latest()?.account_nonce(addr)
    }

    fn account_balance(
        &self,
        addr: reth_primitives::Address
    ) -> reth_provider::ProviderResult<Option<reth_primitives::U256>> {
        self.0.latest()?.account_balance(addr)
    }

    fn bytecode_by_hash(
        &self,
        code_hash: reth_primitives::B256
    ) -> reth_provider::ProviderResult<Option<reth_primitives::Bytecode>> {
        self.0.latest()?.bytecode_by_hash(code_hash)
    }
}

impl<DB> AccountReader for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn basic_account(
        &self,
        address: reth_primitives::Address
    ) -> reth_provider::ProviderResult<Option<reth_primitives::Account>> {
        self.0.latest()?.basic_account(address)
    }
}

impl<DB> BlockHashReader for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn block_hash(
        &self,
        number: reth_primitives::BlockNumber
    ) -> reth_provider::ProviderResult<Option<reth_primitives::B256>> {
        self.0.latest()?.block_hash(number)
    }

    fn convert_block_hash(
        &self,
        hash_or_number: reth_primitives::BlockHashOrNumber
    ) -> reth_provider::ProviderResult<Option<reth_primitives::B256>> {
        self.0.latest()?.convert_block_hash(hash_or_number)
    }

    fn canonical_hashes_range(
        &self,
        start: reth_primitives::BlockNumber,
        end: reth_primitives::BlockNumber
    ) -> reth_provider::ProviderResult<Vec<reth_primitives::B256>> {
        self.0.latest()?.canonical_hashes_range(start, end)
    }
}

impl<DB> StateRootProvider for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn state_root(
        &self,
        hashed_state: HashedPostState
    ) -> reth_provider::ProviderResult<reth_primitives::B256> {
        self.0.latest()?.state_root(hashed_state)
    }

    fn state_root_from_nodes(
        &self,
        input: TrieInput
    ) -> reth_provider::ProviderResult<reth_primitives::B256> {
        self.0.latest()?.state_root_from_nodes(input)
    }

    fn state_root_with_updates(
        &self,
        hashed_state: HashedPostState
    ) -> reth_provider::ProviderResult<(reth_primitives::B256, TrieUpdates)> {
        self.0.latest()?.state_root_with_updates(hashed_state)
    }

    fn state_root_from_nodes_with_updates(
        &self,
        input: TrieInput
    ) -> reth_provider::ProviderResult<(reth_primitives::B256, TrieUpdates)> {
        self.0.latest()?.state_root_from_nodes_with_updates(input)
    }
}

impl<DB> StorageRootProvider for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn storage_root(
        &self,
        address: reth_primitives::Address,
        hashed_storage: HashedStorage
    ) -> reth_provider::ProviderResult<reth_primitives::B256> {
        self.0.latest()?.storage_root(address, hashed_storage)
    }
}

impl<DB> StateProofProvider for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn proof(
        &self,
        input: TrieInput,
        address: reth_primitives::Address,
        slots: &[reth_primitives::B256]
    ) -> reth_provider::ProviderResult<AccountProof> {
        self.0.latest()?.proof(input, address, slots)
    }

    fn witness(
        &self,
        input: TrieInput,
        target: HashedPostState
    ) -> reth_provider::ProviderResult<
        std::collections::HashMap<reth_primitives::B256, reth_primitives::Bytes>
    > {
        self.0.latest()?.witness(input, target)
    }

    fn multiproof(
        &self,
        input: TrieInput,
        targets: std::collections::HashMap<
            reth_primitives::B256,
            std::collections::HashSet<reth_primitives::B256>
        >
    ) -> reth_provider::ProviderResult<MultiProof> {
        self.0.latest()?.multiproof(input, targets)
    }
}
