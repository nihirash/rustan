use crate::error::{Error, Result};
use crate::mime::filename_to_mime;
use crate::pipe::router::get_root_dir;
use crate::protocol::request::Request;
use crate::protocol::response::{Response, StatusCode};
use crate::protocol::NOT_ALLOWED;
use bytes::Bytes;
use is_executable::IsExecutable;
use log::debug;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tokio::fs;

pub async fn read_file(path: PathBuf) -> Result<Vec<u8>> {
    fs::read(path)
        .await
        .map_err(|e| Error::new_io(e.to_string().as_str()))
}

async fn process_plain_file(file_path: PathBuf) -> Result<Response> {
    debug!("Processing file: {}", file_path.to_string_lossy());

    let mime = filename_to_mime(file_path.to_string_lossy().to_string());
    let content = read_file(file_path).await?;

    Ok(Response::new_success(mime, Bytes::from(content)))
}

fn is_executable(path: PathBuf) -> bool {
    path.is_executable()
}

async fn process_cgi(path: PathBuf, request: Request) -> Result<Response> {
    debug!("Executed cgi: {}", path.to_string_lossy());

    let data = request.data.unwrap_or(Bytes::new());

    let result = Command::new(path.clone())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .current_dir(path.parent().unwrap().as_os_str())
        .spawn()
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    let mut stdin = result.stdin.unwrap();

    stdin
        .write(&data[..])
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    stdin
        .flush()
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    drop(stdin);

    let mut buf: Vec<u8> = Vec::new();
    let mut output = result.stdout.unwrap();

    let mut reader = BufReader::new(&mut output);

    let mut status_code_b: Vec<u8> = Vec::with_capacity(2);
    let mut status_line: Vec<u8> = Vec::new();

    reader
        .read_until(32u8, &mut status_code_b)
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    // align char numbers
    let status_code_num = *status_code_b.first().unwrap() - 48;
    let status_code = StatusCode::from_number(status_code_num);

    reader
        .read_until(10u8, &mut status_line)
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    let status_line_str = String::from_utf8(status_line)
        .map(|s| s.trim_end().to_string())
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    reader
        .read_to_end(&mut buf)
        .map_err(|e| Error::new_io(e.to_string().as_str()))?;

    Ok(Response::new(
        status_code,
        status_line_str,
        Some(Bytes::from(buf)),
    ))
}

pub async fn process_file(request: Request) -> Result<Response> {
    let host = request.host.clone();
    let locator = request.locator.clone();

    let mut file_path = get_root_dir()?;
    file_path.push(host);
    file_path.push(&locator.as_str()[1..]);

    if is_executable(file_path.clone()) {
        process_cgi(file_path, request).await
    } else {
        if request.data_len > 0 {
            Ok(Response::new_client_error(NOT_ALLOWED.to_string()))
        } else {
            process_plain_file(file_path).await
        }
    }
}
