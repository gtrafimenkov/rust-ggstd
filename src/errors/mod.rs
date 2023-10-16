//! Package errors implements functions to manipulate errors.

mod errors;

pub use errors::{
    copy_stdio_error, copy_stdio_option_error, is_rust_eof, new_static, new_stdio_other_error,
    new_str, new_unexpected_eof, ErrorStaticString, ErrorString, RUST_EOF,
};
