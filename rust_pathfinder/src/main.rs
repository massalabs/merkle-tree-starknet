use anyhow::{Context, Result};
use bitvec::{order::Msb0, prelude::BitSlice};
use pathfinder_common::hash::PedersenHash;
use pathfinder_crypto::Felt;
use pathfinder_merkle_tree::{storage::Storage, tree::MerkleTree};
use pathfinder_storage::StoredNode;
use rust_common::{
    init_runner,
    tree::{Key, TestTree, Value},
};
use std::collections::HashMap;

extern crate log;

type TreeType = MerkleTree<PedersenHash, 251>;

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

pub struct TestEnvironment {
    pub tree: TreeType,
    pub storage: TestStorage,
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self {
            tree: TreeType::empty(),
            storage: TestStorage::default(),
        }
    }
}

impl TestTree for TestEnvironment {
    fn insert(&mut self, key: &Key, value: &Value) -> Result<()> {
        self.tree.set(
            &self.storage,
            key.to_owned(),
            Felt::from_be_slice(value)?,
        )?;
        self.commit()?;
        Ok(())
    }

    fn remove(&mut self, key: &Key) -> Result<()> {
        // println!("remove {}", key);
        self.insert(key, &Felt::ZERO.as_be_bytes().to_vec())?;
        self.commit()?;
        Ok(())
    }

    fn get(&self, key: &Key) -> Result<Value> {
        // println!("get {}", key);
        Ok(self
            .tree
            .to_owned()
            .get(&self.storage, key.to_owned())?
            .map_or(vec![], |x| x.as_be_bytes().to_vec()))
    }

    fn contains(&self, key: &Key) -> Result<bool> {
        // println!("contains {}", key);
        self.get(key).map(|x| x.iter().any(|x| x != &0))
    }

    fn commit(&mut self) -> Result<Option<Vec<u8>>> {
        Ok(Some(
            self.tree
                .to_owned()
                .commit(&self.storage)?
                .root
                .as_be_bytes()
                .to_vec(),
        ))
    }

    fn root_hash(&mut self) -> Result<Value> {
        self.commit()?.ok_or(anyhow::Error::msg("No root hash"))
    }
}

fn main() {
    let result = init_runner(|| Box::new(TestEnvironment::new()));

    log::info!("Pathfinder | Results {:?}", result);
}
