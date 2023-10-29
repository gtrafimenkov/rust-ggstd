// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

use ggstd::bytes::Buffer;

use std::io::Write;

fn main() {
    let mut buf = Buffer::new();
    buf.write_all("hello world".as_bytes()).unwrap();
    println!("{}", buf.string());

    let mut buf = Buffer::new();
    buf.write_byte(33).unwrap();
    buf.write_all("hello world".as_bytes()).unwrap();
    println!("{}", buf.read_byte().unwrap());
    println!("{}", buf.string());
}
