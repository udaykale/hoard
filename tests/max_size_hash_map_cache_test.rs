use hoard::hoard::{Cache, EvictionPolicy, KeyValueStore};
use hoard::max_size_hash_map_cache::MaxSizeHashMapCache;
use hoard::serde::{Deserializer, Serializer};

struct Test(i32);

impl Test {
    fn new(val: i32) -> Test {
        Test(val)
    }
}

impl Deserializer for Test {}

impl Serializer for Test {}

impl Clone for Test {
    fn clone(&self) -> Self {
        Test::new(self.0)
    }
}

#[test]
fn create_in_cache() {
    let key = &String::from("10");
    let mut c = MaxSizeHashMapCache::new(10);
    c.create(&key, Test::new(10));
    let value = c.read(&key);

    match value {
        Some(x) => {
            let value = x;
            assert_eq!(10, value.0)
        }
        None => panic!("No value was returned!!")
    }
}