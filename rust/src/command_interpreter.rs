use std::ffi::CStr;

use bitvec::vec::BitVec;
use bonsai_trie::{
    databases::RocksDB,
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage, BonsaiStorageError,
};
use rust_ffi::{Command, CommandId};
use starknet_types_core::{felt::Felt, hash::Pedersen};

// use crate::SharedTree;

pub fn run_command<'a>(
    command: &Command,
    id_builder: &mut BasicIdBuilder,
    bonsai_storage: &mut BonsaiStorage<BasicId, RocksDB<'a, BasicId>, Pedersen>,
) -> Result<Option<String>, BonsaiStorageError> {
    match command.command {
        CommandId::Insert => {
            let key =
                unsafe { CStr::from_ptr(command.arg1) }.to_bytes().to_vec();
            let value =
                unsafe { CStr::from_ptr(command.arg2) }.to_str().unwrap();
            bonsai_storage
                .insert(&BitVec::from_vec(key), &Felt::from_hex(value).unwrap())
                .map(|_| None)
        }
        CommandId::Remove => {
            let key =
                unsafe { CStr::from_ptr(command.arg1) }.to_bytes().to_vec();
            bonsai_storage.remove(&BitVec::from_vec(key)).map(|_| None)
        }
        CommandId::Commit => {
            let id = id_builder.new_id();
            bonsai_storage.commit(id).map(|_| None)
        }
        CommandId::End => {
            unimplemented!("End")
        }
        CommandId::CheckRootHash => {
            let hash = bonsai_storage.root_hash().unwrap().to_hex_string();
            let ref_root_hash =
                unsafe { CStr::from_ptr(command.arg1).to_str().unwrap() };
            assert_eq!(&hash, ref_root_hash);
            println!("root: {:#?}", hash);
            println!("ref_root_hash: {:#?}", ref_root_hash);

            Ok(None)
        }
    }
}
