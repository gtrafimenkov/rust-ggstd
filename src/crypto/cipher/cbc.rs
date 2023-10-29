// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Cipher block chaining (CBC) mode.
//!
//! CBC provides confidentiality by xoring (chaining) each plaintext block
//! with the previous ciphertext block before applying the block cipher.
//!
//! See NIST SP 800-38A, pp 10-11

use crate::compat;
use crate::crypto::cipher::{Block, BlockMode};
use crate::crypto::subtle;

/// CBCEncrypter implements encryption in cipher block chaining
/// mode, using the given Block.
pub struct CBCEncrypter<'a, B: Block> {
    b: &'a B,
    block_size: usize,
    iv: Vec<u8>,
    tmp: Vec<u8>,
}

impl<'a, B: Block> CBCEncrypter<'a, B> {
    /// new returns a BlockMode which encrypts in cipher block chaining
    /// mode, using the given Block. The length of iv must be the same as the
    /// Block's block size.
    pub fn new(b: &'a B, iv: &[u8]) -> Self {
        let blocksize = b.block_size();
        if iv.len() != blocksize {
            panic!("cipher.NewCBCEncrypter: IV length must equal block size");
        }
        Self {
            b,
            block_size: blocksize,
            iv: iv.to_vec(),
            tmp: vec![0; blocksize],
        }
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        if iv.len() != self.iv.len() {
            panic!("cipher: incorrect length IV");
        }
        compat::copy(&mut self.iv, iv);
    }
}

impl<'a, B: Block> BlockMode for CBCEncrypter<'a, B> {
    fn block_size(&self) -> usize {
        self.block_size
    }

    fn crypt_blocks(&mut self, dst: &mut [u8], src: &[u8]) {
        let block_size = self.block_size;
        if src.len() % block_size != 0 {
            panic!("crypto/cipher: input not full blocks");
        }
        if dst.len() < src.len() {
            panic!("crypto/cipher: output smaller than input");
        }
        if src.is_empty() {
            return;
        }

        let tmp = &mut self.tmp;

        // The first block is special because it uses the saved iv.
        subtle::xor_bytes(tmp, &src[..block_size], &self.iv);
        self.b.encrypt(&mut dst[..block_size], tmp);

        // The rest is encrypted using the previous block as IV
        let mut offset = block_size;
        let mut iv_offset = 0;

        while offset < src.len() {
            subtle::xor_bytes(
                tmp,
                &src[offset..offset + block_size],
                &dst[iv_offset..iv_offset + block_size],
            );
            self.b.encrypt(&mut dst[offset..offset + block_size], tmp);
            offset += block_size;
            iv_offset += block_size;
        }
        // save the last encrypted block as the IV for the next encryption
        compat::copy(&mut self.iv, &dst[iv_offset - block_size..iv_offset]);
    }

    fn crypt_blocks_inplace(&mut self, data: &mut [u8]) {
        let block_size = self.block_size;
        if data.len() % block_size != 0 {
            panic!("crypto/cipher: input not full blocks");
        }
        if data.is_empty() {
            return;
        }

        let tmp = &mut self.tmp;

        // The first block is special because it uses the saved iv.
        subtle::xor_bytes(tmp, &data[..block_size], &self.iv);
        self.b.encrypt(&mut data[..block_size], tmp);

        // The rest is encrypted using the previous block as IV
        let mut prev = 0;
        let mut start = block_size;
        let mut end = start + block_size;
        while start < data.len() {
            subtle::xor_bytes(tmp, &data[start..end], &data[prev..start]);
            self.b.encrypt(&mut data[start..end], tmp);
            (prev, start, end) = (start, end, end + block_size);
        }
        // save the last encrypted block as the IV for the next encryption
        compat::copy(&mut self.iv, &data[data.len() - block_size..]);
    }
}

