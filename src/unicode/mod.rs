//! Package unicode provides data and functions to test some properties of
//! Unicode code points.

mod graphic;
mod letter;
mod rune;
mod tables;
pub mod utf8;

pub use letter::{MAX_ASCII, MAX_LATIN1, MAX_RUNE, REPLACEMENT_CHAR};
pub use rune::Rune;
