// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package base64 implements base64 encoding as specified by RFC 4648.

mod base64;

pub use base64::{
    get_raw_std_encoding, get_raw_std_encoding_strict, get_raw_url_encoding,
    get_raw_url_encoding_strict, get_std_encoding, get_std_encoding_strict, get_url_encoding,
    get_url_encoding_strict, Decoder, Encoder, Encoding, Error,
};

#[cfg(test)]
mod base64_test;
