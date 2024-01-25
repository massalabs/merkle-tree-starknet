use std::ffi::CStr;

use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage, BonsaiStorageConfig,
};

use rust_ffi::{TestCommand, TestCommandList};
// use rust_ffi::{get_test_cases};

use bitvec::prelude::*;
use mp_felt::Felt252Wrapper;

use crate::shared_tree::SharedTree;

mod command_interpreter;
mod shared_tree;

fn main() {
    let config = BonsaiStorageConfig::default();
    let db1 = create_rocks_db("./rocksdb").unwrap();

    // Create a BonsaiStorage with default parameters.

    let database = RocksDB::new(&db1, RocksDBConfig::default());
    let mut bonsai_storage =
        BonsaiStorage::new(database, config.clone()).unwrap();

    let mut shared_tree = SharedTree::new(bonsai_storage);
    // insert the pair in the shared tree
    shared_tree
        .insert(vec![1, 2, 1], "0x66342762FDD54D033c195fec3ce2568b62052e")
        .unwrap();
    // Insert a second item `pair2`.
    shared_tree
        .insert(vec![1, 2, 2], "0x66342762FD54D033c195fec3ce2568b62052e")
        .unwrap();

    shared_tree.commit().unwrap();

    let db2 = create_rocks_db("./rocksdb2").unwrap();

    let database2 = RocksDB::new(&db2, RocksDBConfig::default());
    let mut bonsai_storage2 = BonsaiStorage::new(database2, config).unwrap();
    let mut share_tree2 = SharedTree::new(bonsai_storage2);

    let test1 = rust_ffi::get_test1();
    let commands = unsafe {
        std::slice::from_raw_parts(
            test1.test_commands as *mut TestCommand,
            test1.len,
        )
        .to_owned()
    };
    for command in commands {
        command_interpreter::run_command(&command, &mut share_tree2);
    }
    rust_ffi::free_test(test1);


    assert_eq!(
        shared_tree.bonsai_storage.root_hash().unwrap(),
        share_tree2.bonsai_storage.root_hash().unwrap()
    );
    println!("root: {:#?}", shared_tree.bonsai_storage.root_hash());
    println!("root: {:#?}", share_tree2.bonsai_storage.root_hash());
}

