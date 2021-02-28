use std::io::{Read, Result, Write};
use std::net::TcpStream;
use std::os::unix::net::UnixStream;
use std::time::Duration;

pub trait Stream: Read + Write {
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()>;
    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()>;
}

impl Stream for TcpStream {
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        TcpStream::set_nonblocking(self, nonblocking)
    }

    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
        TcpStream::set_read_timeout(self, dur)
    }
}

impl Stream for UnixStream {
    fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        UnixStream::set_nonblocking(self, nonblocking)
    }

    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
        UnixStream::set_read_timeout(self, dur)
    }
}
