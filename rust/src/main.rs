use std::ffi::CString;

use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage, BonsaiStorageConfig, BonsaiStorageError, BonsaiTrieHash,
    Membership, ProofNode,
};

use rust_ffi::{
    get_string, TestCommand2, TestCommandList, TestCommandList2, TestId,
};
// use rust_ffi::{get_test_cases};

use bitvec::prelude::*;
use mp_felt::Felt252Wrapper;

fn main() {
    // Get the underlying key-value store.
    let db = create_rocks_db("./rocksdb").unwrap();

    // Create a BonsaiStorage with default parameters.
    let config = BonsaiStorageConfig::default();
    let mut bonsai_storage =
        BonsaiStorage::new(RocksDB::new(&db, RocksDBConfig::default()), config)
            .unwrap();

    // Create a simple incremental ID builder for commit IDs.
    // This is not necessary, you can use any kind of strictly monotonically increasing value to tag your commits.
    let mut id_builder = BasicIdBuilder::new();

    // let _hello = Felt252Wrapper::from_dec_str("hello").unwrap();
    // let _world = Felt252Wrapper::from_dec_str("world").unwrap();

    // Insert an item `pair1`.
    let pair1 = (
        vec![1, 2, 1],
        Felt252Wrapper::from_hex_be("0x66342762FDD54D033c195fec3ce2568b62052e")
            .unwrap(),
    );
    let bitvec_1 = BitVec::from_vec(pair1.0.clone());
    bonsai_storage.insert(&bitvec_1, &pair1.1).unwrap();

    // Insert a second item `pair2`.
    // let pair2 = (
    //     vec![1, 2, 2],
    //     Felt252Wrapper::from_hex_be("0x66342762FD54D033c195fec3ce2568b62052e")
    //         .unwrap(),
    // );
    let bitvec = BitVec::from_vec(vec![1, 2, 2]);
    bonsai_storage
        .insert(
            &bitvec,
            &Felt252Wrapper::from_hex_be(
                "0x66342762FD54D033c195fec3ce2568b62052e",
            )
            .unwrap(),
        )
        .unwrap();

    // Commit the insertion of `pair1` and `pair2`.
    let id1 = id_builder.new_id();
    bonsai_storage.commit(id1);

    // Insert a new item `pair3`.
    let pair3 = (
        vec![1, 2, 2],
        Felt252Wrapper::from_hex_be("0x664D033c195fec3ce2568b62052e").unwrap(),
    );
    let bitvec = BitVec::from_vec(pair3.0.clone());
    bonsai_storage.insert(&bitvec, &pair3.1).unwrap();

    // Commit the insertion of `pair3`. Save the commit ID to the `revert_to_id` variable.
    let revert_to_id = id_builder.new_id();
    bonsai_storage.commit(revert_to_id);

    // Remove `pair3`.
    bonsai_storage.remove(&bitvec).unwrap();

    // Commit the removal of `pair3`.
    bonsai_storage.commit(id_builder.new_id());

    // Print the root hash and item `pair1`.
    println!("root: {:#?}", bonsai_storage.root_hash());

    let val = bonsai_storage.get(&bitvec_1).unwrap();
    println!("value: {:#?}", val);

    assert_eq!(val.unwrap(), pair1.1);

    // Revert the collection state back to the commit tagged by the `revert_to_id` variable.
    bonsai_storage.revert_to(revert_to_id).unwrap();

    // Print the root hash and item `pair3`.
    println!("root: {:#?}", bonsai_storage.root_hash());
    println!("value: {:#?}", bonsai_storage.get(&bitvec).unwrap());

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

    let test2: TestCommandList2 = rust_ffi::get_test2();

    let command_array = unsafe {
        std::slice::from_raw_parts(
            test2.test_commands as *mut TestCommand2, //&mut [&mut TestCommand2; 2]
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

    // Utiliser la méthode as_slice
    //let slice_ref: &[i32] = container.as_slice();

    // // Launch two threads that will simultaneously take transactional states to the commit identified by `id1`,
    // // asserting in both of them that the item `pair1` is present and has the right value.
    // std::thread::scope(|s| {
    //     s.spawn(|| {
    //         let bonsai_at_txn = bonsai_storage
    //             .get_transactional_state(id1, bonsai_storage.get_config())
    //             .unwrap()
    //             .unwrap();
    //         let bitvec = BitVec::from_vec(pair1.0.clone());
    //         assert_eq!(bonsai_at_txn.get(&bitvec).unwrap().unwrap(), pair1.1);
    //     });

    //     s.spawn(|| {
    //         let bonsai_at_txn = bonsai_storage
    //             .get_transactional_state(id1, bonsai_storage.get_config())
    //             .unwrap()
    //             .unwrap();
    //         let bitvec = BitVec::from_vec(pair1.0.clone());
    //         assert_eq!(bonsai_at_txn.get(&bitvec).unwrap().unwrap(), pair1.1);
    //     });
    // });

    // // Read item `pair1`.
    // let pair1_val = bonsai_storage
    //     .get(&BitVec::from_vec(vec![1, 2, 2]))
    //     .unwrap();

    // // Insert a new item and commit.
    // let pair4 = (
    //     vec![1, 2, 3],
    //     Felt252Wrapper::from_hex_be("0x66342762FDD54D033c195fec3ce2568b62052e").unwrap(),
    // );
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
