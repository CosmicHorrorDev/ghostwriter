#[cfg(test)]
mod tests;

use std::io::{self, IntoInnerError, Seek, SeekFrom, Write};

#[derive(Debug)]
struct EagerBufWriter<W: Write> {
    inner: Option<W>,
}

impl<W: Write> EagerBufWriter<W> {
    pub fn new(inner: W) -> EagerBufWriter<W> {
        todo!()
    }

    pub fn with_capacity(capacity: usize, inner: W) -> EagerBufWriter<W> {
        todo!()
    }

    pub fn buffer(&self) -> &[u8] {
        todo!()
    }

    pub fn get_ref(&self) -> &W {
        todo!()
    }

    pub fn into_inner(self) -> Result<W, IntoInnerError<EagerBufWriter<W>>> {
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
        todo!()
    }
}
