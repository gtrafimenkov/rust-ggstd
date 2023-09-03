pub mod io;

pub use io::{
    copy, copy_buffer, copy_n, read_all, read_full, ByteReader, Discard, Seek, ERR_NO_PROGRESS,
};

#[cfg(test)]
mod io_test;
