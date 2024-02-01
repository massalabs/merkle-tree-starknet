use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    BonsaiStorage, BonsaiStorageConfig,
};

use command_interpreter::run_test;

// use rust_ffi::{get_test_cases};

use bitvec::prelude::*;
use starknet_types_core::hash::Pedersen;

// use crate::shared_tree::SharedTree;

mod command_interpreter;
//mod shared_tree;

fn main() {
    let config = BonsaiStorageConfig::default();
    let db1 = create_rocks_db("./rocksdb").unwrap();

    // Create a BonsaiStorage with default parameters.
    let database = RocksDB::new(&db1, RocksDBConfig::default());
    let bonsai_storage: BonsaiStorage<_, _, Pedersen> =
        BonsaiStorage::new(database, config.clone()).unwrap();

    // let command_list = rust_ffi::get_test1();
    // run_test(&command_list, bonsai_storage);
    // rust_ffi::free_test(command_list);

    let command_list = rust_ffi::get_test3();
    run_test(&command_list, bonsai_storage);
    rust_ffi::free_test(command_list);
}
