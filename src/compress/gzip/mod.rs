//! Package gzip implements reading and writing of gzip format compressed files,
//! as specified in RFC 1952.

mod gunzip;

pub use gunzip::{Header, Reader, ERR_CHECKSUM_MSG, ERR_INVALID_HEADER};

#[cfg(test)]
mod gunzip_test;
