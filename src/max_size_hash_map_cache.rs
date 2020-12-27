use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{hoard, types};
use crate::hoard::{Cache, EvictionPolicy, KeyValueStore};
use crate::serde::{Deserializer, Serializer};
use crate::types::Error;
use crate::types::ErrorKind::SQL;

pub struct HashMapKeyValueStore<T> {
    kvs: HashMap<String, T>
}

impl<T: Serializer + Deserializer + Clone> HashMapKeyValueStore<T> {
    fn new() -> HashMapKeyValueStore<T> {
        HashMapKeyValueStore { kvs: HashMap::new() }
    }
}

impl<T> KeyValueStore<T> for HashMapKeyValueStore<T> where T: Serializer + Deserializer + Clone {
    fn create(&mut self, key: &String, value: T) -> Option<T> {
        self.kvs.insert(key.to_owned(), value)
    }

    fn read(&mut self, key: &String) -> Option<&T> {
        self.kvs.get(key)
    }
}

pub struct MaxSizeEvictionPolicy {
    max_size: i32,
    current_size: i32,
}

impl MaxSizeEvictionPolicy {
    fn new(max_size: i32) -> MaxSizeEvictionPolicy {
        MaxSizeEvictionPolicy {
            max_size,
            current_size: 0,
        }
    }
}

impl<T, U> EvictionPolicy<T, U> for MaxSizeEvictionPolicy
    where T: Serializer + Deserializer + Clone,
          U: KeyValueStore<T> {
    fn pre_read(&mut self, key: &String, kvs: &U) -> types::Result<T> {
        Ok(None)
    }

    fn post_read(&mut self, key: &String, kvs: &U) -> types::Result<T> {
        Ok(None)
    }

    fn pre_create(&mut self, key: &String, kvs: &U) -> types::Result<T> {
        if self.max_size > self.current_size {
            return Ok(None);
        }
        Err(Error { kind: SQL, message: String::from("sas") })
    }

    fn post_create(&mut self, key: &String, kvs: &U) -> types::Result<T> {
        Ok(None)
    }
}

pub struct MaxSizeHashMapCache<T> where T: Serializer + Deserializer + Clone, {
    phantom: PhantomData<T>
}

impl<T> MaxSizeHashMapCache<T> where T: Serializer + Deserializer + Clone, {
    pub fn new(max_size: i32) -> Cache<T, HashMapKeyValueStore<T>, MaxSizeEvictionPolicy> {
        Cache::new(HashMapKeyValueStore::new(), MaxSizeEvictionPolicy::new(max_size))
    }
}

#[cfg(test)]
mod tests {

}
