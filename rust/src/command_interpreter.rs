use std::ffi::CStr;

use bitvec::vec::BitVec;
use bonsai_trie::{
    databases::RocksDB,
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage,
};
use rust_ffi::{CommandId, TestCommand};
use starknet_types_core::{felt::Felt, hash::Pedersen};

// use crate::SharedTree;

pub fn run_command<'a>(
    command: &TestCommand,
    id_builder: &mut BasicIdBuilder,
    bonsai_storage: &mut BonsaiStorage<BasicId, RocksDB<'a, BasicId>, Pedersen>,
) {
    match command.command {
        CommandId::Insert => {
            let key =
                unsafe { CStr::from_ptr(command.arg1) }.to_bytes().to_vec();
            let value =
                unsafe { CStr::from_ptr(command.arg2) }.to_str().unwrap();
            bonsai_storage
                .insert(&BitVec::from_vec(key), &Felt::from_hex(value).unwrap())
                .unwrap();
        }
        CommandId::Remove => {
            let key =
                unsafe { CStr::from_ptr(command.arg1) }.to_bytes().to_vec();
            bonsai_storage.remove(&BitVec::from_vec(key)).unwrap();
        }
        CommandId::Commit => {
            let id = id_builder.new_id();
            bonsai_storage.commit(id).unwrap();
            // shared_tree.bonsai_storage.commit(id_builder.new_id());
        }
        CommandId::End => {
            unimplemented!("End")
        }
        CommandId::RootHash => {
            unimplemented!("RootHash")
        }
    }
}
