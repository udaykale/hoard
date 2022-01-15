pub enum ErrorKind {
    UnableToDeserialize,
    UnableToSerialize,
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

pub type Result<T> = core::result::Result<Option<T>, Error>;
