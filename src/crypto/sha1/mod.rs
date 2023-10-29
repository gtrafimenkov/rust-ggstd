// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package sha1 implements the SHA-1 hash algorithm as defined in RFC 3174.
//!
//! SHA-1 is cryptographically broken and should not be used for secure
//! applications.

mod sha1;

pub use sha1::{sum, Digest, BLOCK_SIZE, SIZE};

#[cfg(test)]
mod sha1_test;
