pub mod connection;
pub mod directory;
pub mod file;
pub mod router;

use crate::error::Result;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use core::future::Future;
use router::route;

use bytes::BytesMut;
use connection::Connection;

fn error_handler_middleware(result: Result<Response>) -> Response {
    match result {
        Err(e) => Response::new_server_error(e.to_string()),
        Ok(r) => r,
    }
}

async fn request_data_loader<T>(
    request: Request,
    connection: &mut Connection,
    fun: impl Fn(Request) -> T,
) -> Result<Response>
where
    T: Future<Output = Result<Response>>,
{
    let count = request.data_len;

    if count == 0 {
        fun(request).await
    } else {
        let datum = connection.read_count(count).await?;
        let req = request.append_data(datum)?;
        fun(req).await
    }
}

pub async fn handler(connection: &mut Connection) -> Result<Response> {
    let req_string: String = connection.read_line().await?.trim_end().to_string();

    let request = Request::create_from_request_line(req_string);

    let response = match request {
        Err(e) => Response::new_client_error(e.to_string()),
        Ok(req) => {
            error_handler_middleware(request_data_loader(req.clone(), connection, route).await)
        }
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
