// Allows us to impl revm::DatabaseRef on the default provider type.
use reth_provider::{
    AccountReader, BlockHashReader, BlockIdReader, BlockNumReader, StateProofProvider,
    StateProvider, StateProviderFactory 
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
// impl<DB> BlockHashReader for RethDbWrapper<DB>
// where
//     DB: StateProviderFactory + Unpin + Clone + 'static
// {
//     fn block_hash(
//         &self,
//         number: reth_primitives::BlockNumber
//     ) -> reth_provider::ProviderResult<Option<reth_primitives::B256>> {
//         self.0.block_hash(number)
//     }
//
//     fn canonical_hashes_range(
//         &self,
//         start: reth_primitives::BlockNumber,
//         end: reth_primitives::BlockNumber
//     ) -> reth_provider::ProviderResult<Vec<reth_primitives::B256>> {
//         self.0.canonical_hashes_range(start, end)
//     }
//
//     fn convert_block_hash(
//         &self,
//         hash_or_number: reth_primitives::BlockHashOrNumber
//     ) -> reth_provider::ProviderResult<Option<reth_primitives::B256>> {
//         self.0.convert_block_hash(hash_or_number)
//     }
// }

impl<DB> BlockNumReader for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn chain_info(&self) -> reth_provider::ProviderResult<ChainInfo> {
        self.0.chain_info()
    }

    fn block_number(
        &self,
        hash: reth_primitives::B256
    ) -> reth_provider::ProviderResult<Option<reth_primitives::BlockNumber>> {
        self.0.block_number(hash)
    }

    fn convert_number(
        &self,
        id: reth_primitives::BlockHashOrNumber
    ) -> reth_provider::ProviderResult<Option<reth_primitives::B256>> {
        self.0.convert_block_number(num)
    }

    fn best_block_number(&self) -> reth_provider::ProviderResult<reth_primitives::BlockNumber> {
        self.0.best_block_number()
    }

    fn last_block_number(&self) -> reth_provider::ProviderResult<reth_primitives::BlockNumber> {
        self.0.last_block_number()
    }

    fn convert_hash_or_number(
        &self,
        id: reth_primitives::BlockHashOrNumber
    ) -> reth_provider::ProviderResult<Option<reth_primitives::BlockNumber>> {
        self.0.convert_hash_or_number(id)
    }
}

impl<DB> BlockIdReader for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn pending_block_num_hash(&self) -> ProviderResult<Option<reth_primitives::BlockNumHash>> {
        self.0.pending_block_num_hash()
    }

    fn safe_block_num_hash(&self) -> ProviderResult<Option<reth_primitives::BlockNumHash>> {
        self.0.safe_block_num_hash()
    }

    fn finalized_block_num_hash(&self) -> ProviderResult<Option<reth_primitives::BlockNumHash>> {
        self.0.finalized_block_num_hash()
    }
}

impl<DB> StateProviderFactory for RethDbWrapper<DB>
where
    DB: StateProviderFactory + Unpin + Clone + 'static
{
    fn latest(&self) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.latest()
    }

    fn pending(&self) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.pending()
    }

    fn state_by_block_id(
        &self,
        block_id: reth_primitives::BlockId
    ) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.state_by_block_id(block_id)
    }

    fn state_by_block_hash(
        &self,
        block: reth_primitives::BlockHash
    ) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.state_by_block_hash(block)
    }

    fn history_by_block_hash(
        &self,
        block: reth_primitives::BlockHash
    ) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.history_by_block_hash(block)
    }

    fn pending_state_by_hash(
        &self,
        block_hash: reth_primitives::B256
    ) -> reth_provider::ProviderResult<Option<reth_provider::StateProviderBox>> {
        self.0.pending_state_by_hash(block_hash)
    }

    fn state_by_block_number_or_tag(
        &self,
        number_or_tag: reth_primitives::BlockNumberOrTag
    ) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.state_by_block_number_or_tag(number_or_tag)
    }

    fn history_by_block_number(
        &self,
        block: reth_primitives::BlockNumber
    ) -> reth_provider::ProviderResult<reth_provider::StateProviderBox> {
        self.0.history_by_block_number(block)
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
