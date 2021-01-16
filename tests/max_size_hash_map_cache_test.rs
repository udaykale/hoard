use hoard::hoard::{Cache, EvictionPolicy, KeyValueStore};
use hoard::max_size_hash_map_cache::MaxSizeHashMapCache;
use hoard::serde::{Deserializer, Serializer};
use hoard::types::{Error, ErrorKind};

struct Test(usize);

impl Test {
    fn new(val: usize) -> Test {
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
fn create_and_read_cache() {
    let key = &String::from("10");
    let mut c = MaxSizeHashMapCache::new(5);
    c.create(&key, Test::new(10));
    let value = c.read(&key);

    match value {
        Ok(x) => {
            match x {
                Some(x) => {
                    let value = x;
                    assert_eq!(10, value.0)
                }
                None => panic!("No value was returned!!")
            }
        }
        Err(_) => panic!("There was an unexpected error")
    }
}

#[test]
fn cache_size_exceeds() {
    let key = &String::from("10");
    let mut c = MaxSizeHashMapCache::new(5);
    c.create(&String::from("1"), Test::new(10));
    c.create(&String::from("2"), Test::new(10));
    c.create(&String::from("3"), Test::new(10));
    c.create(&String::from("4"), Test::new(10));
    c.create(&String::from("5"), Test::new(10));

    let value = c.create(&key, Test::new(10));

    match value {
        Ok(x) => {
            panic!("Cache should have maxed out.")
        }
        Err(e) => {
            assert!(matches!(e.kind, ErrorKind::SIZE));
            assert_eq!(e.message, "Size of cache cannot exceed 5. Was 5");
        }
    }
}