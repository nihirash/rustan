#[macro_use]
pub mod error;
pub mod configuration;
pub mod mime;
pub mod pipe;
pub mod protocol;

use configuration::{Configuration, SETTINGS};
use error::{Error, Result};
use log::{error, info};
use pipe::{connection::Connection, handler};
use tokio::net::TcpListener;

async fn create_server(config: Configuration) -> Result<()> {
    let listener = io_err!(TcpListener::bind(config.host).await)?;

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
    let config = SETTINGS.read().await.to_owned();
    info!("Loaded config:\n{}", config);

    match create_server(config).await {
        Err(e) => error!("Server error: {}", e.to_string()),
        Ok(_) => (),
    };

    Ok(())
}
