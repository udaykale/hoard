use std::marker::PhantomData;

use common::eviction_policy::EvictionPolicy;
use common::key::Key;
use common::key_value_store::KeyValueStore;
use common::types::{Error, Result};
use common::value::Value;

pub struct Cache<K, V, S, E> where K: Key, V: Value, S: KeyValueStore<K, V>, E: EvictionPolicy<K, V, S> {
    kvs: S,
    ep: E,
    key: PhantomData<K>,
    val: PhantomData<V>,
}

impl<K, V, S, E> Cache<K, V, S, E> where K: Key, V: Value, S: KeyValueStore<K, V>, E: EvictionPolicy<K, V, S> {
    pub fn new(kvs: S, ep: E) -> Cache<K, V, S, E> {
        Cache { kvs, ep, key: Default::default(), val: Default::default() }
    }

    pub fn len(&self) -> usize {
        return self.kvs.len();
    }

    pub fn read(&self, key: &K) -> Result<&V> {
        let pre_res = self.ep.pre_read(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.read(key);
                let post_res = self.ep.post_read(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
                }
            }

            Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
        }
    }

    pub fn create(&mut self, key: &K, value: &V) -> Result<V> {
        let pre_res = self.ep.pre_create(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.create(key, value);
                let post_res = self.ep.post_create(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
                }
            }
            Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
        }
    }

    pub fn update(&mut self, key: &K, value: &V) -> Result<V> {
        let pre_res = self.ep.pre_update(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.update(key, value);
                let post_res = self.ep.post_update(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
                }
            }
            Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<V> {
        let pre_res = self.ep.pre_delete(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.delete(key);
                let post_res = self.ep.post_delete(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
                }
            }
            Err(error) => { Result::Err(Error { kind: error.kind, message: error.message }) }
        }
    }
}
