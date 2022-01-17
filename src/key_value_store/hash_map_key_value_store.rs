use std::collections::HashMap;

use common::key::Key;
use common::key_value_store::KeyValueStore;
use common::types::Result;
use common::value::Value;

pub struct HashmapKeyValueStore<K, V> {
    kvs: HashMap<K, V>,
}

impl<K, V> HashmapKeyValueStore<K, V> {
    pub fn new() -> HashmapKeyValueStore<K, V> {
        HashmapKeyValueStore { kvs: HashMap::new() }
    }
}

impl<K, V> KeyValueStore<K, V> for HashmapKeyValueStore<K, V> where K: Key, V: Value {
    fn len(&self) -> usize {
        self.kvs.len()
    }

    fn read(&self, key: &K) -> Result<&V> {
        let res = self.kvs.get(key);
        Result::Ok(res)
    }

    fn create(&mut self, key: &K, value: &V) -> Result<V> {
        let res = self.kvs.insert(key.clone(), value.clone());
        Result::Ok(res)
    }

    fn update(&mut self, key: &K, value: &V) -> Result<V> {
        let res = self.kvs.insert(key.clone(), value.clone());
        Result::Ok(res)
    }

    fn delete(&mut self, key: &K) -> Result<V> {
        let res = self.kvs.remove(key);
        Result::Ok(res)
    }
}
