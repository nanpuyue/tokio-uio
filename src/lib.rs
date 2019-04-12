use std::io::{Error, Read, Result, Write};
use std::path::Path;

use futures::{Async, Poll};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_reactor::PollEvented;

mod mio;
mod stream;

pub struct Uio {
    io: PollEvented<mio::Uio>,
}

impl Uio {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let uio = mio::Uio::open(path)?;
        Ok(Self {
            io: PollEvented::new(uio),
        })
    }

    pub fn enable_intrpts(&mut self) -> Result<()> {
        match self.write(&1u32.to_ne_bytes())? {
            4 => Ok(()),
            _ => unreachable!(),
        }
    }
}

impl Read for Uio {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.io.read(buf)
    }
}

impl Write for Uio {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.io.get_mut().write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.get_mut().flush()
    }
}

impl AsyncRead for Uio {
    fn poll_read(&mut self, buf: &mut [u8]) -> Result<Async<usize>> {
        self.io.poll_read(buf)
    }
}

impl AsyncWrite for Uio {
    fn poll_write(&mut self, buf: &[u8]) -> Result<Async<usize>> {
        self.write(buf).map(Async::Ready)
    }

    fn shutdown(&mut self) -> Poll<(), Error> {
        self.io.shutdown()
    }
}
