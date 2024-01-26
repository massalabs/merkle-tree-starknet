use std::ffi::CStr;

use bitvec::vec::BitVec;
use bonsai_trie::{
    databases::RocksDB, id::Id, BonsaiStorage, BonsaiStorageError,
};
use rust_ffi::{Command, CommandId};
use starknet_types_core::{felt::Felt, hash::Pedersen};

/// A basic ID type that can be used for testing.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct TestId(pub u64);

impl Id for TestId {
    fn serialize(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

pub fn run_command<'a>(
    command: &Command,
    bonsai_storage: &mut BonsaiStorage<TestId, RocksDB<'a, TestId>, Pedersen>,
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
            // let id = id_builder.new_id();
            let id = command.id();
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
        CommandId::RevertTo => {
            let id = command.id();
            println!("revert_to {:?}", id);
            bonsai_storage.revert_to(id)
        }
        CommandId::Get => {
            let key = command.key();
            let value = command.value();

            println!("get {:?} {}", key, value);
            assert_eq!(
                bonsai_storage.get(&BitVec::from_vec(key)).unwrap().unwrap(),
                Felt::from_hex(value).unwrap()
            );
            Ok(())
        }
        CommandId::Contains => {
            let key = command.key();
            let value = command.value();

            println!("contains {:?} {}", key, value);
            let res = bonsai_storage.contains(&BitVec::from_vec(key)).unwrap();
            assert_eq!(res, value.parse::<bool>().unwrap());
            Ok(())
        }
        CommandId::GetProof => {
            let key = command.key();
            let value = command.value();
            let v = value.to_string().split(',').map(|v| v.to_string()).collect::<Vec<String>>();

            println!("get_proof {:?} {}", key, value);
            let proof = bonsai_storage
                .get_proof(&BitVec::from_vec(key))
                .unwrap();

            for i in 0..proof.len() {
                assert_eq!(v[i], proof[i].hash::<Pedersen>().to_hex_string());
            }

            Ok(())
        },
        CommandId::VerifyProof => todo!(),
    }
}

trait CommandTrait {
    fn key(&self) -> Vec<u8>;
    fn value(&self) -> &str;
    fn id(&self) -> TestId;
}

impl CommandTrait for Command {
    fn value(&self) -> &str {
        match self.id {
            CommandId::Insert | CommandId::Contains | CommandId::Get | CommandId::GetProof => {
                unsafe { CStr::from_ptr(self.arg2) }.to_str().unwrap()
            }
            CommandId::CheckRootHash => {
                unsafe { CStr::from_ptr(self.arg1) }.to_str().unwrap()
            }
            _ => unimplemented!("Command has no value"),
        }
    }

    fn key(&self) -> Vec<u8> {
        match self.id {
            CommandId::Remove | CommandId::Insert | CommandId::Contains | CommandId::Get | CommandId::GetProof => {
                unsafe { CStr::from_ptr(self.arg1) }.to_bytes().to_vec()
            }
            _ => unimplemented!("Command has no key"),
        }
    }

    fn id(&self) -> TestId {
        match self.id {
            CommandId::Commit | CommandId::RevertTo => {
                let id =
                    { unsafe { CStr::from_ptr(self.arg1) }.to_str().unwrap() }
                        .parse::<u64>()
                        .unwrap();
                TestId(id)
            }
            _ => unimplemented!("Command has no id"),
        }
    }
}
pub fn run_test(
    command_list: &rust_ffi::CommandList,

    mut bonsai_storage: BonsaiStorage<TestId, RocksDB<'_, TestId>, Pedersen>,
) {
    let commands = unsafe {
        std::slice::from_raw_parts(
            command_list.test_commands as *mut Command,
            command_list.len,
        )
    };

    for command in commands {
        let _ = run_command(&command, &mut bonsai_storage).unwrap();
    }
}
