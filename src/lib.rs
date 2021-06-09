#[cfg(test)]
mod tests;

use std::{
    io::{self, BufWriter, IntoInnerError, Seek, SeekFrom, Write},
    sync::{
        mpsc::{sync_channel, SyncSender},
        Arc,
    },
    thread::{self, JoinHandle},
};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

enum Message {
    Contents(Vec<u8>),
    Flush,
    End,
}

fn spawn_writer_thread<W: 'static + Write + Send>(
    mut writer: BufWriter<W>,
) -> (SyncSender<Message>, JoinHandle<BufWriter<W>>) {
    let (sender, receiver) = sync_channel(0);
    let handle = thread::spawn(move || loop {
        match receiver.recv() {
            Ok(msg) => match msg {
                Message::Contents(contents) => {
                    if let Err(error) = writer.write_all(&contents) {
                        todo!()
                    }
                }
                Message::Flush => {
                    if let Err(error) = writer.flush() {
                        todo!()
                    }
                }
                Message::End => {
                    if let Err(error) = writer.flush() {
                        todo!()
                    }

                    return writer;
                }
            },
            Err(_) => {
                // TODO: what should be done here? This means the channel would be closed which
                // means that `EagerBufWriter` wouldn't exist anymore either
                return writer;
            }
        }
    });

    (sender, handle)
}

// TODO: have some mutex<arc<e>> or something for errors?
// TODO: keep a condvar for being done flushing
#[derive(Debug)]
pub struct EagerBufWriter<W: 'static + Write + Send> {
    write_thread: JoinHandle<BufWriter<W>>,
    sender: SyncSender<Message>,
}

impl<W: 'static + Write + Send> EagerBufWriter<W> {
    pub fn new(writer: W) -> Self {
        EagerBufWriter::with_capacity(DEFAULT_BUF_SIZE, writer)
    }

    pub fn with_capacity(capacity: usize, writer: W) -> Self {
        // let buf_writer = BufWriter::with_capacity(capacity, writer);
        // let flushed = Condvar junk
        // some sort of return error channel? Can I group stuff up better?
        let (sender, handle) = spawn_writer_thread(BufWriter::with_capacity(capacity, writer));

        Self {
            write_thread: handle,
            sender,
        }
    }

    fn flush_buf(&mut self) -> io::Result<()> {
        todo!("What is this actually used for")
    }

    fn write_to_buf(&mut self, buf: &[u8]) -> usize {
        todo!("What is this actually used for");
    }

    // TODO: will this be allowed with this implementation? The writer will be in another thread
    // TODO: planning on trying to get this working by keeping the writer in an `Arc`
    pub fn get_ref(&self) -> &W {
        todo!()
    }

    // TODO: will this be allowed with this implementation? The writer will be in another thread
    // TODO: planning on trying to get this working by keeping the writer in an `Arc`
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

    pub fn into_inner(mut self) -> Result<W, IntoInnerError<Self>> {
        todo!()
    }
}

impl<W: Write + Send> Write for EagerBufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl<W: 'static + Write + Send + Seek> Seek for EagerBufWriter<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.flush_buf()?;
        self.get_mut().seek(pos)
    }
}

impl<W: Write + Send> Drop for EagerBufWriter<W> {
    fn drop(&mut self) {
        todo!()
    }
}
