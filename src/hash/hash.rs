// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// Hash is the common interface implemented by all hash functions.
///
// Hash implementations in the standard library (e.g. hash/crc32 and
// crypto/sha256) implement the encoding.BinaryMarshaler and
// encoding.BinaryUnmarshaler interfaces. Marshaling a hash implementation
// allows its internal state to be saved and used for additional processing
// later, without having to re-write the data previously written to the hash.
// The hash state may contain portions of the input in its original form,
// which users are expected to handle for any possible security implications.
///
/// Compatibility: Any future changes to hash or crypto packages will endeavor
/// to maintain compatibility with state encoded using previous versions.
/// That is, any released versions of the packages should be able to
/// decode data written with any previously released version,
/// subject to issues such as security fixes.
/// See the Go compatibility document for background: <https://golang.org/doc/go1compat>
pub trait Hash: std::io::Write {
    /// sum appends the current hash to b and returns the resulting slice.
    /// It does not change the underlying hash state.
    fn sum(&self, b: &[u8]) -> Vec<u8>;

    /// reset resets the Hash to its initial state.
    fn reset(&mut self);

    /// size returns the number of bytes Sum will return.
    fn size(&self) -> usize;

    /// block_size returns the hash's underlying block size.
    /// The Write method must be able to accept any amount
    /// of data, but it may operate more efficiently if all writes
    /// are a multiple of the block size.
    fn block_size(&self) -> usize;
}

/// Hash32 is the common interface implemented by all 32-bit hash functions.
pub trait Hash32: Hash {
    fn sum32(&self) -> u32;
}

/// Hash64 is the common interface implemented by all 64-bit hash functions.
pub trait Hash64: Hash {
    fn sum64(&self) -> u64;
}
