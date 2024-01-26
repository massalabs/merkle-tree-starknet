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
) -> Result<(), BonsaiStorageError> {
    match command.id {
        CommandId::Insert => {
            let key = command.key();
            let value = command.value();

            println!("insert {:?} {}", key, value);
            bonsai_storage
                .insert(&BitVec::from_vec(key), &Felt::from_hex(value).unwrap())
        }
        CommandId::Remove => {
            let key = command.key();
            println!("remove {:?}", key);
            bonsai_storage.remove(&BitVec::from_vec(key))
        }
        CommandId::Commit => {
            let id = id_builder.new_id();
            println!("commint {:?}", id);
            bonsai_storage.commit(id)
        }
        CommandId::CheckRootHash => {
            let hash = bonsai_storage.root_hash().unwrap().to_hex_string();
            let ref_root_hash = command.value();
            assert_eq!(&hash, ref_root_hash);
            println!("root: {:#?}", hash);
            println!("ref_root_hash: {:#?}", ref_root_hash);

            Ok(())
        }
    }
}

trait CommandTrait {
    fn key(&self) -> Vec<u8>;
    fn value(&self) -> &str;
}

impl CommandTrait for Command {
    fn value(&self) -> &str {
        match self.id {
            CommandId::Insert => {
                unsafe { CStr::from_ptr(self.arg2) }.to_str().unwrap()
            }
            CommandId::Remove => unimplemented!("Remove has no value"),
            CommandId::Commit => unimplemented!("Commit has no value"),
            CommandId::CheckRootHash => {
                unsafe { CStr::from_ptr(self.arg1) }.to_str().unwrap()
            }
        }
    }

    fn key(&self) -> Vec<u8> {
        match self.id {
            CommandId::Insert => {
                unsafe { CStr::from_ptr(self.arg1) }.to_bytes().to_vec()
            }
            CommandId::Remove => {
                unsafe { CStr::from_ptr(self.arg1) }.to_bytes().to_vec()
            }
            CommandId::Commit => unimplemented!("Commit has no key"),
            CommandId::CheckRootHash => {
                unimplemented!("CheckRootHash has no key")
            }
        }
    }
}
pub fn run_test(
    command_list: &rust_ffi::CommandList,
    mut id_builder: BasicIdBuilder,
    mut bonsai_storage: BonsaiStorage<
        bonsai_trie::id::BasicId,
        RocksDB<'_, bonsai_trie::id::BasicId>,
        Pedersen,
    >,
) {
    let commands = unsafe {
        std::slice::from_raw_parts(
            command_list.test_commands as *mut Command,
            command_list.len,
        )
    };

    for command in commands {
        let _ = run_command(&command, &mut id_builder, &mut bonsai_storage)
            .unwrap();
    }
}
