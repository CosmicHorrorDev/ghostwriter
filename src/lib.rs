#[cfg(test)]
mod tests;

use std::io::{self, IntoInnerError, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct EagerBufWriter<W: Write> {
    inner: Option<W>,
}

impl<W: Write> EagerBufWriter<W> {
    pub fn new(inner: W) -> EagerBufWriter<W> {
        todo!()
    }

    pub fn with_capacity(capacity: usize, inner: W) -> EagerBufWriter<W> {
        todo!()
    }

    pub fn flush_buf(&mut self) -> io::Result<()> {
        todo!("What is this actually used for")
        // /// Helper struct to ensure the buffer is updated after all the writes
        // /// are complete. It tracks the number of written bytes and drains them
        // /// all from the front of the buffer when dropped.
        // struct BufGuard<'a> {
        //     buffer: &'a mut Vec<u8>,
        //     written: usize,
        // }

        // impl<'a> BufGuard<'a> {
        //     fn new(buffer: &'a mut Vec<u8>) -> Self {
        //         Self { buffer, written: 0 }
        //     }

        //     /// The unwritten part of the buffer
        //     fn remaining(&self) -> &[u8] {
        //         &self.buffer[self.written..]
        //     }

        //     /// Flag some bytes as removed from the front of the buffer
        //     fn consume(&mut self, amt: usize) {
        //         self.written += amt;
        //     }

        //     /// true if all of the bytes have been written
        //     fn done(&self) -> bool {
        //         self.written >= self.buffer.len()
        //     }
        // }

        // impl Drop for BufGuard<'_> {
        //     fn drop(&mut self) {
        //         if self.written > 0 {
        //             self.buffer.drain(..self.written);
        //         }
        //     }
        // }

        // let mut guard = BufGuard::new(&mut self.buf);
        // let inner = self.inner.as_mut().unwrap();
        // while !guard.done() {
        //     self.panicked = true;
        //     let r = inner.write(guard.remaining());
        //     self.panicked = false;

        //     match r {
        //         Ok(0) => {
        //             return Err(Error::new(
        //                 ErrorKind::WriteZero,
        //                 "failed to write the buffered data",
        //             ));
        //         }
        //         Ok(n) => guard.consume(n),
        //         Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
        //         Err(e) => return Err(e),
        //     }
        // }
        // Ok(())
    }

    pub fn write_to_buf(&mut self, buf: &[u8]) -> usize {
        todo!("What is this actually used for");

        // let available = self.buf.capacity() - self.buf.len();
        // let amt_to_buffer = available.min(buf.len());
        // self.buf.extend_from_slice(&buf[..amt_to_buffer]);
        // amt_to_buffer
    }

    // TODO: will this be allowed with this implementation? The writer will be in another thread
    pub fn get_ref(&self) -> &W {
        todo!()
    }

    // TODO: will this be allowed with this implementation? The writer will be in another thread
    pub fn get_mut(&mut self) -> &mut W {
        todo!()
    }

    // TODO: will this be allowed, the buffer will be a channel
    pub fn buffer(&self) -> &[u8] {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn into_inner(mut self) -> Result<W, IntoInnerError<EagerBufWriter<W>>> {
        todo!()
    }
}

impl<W: Write> Write for EagerBufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl<W: Write + Seek> Seek for EagerBufWriter<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.flush_buf()?;
        self.get_mut().seek(pos)
    }
}

impl<W: Write> Drop for EagerBufWriter<W> {
    fn drop(&mut self) {
        todo!()
    }
}
