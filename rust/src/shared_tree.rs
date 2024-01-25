use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage, BonsaiStorageConfig, BonsaiStorageError,
};

use starknet_types_core::hash::StarkHash;
// use crate::rock::OptimisticTransactionDB;

// use crate::rocksdb::OptimisticTransactionDB;
use bitvec::prelude::*;
use mp_felt::Felt252Wrapper;

pub struct SharedTree<'a> {
    pub bonsai_storage: BonsaiStorage<BasicId, RocksDB<'a, BasicId>, StarkHash>,
    pub id_builder: BasicIdBuilder,
}

impl<'a> SharedTree<'a> {
    // Create a new shared tree.
    pub fn new(
        bonsai_storage: BonsaiStorage<BasicId, RocksDB<'a, BasicId>, Pedersen>,
    ) -> Self {
        // Get the underlying key-value store.
        // let db = create_rocks_db("./rocksdb").unwrap();

        // // Create a BonsaiStorage with default parameters.
        // let config = BonsaiStorageConfig::default();
        // let database = RocksDB::new(&db, RocksDBConfig::default());
        // let mut bonsai_storage =
        //     BonsaiStorage::new(database, config)
        //         .unwrap();

        // Create a simple incremental ID builder for commit IDs.
        // This is not necessary, you can use any kind of strictly monotonically
        // increasing value to tag your commits.
        let id_builder = BasicIdBuilder::new();

        SharedTree {
            bonsai_storage,
            id_builder,
        }
    }

    // Insert the key-value pair into the tree.
    pub fn insert(
        &mut self,
        k: Vec<u8>,
        value: &str,
    ) -> Result<(), BonsaiStorageError> {
        let key = BitVec::from_vec(k);

        let val = &Felt252Wrapper::from_hex_be(value).map_err(|e| {
            BonsaiStorageError::Trie("Error Felt252".to_string())
        })?;

        self.bonsai_storage.insert(&key, val)
    }

    pub fn remove(&mut self, k: Vec<u8>) -> Result<(), BonsaiStorageError> {
        let key = BitVec::from_vec(k);
        self.bonsai_storage.remove(&key)
    }

    pub fn commit(&mut self) -> Result<BasicId, BonsaiStorageError> {
        let id = self.id_builder.new_id();
        self.bonsai_storage.commit(id.clone())?;
        Ok(id)
    }
}
