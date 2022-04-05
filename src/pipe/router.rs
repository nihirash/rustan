use bytes::Bytes;
use log::debug;
use std::{env, path::PathBuf};

use tokio::fs;

use crate::error::{Error, Result};
use crate::mime::filename_to_mime;
use crate::protocol::request::Request;
use crate::protocol::response::Response;

fn is_directory_locator(locator: String) -> bool {
    locator.ends_with("/")
}

fn get_root_dir() -> Result<PathBuf> {
    env::current_dir().map_err(|e| Error::new_io(e.to_string().as_str()))
}

async fn is_host_exists(host: String) -> Result<bool> {
    let mut path = get_root_dir()?;
    path.push(host.as_str());

    fs::read_dir(path)
        .await
        .map_or_else(|_| Ok(false), |_| Ok(true))
}

async fn process_plain_file(host: String, locator: String) -> Result<Response> {
    let mut file_path = get_root_dir()?;
    file_path.push(host.as_str());
    file_path.push(&locator.as_str()[1..]);

    debug!(
        "Processing locator: {} file: {}",
        locator,
        file_path.to_string_lossy()
    );

    let mime = filename_to_mime(locator);
    let content = fs::read(file_path)
        .await
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    Ok(Response::new_success(mime, Bytes::from(content)))
}

async fn process_directory(host: String, locator: String) -> Result<Response> {
    let index_gmi = format!("{}index.gmi", locator);
    let index_txt = format!("{}index.txt", locator);

    process_plain_file(host.clone(), index_gmi)
        .await
        .or(process_plain_file(host, index_txt).await)
}

async fn process_request(host: String, locator: String) -> Result<Response> {
    if is_directory_locator(locator.clone()) {
        process_directory(host, locator).await
    } else {
        process_plain_file(host, locator).await
    }
}

pub async fn route(request: Request) -> Result<Response> {
    let host = request.host;

    let is_any_exists = is_host_exists("any".to_string()).await?;
    let is_required_host_exists = is_host_exists(host.clone()).await?;

    // TODO: Remove this crutch
    if request.data_len > 0 {
        Ok(Response::new_server_error(
            "Uploads not implemented".to_string(),
        ))
    } else {
        if is_any_exists || is_required_host_exists {
            let selected_host = if is_required_host_exists {
                host
            } else {
                "any".to_string()
            };

            debug!("Processing host: {}", selected_host);

            process_request(selected_host, request.locator).await
        } else {
            Ok(Response::new_server_error("Host not served".to_string()))
        }
    }
}

#[test]
fn is_directory_locator_test() {
    assert_eq!(is_directory_locator("/".to_string()), true);
    assert_eq!(is_directory_locator("/some path/".to_string()), true);
    assert_eq!(
        is_directory_locator("/nested/path/to/directory/".to_string()),
        true
    );
    assert_eq!(is_directory_locator("/index.gmi".to_string()), false);
    assert_eq!(
        is_directory_locator("/long/path/to/file".to_string()),
        false
    );
}
