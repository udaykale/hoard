use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use common::cache::Cache;
use common::key::Key;
use common::types::{Error, ErrorKind, Result};
use common::value::Value;
use hashmap_cache::hash_map_key_value_store::HashmapKeyValueStore;
use hashmap_cache::max_size_eviction_policy::MaxSizeEvictionPolicy;

pub struct MaxSizeHashMapCache<K, V> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K, V> MaxSizeHashMapCache<K, V> where K: Key, V: Value {
    pub fn new(max_size: usize) -> Cache<K, V, HashmapKeyValueStore<K, V>, MaxSizeEvictionPolicy> {
        Cache::new(HashmapKeyValueStore::new(), MaxSizeEvictionPolicy::new(max_size))
    }
}

#[test]
fn test() {
    struct TestKey {
        val0: u8,
        val1: String,
    }

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.val0.hash(state);
            self.val1.hash(state);
        }
    }

    impl PartialEq<Self> for TestKey {
        fn eq(&self, other: &Self) -> bool {
            self.val0 == other.val0 && self.val1 == other.val1
        }
    }

    impl Eq for TestKey {}

    impl Clone for TestKey {
        fn clone(&self) -> Self {
            return TestKey { val0: 0, val1: "".to_string() };
        }
    }

    impl Key for TestKey {
        fn new(bytes: &[u8]) -> Result<TestKey> {
            return match bytes.len() {
                0 | 1 => { Result::Err(Error { kind: ErrorKind::UnableToDeserialize, message: "se".to_string() }) }
                _ => {
                    let val = String::from_utf8(Vec::from(bytes));
                    match val {
                        Err(_) => { Result::Err(Error { kind: ErrorKind::UnableToDeserialize, message: "".to_string() }) }
                        Ok(_) => {
                            let res = TestKey { val0: bytes[0], val1: val.unwrap() };
                            Result::Ok(Option::Some(res))
                        }
                    }
                }
            };
        }

        fn to_bytes(&self) -> &[u8] {
            &[0, 1]
        }
    }

    struct TestValue {
        val0: u8,
        val1: String,
    }

    impl Clone for TestValue {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    impl Value for TestValue {
        fn new(bytes: &[u8]) -> Result<Self> {
            todo!()
        }

        fn to_bytes(&self) -> &[u8] {
            todo!()
        }
    }

    let mut cache: Cache<TestKey, TestValue, HashmapKeyValueStore<TestKey, TestValue>, MaxSizeEvictionPolicy> =
        MaxSizeHashMapCache::new(2);

    // TODO
    //  Read, create, update, delete, max size create
    cache.read(&TestKey { val0: 0, val1: "".to_string() });
    cache.create(&TestKey { val0: 0, val1: "".to_string() }, &TestValue { val0: 0, val1: "".to_string() });
    cache.update(&TestKey { val0: 0, val1: "".to_string() }, &TestValue { val0: 0, val1: "".to_string() });
    cache.delete(&TestKey { val0: 0, val1: "".to_string() });
    cache.create(&TestKey { val0: 0, val1: "".to_string() }, &TestValue { val0: 0, val1: "".to_string() });
    cache.create(&TestKey { val0: 0, val1: "".to_string() }, &TestValue { val0: 0, val1: "".to_string() });

    let obj: Result<TestKey> = TestKey::new(&[2, 21, 78]);

    match obj {
        Err(_) => {}
        Ok(_) => {
            let unwrapped_obj = obj.unwrap_or(None).unwrap();
            assert_eq!(unwrapped_obj.val0, 2);
            assert_eq!(unwrapped_obj.val1, "\u{2}\u{15}N");
        }
    }
}