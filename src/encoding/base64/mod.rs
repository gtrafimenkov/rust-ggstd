//! Package base64 implements base64 encoding as specified by RFC 4648.

mod base64;

pub use base64::{
    get_raw_std_encoding, get_raw_std_encoding_strict, get_raw_url_encoding,
    get_raw_url_encoding_strict, get_std_encoding, get_std_encoding_strict, get_url_encoding,
    get_url_encoding_strict, Decoder, Encoder, Encoding, Error,
};

#[cfg(test)]
mod base64_test;
