//! Package crc32 implements the 32-bit cyclic redundancy check, or CRC-32,
//! checksum. See <https://en.wikipedia.org/wiki/Cyclic_redundancy_check> for
//! information.
//!
//! Polynomials are represented in LSB-first form also known as reversed representation.
//!
//! See <https://en.wikipedia.org/wiki/Mathematics_of_cyclic_redundancy_checks#Reversed_representations_and_reciprocal_polynomials>
//! for information.

mod crc32;
mod crc32_generic;

pub use crc32::{
    checksum, checksum_ieee, new, new_ieee, update, Digest, PredefinedPolynomials, Table,
    IEEE_TABLE, SIZE,
};

#[cfg(test)]
mod crc32_test;
