use std::marker::PhantomData;

use crate::serde::{Deserializer, Serializer};
use crate::types::Result;

pub trait KeyValueStore<T> where T: Serializer + Deserializer + Clone {
    fn len(&self) -> usize;
    fn read(&self, key: &String) -> Result<&T>;
    fn create(&mut self, key: &String, value: T) -> Result<T>;
    fn update(&mut self, key: &String, value: T) -> Result<T>;
    fn delete(&mut self, key: &String) -> Result<T>;
}

pub trait EvictionPolicy<T, U>
    where T: Serializer + Deserializer + Clone,
          U: KeyValueStore<T> {
    fn pre_read(&self, key: &String, kvs: &U) -> Result<&T>;
    fn post_read(&self, key: &String, kvs: &U) -> Result<&T>;
    fn pre_create(&mut self, key: &String, kvs: &U) -> Result<T>;
    fn post_create(&mut self, key: &String, kvs: &U) -> Result<T>;
    fn pre_update(&mut self, key: &String, kvs: &U) -> Result<T>;
    fn post_update(&mut self, key: &String, kvs: &U) -> Result<T>;
    fn pre_delete(&mut self, key: &String, kvs: &U) -> Result<T>;
    fn post_delete(&mut self, key: &String, kvs: &U) -> Result<T>;
}

pub struct Cache<T, U, V>
    where T: Serializer + Deserializer + Clone,
          U: KeyValueStore<T>,
          V: EvictionPolicy<T, U> {
    kvs: U,
    ep: V,
    phantom: PhantomData<T>,
}

impl<T, U, V> Cache<T, U, V>
    where T: Serializer + Deserializer + Clone,
          U: KeyValueStore<T>,
          V: EvictionPolicy<T, U> {
    pub fn new(kvs: U, ep: V) -> Cache<T, U, V> {
        Cache { kvs, ep, phantom: Default::default() }
    }

    pub fn len(&self) -> usize {
        self.kvs.len()
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

    pub fn update(&mut self, key: &String, value: T) -> Result<T> {
        let pre_res = self.ep.pre_update(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.update(key, value);
                let post_res = self.ep.post_update(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(_) => { post_res }
                }
            }
            Err(_) => { pre_res }
        }
    }

    pub fn delete(&mut self, key: &String) -> Result<T> {
        let pre_res = self.ep.pre_delete(key, &self.kvs);
        match pre_res {
            Ok(_) => {
                let value = self.kvs.delete(key);
                let post_res = self.ep.post_delete(key, &self.kvs);
                match post_res {
                    Ok(_) => { value }
                    Err(_) => { post_res }
                }
            }
            Err(_) => { pre_res }
        }
    }
}