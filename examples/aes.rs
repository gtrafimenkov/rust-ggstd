// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::crypto::aes;
use ggstd::crypto::cipher::{self, Block, BlockMode};
use ggstd::encoding::hex;

fn main() {
    aes_basic();
    aes_cbc();
}

const KEY: &[u8] = b"0123456789abcdef";
const PLAINTEXT: &[u8] = b"hello world.....hello world.....";

fn aes_basic() {
    let block = aes::Cipher::new(KEY).unwrap();
    assert!(
        PLAINTEXT.len() % aes::BLOCK_SIZE == 0,
        "Plaintext length must be a multiple of the block size"
    );

    let mut ciphertext = [0; PLAINTEXT.len()];

    block.encrypt(&mut ciphertext, PLAINTEXT);
    block.encrypt(
        &mut ciphertext[aes::BLOCK_SIZE..],
        &PLAINTEXT[aes::BLOCK_SIZE..],
    );

    let mut decrypted = [0; PLAINTEXT.len()];
    block.decrypt(&mut decrypted, &ciphertext);
    block.decrypt(
        &mut decrypted[aes::BLOCK_SIZE..],
        &ciphertext[aes::BLOCK_SIZE..],
    );

    println!("plaintext:      {}", String::from_utf8_lossy(PLAINTEXT));
    println!("aes basic:      {}", hex::encode_to_string(&ciphertext));
    println!("decrypted:      {}", String::from_utf8_lossy(&decrypted));
}

fn aes_cbc() {
    let block = aes::Cipher::new(KEY).unwrap();
    assert!(
        PLAINTEXT.len() % aes::BLOCK_SIZE == 0,
        "Plaintext length must be a multiple of the block size"
    );

    let mut ciphertext = [0; PLAINTEXT.len()];

    let iv = vec![0; aes::BLOCK_SIZE];

    let mut mode = cipher::CBCEncrypter::new(&block, &iv);
    mode.crypt_blocks(&mut ciphertext, PLAINTEXT);

    let mut decrypted = [0; PLAINTEXT.len()];
    let mut mode = cipher::CBCDecrypter::new(&block, &iv);
    mode.crypt_blocks(&mut decrypted, &ciphertext);

    println!();
    println!("plaintext:      {}", String::from_utf8_lossy(PLAINTEXT));
    println!("aes cbc (iv=0): {}", hex::encode_to_string(&ciphertext));
    println!("decrypted:      {}", String::from_utf8_lossy(&decrypted));
}
