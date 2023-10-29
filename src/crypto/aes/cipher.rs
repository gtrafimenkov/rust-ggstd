// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::block::{decrypt_block_go, encrypt_block_go};
use crate::crypto::cipher::Block;

/// The AES block size in bytes.
pub const BLOCK_SIZE: usize = 16;

/// Cipher is an instance of AES encryption using a particular key.
pub struct Cipher {
    enc: Vec<u32>,
    dec: Vec<u32>,
}

#[derive(Debug)]
pub struct KeySizeError(usize);

impl std::fmt::Display for KeySizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "crypto/aes: invalid key size: {}", self.0)
    }
}

/// new_cipher_generic creates and returns a new cipher::Block
/// implemented in pure Go.
fn new_cipher_generic(key: &[u8]) -> Cipher {
    let n = key.len() + 28;
    let mut c = Cipher {
        enc: vec![0; n],
        dec: vec![0; n],
    };
    super::block::expand_key_go(key, &mut c.enc, &mut c.dec);
    c
}

impl Cipher {
    /// new creates and returns a new AES Cipher.
    /// The key argument should be the AES key,
    /// either 16, 24, or 32 bytes to select
    /// AES-128, AES-192, or AES-256.
    pub fn new(key: &[u8]) -> Result<Cipher, KeySizeError> {
        let k = key.len();
        match k {
            16 | 24 | 32 => {}
            _ => return Err(KeySizeError(k)),
        };
        // 	if boring.Enabled {
        // 		return boring.NewAESCipher(key)
        // 	}
        Ok(new_cipher_generic(key))
    }
}

impl Block for Cipher {
    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    fn encrypt(&self, dst: &mut [u8], src: &[u8]) {
        if src.len() < BLOCK_SIZE {
            panic!("crypto/aes: input not full block");
        }
        if dst.len() < BLOCK_SIZE {
            panic!("crypto/aes: output not full block");
        }
        // rust ggstd: in Rust overlap between mutable and immutable slice is not possible
        // 	if alias.InexactOverlap(dst[..block_size], src[..block_size]) {
        // 		panic!("crypto/aes: invalid buffer overlap");
        // 	}
        encrypt_block_go(&self.enc, dst, src);
    }

    fn decrypt(&self, dst: &mut [u8], src: &[u8]) {
        if src.len() < BLOCK_SIZE {
            panic!("crypto/aes: input not full block");
        }
        if dst.len() < BLOCK_SIZE {
            panic!("crypto/aes: output not full block");
        }
        // rust ggstd: in Rust overlap between mutable and immutable slice is not possible
        // if alias.InexactOverlap(dst[..block_size], src[..block_size]) {
        //     panic!("crypto/aes: invalid buffer overlap");
        // }
        decrypt_block_go(&self.dec, dst, src);
    }
}
