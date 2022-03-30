use crate::error::{Error, Result};
use crate::protocol::{EMPTY_REQ, PARSE_ERR, WRONG_DATA_SIZE};

use bytes::Bytes;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Request {
    host: String,
    locator: String,
    data_len: usize,
    data: Option<Bytes>,
}

impl Default for Request {
    fn default() -> Request {
        Request {
            host: "localhost".to_string(),
            locator: "/".to_string(),
            data_len: 0,
            data: None,
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} with body that contains {} byte(s)",
            self.host, self.locator, self.data_len
        )
    }
}

impl Request {
    /// Converts string to request
    fn try_parse_line(request: String) -> Result<Request> {
        let tokens: Vec<&str> = request.split(" ").collect();

        // Request format "hostname SPACE full-locator SPACE post-data-lenght CRLF"
        // So first formal test - count of elements in request line
        if tokens.len() != 3 {
            Result::Err(Error::new_request_error(PARSE_ERR))
        } else {
            let host_str = tokens
                .get(0)
                .ok_or(Error::new_unexpected("Host lost from string"))?;

            let locator_str = tokens
                .get(1)
                .ok_or(Error::new_unexpected("Locator lost from string"))?;

            let size_value: usize = tokens
                .get(2)
                .ok_or(Error::new_unexpected("Data len lost from string"))?
                .to_string()
                .parse::<usize>()
                .map_err(|_| Error::new_request_error(PARSE_ERR))?;

            Result::Ok(Request {
                host: host_str.to_string(),
                locator: locator_str.to_string(),
                data_len: size_value,
                data: None,
            })
        }
    }

    /// Creates Request structure(important! there still no request data)
    pub fn create_from_request_line(request: String) -> Result<Request> {
        let result: Result<Request> = if request.is_empty() {
            Result::Err(Error::new_request_error(EMPTY_REQ))
        } else {
            Request::try_parse_line(request)
        };

        result
    }

    /// Append data to request object
    pub fn append_data(&self, data: Bytes) -> Result<Request> {
        if data.len() != self.data_len {
            Result::Err(Error::new_request_error(WRONG_DATA_SIZE))
        } else {
            let mut model = self.clone();
            model.data = Some(data.clone());

            Result::Ok(model)
        }
    }
}

// ----------------- Tests section --------------------

#[test]
fn create_from_request_line_empty_line() {
    let result = Request::create_from_request_line("".to_string());
    let expect = Result::Err(Error::new_request_error(EMPTY_REQ));
    assert!(result.is_err());

    assert_eq!(result, expect);
}

#[test]
fn create_from_request_line_wrong_data_len() {
    let result = Request::create_from_request_line("somehost /some/path not-a-number".to_string());
    let except = Result::Err(Error::new_request_error(PARSE_ERR));

    assert!(result.is_err());
    assert_eq!(except, result);
}

#[test]
fn create_from_request_line_zero_body() {
    let result = Request::create_from_request_line("my-good-host.com /resource 0".to_string());
    let except = Result::Ok(Request {
        host: "my-good-host.com".to_string(),
        locator: "/resource".to_string(),
        data_len: 0,
        data: None,
    });

    assert!(result.is_ok());
    assert_eq!(result, except);
}

#[test]
fn create_from_request_line_body_contains_size() {
    let result = Request::create_from_request_line("host.com /addr 12".to_string());
    let except = Result::Ok(Request {
        host: "host.com".to_string(),
        locator: "/addr".to_string(),
        data_len: 12,
        data: None,
    });

    assert!(result.is_ok());
    assert_eq!(result, except);
}

#[test]
fn append_data_empty_data_but_data_len_is_set() {
    let result = Request::create_from_request_line("host /addr 12".to_string())
        .and_then(|res| res.append_data(Bytes::new()));

    let except = Result::Err(Error::new_request_error(WRONG_DATA_SIZE));

    assert!(result.is_err());
    assert_eq!(except, result);
}

#[test]
fn append_data_right_size() {
    let byte_data = Bytes::from(&b"Hello world!"[..]);

    let result = Request::create_from_request_line("host /addr 12".to_string())
        .and_then(|res| res.append_data(byte_data.clone()));

    let except = Result::Ok(Request {
        host: "host".to_string(),
        locator: "/addr".to_string(),
        data_len: 12,
        data: Some(byte_data.clone()),
    });

    assert!(result.is_ok());
    assert_eq!(except, result);
}

#[test]
fn append_data_wrong_size() {
    let byte_data = Bytes::from(&b"lorem ipsum"[..]);

    let result = Request::create_from_request_line("host /addr 12".to_string())
        .and_then(|res| res.append_data(byte_data.clone()));

    let except = Result::Err(Error::new_request_error(WRONG_DATA_SIZE));

    assert!(result.is_err());
    assert_eq!(except, result);
}
