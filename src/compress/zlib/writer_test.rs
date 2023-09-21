// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::writer::{
    new_writer_level_dict, BEST_COMPRESSION, BEST_SPEED, DEFAULT_COMPRESSION, HUFFMAN_ONLY,
    NO_COMPRESSION,
};
use crate::bytes;
use crate::compress::zlib;
use crate::os as ggos;
use std::io::Read;

const FILE_NAMES: &[&str] = &[
    "src/compress/testdata/gettysburg.txt",
    "src/compress/testdata/e.txt",
    "src/compress/testdata/pi.txt",
];

const DATA: &[&str] = &["test a reasonable sized string that can be compressed"];

/// Tests that compressing and then decompressing the given file at the given compression level and dictionary
/// yields equivalent bytes to the original file.
fn test_file_level_dict(file_name: &str, level: isize, d: &[u8]) {
    let b0 = ggos::read_file(file_name).unwrap();
    test_level_dict(file_name, &b0, level, d);
}

fn test_level_dict(_name: &str, b0: &[u8], level: isize, dict: &[u8]) {
    let mut b = bytes::Buffer::new();

    // compress
    let mut zlibw = new_writer_level_dict(&mut b, level, dict).unwrap();
    zlibw.write(b0).unwrap();
    zlibw.close().unwrap();

    // decompress
    let mut r = zlib::Reader::new_dict(&mut b, dict).unwrap();
    let mut decompressed = Vec::new();
    r.read_to_end(&mut decompressed).unwrap();

    // compare
    assert_eq!(b0, decompressed);
}

fn test_file_level_dict_reset(file_name: &str, level: isize, dict: &[u8]) {
    let mut buf = bytes::Buffer::new();
    let mut zlibw = zlib::new_writer_level_dict(&mut buf, level, dict).unwrap();

    // Compress once.
    let data = ggos::read_file(file_name).unwrap();
    zlibw.write(&data).unwrap();
    zlibw.close().unwrap();

    // Reset and compress again.
    let data = ggos::read_file(file_name).unwrap();
    let mut buf2 = bytes::Buffer::new();
    zlibw.reset(&mut buf2);
    zlibw.write(&data).unwrap();
    zlibw.close().unwrap();

    assert_eq!(buf.len(), buf2.len());
    assert_eq!(bytes::Buffer::bytes(&buf), bytes::Buffer::bytes(&buf2));
}

#[test]
fn test_writer() {
    for (i, s) in DATA.iter().enumerate() {
        let b = s.as_bytes();
        let tag = format!("#{}", i);
        test_level_dict(&tag, b, DEFAULT_COMPRESSION, &[]);
        test_level_dict(&tag, b, NO_COMPRESSION, &[]);
        test_level_dict(&tag, b, HUFFMAN_ONLY, &[]);
        for level in BEST_SPEED..=BEST_COMPRESSION {
            test_level_dict(&tag, b, level, &[]);
        }
    }
}

#[test]
fn test_writer_big() {
    for (_i, file_name) in FILE_NAMES.iter().enumerate() {
        test_file_level_dict(file_name, DEFAULT_COMPRESSION, &[]);
        test_file_level_dict(file_name, NO_COMPRESSION, &[]);
        test_file_level_dict(file_name, HUFFMAN_ONLY, &[]);
        for level in BEST_SPEED..=BEST_COMPRESSION {
            test_file_level_dict(file_name, level, &[]);
        }
    }
}

#[test]
fn test_writer_dict() {
    let dictionary = b"0123456789.";
    for file_name in FILE_NAMES {
        test_file_level_dict(file_name, DEFAULT_COMPRESSION, dictionary);
        test_file_level_dict(file_name, NO_COMPRESSION, dictionary);
        test_file_level_dict(file_name, HUFFMAN_ONLY, dictionary);
        for level in BEST_SPEED..=BEST_COMPRESSION {
            test_file_level_dict(file_name, level, dictionary);
        }
    }
}

#[test]
fn test_writer_reset() {
    let dictionary = b"0123456789.";
    for file_name in FILE_NAMES {
        test_file_level_dict_reset(file_name, NO_COMPRESSION, &[]);
        test_file_level_dict_reset(file_name, DEFAULT_COMPRESSION, &[]);
        test_file_level_dict_reset(file_name, HUFFMAN_ONLY, &[]);
        test_file_level_dict_reset(file_name, NO_COMPRESSION, dictionary);
        test_file_level_dict_reset(file_name, DEFAULT_COMPRESSION, dictionary);
        test_file_level_dict_reset(file_name, HUFFMAN_ONLY, dictionary);
        for level in BEST_SPEED..=BEST_COMPRESSION {
            test_file_level_dict_reset(file_name, level, &[]);
        }
    }
}

#[test]
fn test_writer_dict_is_used() {
    let input = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let mut buf = bytes::Buffer::new();
    {
        let mut compressor = new_writer_level_dict(&mut buf, BEST_COMPRESSION, input).unwrap();
        compressor.write(input).unwrap();
        compressor.close().unwrap();
    }
    let expected_max_size = 25;
    let output = bytes::Buffer::bytes(&mut buf);
    assert!(
        output.len() <= expected_max_size,
        "result too large (got {}, want <= {} bytes). Is the dictionary being used?",
        output.len(),
        expected_max_size
    );
}
