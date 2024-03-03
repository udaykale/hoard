use std::hash::{Hash};
use serde::{Deserialize, Serialize};
use hoard::max_size_hash_map_cache::MaxSizeHashMapCache;
use hoard::types::ErrorKind;

#[derive(Deserialize, Serialize, Clone)]
struct TestValue(usize);

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct TestKey(String);


impl TestValue {
    fn new(val: usize) -> TestValue {
        TestValue(val)
    }
}

#[test]
fn create_and_read_cache() {
    let key = TestKey(String::from("10"));
    let mut c = MaxSizeHashMapCache::new(5);
    let _ = c.create(&key, TestValue::new(10));
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
    let key = TestKey(String::from("10"));
    let mut c = MaxSizeHashMapCache::new(5);
    let _ = c.create(&TestKey(String::from("1")), TestValue::new(10));
    let _ = c.create(&TestKey(String::from("2")), TestValue::new(10));
    let _ = c.create(&TestKey(String::from("3")), TestValue::new(10));
    let _ = c.create(&TestKey(String::from("4")), TestValue::new(10));
    let _ = c.create(&TestKey(String::from("5")), TestValue::new(10));

    let value = c.create(&key, TestValue::new(10));

    match value {
        Ok(_) => {
            panic!("Cache should have maxed out.")
        }
        Err(e) => {
            assert!(matches!(e.kind, ErrorKind::SIZE));
            assert_eq!(e.message, "Size of cache cannot exceed 5. Was 5");
        }
    }
}