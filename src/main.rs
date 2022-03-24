pub mod error;
pub mod mime;
pub mod protocol;

use error::Error;
use log::info;
use protocol::request::Request;
fn main() {
    env_logger::init();

    info!("Error test: {}", Error::new_io("IO error"));

    info!(
        "Request: {}",
        Request::create_from_request_line("host.com /path 0".to_string()).unwrap()
    );
}
