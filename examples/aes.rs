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

    let mut ciphertext1 = [0; PLAINTEXT.len()];
    block.encrypt(&mut ciphertext1, PLAINTEXT);
    block.encrypt(
        &mut ciphertext1[aes::BLOCK_SIZE..],
        &PLAINTEXT[aes::BLOCK_SIZE..],
    );

    let mut ciphertext2 = [0; PLAINTEXT.len()];
    block.encrypt(&mut ciphertext2, PLAINTEXT);
    block.encrypt(
        &mut ciphertext2[aes::BLOCK_SIZE..],
        &PLAINTEXT[aes::BLOCK_SIZE..],
    );

    let mut decrypted1 = [0; PLAINTEXT.len()];
    block.decrypt(&mut decrypted1, &ciphertext1);
    block.decrypt(
        &mut decrypted1[aes::BLOCK_SIZE..],
        &ciphertext1[aes::BLOCK_SIZE..],
    );

    let mut decrypted2 = [0; PLAINTEXT.len()];
    block.decrypt(&mut decrypted2, &ciphertext2);
    block.decrypt(
        &mut decrypted2[aes::BLOCK_SIZE..],
        &ciphertext2[aes::BLOCK_SIZE..],
    );

    println!("plaintext:        {}", String::from_utf8_lossy(PLAINTEXT));
    println!("aes basic 1:      {}", hex::encode_to_string(&ciphertext1));
    println!("aes basic 2:      {}", hex::encode_to_string(&ciphertext2));
    println!("decrypted 1:      {}", String::from_utf8_lossy(&decrypted1));
    println!("decrypted 2:      {}", String::from_utf8_lossy(&decrypted2));
}

fn aes_cbc() {
    let block = aes::Cipher::new(KEY).unwrap();
    assert!(
        PLAINTEXT.len() % aes::BLOCK_SIZE == 0,
        "Plaintext length must be a multiple of the block size"
    );

    let iv = vec![0; aes::BLOCK_SIZE];
    let mut mode = cipher::CBCEncrypter::new(&block, &iv);

    let mut ciphertext1 = [0; PLAINTEXT.len()];
    mode.crypt_blocks(&mut ciphertext1, PLAINTEXT);

    let mut ciphertext2 = [0; PLAINTEXT.len()];
    mode.crypt_blocks(&mut ciphertext2, PLAINTEXT);

    let mut mode = cipher::CBCDecrypter::new(&block, &iv);

    let mut decrypted1 = [0; PLAINTEXT.len()];
    mode.crypt_blocks(&mut decrypted1, &ciphertext1);

    let mut decrypted2 = [0; PLAINTEXT.len()];
    mode.crypt_blocks(&mut decrypted2, &ciphertext2);

    println!();
    println!("plaintext:        {}", String::from_utf8_lossy(PLAINTEXT));
    println!("aes cbc (iv=0) 1: {}", hex::encode_to_string(&ciphertext1));
    println!("aes cbc (iv=0) 2: {}", hex::encode_to_string(&ciphertext2));
    println!("decrypted 1:      {}", String::from_utf8_lossy(&decrypted1));
    println!("decrypted 2:      {}", String::from_utf8_lossy(&decrypted2));
}
