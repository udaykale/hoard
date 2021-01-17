use std::collections::HashMap;
use std::marker::PhantomData;

use crate::hoard::{Cache, EvictionPolicy, KeyValueStore};
use crate::serde::{Deserializer, Serializer};
use crate::types::{Error, Result};
use crate::types::ErrorKind::{ALREADY_EXISTS, NO_EXISTING_VALUE, SIZE};

pub struct HashMapKeyValueStore<T> {
    kvs: HashMap<String, T>
}

impl<T> HashMapKeyValueStore<T> where T: Serializer + Deserializer + Clone {
    fn new() -> HashMapKeyValueStore<T> {
        HashMapKeyValueStore { kvs: HashMap::new() }
    }
}

impl<T> KeyValueStore<T> for HashMapKeyValueStore<T> where T: Serializer + Deserializer + Clone {
    fn len(&self) -> usize {
        self.kvs.len()
    }

    fn read(&self, key: &String) -> Result<&T> {
        let res = self.kvs.get(key);
        Ok(res)
    }

    fn create(&mut self, key: &String, value: T) -> Result<T> {
        if self.kvs.contains_key(key) {
            return Err(Error { kind: ALREADY_EXISTS, message: format!("Value Already Exists") });
        }

        let res = self.kvs.insert(key.to_owned(), value);
        Ok(res)
    }

    fn update(&mut self, key: &String, value: T) -> Result<T> {
        if !self.kvs.contains_key(key) {
            return Err(Error { kind: NO_EXISTING_VALUE, message: format!("Value Does not Exists") });
        }

        let res = self.kvs.insert(key.to_owned(), value);
        Ok(res)
    }

    fn delete(&mut self, key: &String) -> Result<T> {
        let res = self.kvs.remove(key);
        Ok(res)
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

impl<T, U> EvictionPolicy<T, U> for MaxSizeEvictionPolicy
    where T: Serializer + Deserializer + Clone,
          U: KeyValueStore<T> {
    fn pre_read(&self, key: &String, kvs: &U) -> Result<&T> {
        Ok(None)
    }

    fn post_read(&self, key: &String, kvs: &U) -> Result<&T> {
        Ok(None)
    }

    fn pre_create(&mut self, key: &String, kvs: &U) -> Result<T> {
        if self.max_size > kvs.len() {
            return Ok(None);
        }
        Err(Error { kind: SIZE, message: format!("Size of cache cannot exceed {}", self.max_size) })
    }

    fn post_create(&mut self, key: &String, kvs: &U) -> Result<T> {
        Ok(None)
    }

    fn pre_update(&mut self, key: &String, kvs: &U) -> Result<T> {
        Ok(None)
    }

    fn post_update(&mut self, key: &String, kvs: &U) -> Result<T> {
        Ok(None)
    }

    fn pre_delete(&mut self, key: &String, kvs: &U) -> Result<T> {
        Ok(None)
    }

    fn post_delete(&mut self, key: &String, kvs: &U) -> Result<T> {
        Ok(None)
    }
}

pub struct MaxSizeHashMapCache<T> where T: Serializer + Deserializer + Clone, {
    phantom: PhantomData<T>
}

impl<T> MaxSizeHashMapCache<T> where T: Serializer + Deserializer + Clone, {
    pub fn new(max_size: usize) -> Cache<T, HashMapKeyValueStore<T>, MaxSizeEvictionPolicy> {
        Cache::new(HashMapKeyValueStore::new(), MaxSizeEvictionPolicy::new(max_size))
    }
}

#[cfg(test)]
mod tests {}
