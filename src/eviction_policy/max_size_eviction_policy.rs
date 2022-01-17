use common::eviction_policy::EvictionPolicy;
use common::key::Key;
use common::key_value_store::KeyValueStore;
use common::types::{Error, ErrorKind, Result};
use common::value::Value;

pub struct MaxSizeEvictionPolicy {
    max_size: usize,
}

impl MaxSizeEvictionPolicy {
    pub fn new(max_size: usize) -> MaxSizeEvictionPolicy {
        MaxSizeEvictionPolicy { max_size }
    }
}

impl<K, V, S> EvictionPolicy<K, V, S> for MaxSizeEvictionPolicy
    where K: Key, V: Value, S: KeyValueStore<K, V> {
    fn pre_read(&self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }

    fn post_read(&self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }

    fn pre_create(&mut self, _key: &K, _kvs: &S) -> Result<bool> {
        if _kvs.len() == self.max_size {
            return Result::Err(Error { kind: ErrorKind::UnableToDeserialize, message: "".to_string() });
        }
        return Result::Ok(Option::Some(true));
    }

    fn post_create(&mut self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }

    fn pre_update(&mut self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }

    fn post_update(&mut self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }

    fn pre_delete(&mut self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }

    fn post_delete(&mut self, _key: &K, _kvs: &S) -> Result<bool> {
        return Result::Ok(Option::Some(true));
    }
}
