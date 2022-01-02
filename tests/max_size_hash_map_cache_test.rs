extern crate hoard;

use hoard::common::serde::{Deserializer, Serializer};
use hoard::common::types::ErrorKind;
use hoard::max_size_hash_map_cache::MaxSizeHashMapCache;

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
    let mut c = MaxSizeHashMapCache::new(2);
    c.create(&String::from("1"), Test::new(10));
    c.create(&String::from("2"), Test::new(12));
    let value = c.read(&String::from("2"));

    match value {
        Ok(x) => {
            match x {
                Some(x) => {
                    let value = x;
                    assert_eq!(12, value.0)
                }
                None => panic!("No value was returned!!")
            }
        }
        Err(_) => panic!("There was an unexpected error")
    }
}

#[test]
fn create_existing_value() {
    let mut c = MaxSizeHashMapCache::new(5);
    c.create(&String::from("1"), Test::new(10));
    let value = c.create(&String::from("1"), Test::new(11));

    match value {
        Ok(_) => {
            panic!("There was an unexpected error")
        }
        Err(e) => {
            assert!(matches!(e.kind, ErrorKind::AlreadyExists));
            assert_eq!(e.message, "Value Already Exists");
        }
    }
}

#[test]
fn update_and_read_cache() {
    let mut c = MaxSizeHashMapCache::new(5);
    c.create(&String::from("1"), Test::new(10));
    c.update(&String::from("1"), Test::new(9));
    let value = c.read(&String::from("1"));

    match value {
        Ok(x) => {
            match x {
                Some(x) => {
                    let value = x;
                    assert_eq!(9, value.0)
                }
                None => panic!("No value was returned!!")
            }
        }
        Err(_) => panic!("There was an unexpected error")
    }
}

#[test]
fn update_a_non_existing_value() {
    let mut c = MaxSizeHashMapCache::new(5);
    let value = c.update(&String::from("1"), Test::new(9));

    match value {
        Ok(_) => {
            panic!("There was an unexpected error")
        }
        Err(e) => {
            assert!(matches!(e.kind, ErrorKind::NoExistingValue));
            assert_eq!(e.message, "Value Does not Exists");
        }
    }
}

#[test]
fn delete_and_read_cache() {
    let mut c = MaxSizeHashMapCache::new(5);
    c.create(&String::from("1"), Test::new(1));
    c.create(&String::from("2"), Test::new(2));
    c.delete(&String::from("1"));
    let value = c.read(&String::from("1"));

    match value {
        Ok(x) => {
            match x {
                Some(_) => {
                    panic!("No value should have been returned")
                }
                None => {}
            }
        }
        Err(_) => panic!("There was an unexpected error")
    }
}

// TODO Test cases
//  Delete Non existing value
//  Read Non existing value

#[test]
fn cache_size_exceeds() {
    let mut c = MaxSizeHashMapCache::new(2);
    c.create(&String::from("1"), Test::new(10));
    c.create(&String::from("2"), Test::new(10));

    let value = c.create(&String::from("6"), Test::new(10));

    match value {
        Ok(x) => {
            panic!("Cache should have maxed out.")
        }
        Err(e) => {
            assert!(matches!(e.kind, ErrorKind::Size));
            assert_eq!(e.message, "Size of cache cannot exceed 2");
        }
    }
}

// #[test]
// fn check_cache_size() {
//     let mut c = MaxSizeHashMapCache::new(2);
//     c.create(&String::from("1"), Test::new(10));
//
//     assert_eq!(c.len(), 1);
//
//     c.create(&String::from("2"), Test::new(10));
//     c.create(&String::from("3"), Test::new(10));
//
//     assert_eq!(c.len(), 2);
//
//     c.update(&String::from("3"), Test::new(10));
//
//     assert_eq!(c.len(), 2);
// }