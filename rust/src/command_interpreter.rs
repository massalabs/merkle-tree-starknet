use std::ffi::CStr;

use bonsai_trie::{
    databases::{create_rocks_db, RocksDB, RocksDBConfig},
    id::BasicIdBuilder,
    BonsaiStorage, BonsaiStorageConfig,
};
use rust_ffi::{CommandId, TestCommand};

use crate::SharedTree;

pub fn run_command(command: &TestCommand, shared_tree: &mut SharedTree) {
    match command.command {
        CommandId::Insert => {
            let key =
                unsafe { CStr::from_ptr(command.arg1) }.to_bytes().to_vec();
            let value =
                unsafe { CStr::from_ptr(command.arg2) }.to_str().unwrap();
            shared_tree.insert(key, value).unwrap();
        }
        CommandId::Remove => {
            let key =
                unsafe { CStr::from_ptr(command.arg1) }.to_bytes().to_vec();
            shared_tree.remove(key).unwrap();
        }
        CommandId::Commit => {
            shared_tree.commit().unwrap();
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
