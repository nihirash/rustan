#[macro_use]
pub mod error;
pub mod mime;
pub mod pipe;
pub mod protocol;

use tokio::net::TcpListener;

use error::{Error, Result};
use log::{error, info};
use pipe::{connection::Connection, handler};

async fn create_server() -> Result<()> {
    let listener = io_err!(TcpListener::bind("127.0.0.1:3000").await)?;

    loop {
        let (socket, ip) = io_err!(listener.accept().await)?;

        info!("Handling connection for {}", ip.to_string());

        tokio::spawn(async move {
            match handler(&mut Connection::new(socket)).await {
                Ok(r) => info!("Request processed successfully: {}", r.to_string()),
                Err(e) => error!(
                    "Request from {} produced issue: {}",
                    ip.to_string(),
                    e.to_string()
                ),
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting application!");
    match create_server().await {
        Err(e) => error!("Server error: {}", e.to_string()),
        Ok(_) => (),
    };

    Ok(())
}
