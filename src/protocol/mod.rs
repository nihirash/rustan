pub mod request;
pub mod response;

use crate::error::Error;
use std::result;

pub const PARSE_ERR: &str = "Can't parse string";
pub const EMPTY_REQ: &str = "Empty request";
pub const WRONG_DATA_SIZE: &str = "Wrong data size";

pub type Result<T> = result::Result<T, Error>;
