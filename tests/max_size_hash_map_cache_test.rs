use serde::{Deserialize, Serialize};
use hoard::max_size_hash_map_cache::MaxSizeHashMapCache;
use hoard::types::{ErrorKind};

#[derive(Deserialize, Serialize, Clone)]
struct Test(usize);

impl Test {
    fn new(val: usize) -> Test {
        Test(val)
    }
}

#[test]
fn create_and_read_cache() {
    let key = &String::from("10");
    let mut c = MaxSizeHashMapCache::new(5);
    let _ = c.create(&key, Test::new(10));
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
    let _ = c.create(&String::from("1"), Test::new(10));
    let _ = c.create(&String::from("2"), Test::new(10));
    let _ = c.create(&String::from("3"), Test::new(10));
    let _ = c.create(&String::from("4"), Test::new(10));
    let _ = c.create(&String::from("5"), Test::new(10));

    let value = c.create(&key, Test::new(10));

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