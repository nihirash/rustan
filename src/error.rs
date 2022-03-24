use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    Io,
    Value,
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
            kind: kind,
            msg: msg.into(),
        }
    }

    pub fn new_io(msg: &str) -> Error {
        Error::new(ErrorKind::Io, msg)
    }

    pub fn new_value(msg: &str) -> Error {
        Error::new(ErrorKind::Value, msg)
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
            ErrorKind::Value => write!(f, "Value error: {}", self.msg),
            ErrorKind::Other => write!(f, "Other error: {}", self.msg),
            ErrorKind::Unexpected => write!(f, "Unexpected error: {}", self.msg),
        }
    }
}
