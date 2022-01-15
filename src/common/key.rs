use std::hash::Hash;

use common::types::Result;

pub trait Key: Hash + Eq + PartialEq + Sized + Clone {
    fn new(bytes: &[u8]) -> Result<Self>;

    fn to_bytes(&self) -> &[u8];
}
