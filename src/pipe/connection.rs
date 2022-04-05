use std::io::Cursor;

use crate::error::{Error, Result};
use bytes::BytesMut;
use log::debug;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// From procotol spec
const BUFFER_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: stream,
            buffer: BytesMut::with_capacity(BUFFER_SIZE),
        }
    }

    /// Function for reading request
    pub async fn read_line(&mut self) -> Result<String> {
        let mut buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);

        self.stream
            .read_buf(&mut self.buffer)
            .await
            .map_err(|e| Error::new_io(e.to_string().as_str()))?;

        let mut reader = Cursor::new(&self.buffer[..]);

        let count = reader
            .read_until(10u8, &mut buffer) // Cause CRLF is ending
            .await
            .map_err(|e| Error::new_io(e.to_string().as_str()))?;

        debug!("Request read: {} bytes", count);

        String::from_utf8(buffer)
            .map(|s| s.trim_end().to_string())
            .map_err(|e| Error::new_io(e.to_string().as_str()))
    }

    /// Output buffer to socket
    pub async fn write_buf(&mut self, mut buf: BytesMut) -> Result<()> {
        let bytes = self
            .stream
            .write_buf(&mut buf)
            .await
            .map_err(|e| Error::new_io(e.to_string().as_str()))?;

        debug!(
            "Connection with {}. {} bytes sent",
            self.stream.peer_addr().unwrap(),
            bytes
        );

        Ok(())
    }
}
