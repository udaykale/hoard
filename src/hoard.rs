use std::hash::Hash;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::types::Result;


pub trait KeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    fn create(&mut self, key: &K, value: V) -> Result<V>;
    fn read(&self, key: &K) -> Result<&V>;
    fn size(&self) -> usize;
    // fn update< V>(&mut self, key: String, value: V) -> Option<V> where V: CacheValue;
    // fn delete< V>(&mut self, key: &String) -> Option<V> where V: CacheValue;
}

pub trait EvictionPolicy<'de, K, V, U>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          U: KeyValueStore<'de, K, V> {
    fn pre_read(&self, key: &K, kvs: &U) -> Result<&V>;
    fn post_read(&self, key: &K, kvs: &U) -> Result<&V>;
    fn pre_create(&mut self, key: &K, kvs: &U) -> Result<V>;
    fn post_create(&mut self, key: &K, kvs: &U) -> Result<V>;
}

pub struct Cache<'de, K: 'de, V: 'de, U, E>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          U: KeyValueStore<'de, K, V>,
          E: EvictionPolicy<'de, K, V, U> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _de: PhantomData<&'de ()>,
    kvs: U,
    ep: E,
}

impl<'de, K, V, U, E> Cache<'de, K, V, U, E>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          U: KeyValueStore<'de, K, V>,
          E: EvictionPolicy<'de, K, V, U> {
    pub fn new(kvs: U, ep: E) -> Cache<'de, K, V, U, E> {
        Cache { kvs, ep, _k: Default::default(), _v: Default::default(), _de: Default::default() }
    }

    pub fn read(&self, key: &K) -> Result<&V> {
        let pre_res = self.ep.pre_read(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.read(key);
                let post_res = self.ep.post_read(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(_) => { post_res }
                }
            }
            Err(_) => { pre_res }
        }
    }

    pub fn create(&mut self, key: &K, value: V) -> Result<V> {
        let pre_res = self.ep.pre_create(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.create(key, value);
                let post_res = self.ep.post_create(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(_) => { post_res }
                }
            }
            Err(_) => { pre_res }
        }
    }

    // fn update(&mut self, key: String, value: V) -> Option<V>;
    // fn delete(&mut self, key: &String) -> Option<V>;
}