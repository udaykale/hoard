use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

use crate::{types};
use crate::hoard::{Cache, EvictionPolicy, KeyValueStore};
use crate::types::Error;
use crate::types::ErrorKind::SIZE;

pub struct HashMapKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone {
    kvs: HashMap<K, V>,
    _de: PhantomData<&'de ()>,
}

impl<'de, K: 'de, V: 'de> HashMapKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone {
    fn new() -> HashMapKeyValueStore<'de, K, V> {
        HashMapKeyValueStore { _de: Default::default(), kvs: HashMap::new() }
    }
}

impl<'de, K, V> KeyValueStore<'de, K, V> for HashMapKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone {
    fn create(&mut self, key: &K, value: V) -> types::Result<V> {
        let res = self.kvs.insert(key.to_owned(), value);
        Ok(res)
    }

    fn read(&self, key: &K) -> types::Result<&V> {
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

impl<'de, K, V, U> EvictionPolicy<'de, K, V, U> for MaxSizeEvictionPolicy
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, K, V> {
    fn pre_read(&self, key: &K, kvs: &U) -> types::Result<&V> {
        Ok(None)
    }

    fn post_read(&self, key: &K, kvs: &U) -> types::Result<&V> {
        Ok(None)
    }

    fn pre_create(&mut self, key: &K, kvs: &U) -> types::Result<V> {
        if self.max_size > kvs.size() {
            return Ok(None);
        }
        Err(Error { kind: SIZE, message: format!("Size of cache cannot exceed {}. Was {}", self.max_size, kvs.size()) })
    }

    fn post_create(&mut self, key: &K, kvs: &U) -> types::Result<V> {
        Ok(None)
    }
}

pub struct MaxSizeHashMapCache<K, V> where V: Clone {
    _v: PhantomData<V>,
    _k: PhantomData<K>,
}

impl<'de, K, V> MaxSizeHashMapCache<K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone {
    pub fn new(max_size: usize) -> Cache<'de, K, V, HashMapKeyValueStore<'de, K, V>, MaxSizeEvictionPolicy> {
        Cache::new(HashMapKeyValueStore::new(), MaxSizeEvictionPolicy::new(max_size))
    }
}

#[cfg(test)]
mod tests {}
