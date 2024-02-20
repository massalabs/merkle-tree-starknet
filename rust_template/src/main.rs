use anyhow::{Context, Result};
use bitvec::{order::Msb0, prelude::BitSlice};
use rust_common::{
    init_runner,
    tree::{Key, TestTree, Value},
};
use std::collections::HashMap;

extern crate log;

pub struct TestEnvironment {
    // wrap your implementation here
}

// implement the TestEnvironment constructor
impl TestEnvironment {
    pub fn new() -> Self {
        Self {}
    }
}

// implement the TestTree trait for your implementation
impl TestTree for TestEnvironment {
    fn insert(&mut self, key: &Key, value: &Value) -> Result<()>;

    fn remove(&mut self, key: &Key) -> Result<()>;

    fn get(&self, key: &Key) -> Result<Value>;

    fn contains(&self, key: &Key) -> Result<bool>;

    fn commit(&mut self) -> Result<Option<Vec<u8>>>;

    fn root_hash(&mut self) -> Result<Value>;
}

fn main() {
    let result = init_runner(|| Box::new(TestEnvironment::new()));

    log::info!("Pathfinder | Results {:?}", result);
}
