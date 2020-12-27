use std::marker::PhantomData;

use crate::serde::{Deserializer, Serializer};
use crate::types::{Error, Result};
use crate::types;

pub trait KeyValueStore<T> where T: Serializer + Deserializer + Clone {
    fn create(&mut self, key: &String, value: T) -> Option<T>;
    fn read(&mut self, key: &String) -> Option<&T>;
    // fn update< T>(&mut self, key: String, value: T) -> Option<T> where T: CacheValue;
    // fn delete< T>(&mut self, key: &String) -> Option<T> where T: CacheValue;
}

pub trait EvictionPolicy<T, U>
    where T: Serializer + Deserializer + Clone,
          U: KeyValueStore<T> {
    fn pre_read(&mut self, key: &String, kvs: &U) -> types::Result<T>;
    fn post_read(&mut self, key: &String, kvs: &U) -> types::Result<T>;
    fn pre_create(&mut self, key: &String, kvs: &U) -> types::Result<T>;
    fn post_create(&mut self, key: &String, kvs: &U) -> types::Result<T>;
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

    pub fn read(&mut self, key: &String) -> Option<T> {
        self.ep.pre_read(key, &self.kvs);
        let op_value = self.kvs.read(key);
        let value = match op_value {
            None => { None }
            Some(value) => { Some(value.clone()) }
        };
        self.ep.post_read(key, &self.kvs);
        value
    }

    pub fn create(&mut self, key: &String, value: T) -> Option<T> {
        self.ep.pre_create(key, &self.kvs);
        let value = self.kvs.create(key, value);
        self.ep.post_create(key, &self.kvs);
        value
    }

    // fn update(&mut self, key: String, value: T) -> Option<T>;
    // fn delete(&mut self, key: &String) -> Option<T>;
}