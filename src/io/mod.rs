//! Package io provides basic interfaces to I/O primitives.
//! Its primary job is to wrap existing implementations of such primitives,
//! such as those in package os, into shared public interfaces that
//! abstract the functionality, plus some other related primitives.
//!
//! Because these interfaces and primitives wrap lower-level operations with
//! various implementations, unless otherwise informed clients should not
//! assume they are safe for parallel execution.

pub mod io;

pub use io::{
    copy, copy_, copy_buffer, copy_n, err_no_progress, is_short_write_error, new_error_short_write,
    read_all, read_full, write_string, ByteReader, Discard, Seek,
};

#[cfg(test)]
mod io_test;
