// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// A Block represents an implementation of block cipher
/// using a given key. It provides the capability to encrypt
/// or decrypt individual blocks. The mode implementations
/// extend that capability to streams of blocks.
pub trait Block {
    /// block_size returns the cipher's block size.
    fn block_size(&self) -> usize;

    /// encrypt encrypts the first block in src into dst.
    fn encrypt(&self, dst: &mut [u8], src: &[u8]);

    /// decrypt decrypts the first block in src into dst.
    fn decrypt(&self, dst: &mut [u8], src: &[u8]);
}

/// A Stream represents a stream cipher.
pub trait Stream {
    /// xor_key_stream XORs each byte in the given slice with a byte from the
    /// cipher's key stream.
    // Dst and src must overlap entirely or not at all.
    ///
    /// If dst.len() < src.len(), xor_key_stream should panic. It is acceptable
    /// to pass a dst bigger than src, and in that case, xor_key_stream will
    /// only update dst[..src.len()] and will not touch the rest of dst.
    ///
    /// Multiple calls to xor_key_stream behave as if the concatenation of
    /// the src buffers was passed in a single run. That is, Stream
    /// maintains state and does not reset at each xor_key_stream call.
    fn xor_key_stream(&mut self, dst: &mut [u8], src: &[u8]);
}

/// A BlockMode represents a block cipher running in a block-based mode (CBC,
/// ECB etc).
pub trait BlockMode {
    /// block_size returns the mode's block size.
    fn block_size(&self) -> usize;

    /// crypt_blocks encrypts or decrypts a number of blocks. The length of
    /// src must be a multiple of the block size.
    // Dst and src must overlap
    // entirely or not at all.
    //
    /// If dst.len() < src.len(), crypt_blocks should panic. It is acceptable
    /// to pass a dst bigger than src, and in that case, crypt_blocks will
    /// only update dst[..src.len()] and will not touch the rest of dst.
    ///
    /// Multiple calls to crypt_blocks behave as if the concatenation of
    /// the src buffers was passed in a single run. That is, BlockMode
    /// maintains state and does not reset at each crypt_blocks call.
    fn crypt_blocks(&mut self, dst: &mut [u8], src: &[u8]);

    /// crypt_blocks_inplace is similar to crypt_blocks, but encrypts or decrypts
    /// a number of blocks in the same buffer.
    fn crypt_blocks_inplace(&mut self, data: &mut [u8]);
}
