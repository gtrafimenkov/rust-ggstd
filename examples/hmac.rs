// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

use ggstd::crypto::hmac;
use ggstd::crypto::sha256;
use ggstd::encoding::hex;
use ggstd::hash::Hash;
use std::io::Write;

fn main() {
    let key = b"this is a key";
    let message = b"hello there";
    let mut m = hmac::HMAC::new(sha256::Digest::new, key);
    m.write_all(message).unwrap();
    let mac = m.sum(&[]);
    println!("{}", hex::encode_to_string(&mac));

    // Output:
    // 3f2b8ac8dc51f89e769930d8880312b639c1aa26c18a4c8bd3b3496a5de67540

    assert!(valid_mac(
        message,
        &hex::decode_string("3f2b8ac8dc51f89e769930d8880312b639c1aa26c18a4c8bd3b3496a5de67540").0,
        key
    ));
}

fn valid_mac(message: &[u8], message_mac: &[u8], key: &[u8]) -> bool {
    let mut mac = hmac::HMAC::new(sha256::Digest::new, key);
    mac.write_all(message).unwrap();
    let expected_mac = mac.sum(&[]);
    hmac::equal(message_mac, &expected_mac)
}
