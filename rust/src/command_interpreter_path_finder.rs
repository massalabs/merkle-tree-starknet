use std::collections::HashMap;
use std::ffi::CStr;

use bitvec::vec::BitVec;
use rust_ffi::{Command, CommandId};
// use starknet_types_core::{felt::Felt};


use anyhow::Context;
use pathfinder_common::hash::PedersenHash;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::storage::Storage;
use pathfinder_merkle_tree::tree::MerkleTree;
use pathfinder_storage::{StoredNode, Node};

use super::*;

pub type TestTree = MerkleTree<PedersenHash, 251>;

#[derive(Default, Debug)]
pub struct TestStorage {
    nodes: HashMap<u64, (Felt, StoredNode)>,
    leaves: HashMap<Felt, Felt>,
}

impl Storage for TestStorage {
    fn get(&self, node: u64) -> anyhow::Result<Option<StoredNode>> {
        Ok(self.nodes.get(&node).map(|x| x.1.clone()))
    }

    fn hash(&self, node: u64) -> anyhow::Result<Option<Felt>> {
        Ok(self.nodes.get(&node).map(|x| x.0))
    }

    fn leaf(&self, path: &BitSlice<u8, Msb0>) -> anyhow::Result<Option<Felt>> {
        let key = Felt::from_bits(path).context("Mapping path to felt")?;

        Ok(self.leaves.get(&key).cloned())
    }
}

pub fn run_command<'a>(
    command: &Command,
    tree: &mut TestTree,
    storage: &mut TestStorage,
) {

    let _r = match command.id {
        CommandId::Insert => {
            let key = command.key();
            let value = command.value();
            println!("insert {:?} {}", &key, value);
            let key_bitvec = BitVec::from_vec(key);
            let felt = Felt::from_hex_str(&value).unwrap();

            tree.set(storage, key_bitvec, felt).unwrap();

            let _update = tree.to_owned().commit(storage).unwrap();
        }
        CommandId::Remove => {
            let key = command.key();
            let key_bitvec = BitVec::from_vec(key);
            println!("remove {:?}", key_bitvec);
            tree.set(storage, key_bitvec, Felt::ZERO).unwrap();
            let _update = tree.to_owned().commit(storage).unwrap();
        }

        CommandId::CheckRootHash => {
            let update = tree.to_owned().commit(storage).unwrap();
            println!("CheckRootHash = {:?}", update.root);
            assert_eq!(update.root, Felt::from_hex_str(&command.value()).unwrap());
        }

        CommandId::Get => {
            let key = command.key();
            let value = command.value();
            let result = tree.to_owned().get(storage, BitVec::from_vec(key));
            assert_eq!(Felt::from_hex_str(&value).unwrap(),result.unwrap().unwrap());
        }
        CommandId::Contains => {
            let key = command.key();
            let value = command.value();

            let result = tree.to_owned().get(storage, BitVec::from_vec(key));

            let exist = match result.unwrap() {
                Some(_) => true,
                None => false,
            };

            assert_eq!(exist, value.parse::<bool>().unwrap());
        }
    };
}

pub fn commit_and_persist(
    tree: TestTree,
    storage: &mut TestStorage,
) -> (Felt, u64) {
    use pathfinder_storage::Child;

    for (key, value) in &tree.leaves {
        let key = Felt::from_bits(key).unwrap();
        storage.leaves.insert(key, *value);
    }

    let update = tree.commit(storage).unwrap();

    let mut indices = HashMap::new();
    let mut idx = storage.nodes.len();
    for hash in update.nodes.keys() {
        indices.insert(*hash, idx as u64);
        idx += 1;
    }

    for (hash, node) in update.nodes {
        let node = match node {
            Node::Binary { left, right } => {
                let left = match left {
                    Child::Id(idx) => idx,
                    Child::Hash(hash) => *indices
                        .get(&hash)
                        .expect("Left child should have an index"),
                };

                let right = match right {
                    Child::Id(idx) => idx,
                    Child::Hash(hash) => *indices
                        .get(&hash)
                        .expect("Right child should have an index"),
                };

                StoredNode::Binary { left, right }
            }
            Node::Edge { child, path } => {
                let child = match child {
                    Child::Id(idx) => idx,
                    Child::Hash(hash) => *indices
                        .get(&hash)
                        .expect("Child should have an index"),
                };

                StoredNode::Edge { child, path }
            }
            Node::LeafBinary => StoredNode::LeafBinary,
            Node::LeafEdge { path } => StoredNode::LeafEdge { path },
        };

        storage
            .nodes
            .insert(*indices.get(&hash).unwrap(), (hash, node));
    }

    let index = *indices.get(&update.root).unwrap();

    (update.root, index)
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
            // | CommandId::GetProof
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
pub fn run_pathfinder_test(
    command_list: &rust_ffi::CommandList,
    tree: &mut TestTree,
    storage: &mut TestStorage,
) {
    let commands = unsafe {
        std::slice::from_raw_parts(
            command_list.test_commands as *mut Command,
            command_list.len,
        )
    };

    for command in commands {
        let _ = run_command(&command, tree, storage);
    }
}
