pub mod io;

pub use io::{
    copy, copy_buffer, copy_n, read_all, read_full, ByteReader, Discard, Error, ReadCloser,
    ReadWriter, Reader, Result, Seek, Writer, ERR_NO_PROGRESS,
};

#[cfg(test)]
mod io_test;
