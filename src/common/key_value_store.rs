use common::key::Key;
use common::types::Result;
use common::value::Value;

pub trait KeyValueStore<K, V> where K: Key, V: Value {
    fn len(&self) -> usize;

    fn read(&self, key: &K) -> Result<&V>;

    fn create(&mut self, key: &K, value: &V) -> Result<V>;

    fn update(&mut self, key: &K, value: &V) -> Result<V>;

    fn delete(&mut self, key: &K) -> Result<V>;
}
