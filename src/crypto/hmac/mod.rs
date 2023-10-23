// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package hmac implements the Keyed-Hash Message Authentication Code (HMAC) as
//! defined in U.S. Federal Information Processing Standards Publication 198.
//! An HMAC is a cryptographic hash that uses a key to sign a message.
//! The receiver verifies the hash by recomputing it using the same key.
//!
//! Receivers should be careful to use Equal to compare MACs in order to avoid
//! timing side-channels:
//!
//!     use ggstd::crypto::hmac;
//!     use ggstd::crypto::sha256;
//!     use ggstd::hash::Hash;
//!     use std::io::Write;
//!
//!     /// valid_mac reports whether message_mac is a valid HMAC tag for message.
//!     fn valid_mac(message: &[u8], message_mac: &[u8], key: &[u8]) -> bool {
//!         let mut mac = hmac::HMAC::new(sha256::Digest::new, key);
//!         mac.write_all(message).unwrap();
//!         let expected_mac = mac.sum(&[]);
//!         hmac::equal(message_mac, &expected_mac)
//!     }

mod hmac;

pub use hmac::{equal, HMAC};

#[cfg(test)]
mod hmac_test;
