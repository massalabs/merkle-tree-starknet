use std::ffi::CStr;

use bitvec::prelude::*;

use bitvec::{order::Msb0, vec::BitVec};
use bonsai_trie::{
    databases::RocksDB,
    id::Id,
    id::{BasicId, BasicIdBuilder},
    BonsaiStorage,
};
use rust_ffi::{Command, CommandId};
use starknet_types_core::{felt::Felt, hash::Pedersen};

/// A basic ID type that can be used for testing.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct TestId(pub u64);

impl Id for TestId {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

fn extend_key_251(key: BitVec<u8, Msb0>) -> BitVec<u8, Msb0> {
    //
    // let key: BitVec<u8, Msb0> = BitVec::from_vec(val.to_be_bytes().to_vec());

    let mut key251: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0; 251];
    key251.truncate(key251.len() - key.len());

    key251.extend(key.iter());
    // println!("{:?}", &key251);
    key251
}

pub fn run_command<'a>(
    command: &Command,
    bonsai_storage: &mut BonsaiStorage<BasicId, RocksDB<'a, BasicId>, Pedersen>,
) {
    let mut id_builder = BasicIdBuilder::new();
    let _r = match command.id {
        CommandId::Insert => {
            let key = command.key();
            let value = command.value();

            println!("insert {:?} {}", key, value);
            let key_bitvec = BitVec::from_vec(key);
            let key_bitvec = extend_key_251(key_bitvec);

            // println!("key_bitvec: {:#?}", key_bitvec);
            let felt = Felt::from_hex(&value).unwrap();
            // println!("felt: {:#?}", felt);
            bonsai_storage.insert(&key_bitvec, &felt).unwrap();
            bonsai_storage.commit(id_builder.new_id())
        }
        CommandId::Remove => {
            let key = command.key();
            println!("remove {:?}", key);

            let key_bitvec = BitVec::from_vec(key);
            let key_bitvec = extend_key_251(key_bitvec);

            bonsai_storage.remove(&key_bitvec).unwrap();
            bonsai_storage.commit(id_builder.new_id())
        }

        CommandId::CheckRootHash => {
            let hash = bonsai_storage.root_hash().unwrap().to_hex_string();
            let ref_root_hash = command.value();
            assert_eq!(&hash, &ref_root_hash);
            println!("CheckRootHash: {:#?}", ref_root_hash);

            Ok(())
        }
        CommandId::Get => {
            let key = command.key();
            let value = command.value();

            println!("get {:?} {}", key, value);
            let key_bitvec = BitVec::from_vec(key);
            let key_bitvec = extend_key_251(key_bitvec);

            let res = bonsai_storage.get(&key_bitvec).unwrap().unwrap();
            // println!("res: {:#?}", res);
            assert_eq!(res, Felt::from_hex(&value).unwrap());
            Ok(())
        }
        CommandId::Contains => {
            let key = command.key();
            let value = command.value();

            println!("contains {:?} {}", key, value);
            let key_bitvec = BitVec::from_vec(key);
            let key_bitvec = extend_key_251(key_bitvec);

            let res = bonsai_storage.contains(&key_bitvec).unwrap();
            assert_eq!(res, value.parse::<bool>().unwrap());
            Ok(())
        }
    };
}

trait CommandTrait {
    fn key(&self) -> Vec<u8>;
    fn value(&self) -> String;
    fn get_arg1(&self) -> String;
}

impl CommandTrait for Command {
    fn value(&self) -> String {
        match self.id {
            CommandId::Insert
            | CommandId::Contains
            // | CommandId::GetProof
            | CommandId::Get => unsafe { CStr::from_ptr(self.arg2) }
                .to_str()
                .unwrap()
                .to_string(),
            CommandId::CheckRootHash => self.get_arg1(),
            _ => unimplemented!("Command has no value"),
        }
    }

    fn get_arg1(&self) -> String {
        let arr = unsafe {
            std::slice::from_raw_parts(self.arg1.ptr as *mut u8, self.arg1.len)
        };

        String::from_utf8(arr.to_vec()).unwrap()
    }

    fn key(&self) -> Vec<u8> {
        match self.id {
            CommandId::Remove
            | CommandId::Insert
            | CommandId::Contains
            | CommandId::Get => {
                let arr = unsafe {
                    std::slice::from_raw_parts(
                        self.arg1.ptr as *mut u8,
                        self.arg1.len,
                    )
                };
                arr.to_vec()
            }
            _ => unimplemented!("Command has no key"),
        }
    }
}
pub fn run_bonsai_test(
    command_list: &rust_ffi::CommandList,

    mut bonsai_storage: BonsaiStorage<BasicId, RocksDB<'_, BasicId>, Pedersen>,
) {
    let commands = unsafe {
        std::slice::from_raw_parts(
            command_list.test_commands as *mut Command,
            command_list.len,
        )
    };

    for command in commands {
        let _ = run_command(&command, &mut bonsai_storage);
    }
}
