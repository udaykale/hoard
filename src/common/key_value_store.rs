use common::serde::{Deserializer, Serializer};
use common::types::Result;


pub trait KeyValueStore<T> where T: Serializer + Deserializer + Clone {
    fn len(&self) -> usize;
    fn read(&self, key: &String) -> Result<&T>;
    fn create(&mut self, key: &String, value: T) -> Result<T>;
    fn update(&mut self, key: &String, value: T) -> Result<T>;
    fn delete(&mut self, key: &String) -> Result<T>;
}
