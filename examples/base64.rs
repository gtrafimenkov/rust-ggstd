// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::encoding::base64;
use std::io::Write;

fn main() {
    example();
    example_encoding_encode_to_string();
    example_encoding_encode();
    example_encoding_decode_string();
    example_encoding_decode();
    example_new_encoder();
}

fn example() {
    let msg = "Hello, 世界";
    let encoded = base64::get_std_encoding().encode_to_string(msg.as_bytes());
    println!("{}", encoded);
    let (decoded, err) = base64::get_std_encoding().decode_string(&encoded);
    if err.is_some() {
        println!("decode error: {:?}", err);
        return;
    }
    println!("{}", String::from_utf8_lossy(&decoded));
    // Output:
    // SGVsbG8sIOS4lueVjA==
    // Hello, 世界
}

fn example_encoding_encode_to_string() {
    let data = "any + old & data".as_bytes();
    let str = base64::get_std_encoding().encode_to_string(data);
    println!("{}", str)
    // Output:
    // YW55ICsgb2xkICYgZGF0YQ==
}

fn example_encoding_encode() {
    let data = "Hello, world!".as_bytes();
    let mut dst = vec![0; base64::get_std_encoding().encoded_len(data.len())];
    base64::get_std_encoding().encode(&mut dst, data);
    println!("{}", String::from_utf8_lossy(&dst));
    // Output:
    // SGVsbG8sIHdvcmxkIQ==
}

fn example_encoding_decode_string() {
    let str = "c29tZSBkYXRhIHdpdGggACBhbmQg77u/";
    let (data, err) = base64::get_std_encoding().decode_string(str);
    if err.is_some() {
        println!("error: {:?}", err);
        return;
    }
    println!("{}", ggstd::strconv::quote_bytes_string(&data));
    // Output:
    // "some data with \x00 and \ufeff"
}

fn example_encoding_decode() {
    let str = "SGVsbG8sIHdvcmxkIQ==";
    let mut dst = vec![0; base64::get_std_encoding().decoded_len(str.len())];
    let (n, err) = base64::get_std_encoding().decode(&mut dst, str.as_bytes());
    if err.is_some() {
        println!("decode error: {:?}", err);
        return;
    }
    let dst = &dst[..n];
    println!("\"{}\"", String::from_utf8(dst.to_vec()).unwrap());
    // Output:
    // "Hello, world!"
}

fn example_new_encoder() {
    let input = b"foo\x00bar";
    let stdout = std::io::stdout();
    let mut stdout_writer = stdout.lock();
    let mut encoder = base64::Encoder::new(base64::get_std_encoding(), &mut stdout_writer);
    encoder.write(input).unwrap();
    // Must close the encoder when finished to flush any partial blocks.
    // If you comment out the following line, the last partial block "r"
    // won't be encoded.
    encoder.close().unwrap();
    // Output:
    // Zm9vAGJhcg==
}
