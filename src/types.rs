pub enum ErrorKind {
    SQL
}

pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String,
}

pub type Result<T> = core::result::Result<Option<T>, Error>;
