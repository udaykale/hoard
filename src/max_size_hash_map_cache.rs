use std::collections::HashMap;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

use crate::{types};
use crate::hoard::{Cache, EvictionPolicy, KeyValueStore};
use crate::types::Error;
use crate::types::ErrorKind::SIZE;

pub struct HashMapKeyValueStore<T> {
    kvs: HashMap<String, T>,
}

impl<'de, T: 'de> HashMapKeyValueStore<T> where T: Serialize + Deserialize<'de> + Clone {
    fn new() -> HashMapKeyValueStore<T> {
        HashMapKeyValueStore { kvs: HashMap::new() }
    }
}

impl<'de, T: 'de> KeyValueStore<'de, T> for HashMapKeyValueStore<T> where T: Serialize + Deserialize<'de> + Clone {
    fn create(&mut self, key: &String, value: T) -> types::Result<T> {
        let res = self.kvs.insert(key.to_owned(), value);
        Ok(res)
    }

    fn read(&self, key: &String) -> types::Result<&T> {
        let res = self.kvs.get(key);
        Ok(res)
    }

    fn size(&self) -> usize {
        self.kvs.len()
    }
}

pub struct MaxSizeEvictionPolicy {
    max_size: usize,
}

impl MaxSizeEvictionPolicy {
    fn new(max_size: usize) -> MaxSizeEvictionPolicy {
        MaxSizeEvictionPolicy {
            max_size,
        }
    }
}

impl<'de, T: 'de, U: 'de> EvictionPolicy<'de, T, U> for MaxSizeEvictionPolicy
    where T: Serialize + Deserialize<'de> + Clone, U: KeyValueStore<'de, T> + 'de {
    fn pre_read(&self, key: &String, kvs: &U) -> types::Result<&T> {
        Ok(None)
    }

    fn post_read(&self, key: &String, kvs: &U) -> types::Result<&T> {
        Ok(None)
    }

    fn pre_create(&mut self, key: &String, kvs: &U) -> types::Result<T> {
        if self.max_size > kvs.size() {
            return Ok(None);
        }
        Err(Error { kind: SIZE, message: format!("Size of cache cannot exceed {}. Was {}", self.max_size, kvs.size()) })
    }

    fn post_create(&mut self, key: &String, kvs: &U) -> types::Result<T> {
        Ok(None)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MaxSizeHashMapCache<T> where T: Clone {
    phantom: PhantomData<T>,
}

impl<'de, T> MaxSizeHashMapCache<T> where T: Serialize + Deserialize<'de> + Clone, {
    pub fn new(max_size: usize) -> Cache<'de, T, HashMapKeyValueStore<T>, MaxSizeEvictionPolicy> {
        Cache::new(HashMapKeyValueStore::new(), MaxSizeEvictionPolicy::new(max_size))
    }
}

#[cfg(test)]
mod tests {}