/// CBCDecrypter implements decryption in cipher block chaining
/// mode, using the given Block.
pub struct CBCDecrypter<'a, B: Block> {
    b: &'a B,
    block_size: usize,
    iv: Vec<u8>,
    tmp: Vec<u8>,
    tmp_next_iv: Vec<u8>,
}

impl<'a, B: Block> CBCDecrypter<'a, B> {
    /// new returns a BlockMode which decrypts in cipher block chaining
    /// mode, using the given Block. The length of iv must be the same as the
    /// Block's block size and must match the iv used to encrypt the data.
    pub fn new(b: &'a B, iv: &[u8]) -> Self {
        let blocksize = b.block_size();
        if iv.len() != blocksize {
            panic!("cipher.NewCBCDecrypter: IV length must equal block size");
        }
        Self {
            b,
            block_size: blocksize,
            iv: iv.to_vec(),
            tmp: vec![0; blocksize],
            tmp_next_iv: vec![0; blocksize],
        }
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        if iv.len() != self.iv.len() {
            panic!("cipher: incorrect length IV");
        }
        compat::copy(&mut self.iv, iv);
    }
}

impl<'a, B: Block> BlockMode for CBCDecrypter<'a, B> {
    fn block_size(&self) -> usize {
        self.block_size
    }

    fn crypt_blocks(&mut self, dst: &mut [u8], src: &[u8]) {
        let block_size = self.block_size;
        if src.len() % block_size != 0 {
            panic!("crypto/cipher: input not full blocks");
        }
        if dst.len() < src.len() {
            panic!("crypto/cipher: output smaller than input");
        }
        if src.is_empty() {
            return;
        }

        let tmp = &mut self.tmp;

        // For each block, we need to xor the decrypted data with the previous block's ciphertext (the iv).
        // The first block is special because it uses the saved iv.
        self.b.decrypt(tmp, &src[..block_size]);
        subtle::xor_bytes(&mut dst[..block_size], tmp, &self.iv);

        // The rest is encrypted using the previous block as IV
        let mut prev = 0;
        let mut start = block_size;
        let mut end = start + block_size;
        while start < src.len() {
            self.b.decrypt(tmp, &src[start..end]);
            subtle::xor_bytes(&mut dst[start..end], tmp, &src[prev..start]);
            (prev, start, end) = (start, end, end + block_size);
        }
        // save the last source block as the IV for the next decryption
        compat::copy(&mut self.iv, &src[src.len() - block_size..]);
    }

    fn crypt_blocks_inplace(&mut self, data: &mut [u8]) {
        let block_size = self.block_size;
        if data.len() % block_size != 0 {
            panic!("crypto/cipher: input not full blocks");
        }
        if data.is_empty() {
            return;
        }

        let tmp = &mut self.tmp;

        // For each block, we need to xor the decrypted data with the previous block's ciphertext (the iv).
        // To avoid making a copy each time, we loop over the blocks BACKWARDS.
        let mut end = data.len();
        let mut start = end - block_size;
        let mut prev = start - block_size;

        // Copy the last block of ciphertext in preparation as the new iv.
        compat::copy(&mut self.tmp_next_iv, &data[start..end]);

        // Loop over all but the first block.
        while start > 0 {
            self.b.decrypt(tmp, &data[start..end]);
            // we need:
            //   - data[start..end] as writable
            //   - data[prev..start] as readable
            subtle::xor_bytes(
                unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr().add(start), block_size) },
                unsafe { std::slice::from_raw_parts(data.as_ptr().add(prev), block_size) },
                tmp,
            );
            (end, start, prev) = (start, prev, prev.wrapping_sub(block_size));
        }

        // The first block is special because it uses the saved iv.
        self.b.decrypt(tmp, &data[..block_size]);
        subtle::xor_bytes(&mut data[..block_size], tmp, &self.iv);

        // Set the new iv to the first block we copied earlier.
        compat::copy(&mut self.iv, &self.tmp_next_iv);
    }
}
