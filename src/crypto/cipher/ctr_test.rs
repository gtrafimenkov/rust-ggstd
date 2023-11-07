// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2015 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;
use crate::crypto::cipher::{self, Block, Stream};

struct NoopBlock(usize);

impl Block for NoopBlock {
    fn block_size(&self) -> usize {
        self.0
    }

    fn encrypt(&self, dst: &mut [u8], src: &[u8]) {
        compat::copy(dst, src);
    }

    fn decrypt(&self, dst: &mut [u8], src: &[u8]) {
        compat::copy(dst, src);
    }

    fn encrypt_inplace(&self, _buffer: &mut [u8]) {}

    fn decrypt_inplace(&self, _buffer: &mut [u8]) {}
}

fn inc(b: &mut [u8]) {
    for i in (0..b.len()).rev() {
        b[i] += 1;
        if b[i] != 0 {
            break;
        }
    }
}

fn xor(a: &mut [u8], b: &[u8]) {
    for i in 0..a.len() {
        a[i] ^= b[i];
    }
}

#[test]
fn test_ctr() {
    let mut size = 64;
    while size <= 1024 {
        let iv = vec![0; size];
        let b = NoopBlock(size);
        let mut ctr = cipher::CTR::new(&b, &iv);
        let src = vec![0xff; 1024];
        let mut want = vec![0; 1024];
        compat::copy(&mut want, &src);
        let mut counter = vec![0; size];
        for i in 1..want.len() / size {
            inc(&mut counter);
            xor(&mut want[i * size..(i + 1) * size], &counter);
        }
        let mut dst = vec![0; 1024];
        ctr.xor_key_stream(&mut dst, &src);
        assert_eq!(
            dst, want,
            "for size {}\nhave {:?}\nwant {:?}",
            size, dst, want
        );
        size *= 2;
    }
}
