use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    Io,
    RequestError,
    Other,
    Unexpected,
}

impl Default for ErrorKind {
    fn default() -> ErrorKind {
        ErrorKind::Io
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Error {
        Error {
            kind,
            msg: msg.into(),
        }
    }

    pub fn new_io(msg: &str) -> Error {
        Error::new(ErrorKind::Io, msg)
    }

    pub fn new_request_error(msg: &str) -> Error {
        Error::new(ErrorKind::RequestError, msg)
    }

    pub fn new_other(msg: &str) -> Error {
        Error::new(ErrorKind::Other, msg)
    }

    pub fn new_unexpected(msg: &str) -> Error {
        Error::new(ErrorKind::Unexpected, msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Io => write!(f, "Io error: {}", self.msg),
            ErrorKind::RequestError => write!(f, "Request error: {}", self.msg),
            ErrorKind::Other => write!(f, "Other error: {}", self.msg),
            ErrorKind::Unexpected => write!(f, "Unexpected error: {}", self.msg),
        }
    }
}

#[macro_export]
macro_rules! io_err {
    ($x: expr ) => {
        $x.map_err(|e| Error::new_io(e.to_string().as_str()))
    };
}