fn main_old() {
    // let mut shared_tree = SharedTree::new();

    // // insert the pair in the shared tree
    // shared_tree
    //     .insert(vec![1, 2, 1], "0x66342762FDD54D033c195fec3ce2568b62052e")
    //     .unwrap();

    // // Insert a second item `pair2`.
    // shared_tree
    //     .insert(vec![1, 2, 2], "0x66342762FD54D033c195fec3ce2568b62052e")
    //     .unwrap();

    // // Commit the insertion of `pair1` and `pair2`.
    // let id1 = id_builder.new_id();
    // bonsai_storage.commit(id1);

    // // Insert a new item `pair3`.
    // let pair3 = (
    //     vec![1, 2, 2],
    //     Felt252Wrapper::from_hex_be("0x664D033c195fec3ce2568b62052e").unwrap(),
    // );
    // let bitvec = BitVec::from_vec(pair3.0.clone());
    // bonsai_storage.insert(&bitvec, &pair3.1).unwrap();

    // // Commit the insertion of `pair3`. Save the commit ID to the `revert_to_id`
    // // variable.
    // let revert_to_id = id_builder.new_id();
    // bonsai_storage.commit(revert_to_id);

    // // Remove `pair3`.
    // bonsai_storage.remove(&bitvec).unwrap();

    // // Commit the removal of `pair3`.
    // bonsai_storage.commit(id_builder.new_id());

    // // Print the root hash and item `pair1`.
    // println!("root: {:#?}", bonsai_storage.root_hash());

    // let val = bonsai_storage.get(&bitvec_1).unwrap();
    // println!("value: {:#?}", val);

    // assert_eq!(val.unwrap(), pair1.1);

    // // Revert the collection state back to the commit tagged by the
    // // `revert_to_id` variable.
    // bonsai_storage.revert_to(revert_to_id).unwrap();

    // // Print the root hash and item `pair3`.
    // println!("root: {:#?}", bonsai_storage.root_hash());
    // println!("value: {:#?}", bonsai_storage.get(&bitvec).unwrap());

    let data = vec![1, 2, 3, 4, 5];
    //let container = rust_ffi::TestModule::GenericSliceContainer { data };

    // let result = rust_ffi::get_test(3);

    let tests = rust_ffi::get_test_cases();
    for id in tests.test_cases.into_iter() {
        println!("{:?}", id);
        // let id = rust_ffi::TestId::Test1;
        let commands: TestCommandList = rust_ffi::get_test(id);

        // Access elements using raw pointer
        let command_array = unsafe {
            std::slice::from_raw_parts(commands.test_commands, commands.len)
                .to_owned()
        };

        for command in command_array {
            println!("command: {:?}", &command);
        }
    }

    /*     let test2: TestCommandList = rust_ffi::get_test2();

       let command_array = unsafe {
           std::slice::from_raw_parts(
               test2.test_commands as *mut TestCommand, //&mut [&mut TestCommand; 2]
               test2.len,
           )
           .to_owned()
       };

       for command in command_array {
           let cmd = command.command;
           let arg1 = unsafe { CString::from_raw(command.arg1 as *mut i8) };
           let arg2 = unsafe { CString::from_raw(command.arg2 as *mut i8) };
           println!("command2: {:?} {:?} {:?}", cmd, arg1, arg2);
       }
       rust_ffi::free_test(test2);
    */

    // let leak: *mut VecCommands = rust_ffi::leak();
    // rust_ffi::destroy_leak(leak);

    // Utiliser la méthode as_slice
    //let slice_ref: &[i32] = container.as_slice();

    // // Launch two threads that will simultaneously take transactional states
    // to the commit identified by `id1`, // asserting in both of them that
    // the item `pair1` is present and has the right value.
    // std::thread::scope(|s| {
    //     s.spawn(|| {
    //         let bonsai_at_txn = bonsai_storage
    //             .get_transactional_state(id1, bonsai_storage.get_config())
    //             .unwrap()
    //             .unwrap();
    //         let bitvec = BitVec::from_vec(pair1.0.clone());
    //         assert_eq!(bonsai_at_txn.get(&bitvec).unwrap().unwrap(),
    // pair1.1);     });

    //     s.spawn(|| {
    //         let bonsai_at_txn = bonsai_storage
    //             .get_transactional_state(id1, bonsai_storage.get_config())
    //             .unwrap()
    //             .unwrap();
    //         let bitvec = BitVec::from_vec(pair1.0.clone());
    //         assert_eq!(bonsai_at_txn.get(&bitvec).unwrap().unwrap(),
    // pair1.1);     });
    // });

    // // Read item `pair1`.
    // let pair1_val = bonsai_storage
    //     .get(&BitVec::from_vec(vec![1, 2, 2]))
    //     .unwrap();

    // // Insert a new item and commit.
    // let pair4 = (
    //     vec![1, 2, 3],
    //     Felt252Wrapper::from_hex_be("
    // 0x66342762FDD54D033c195fec3ce2568b62052e").unwrap(), );
    // bonsai_storage
    //     .insert(&BitVec::from_vec(pair4.0.clone()), &pair4.1)
    //     .unwrap();
    // bonsai_storage.commit(id_builder.new_id()).unwrap();
    // let proof = bonsai_storage
    //     .get_proof(&BitVec::from_vec(pair3.0.clone()))
    //     .unwrap();
    // assert_eq!(
    //     BonsaiStorage::<BasicId, RocksDB<BasicId>>::verify_proof(
    //         bonsai_storage.root_hash().unwrap(),
    //         &BitVec::from_vec(pair3.0.clone()),
    //         pair3.1,
    //         &proof
    //     ),
    //     Some(Membership::Member)
    // );

    println!("You are the boss !, Well Done !");
}
