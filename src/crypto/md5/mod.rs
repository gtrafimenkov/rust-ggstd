//! Package md5 implements the MD5 hash algorithm as defined in RFC 1321.
//!
//! MD5 is cryptographically broken and should not be used for secure
//! applications.

mod md5;
mod md5block;

pub use md5::{new, sum, BLOCK_SIZE, SIZE};

#[cfg(test)]
mod md5_test;
