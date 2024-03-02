use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

use crate::types::Result;

pub trait KeyValueStore<'de, T> where T: Serialize + Deserialize<'de> + Clone {
    fn create(&mut self, key: &String, value: T) -> Result<T>;
    fn read(&self, key: &String) -> Result<&T>;
    fn size(&self) -> usize;
    // fn update< T>(&mut self, key: String, value: T) -> Option<T> where T: CacheValue;
    // fn delete< T>(&mut self, key: &String) -> Option<T> where T: CacheValue;
}

pub trait EvictionPolicy<'de, T: 'de, U: 'de>
    where T: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, T> {
    fn pre_read(&self, key: &String, kvs: &U) -> Result<&T>;
    fn post_read(&self, key: &String, kvs: &U) -> Result<&T>;
    fn pre_create(&mut self, key: &String, kvs: &U) -> Result<T>;
    fn post_create(&mut self, key: &String, kvs: &U) -> Result<T>;
}

pub struct Cache<'de, T: 'de, U: 'de, V: 'de>
    where T: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, T>,
          V: EvictionPolicy<'de, T, U> {
    kvs: U,
    ep: V,
    phantom: PhantomData<T>,
    _phantom: PhantomData<&'de ()>,
}

impl<'de, T, U, V> Cache<'de, T, U, V>
    where T: Serialize + Deserialize<'de> + Clone,
          U: KeyValueStore<'de, T>,
          V: EvictionPolicy<'de, T, U> {
    pub fn new(kvs: U, ep: V) -> Cache<'de, T, U, V> {
        Cache { kvs, ep, phantom: Default::default(), _phantom: Default::default() }
    }

    pub fn read(&self, key: &String) -> Result<&T> {
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

    pub fn create(&mut self, key: &String, value: T) -> Result<T> {
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

    // fn update(&mut self, key: String, value: T) -> Option<T>;
    // fn delete(&mut self, key: &String) -> Option<T>;
}