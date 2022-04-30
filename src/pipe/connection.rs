use crate::error::{Error, Result};

use bytes::{Bytes, BytesMut};
use log::debug;
use std::pin::Pin;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

/// From procotol spec
const BUFFER_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Connection {
    reader: BufReader<&'static mut TcpStream>,
    writer: BufWriter<&'static mut TcpStream>,
    _stream: Pin<Box<TcpStream>>,
}

#[allow(mutable_transmutes)]
impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let pin = Box::pin(stream);

        unsafe {
            Self {
                reader: BufReader::new(std::mem::transmute(&*pin)),
                writer: BufWriter::new(std::mem::transmute(&*pin)),
                _stream: pin,
            }
        }
    }

    fn get_reader(&mut self) -> &mut BufReader<&mut TcpStream> {
        unsafe { std::mem::transmute(&self.reader) }
    }

    fn get_writer(&mut self) -> &mut BufWriter<&mut TcpStream> {
        unsafe { std::mem::transmute(&self.writer) }
    }

    /// Read N bytes from TcpStream
    pub async fn read_count(&mut self, count: usize) -> Result<Bytes> {
        let mut left = count;
        let mut buffer: Vec<u8> = Vec::new();

        while left > 0 {
            debug!("Reading request data. {} bytes left", left);
            let datum = self.read_chunk(left).await?;

            if datum.len() == 0 {
                break;
            }

            let mut vectored = Vec::from(&datum[..]);
            buffer.append(&mut vectored);

            left -= datum.len();
        }

        Ok(Bytes::from(buffer))
    }

    /// Reading not more than N bytes.
    /// Usually you'll read from socket less count
    async fn read_chunk(&mut self, count: usize) -> Result<Bytes> {
        let mut buffer: Vec<u8> = Vec::with_capacity(count);
        io_err!(
            self.get_reader()
                .take(count.try_into().unwrap())
                .read_buf(&mut buffer)
                .await
        )?;

        Ok(Bytes::from(buffer))
    }

    /// Function for reading request
    pub async fn read_line(&mut self) -> Result<String> {
        let mut buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);

        let count = io_err!(
            self.get_reader()
                .read_until(10u8, &mut buffer) // Cause CRLF is ending
                .await
        )?;

        debug!("Request read: {} bytes", count);

        io_err!(String::from_utf8(buffer).map(|s| s.trim_end().to_string()))
    }

    /// Output buffer to socket
    pub async fn write_buf(&mut self, mut buf: BytesMut) -> Result<()> {
        io_err!(self.get_writer().write_all_buf(&mut buf).await)?;
        io_err!(self.get_writer().flush().await)?;

        debug!(
            "Connection with {}. {} bytes sent",
            self._stream.peer_addr().unwrap(),
            buf.len()
        );

        Ok(())
    }
}
