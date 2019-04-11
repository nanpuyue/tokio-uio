use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Result, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

use ::mio::unix::EventedFd;
use ::mio::{Evented, Poll, PollOpt, Ready, Token};

pub struct Uio {
    inner: File,
}

impl Uio {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let uio = OpenOptions::new().read(true).write(true).open(path)?;

        let flags = unsafe { libc::fcntl(uio.as_raw_fd(), libc::F_GETFL) };
        if flags < 0 {
            return Err(Error::last_os_error());
        }

        match unsafe { libc::fcntl(uio.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) } {
            0 => Ok(Self { inner: uio }),
            _ => Err(Error::last_os_error()),
        }
    }
}

impl Read for Uio {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for Uio {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}

impl AsRawFd for Uio {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl Evented for Uio {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        EventedFd(&self.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> Result<()> {
        EventedFd(&self.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> Result<()> {
        EventedFd(&self.as_raw_fd()).deregister(poll)
    }
}
