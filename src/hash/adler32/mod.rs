// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package adler32 implements the Adler-32 checksum.
//!
//! It is defined in RFC 1950:
//!
//! Adler-32 is composed of two sums accumulated per byte: s1 is
//! the sum of all bytes, s2 is the sum of all s1 values. Both sums
//! are done modulo 65521. s1 is initialized to 1, s2 to zero.  The
//! Adler-32 checksum is stored as s2*65536 + s1 in most-
//! significant-byte first (network) order.

mod adler32;

pub use adler32::{checksum, new, Digest};

#[cfg(test)]
mod adler32_test;
