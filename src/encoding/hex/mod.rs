//! Package hex implements hexadecimal encoding and decoding.

mod hex;

pub use hex::{decode, decode_string, decoded_len, encode, encode_to_string, encoded_len};

#[cfg(test)]
mod hex_test;
