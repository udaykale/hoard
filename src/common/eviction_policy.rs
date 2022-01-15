use common::key::Key;
use common::key_value_store::KeyValueStore;
use common::types::Result;
use common::value::Value;

pub trait EvictionPolicy<K, V, S> where K: Key, V: Value, S: KeyValueStore<K, V> {
    fn pre_read(&self, key: &K, kvs: &S) -> Result<bool>;

    fn post_read(&self, key: &K, kvs: &S) -> Result<bool>;

    fn pre_create(&mut self, key: &K, kvs: &S) -> Result<bool>;

    fn post_create(&mut self, key: &K, kvs: &S) -> Result<bool>;

    fn pre_update(&mut self, key: &K, kvs: &S) -> Result<bool>;

    fn post_update(&mut self, key: &K, kvs: &S) -> Result<bool>;

    fn pre_delete(&mut self, key: &K, kvs: &S) -> Result<bool>;

    fn post_delete(&mut self, key: &K, kvs: &S) -> Result<bool>;
}
