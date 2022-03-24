use bytes::Bytes;
use int_enum::IntEnum;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntEnum)]
#[repr(u8)]
pub enum StatusCode {
    Success = 2,
    Redirect = 3,
    ClientError = 4,
    ServerError = 5,
}

pub const UNKNOWN_ERROR: &str = "Unknown error";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Response {
    status_code: StatusCode,
    status_line: String,
    content: Option<Bytes>,
}

impl Default for Response {
    fn default() -> Response {
        Response {
            status_code: StatusCode::ClientError,
            status_line: UNKNOWN_ERROR.to_string(),
            content: None,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Response with status code: {:?}, content-type: {:?}, and content-lenght: {}",
            self.status_code,
            self.status_line,
            self.content.to_owned().map_or(0, |c| { c.len() })
        )
    }
}

impl Response {
    pub fn new(status_code: StatusCode, status_line: String, content: Option<Bytes>) -> Response {
        Response {
            status_code: status_code,
            status_line,
            content: content,
        }
    }

    pub fn new_success(content_type: String, content: Bytes) -> Response {
        Response::new(StatusCode::Success, content_type, Some(content))
    }

    pub fn new_client_error(error: String) -> Response {
        Response::new(StatusCode::ClientError, error, None)
    }

    pub fn new_server_error(error: String) -> Response {
        Response::new(StatusCode::ServerError, error, None)
    }

    pub fn new_redirect(location: String) -> Response {
        Response::new(StatusCode::Redirect, location, None)
    }

    pub fn render_header(&self) -> Bytes {
        let line = format!("{} {}\r\n", self.status_code.int_value(), self.status_line);

        Bytes::copy_from_slice(&(line.as_bytes())[..])
    }
}

// ----------------- Tests section --------------------

#[test]
fn render_header_client_error() {
    let result = Response::new_client_error("error".to_string()).render_header();
    let expected = Bytes::from(&b"4 error\r\n"[..]);

    assert_eq!(expected, result)
}

#[test]
fn render_header_server_error() {
    let result = Response::new_server_error("I'm down".to_string()).render_header();
    let expect = Bytes::from(&"5 I'm down\r\n"[..]);

    assert_eq!(result, expect);
}

#[test]
fn render_header_redirect() {
    let result = Response::new_redirect("/new-location".to_string()).render_header();
    let expect = Bytes::from(&"3 /new-location\r\n"[..]);

    assert_eq!(result, expect);
}
