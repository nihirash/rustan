use crate::error::{Error, Result};
use crate::pipe::file::{process_file, read_file};
use crate::pipe::router::get_root_dir;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::protocol::NOT_ALLOWED;
use bytes::Bytes;
use urlencoding::encode;

async fn process_directory_list(host: String, locator: String) -> Result<Response> {
    let mut path = get_root_dir()?;
    path.push(host);
    path.push(&locator.as_str()[1..]);

    let mut file_path = path.clone();
    file_path.push(".listfiles");

    let mut header = read_file(file_path)
        .await
        .map_err(|_| Error::new_request_error(NOT_ALLOWED))?;

    let mut nl: Vec<u8> = vec![13, 10];

    header.append(&mut nl);

    // It works and works faster than tokio::fs
    io_err!(std::fs::read_dir(path))?
        .filter(|f| f.as_ref().unwrap().file_name() != ".listfiles")
        .for_each(|f| {
            let entry = f.unwrap();
            let file_name = entry.file_name();
            let is_dir = entry.file_type().unwrap().is_dir();
            let name = file_name.to_str().unwrap();

            let line = if is_dir {
                format!("=> {}{}/ <{}>\r\n", locator, encode(name).to_string(), name)
            } else {
                format!("=> {}{} {}\r\n", locator, encode(name).to_string(), name)
            };

            let line_str = line.as_str();
            let mut bytes = line_str.as_bytes().to_vec();

            header.append(&mut bytes);
        });

    Ok(Response::new_success(
        "text/gemini".to_string(),
        Bytes::from(header),
    ))
}

pub async fn process_directory(request: Request) -> Result<Response> {
    let locator = request.locator.clone();
    let host = request.host.clone();

    let mut index_gmi_req = request.clone();
    let mut index_txt_req = request.clone();

    index_gmi_req.locator = format!("{}index.gmi", locator);
    index_txt_req.locator = format!("{}index.txt", locator);

    process_file(index_gmi_req)
        .await
        .or(process_file(index_txt_req).await)
        .or(process_directory_list(host, locator).await)
}
