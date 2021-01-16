pub enum ErrorKind {
    SIZE,
    ALREADY_EXISTS,
    NO_EXISTING_VALUE,
}

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

pub type Result<T> = core::result::Result<Option<T>, Error>;
