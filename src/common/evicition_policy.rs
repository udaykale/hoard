use common::key_value_store::KeyValueStore;

use common::serde::{Deserializer, Serializer};
use common::types::Result;

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
