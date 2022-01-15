use common::types::Result;

pub trait Value: Sized + Clone {
    fn new(bytes: &[u8]) -> Result<Self>;

    fn to_bytes(&self) -> &[u8];
}
