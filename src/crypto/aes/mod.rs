// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package aes implements AES encryption (formerly Rijndael), as defined in
//! U.S. Federal Information Processing Standards Publication 197.
//!
//! The AES operations in this package are not implemented using constant-time algorithms.
// An exception is when running on systems with enabled hardware support for AES
// that makes these operations constant-time. Examples include amd64 systems using AES-NI
// extensions and s390x systems using Message-Security-Assist extensions.
// On such systems, when the result of NewCipher is passed to cipher.NewGCM,
// the GHASH operation used by GCM is also constant-time.

mod block;
mod cipher;
mod consts;

pub use cipher::{Cipher, BLOCK_SIZE};

#[cfg(test)]
mod aes_test;
