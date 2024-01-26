use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    id::BasicIdBuilder,
    BonsaiStorage, BonsaiStorageConfig,
};

use rust_ffi::Command;
// use rust_ffi::{get_test_cases};

use bitvec::prelude::*;
use starknet_types_core::{felt::Felt, hash::Pedersen};

// use crate::shared_tree::SharedTree;

mod command_interpreter;
//mod shared_tree;

fn main() {
    let config = BonsaiStorageConfig::default();
    let db1 = create_rocks_db("./rocksdb").unwrap();

    // Create a BonsaiStorage with default parameters.
    let mut id_builder = BasicIdBuilder::new();
    let database = RocksDB::new(&db1, RocksDBConfig::default());
    let mut bonsai_storage: BonsaiStorage<_, _, Pedersen> =
        BonsaiStorage::new(database, config.clone()).unwrap();

    let command_list = rust_ffi::get_test1();

    let commands = unsafe {
        std::slice::from_raw_parts(
            command_list.test_commands as *mut Command,
            command_list.len,
        )
    };

    for command in commands {
        let _ = command_interpreter::run_command(
            &command,
            &mut id_builder,
            &mut bonsai_storage,
        )
        .unwrap();
    }
    rust_ffi::free_test(command_list);
}
