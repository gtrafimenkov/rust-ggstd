//! Package utf8 implements functions and constants to support text encoded in
//! UTF-8. It includes functions to translate between runes and UTF-8 byte sequences.
//! See <https://en.wikipedia.org/wiki/UTF-8>

mod utf8;

pub use utf8::{
    decode_rune, decode_rune_in_string, encode_rune, full_rune, rune_count, rune_len, valid_rune,
    MAX_RUNE, RUNE_ERROR, RUNE_SELF, UTFMAX,
};

#[cfg(test)]
mod utf8_test;
