use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash};
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

use crate::{types};
use crate::broker::{Broker};
use crate::co_located_distributed_fifo_cache::CoLocatedDistributedKeyValueStore;
use crate::hoard::{Cache, EvictionPolicy, KeyValueStore};
use crate::types::Error;
use crate::types::ErrorKind::SIZE;

pub struct CoLocatedKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    kvs: HashMap<K, V>,
    _de: PhantomData<&'de ()>,
}

impl<'de, K: 'de, V: 'de> CoLocatedKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    fn new() -> CoLocatedKeyValueStore<'de, K, V> {
        CoLocatedKeyValueStore { _de: Default::default(), kvs: HashMap::new() }
    }
}

impl<'de, K, V> KeyValueStore<'de, K, V> for CoLocatedKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
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


pub struct FIFOEvictionPolicy {
    max_size: usize,
}

impl FIFOEvictionPolicy {
    // Elements are evicted in the same order as they come in.
    // When a put call is made for a new element (and assuming that the max limit is reached for the memory store)
    // the element that was placed first (First-In) in the store is the candidate for eviction (First-Out)
    fn new(max_size: usize) -> FIFOEvictionPolicy {
        FIFOEvictionPolicy {
            max_size,
        }
    }
    fn default() -> FIFOEvictionPolicy {
        FIFOEvictionPolicy::new(1024)
    }
}

impl<'de, K, V, U> EvictionPolicy<'de, K, V, U> for FIFOEvictionPolicy
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash,
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

pub struct CoLocatedFIFOCache<K, V> {
    _v: PhantomData<V>,
    _k: PhantomData<K>,
}

impl<'de, K, V> CoLocatedFIFOCache<K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    pub fn without_broker(max_size: usize) -> Cache<'de, K, V, CoLocatedKeyValueStore<'de, K, V>, FIFOEvictionPolicy> {
        Cache::new(CoLocatedKeyValueStore::new(), FIFOEvictionPolicy::new(max_size))
    }

    pub fn with_broker(broker: Box<dyn Broker>) -> Cache<'de, K, V, CoLocatedDistributedKeyValueStore<'de, K, V>, FIFOEvictionPolicy> {
        Cache::new(CoLocatedDistributedKeyValueStore::new(broker), FIFOEvictionPolicy::default())
    }
}

#[cfg(test)]
mod tests {}
