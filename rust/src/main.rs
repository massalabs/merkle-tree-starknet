use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    BonsaiStorage, BonsaiStorageConfig,
};
use command_interpreter_bonsai::run_bonsai_test;
use command_interpreter_path_finder::{
    run_pathfinder_test, TestStorage, TestTree,
};
use std::{ffi::CString, io::Write};

extern crate log;
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

fn print_root(tree: &TestTree, storage: &TestStorage) {
    let mut buf = [0; 128];
    match tree.to_owned().commit(storage) {
        Ok(update) => {
            println!("root {}", update.root.as_hex_str(&mut buf));
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
/// Create a key of 251 bits from a u64
fn make_key_251(val: u64) -> BitVec<u8, Msb0> {
    //
    let key: BitVec<u8, Msb0> = BitVec::from_vec(val.to_be_bytes().to_vec());

    let mut key251: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0; 251];
    key251.truncate(key251.len() - key.len());

    key251.extend(key.iter());
    println!("{:?}", &key251);
    key251
}

fn path_finder_minimal_test() {
    let mut tree: pathfinder_merkle_tree::tree::MerkleTree<
        pathfinder_common::hash::PedersenHash,
        251,
    > = TestTree::empty();
    let storage = TestStorage::default();

    let value1 = pathfinder_crypto::Felt::from(1u64);
    let value2 = pathfinder_crypto::Felt::from(2u64);

    dbg!(&value1);
    dbg!(&value2);


    let keys: Vec<BitVec<u8, Msb0>> =
        (0..10).map(|x| make_key_251(x)).collect();

    tree.set(&storage, keys[0].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[1].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[2].clone(), value2).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[3].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[4].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[5].clone(), value2).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[6].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[7].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[8].clone(), value1).unwrap();
    print_root(&tree, &storage);

    tree.set(&storage, keys[9].clone(), value2).unwrap();
    print_root(&tree, &storage);

    // println!("{:?}", tree);
    // dbg!(&tree);

    // let (felt, root_idx) = commit_and_persist(tree.to_owned(), storage);
    // pathfinder_merkle_tree::tree::MerkleTree::get_proof(
    //     0,
    //     &storage,
    // )
    // .unwrap();
}

fn main() {
    path_finder_minimal_test()

    // println!("\n### Bonsai");
    // bonsai_main();
    // println!("\n### PathFinder");
    // path_finder_main();
}
