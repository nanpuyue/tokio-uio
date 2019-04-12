use std::io::Error;

use futures::{Poll, Stream};
use tokio_io::AsyncRead;

use super::Uio;

impl Stream for Uio {
    type Item = u32;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.enable_intrpts()?;
        let buf = &mut [0; 4];

        Ok(self.io.poll_read(buf)?.map(|x| match x {
            4 => Some(u32::from_ne_bytes(*buf)),
            _ => unreachable!(),
        }))
    }
}
