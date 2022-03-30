pub mod connection;

use crate::error::Result;
use crate::protocol::request::Request;
use crate::protocol::response::Response;

use bytes::{Bytes, BytesMut};
use connection::Connection;

pub async fn handler(connection: &mut Connection) -> Result<Response> {
    let req_string: String = connection.read_line().await?.trim_end().to_string();

    let request = Request::create_from_request_line(req_string);

    let response = match request {
        Err(e) => Response::new_client_error(e.to_string()),
        Ok(_) => Response::new_success(
            "text/plain".to_string(),
            Bytes::from(&b"Hello, world!\r\n"[..]),
        ),
    };

    let header = response.render_header();
    let content = response.content.clone();

    connection.write_buf(BytesMut::from(&header[..])).await?;

    match content {
        Some(v) => connection.write_buf(BytesMut::from(&v[..])).await?,
        _ => (),
    };

    Ok(response)
}
