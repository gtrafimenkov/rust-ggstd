//! Package sha256 implements the SHA224 and SHA256 hash algorithms as defined
//! in FIPS 180-4.

mod sha256;
mod sha256block;

pub use sha256::{sum256, Digest};
