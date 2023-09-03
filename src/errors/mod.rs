mod errors;

pub use errors::{
    copy_stdio_error, new_static, new_stdio_other_error, new_str, new_unexpected_eof,
    ErrorStaticString, ErrorString,
};
