//! Package zlib implements reading and writing of zlib format compressed data,
//! as specified in RFC 1950.
//! The implementation provides filters that uncompress during reading
//! and compress during writing.  For example, to write compressed data
//! to a buffer:
//!
//!     use ggstd::bytes;
//!     use ggstd::compress::zlib;
//!
//!     let mut b = bytes::Buffer::new();
//!     let mut w = zlib::new_writer(&mut b);
//!     w.write("hello, world\n".as_bytes()).unwrap();
//!     w.close().unwrap();
//!     println!("{:?}", b.bytes());
//!
//! and to read that data back:
//!
//!     use std::io::Read;
//!
//!     use ggstd::bytes;
//!     use ggstd::compress::zlib;
//!
//!     let buff: &[u8] = &[
//!         120, 156, 202, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 225, 2, 4, 0, 0, 255, 255,
//!         33, 231, 4, 147,
//!     ];
//!
//!     let mut b = bytes::new_reader(buff);
//!     let mut r = zlib::Reader::new(&mut b).unwrap();
//!     let mut output = String::new();
//!     r.read_to_string(&mut output).unwrap();
//!     println!("{}", output);

mod reader;
mod writer;

pub use reader::{new_reader, new_reader_dict, Error, Reader};
pub use writer::{new_writer, new_writer_level, new_writer_level_dict};

#[cfg(test)]
mod reader_test;
#[cfg(test)]
mod writer_test;
