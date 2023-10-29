// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package hex implements hexadecimal encoding and decoding.

mod hex;

pub use hex::{decode, decode_string, decoded_len, encode, encode_to_string, encoded_len};

#[cfg(test)]
mod hex_test;
