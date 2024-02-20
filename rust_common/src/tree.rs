pub type Key = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;
pub type Value = Vec<u8>;

use anyhow::Result;

pub trait TestTree {
    fn insert(&mut self, key: &Key, value: &Value) -> Result<()>;
    fn remove(&mut self, key: &Key) -> Result<()>;
    fn get(&self, key: &Key) -> Result<Value>;
    fn contains(&self, key: &Key) -> Result<bool>;
    fn commit(&mut self) -> Result<Option<Vec<u8>>>;
    fn root_hash(&mut self) -> Result<Value>;
}
