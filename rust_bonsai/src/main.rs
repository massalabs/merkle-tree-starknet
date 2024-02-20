use anyhow::{anyhow, Result};
use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage, BonsaiStorageConfig,
};
use rocksdb::OptimisticTransactionDB;
use rust_common::{
    init_runner,
    tree::{Key, TestTree, Value},
};
use starknet_types_core::{felt, hash::Pedersen};

struct TestEnvironment<'db> {
    pub tree: BonsaiStorage<BasicId, RocksDB<'db, BasicId>, Pedersen>,
    id_builder: BasicIdBuilder,
    // seems to prevent a leak do not remove it
    _storage: Box<OptimisticTransactionDB>,
}

impl<'db> TestEnvironment<'db> {
    fn new() -> Self {
        let storage = Box::new(create_rocks_db("./rocksdb").unwrap());
        // leak the box so we can pass a ref to RockDB::new()
        let storage_ptr: *const OptimisticTransactionDB =
            Box::into_raw(storage);
        let tree: BonsaiStorage<BasicId, RocksDB<'_, BasicId>, Pedersen> =
            BonsaiStorage::new(
                // safe because we have just created the storage above
                unsafe {
                    RocksDB::new(&*storage_ptr, RocksDBConfig::default())
                },
                BonsaiStorageConfig::default().clone(),
            )
            .unwrap();

        TestEnvironment {
            tree,
            id_builder: BasicIdBuilder::new(),
            // rebuild the box from the pointer, so it can be dropped and
            // prevent a leak dropping the pointer manually would be prevent
            // leak but causes a segfault when RosckDB is dropped "later"
            // (maybe thread related because even if the tree is dropped
            // before the pointer it still segfaults.
            _storage: unsafe {
                Box::from_raw(storage_ptr as *mut OptimisticTransactionDB)
            },
        }
    }

    fn id(&mut self) -> BasicId {
        self.id_builder.new_id()
    }
}

impl TestTree for TestEnvironment<'_> {
    fn insert(&mut self, key: &Key, value: &Value) -> Result<()> {
        let felt: [u8; 32] = value[..32]
            .try_into()
            .map_err(|_| anyhow!("Error converting value to [u8; 32]"))?;

        let felt = felt::Felt::from_bytes_be(&felt);
        self.tree.insert(key, &felt).map_err(|e| anyhow!(e))?;
        self.commit()?;
        Ok(())
    }

    fn remove(&mut self, key: &Key) -> Result<()> {
        self.tree.remove(key).map_err(|e| anyhow!(e))?;
        self.commit()?;
        Ok(())
    }

    fn get(&self, key: &Key) -> Result<Value> {
        Ok(self
            .tree
            .get(key)
            .map_err(|e| anyhow!(e))
            .and_then(|x| {
                x.ok_or_else(|| {
                    anyhow!(format!("key not found on get : {:?}", key))
                })
            })?
            .to_bytes_be()
            .to_vec())
    }

    fn contains(&self, key: &Key) -> Result<bool> {
        self.tree
            .get(key)
            .map_err(|e| anyhow!(e))
            .map(|x| x.is_some())
    }

    fn commit(&mut self) -> Result<Option<Vec<u8>>> {
        let id = self.id();
        self.tree.commit(id).map_err(|e| anyhow!(e))?;
        Ok(None)
    }

    fn root_hash(&mut self) -> Result<Value> {
        Ok(self
            .tree
            .root_hash()
            .map_err(|e| anyhow!(e))?
            .to_bytes_be()
            .to_vec())
    }
}

fn main() {
    let result = init_runner(|| Box::new(TestEnvironment::new()));

    log::info!("Bonsai | Results {:?}", result);
}
