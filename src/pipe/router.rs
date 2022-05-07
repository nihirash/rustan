use log::debug;
use std::path::PathBuf;

use tokio::fs;

use crate::configuration::SETTINGS;
use crate::error::Result;
use crate::pipe::directory::process_directory;
use crate::pipe::file::process_file;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::protocol::NOT_SERVED;

fn is_directory_locator(locator: String) -> bool {
    locator.ends_with('/')
}

pub async fn get_root_dir() -> Result<PathBuf> {
    Ok(PathBuf::from(SETTINGS.read().await.to_owned().root_path))
}

async fn is_host_exists(host: String) -> Result<bool> {
    let mut path = get_root_dir().await?;
    path.push(host.as_str());

    fs::read_dir(path)
        .await
        .map_or_else(|_| Ok(false), |_| Ok(true))
}

async fn process_request(request: Request) -> Result<Response> {
    if is_directory_locator(request.locator.clone()) {
        process_directory(request).await
    } else {
        process_file(request).await
    }
}

pub async fn route(request: Request) -> Result<Response> {
    let host = request.host.clone();

    let is_any_exists = is_host_exists("any".to_string()).await?;
    let is_required_host_exists = is_host_exists(host.clone()).await?;

    if is_any_exists || is_required_host_exists {
        let selected_host = if is_required_host_exists {
            host
        } else {
            "any".to_string()
        };

        debug!("Processing host: {}", selected_host);

        let mut updated_request = request.clone();
        updated_request.host = selected_host;

        process_request(updated_request).await
    } else {
        Ok(Response::new_server_error(NOT_SERVED.to_string()))
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
