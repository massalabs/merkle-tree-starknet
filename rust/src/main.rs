use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    BonsaiStorage, BonsaiStorageConfig,
};
use command_interpreter_bonsai::run_bonsai_test;
use command_interpreter_path_finder::{
    run_pathfinder_test, TestStorage, TestTree,
};
use std::ffi::CString;

// use rust_ffi::{get_test_cases};

use bitvec::prelude::*;
use starknet_types_core::hash::Pedersen;

mod command_interpreter_bonsai;
mod command_interpreter_path_finder;

fn bonsai_main() {
    let config = BonsaiStorageConfig::default();
    let db1 = create_rocks_db("./rocksdb").unwrap();

    // Create a BonsaiStorage with default parameters.
    let database = RocksDB::new(&db1, RocksDBConfig::default());
    let bonsai_storage: BonsaiStorage<_, _, Pedersen> =
        BonsaiStorage::new(database, config.clone()).unwrap();

    // let command_list = rust_ffi::get_test1();
    // run_test(&command_list, bonsai_storage);
    // rust_ffi::free_test(command_list);
    // let c_string = CString::new("/home/jf/workspace/rust/starknet/merkle-tree-starknet/scenario/2_crash_the_proof.yml").expect("Failed to create CString");
    let c_string = CString::new(
        "/home/jf/workspace/rust/starknet/merkle-tree-starknet/scenario/2.yml",
    )
    .expect("Failed to create CString");

    // Leak the CString to ensure it lives long enough to be used from other
    // languages
    let scenario3 = c_string.into_raw();

    // let scenario3 = "/home/jf/workspace/rust/starknet/merkle-tree-starknet/scenario/3.yml";
    let command_list = rust_ffi::load_scenario(scenario3);

    // let command_list = rust_ffi::get_test3();
    run_bonsai_test(&command_list, bonsai_storage);
    rust_ffi::free_test(command_list);
    // free leak of the file path
    let _ = unsafe { CString::from_raw(scenario3) };
}

fn path_finder_main() {
    let mut tree = TestTree::empty();
    let mut storage = TestStorage::default();

    let c_string = CString::new(
        "/home/jf/workspace/rust/starknet/merkle-tree-starknet/scenario/2.yml",
    )
    .expect("Failed to create CString");

    // Leak the CString to ensure it lives long enough to be used from other
    // languages
    let scenario3 = c_string.into_raw();

    // let scenario3 = "/home/jf/workspace/rust/starknet/merkle-tree-starknet/scenario/3.yml";
    let command_list = rust_ffi::load_scenario(scenario3);

    run_pathfinder_test(&command_list, &mut tree, &mut storage);
}

fn main() {
    println!("\n### Bonsai");
    bonsai_main();
    println!("\n### PathFinder");
    path_finder_main();
}
