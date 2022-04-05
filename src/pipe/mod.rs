pub mod connection;
pub mod router;

use crate::error::Result;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use router::route;

use bytes::BytesMut;
use connection::Connection;

async fn error_handler_middleware(req: Request) -> Response {
    let result = route(req).await;

    match result {
        Err(e) => Response::new_server_error(e.to_string()),
        Ok(r) => r,
    }
}

pub async fn handler(connection: &mut Connection) -> Result<Response> {
    let req_string: String = connection.read_line().await?.trim_end().to_string();

    let request = Request::create_from_request_line(req_string);

    let response = match request {
        Err(e) => Response::new_client_error(e.to_string()),
        Ok(req) => error_handler_middleware(req).await,
    };

    let header = response.render_header();
    let content = response.content.clone();

    connection.write_buf(BytesMut::from(&header[..])).await?;

    match content {
        Some(v) => connection.write_buf(BytesMut::from(&v[..])).await,
        _ => Ok(()),
    }?;

    Ok(response)
}
