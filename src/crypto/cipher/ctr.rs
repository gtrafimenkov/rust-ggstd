// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Counter (CTR) mode.
//!
//! CTR converts a block cipher into a stream cipher by
//! repeatedly encrypting an incrementing counter and
//! xoring the resulting stream of data with the input.

use crate::compat;
use crate::crypto::cipher::{Block, Stream};
use crate::crypto::subtle;

// See NIST SP 800-38A, pp 13-15

// package cipher

// import (
// 	"bytes"
// 	"crypto/internal/alias"
// 	"crypto/subtle"
// )

pub struct CTR<'a, B: Block> {
    b: &'a B,
    ctr: Vec<u8>,
    out: Vec<u8>,
    out_size: usize,
    out_used: usize,
}

const STREAM_BUFFER_SIZE: usize = 512;

// // ctrAble is an interface implemented by ciphers that have a specific optimized
// // implementation of CTR, like crypto/aes. NewCTR will check for this interface
// // and return the specific Stream if found.
// type ctrAble interface {
// 	NewCTR(iv &[u8]) Stream
// }

impl<'a, B: Block> CTR<'a, B> {
    /// new returns a Stream which encrypts/decrypts using the given Block in
    /// counter mode. The length of iv must be the same as the Block's block size.
    pub fn new(block: &'a B, iv: &[u8]) -> Self {
        // 	if ctr, ok := block.(ctrAble); ok {
        // 		return ctr.NewCTR(iv)
        // 	}
        let block_size = block.block_size();
        if iv.len() != block_size {
            panic!("cipher.NewCTR: IV length must equal block size");
        }
        let buf_size = STREAM_BUFFER_SIZE.max(block_size);
        Self {
            b: block,
            ctr: iv.to_vec(),
            out: vec![0; buf_size],
            out_size: 0,
            out_used: 0,
        }
    }

    fn refill(&mut self) {
        if self.out_used > 0 {
            compat::copy_within(&mut self.out, self.out_used..self.out_size, 0);
            self.out_size -= self.out_used;
            self.out_used = 0;
        }
        let bs = self.b.block_size();
        while self.out_size <= self.out.len() - bs {
            self.b.encrypt(&mut self.out[self.out_size..], &self.ctr);
            self.out_size += bs;

            // Increment counter
            for i in (0..self.ctr.len()).rev() {
                self.ctr[i] = self.ctr[i].wrapping_add(1);
                if self.ctr[i] != 0 {
                    break;
                }
            }
        }
    }
}

impl<'a, B: Block> Stream for CTR<'a, B> {
    fn xor_key_stream(&mut self, dst: &mut [u8], src: &[u8]) {
        if dst.len() < src.len() {
            panic!("crypto/cipher: output smaller than input");
        }
        // 	if alias.InexactOverlap(dst[..src.len()], src) {
        // 		panic("crypto/cipher: invalid buffer overlap")
        // 	}
        let mut dst = dst;
        let mut src = src;
        while !src.is_empty() {
            if self.out_used + self.b.block_size() >= self.out_size {
                self.refill()
            }
            let size = (self.out_size - self.out_used).min(src.len());
            let n = subtle::xor_bytes(
                &mut dst[..size],
                &src[..size],
                &self.out[self.out_used..self.out_size],
            );
            dst = &mut dst[n..];
            src = &src[n..];
            self.out_used += n;
        }
    }

    fn xor_key_stream_inplace(&mut self, data: &mut [u8]) {
        let mut data = data;
        while !data.is_empty() {
            if self.out_used + self.b.block_size() >= self.out_size {
                self.refill()
            }
            let size = (self.out_size - self.out_used).min(data.len());
            let n = subtle::xor_bytes_inplace(
                &mut data[..size],
                &self.out[self.out_used..self.out_size],
            );
            data = &mut data[n..];
            self.out_used += n;
        }
    }
}
