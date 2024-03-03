use std::collections::HashMap;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

use crate::{types};
use crate::hoard::{Cache, EvictionPolicy, KeyValueStore};
use crate::types::Error;
use crate::types::ErrorKind::SIZE;

pub struct HashMapKeyValueStore<V> {
    kvs: HashMap<String, V>,
}

impl<'de, V> HashMapKeyValueStore<V> where V: Serialize + Deserialize<'de> + Clone {
    fn new() -> HashMapKeyValueStore<V> {
        HashMapKeyValueStore { kvs: HashMap::new() }
    }
}

impl<'de, V> KeyValueStore<'de, V> for HashMapKeyValueStore<V> where V: Serialize + Deserialize<'de> + Clone {
    fn create(&mut self, key: &String, value: V) -> types::Result<V> {
        let res = self.kvs.insert(key.to_owned(), value);
        Ok(res)
    }

    fn read(&self, key: &String) -> types::Result<&V> {
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

impl<'de, V, U> EvictionPolicy<'de, V, U> for MaxSizeEvictionPolicy
    where V: Serialize + Deserialize<'de> + Clone, U: KeyValueStore<'de, V> + 'de {
    fn pre_read(&self, key: &String, kvs: &U) -> types::Result<&V> {
        Ok(None)
    }

    fn post_read(&self, key: &String, kvs: &U) -> types::Result<&V> {
        Ok(None)
    }

    fn pre_create(&mut self, key: &String, kvs: &U) -> types::Result<V> {
        if self.max_size > kvs.size() {
            return Ok(None);
        }
        Err(Error { kind: SIZE, message: format!("Size of cache cannot exceed {}. Was {}", self.max_size, kvs.size()) })
    }

    fn post_create(&mut self, key: &String, kvs: &U) -> types::Result<V> {
        Ok(None)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MaxSizeHashMapCache<V> where V: Clone {
    phantom: PhantomData<V>,
}

impl<'de, V> MaxSizeHashMapCache<V> where V: Serialize + Deserialize<'de> + Clone, {
    pub fn new(max_size: usize) -> Cache<'de, V, HashMapKeyValueStore<V>, MaxSizeEvictionPolicy> {
        Cache::new(HashMapKeyValueStore::new(), MaxSizeEvictionPolicy::new(max_size))
    }
}

#[cfg(test)]
mod tests {}
