// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package sha256 implements the SHA224 and SHA256 hash algorithms as defined
//! in FIPS 180-4.

mod sha256;
mod sha256block;

pub use sha256::{sum224, sum256, Digest, BLOCK_SIZE, SIZE, SIZE224};

#[cfg(test)]
mod sha256_test;
