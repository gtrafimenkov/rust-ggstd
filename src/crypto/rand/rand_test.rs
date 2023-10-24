// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::bytes;
use crate::compress::flate;
use crate::crypto::rand;
use std::io::Read;
use std::io::Write;

#[test]
fn test_read() {
    let n = 4_000_000;
    let mut b = vec![0; n];
    let mut r = rand::Reader::new();
    r.read_exact(&mut b).unwrap();

    let mut z = bytes::Buffer::new();
    let mut w = flate::Writer::new(&mut z, flate::BEST_COMPRESSION).unwrap();
    w.write_all(&b).unwrap();
    w.close().unwrap();
    assert!(z.len() > n * 99 / 100);
}

#[test]
fn test_read_empty() {
    let n = 0;
    let mut b = vec![0; n];
    let mut r = rand::Reader::new();
    r.read_exact(&mut b).unwrap();
}
