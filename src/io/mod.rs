// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

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
    copy, copy_buffer, copy_n, err_no_progress, is_eof, is_short_write_error, is_unexpected_eof,
    new_error_short_write, read_all, read_full, write_string, ByteReader, Discard, IoRes, Reader,
    Seek, EOF,
};

#[cfg(test)]
mod io_test;
