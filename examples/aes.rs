// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::crypto::aes;
use ggstd::crypto::cipher::{self, Block, BlockMode, Stream};
use ggstd::encoding::hex;

fn main() {
    let key: &[u8] = b"0123456789abcdef";
    let plaintext: &[u8] = b"hello world.....hello world.....hello world.....";

    println!("plaintext:      {}", String::from_utf8_lossy(plaintext));
    aes_ecb(key, plaintext);
    aes_cbc(key, plaintext);

    // in CTR mode plaintext doesn't have to be a multiple of the block size
    let plaintext: &[u8] = b"hello world.....hello world.....hello worl";
    println!("plaintext:      {}", String::from_utf8_lossy(plaintext));
    aes_ctr(key, plaintext);
}

/// Shows the simplest mode of operation of a block ciper - Electronic Codebook (ECB).
/// In ECB mode, each block of plaintext is independently encrypted into a block of ciphertext,
/// without any interaction between the blocks.
fn aes_ecb(key: &[u8], plaintext: &[u8]) {
    let block = aes::Cipher::new(key).unwrap();
    assert!(
        plaintext.len() % aes::BLOCK_SIZE == 0,
        "Plaintext length must be a multiple of the block size"
    );

    let mut encrypted = vec![0; plaintext.len()];

    for offset in (0..plaintext.len()).step_by(aes::BLOCK_SIZE) {
        block.encrypt(
            &mut encrypted[offset..offset + aes::BLOCK_SIZE],
            &plaintext[offset..offset + aes::BLOCK_SIZE],
        );
    }

    // let mut decrypted = vec![0; plaintext.len()];
    // block.decrypt(&mut decrypted, &encrypted);
    // block.decrypt(
    //     &mut decrypted[aes::BLOCK_SIZE..],
    //     &encrypted[aes::BLOCK_SIZE..],
    // );

    // println!("plaintext:      {}", String::from_utf8_lossy(plaintext));
    println!("ECB encrypted:  {}", hex::encode_to_string(&encrypted));
    // println!("decrypted:      {}", String::from_utf8_lossy(&decrypted));
    // println!()
}

/// In CBC (Cipher Block Chaining) mode, the plaintext is divided into fixed-size blocks,
/// and each block is XORed with the ciphertext of the previous block before being encrypted.
/// The first block is XORed with an Initialization Vector (IV) to start the chain.
fn aes_cbc(key: &[u8], plaintext: &[u8]) {
    let block = aes::Cipher::new(key).unwrap();
    assert!(
        plaintext.len() % aes::BLOCK_SIZE == 0,
        "Plaintext length must be a multiple of the block size"
    );

    // Using all the initialization vector of all zeroes for demo purposes.
    // In practice the IV should be unique for each encryption operation and upredictable,
    // for example, generated using a secure random generator.
    let iv = vec![0; aes::BLOCK_SIZE];
    let mut mode = cipher::CBCEncrypter::new(&block, &iv);

    let mut encrypted = vec![0; plaintext.len()];
    mode.crypt_blocks(&mut encrypted, plaintext);

    // let mut mode = cipher::CBCDecrypter::new(&block, &iv);
    // let mut decrypted = vec![0; plaintext.len()];
    // mode.crypt_blocks(&mut decrypted, &encrypted);

    // println!("plaintext:      {}", String::from_utf8_lossy(plaintext));
    println!("CBC encrypted:  {}", hex::encode_to_string(&encrypted));
    // println!("decrypted:      {}", String::from_utf8_lossy(&decrypted));
    // println!();
}

/// CTR (Counter mode), is a mode of operation that turns a block cipher into a stream cipher.
fn aes_ctr(key: &[u8], plaintext: &[u8]) {
    let block = aes::Cipher::new(key).unwrap();

    // Using all the initialization vector of all zeroes for demo purposes.
    // In practice the IV should be unique for each encryption operation and upredictable,
    // for example, generated using a secure random generator.
    let iv = vec![0; aes::BLOCK_SIZE];

    let mut stream = cipher::CTR::new(&block, &iv);

    let mut encrypted = vec![0; plaintext.len()];
    stream.xor_key_stream(&mut encrypted, &plaintext);

    // // decrypting the ciphertext back to plaintext using the same key, iv, and block
    // let mut stream = cipher::CTR::new(&block, &iv);
    // let mut decrypted = vec![0; plaintext.len()];
    // stream.xor_key_stream(&mut decrypted, &encrypted);

    // println!("plaintext:      {}", String::from_utf8_lossy(plaintext));
    println!("CTR encrypted:  {}", hex::encode_to_string(&encrypted));
    // println!("decrypted:      {}", String::from_utf8_lossy(&decrypted));
    // println!();
}
