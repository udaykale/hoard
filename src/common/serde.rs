pub trait Serde<T> {
    fn to_bytes(&self, object: T) -> Vec<u8>;

    fn to_object(&self) -> Vec<u8>;
}

pub trait Key<T> {
    fn new(bytes: Vec<u8>) -> Key<T>;

    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Value<T> {
    fn new(bytes: Vec<u8>) -> Key<T>;

    fn to_bytes(&self) -> Vec<u8>;
}

pub struct TestObj {
    val0: String,
    val1: usize,
}

impl Key<TestObj> for TestObj {
    fn new(bytes: Vec<u8>) -> Key<TestObj> {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}