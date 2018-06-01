//! Implementation to tcp shipper
//! 
//! Creates a wrapper over tcp stream. ships the raw log line over 
//! the wire.

use std::io::{Write, BufWriter, Error};
use std::net::TcpStream;

#[derive(Debug)]
pub struct TcpShipper {
    inner: BufWriter<TcpStream>
}

impl TcpShipper {
    pub fn new(s: String) -> Result<Self, Error> {
        let stream = TcpStream::connect(s.as_str())?;
        Ok(TcpShipper {inner: BufWriter::new(stream)})
    }

    pub fn send(&mut self, raw: String) -> Result<usize, Error> {
        self.inner.write(raw.as_bytes())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }
}

