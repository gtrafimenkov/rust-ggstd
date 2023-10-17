// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::crypto::md5;
use ggstd::encoding::hex;
use ggstd::hash::Hash;
use std::io::Write;

fn main() {
    if std::env::args().len() > 1 {
        for arg in std::env::args().skip(1) {
            example_new_file(&arg);
        }
    } else {
        example_new();
        example_sum();
    }
}

fn example_new() {
    let mut h = md5::new();
    h.write_all(b"The fog is getting thicker!").unwrap();
    h.write_all(b"And Leon's getting laaarger!").unwrap();
    println!("{}", hex::encode_to_string(&h.sum(&[])));
    // Output: e2c569be17396eca2a2e3c11578123ed
}

fn example_sum() {
    let data = b"These pretzels are making me thirsty.";
    let hash = md5::sum(data);
    println!("{}", hex::encode_to_string(&hash));
    // Output: b0804ec967f48520697662a204f5fe72
}

fn example_new_file(path: &str) {
    let mut f = std::fs::File::open(path).unwrap();
    let mut h = md5::new();
    std::io::copy(&mut f, &mut h).unwrap();
    println!("{}", hex::encode_to_string(&h.sum(&[])));
}
