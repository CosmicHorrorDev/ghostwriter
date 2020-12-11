use crate::EagerBufWriter;

use std::{
    io::{self, Read, Seek, SeekFrom, Write},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

// TODO: add in some counting writer that keeps track of the number of writes it performs
// TODO: Can there also be a delay writer that will delay the writes it performs?

/// A dummy reader intended at testing short-reads propagation.
pub struct ShortReader {
    lengths: Vec<usize>,
}

// FIXME: rustfmt and tidy disagree about the correct formatting of this
// function. This leads to issues for users with editors configured to
// rustfmt-on-save.
impl Read for ShortReader {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        if self.lengths.is_empty() {
            Ok(0)
        } else {
            Ok(self.lengths.remove(0))
        }
    }
}

#[test]
fn buffered_writer() {
    todo!("Figure out how we want to test stuff here");

    let inner = Vec::new();
    let mut writer = EagerBufWriter::with_capacity(2, inner);

    writer.write(&[0, 1]).unwrap();
    assert_eq!(writer.buffer(), []);
    assert_eq!(*writer.get_ref(), [0, 1]);

    writer.write(&[2]).unwrap();
    assert_eq!(writer.buffer(), [2]);
    assert_eq!(*writer.get_ref(), [0, 1]);

    writer.write(&[3]).unwrap();
    assert_eq!(writer.buffer(), [2, 3]);
    assert_eq!(*writer.get_ref(), [0, 1]);

    writer.flush().unwrap();
    assert_eq!(writer.buffer(), []);
    assert_eq!(*writer.get_ref(), [0, 1, 2, 3]);

    writer.write(&[4]).unwrap();
    writer.write(&[5]).unwrap();
    assert_eq!(writer.buffer(), [4, 5]);
    assert_eq!(*writer.get_ref(), [0, 1, 2, 3]);

    writer.write(&[6]).unwrap();
    assert_eq!(writer.buffer(), [6]);
    assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5]);

    writer.write(&[7, 8]).unwrap();
    assert_eq!(writer.buffer(), []);
    assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8]);

    writer.write(&[9, 10, 11]).unwrap();
    assert_eq!(writer.buffer(), []);
    assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

    writer.flush().unwrap();
    assert_eq!(writer.buffer(), []);
    assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn buffered_writer_inner_flushes() {
    let mut w = EagerBufWriter::with_capacity(3, Vec::new());
    w.write(&[0, 1]).unwrap();
    assert_eq!(*w.get_ref(), []);
    let w = w.into_inner().unwrap();
    assert_eq!(w, [0, 1]);
}

#[test]
fn buffered_writer_seek() {
    let mut w = EagerBufWriter::with_capacity(3, io::Cursor::new(Vec::new()));
    w.write_all(&[0, 1, 2, 3, 4, 5]).unwrap();
    w.write_all(&[6, 7]).unwrap();
    assert_eq!(w.seek(SeekFrom::Current(0)).ok(), Some(8));
    assert_eq!(&w.get_ref().get_ref()[..], &[0, 1, 2, 3, 4, 5, 6, 7][..]);
    assert_eq!(w.seek(SeekFrom::Start(2)).ok(), Some(2));
    w.write_all(&[8, 9]).unwrap();
    assert_eq!(
        &w.into_inner().unwrap().into_inner()[..],
        &[0, 1, 8, 9, 4, 5, 6, 7]
    );
}

#[test]
#[should_panic]
fn dont_panic_in_drop_on_panicked_flush() {
    struct FailFlushWriter;

    impl Write for FailFlushWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::last_os_error())
        }
    }

    let writer = FailFlushWriter;
    let _writer = EagerBufWriter::new(writer);

    // If writer panics *again* due to the flush error then the process will
    // abort.
    panic!();
}

#[test]
#[cfg_attr(target_os = "emscripten", ignore)]
fn panic_in_write_doesnt_flush_in_drop() {
    static WRITES: AtomicUsize = AtomicUsize::new(0);

    struct PanicWriter;

    impl Write for PanicWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            WRITES.fetch_add(1, Ordering::SeqCst);
            panic!();
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    thread::spawn(|| {
        let mut writer = EagerBufWriter::new(PanicWriter);
        let _ = writer.write(b"hello world");
        let _ = writer.flush();
    })
    .join()
    .unwrap_err();

    assert_eq!(WRITES.load(Ordering::SeqCst), 1);
}

// TODO: figure this out
// #[bench]
// fn bench_buffered_writer(b: &mut test::Bencher) {
//     b.iter(|| EagerBufWriter::new(io::sink()));
// }

/// A simple `Write` target, designed to be wrapped by `LineWriter` /
/// `BufWriter` / etc, that can have its `write` & `flush` behavior
/// configured
#[derive(Default, Clone)]
struct ProgrammableSink {
    // Writes append to this slice
    pub buffer: Vec<u8>,

    // Flush sets this flag
    pub flushed: bool,

    // If true, writes will always be an error
    pub always_write_error: bool,

    // If true, flushes will always be an error
    pub always_flush_error: bool,

    // If set, only up to this number of bytes will be written in a single
    // call to `write`
    pub accept_prefix: Option<usize>,

    // If set, counts down with each write, and writes return an error
    // when it hits 0
    pub max_writes: Option<usize>,

    // If set, attempting to write when max_writes == Some(0) will be an
    // error; otherwise, it will return Ok(0).
    pub error_after_max_writes: bool,
}

impl Write for ProgrammableSink {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if self.always_write_error {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "test - always_write_error",
            ));
        }

        match self.max_writes {
            Some(0) if self.error_after_max_writes => {
                return Err(io::Error::new(io::ErrorKind::Other, "test - max_writes"));
            }
            Some(0) => return Ok(0),
            Some(ref mut count) => *count -= 1,
            None => {}
        }

        let len = match self.accept_prefix {
            None => data.len(),
            Some(prefix) => data.len().min(prefix),
        };

        let data = &data[..len];
        self.buffer.extend_from_slice(data);

        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.always_flush_error {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "test - always_flush_error",
            ))
        } else {
            self.flushed = true;
            Ok(())
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum RecordedEvent {
    Write(String),
    Flush,
}

#[derive(Debug, Clone, Default)]
struct WriteRecorder {
    pub events: Vec<RecordedEvent>,
}

impl Write for WriteRecorder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use std::str::from_utf8;

        self.events
            .push(RecordedEvent::Write(from_utf8(buf).unwrap().to_string()));
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.events.push(RecordedEvent::Flush);
        Ok(())
    }
}
