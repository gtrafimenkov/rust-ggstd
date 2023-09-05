pub mod io;

pub use io::{
    copy, copy_buffer, copy_n, is_short_write_error, new_error_short_write, read_all, read_full,
    write_string, ByteReader, Discard, Seek, ERR_NO_PROGRESS,
};

#[cfg(test)]
mod io_test;
