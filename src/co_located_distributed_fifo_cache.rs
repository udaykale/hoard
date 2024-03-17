use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash};
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

use crate::{types};
use crate::broker::{Broker};
use crate::hoard::{EvictionPolicy, KeyValueStore};

pub struct CoLocatedDistributedKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    kvs: HashMap<K, V>,
    broker: Box<dyn Broker>,
    _de: PhantomData<&'de ()>,
}

pub trait SyncKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    fn broker_config(&mut self) -> crate::types::Result<V>;
    fn key_value(&self, key: &K) -> crate::types::Result<&V>;
    // fn update< V>(&mut self, key: String, value: V) -> Option<V> where V: CacheValue;
    // fn delete< V>(&mut self, key: &String) -> Option<V> where V: CacheValue;
}

impl<'de, K: 'de, V: 'de> CoLocatedDistributedKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    pub fn new(broker: Box<dyn Broker>) -> CoLocatedDistributedKeyValueStore<'de, K, V> {
        CoLocatedDistributedKeyValueStore { _de: Default::default(), broker, kvs: HashMap::new() }
    }
}

impl<'de, K, V> SyncKeyValueStore<'de, K, V> for CoLocatedDistributedKeyValueStore<'de, K, V>
    where K: Serialize + Deserialize<'de> + Clone + Eq + Hash,
          V: Serialize + Deserialize<'de> + Clone + Eq + Hash {
    fn broker_config(&mut self) -> types::Result<V> {
        todo!()
    }

    fn key_value(&self, key: &K) -> types::Result<&V> {
        todo!()
    }
}

impl<'de, K, V> KeyValueStore<'de, K, V> for CoLocatedDistributedKeyValueStore<'de, K, V>
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

#[cfg(test)]
mod tests {}
