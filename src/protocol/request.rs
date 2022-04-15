use crate::error::{Error, Result};
use crate::protocol::{EMPTY_REQ, PARSE_ERR, WRONG_DATA_SIZE};

use bytes::Bytes;
use log::debug;
use std::fmt;
use url::Url;
use urlencoding::decode;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Request {
    pub host: String,
    pub locator: String,
    pub data_len: usize,
    pub data: Option<Bytes>,
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
            Err(Error::new_request_error(PARSE_ERR))
        } else {
            let host_str = tokens
                .get(0)
                .ok_or(Error::new_unexpected("Host lost from string"))?;

            let locator_str = tokens
                .get(1)
                .ok_or(Error::new_unexpected("Locator lost from string"))?;

            let url = Url::parse(format!("spartan://{}{}", host_str, locator_str).as_str())
                .map_err(|e| Error::new_request_error(e.to_string().as_str()))?;

            let real_path = decode(url.path().to_string().as_str())
                .map(|s| s.to_string())
                .map_err(|e| Error::new_request_error(e.to_string().as_str()))?;

            let size_value: usize = tokens
                .get(2)
                .ok_or(Error::new_unexpected("Data len lost from string"))?
                .to_string()
                .parse::<usize>()
                .map_err(|_| Error::new_request_error(PARSE_ERR))?;

            debug!(
                "Decoded host: {}, path: {}, data_len: {}",
                host_str, real_path, size_value
            );

            Ok(Request {
                host: host_str.to_string(),
                locator: real_path,
                data_len: size_value,
                data: None,
            })
        }
    }

    /// Creates Request structure(important! there still no request data)
    pub fn create_from_request_line(request: String) -> Result<Request> {
        let result: Result<Request> = if request.is_empty() {
            Err(Error::new_request_error(EMPTY_REQ))
        } else {
            Request::try_parse_line(request)
        };

        result
    }

    /// Append data to request object
    pub fn append_data(&self, data: Bytes) -> Result<Request> {
        if data.len() != self.data_len {
            Err(Error::new_request_error(
                format!(
                    "{} expect {} got {}",
                    WRONG_DATA_SIZE,
                    self.data_len,
                    data.len()
                )
                .as_str(),
            ))
        } else {
            let mut model = self.clone();
            model.data = Some(data.clone());

            Ok(model)
        }
    }
}

// ----------------- Tests section --------------------

#[test]
fn create_from_request_path_under_root() {
    let result =
        Request::create_from_request_line("my-good-host.com /../../../../etc/passwd 0".to_string());
    let except = Ok(Request {
        host: "my-good-host.com".to_string(),
        locator: "/etc/passwd".to_string(),
        data_len: 0,
        data: None,
    });

    assert!(result.is_ok());
    assert_eq!(result, except);
}

#[test]
fn create_from_request_line_empty_line() {
    let result = Request::create_from_request_line("".to_string());
    let expect = Err(Error::new_request_error(EMPTY_REQ));
    assert!(result.is_err());

    assert_eq!(result, expect);
}

#[test]
fn create_from_request_line_wrong_data_len() {
    let result = Request::create_from_request_line("somehost /some/path not-a-number".to_string());
    let except = Err(Error::new_request_error(PARSE_ERR));

    assert!(result.is_err());
    assert_eq!(except, result);
}

#[test]
fn create_from_request_line_zero_body() {
    let result =
        Request::create_from_request_line("my-good-host.com /resource%20test 0".to_string());
    let except = Ok(Request {
        host: "my-good-host.com".to_string(),
        locator: "/resource test".to_string(),
        data_len: 0,
        data: None,
    });

    assert!(result.is_ok());
    assert_eq!(result, except);
}

#[test]
fn create_from_request_line_body_contains_size() {
    let result = Request::create_from_request_line("host.com /addr 12".to_string());
    let except = Ok(Request {
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

    let except = Err(Error::new_request_error(WRONG_DATA_SIZE));

    assert!(result.is_err());
    assert_eq!(except, result);
}

#[test]
fn append_data_right_size() {
    let byte_data = Bytes::from(&b"Hello world!"[..]);

    let result = Request::create_from_request_line("host /addr 12".to_string())
        .and_then(|res| res.append_data(byte_data.clone()));

    let except = Ok(Request {
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

    let except = Err(Error::new_request_error(WRONG_DATA_SIZE));

    assert!(result.is_err());
    assert_eq!(except, result);
}
