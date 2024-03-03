use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use crate::types::Result;


pub trait KeyValueStore<'de, V> where V: Serialize + Deserialize<'de> + Clone {
    fn create(&mut self, key: &String, value: V) -> Result<V>;
    fn read(&self, key: &String) -> Result<&V>;
    fn size(&self) -> usize;
    // fn update< V>(&mut self, key: String, value: V) -> Option<V> where V: CacheValue;
    // fn delete< V>(&mut self, key: &String) -> Option<V> where V: CacheValue;
}

pub trait EvictionPolicy<'de, V, U>
    where V: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, V> {
    fn pre_read(&self, key: &String, kvs: &U) -> Result<&V>;
    fn post_read(&self, key: &String, kvs: &U) -> Result<&V>;
    fn pre_create(&mut self, key: &String, kvs: &U) -> Result<V>;
    fn post_create(&mut self, key: &String, kvs: &U) -> Result<V>;
}

pub struct Cache<'de, V: 'de, U, E>
    where V: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, V>,
          E: EvictionPolicy<'de, V, U> {
    kvs: U,
    ep: E,
    phantom: PhantomData<V>,
    _phantom: PhantomData<&'de ()>,
}

impl<'de, V, U, E> Cache<'de, V, U, E>
    where V: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, V>,
          E: EvictionPolicy<'de, V, U> {
    pub fn new(kvs: U, ep: E) -> Cache<'de, V, U, E> {
        Cache { kvs, ep, phantom: Default::default(), _phantom: Default::default() }
    }

    pub fn read(&self, key: &String) -> Result<&V> {
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

    pub fn create(&mut self, key: &String, value: V) -> Result<V> {
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