use bonsai_trie::{
    databases::{RocksDB, create_rocks_db, RocksDBConfig},
    BonsaiStorageError,
    id::{BasicIdBuilder, BasicId},
    BonsaiStorage, BonsaiStorageConfig, BonsaiTrieHash,
    ProofNode, Membership
};



fn main() {
     // Get the underlying key-value store.
     let db = create_rocks_db("./rocksdb").unwrap();

     // Create a BonsaiStorage with default parameters.
     let config = BonsaiStorageConfig::default();
     let mut bonsai_storage = BonsaiStorage::new(RocksDB::new(&db, RocksDBConfig::default()), config).unwrap();

     // Create a simple incremental ID builder for commit IDs.
     // This is not necessary, you can use any kind of strictly monotonically increasing value to tag your commits.
     let mut id_builder = BasicIdBuilder::new();

    println!("Hello, world!");
}

