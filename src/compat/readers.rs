//! readers module provides:
//! - crate::io::Reader and crate::io::ByteReader traits implementations for std::io::BufReader

use crate::io as ggio;
use std::io::BufRead;

impl<R: std::io::Read> ggio::Reader for std::io::BufReader<R> {
    fn read(&mut self, p: &mut [u8]) -> (usize, Option<ggio::Error>) {
        let res = std::io::Read::read(self, p);
        if res.is_err() {
            return (0, Some(ggio::Error::StdIo(res.err().unwrap())));
        }
        let bytes_read = res.unwrap();
        if bytes_read == 0 && p.len() > 0 {
            return (0, Some(ggio::Error::EOF));
        }
        return (bytes_read, None);
    }
}

impl<R: std::io::Read> ggio::ByteReader for std::io::BufReader<R> {
    fn read_byte(&mut self) -> ggio::Result<u8> {
        let buf = match self.fill_buf() {
            Ok(buf) => buf,
            Err(err) => return Err(ggio::Error::StdIo(err)),
        };
        if buf.len() == 0 {
            return Err(ggio::Error::EOF);
        }
        let b = buf[0];
        self.consume(1);
        Ok(b)
    }
}

// /// GGReaderAdapter is an adapter from crate::io::Reader trait to std::io::Read.
// pub struct GGReaderAdapter<'a> {
//     r: &'a mut dyn ggio::Reader,
//     err: Option<ggio::Error>,
// }

// impl<'a> GGReaderAdapter<'a> {
//     pub fn new(r: &'a mut dyn ggio::Reader) -> Self {
//         Self { r, err: None }
//     }
// }

// impl std::io::Read for GGReaderAdapter<'_> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         if self.err.is_none() {
//             let (n, err) = self.r.read(buf);
//             self.err = err;
//             if n > 0 {
//                 // always return data first
//                 return Ok(n);
//             }
//             if n == 0 && self.err.is_none() {
//                 return Ok(n);
//             }
//         }

//         // we have an error

//         let err = self.err.as_ref().unwrap();
//         match err {
//             ggio::Error::EOF => return Ok(0),
//             _ => {
//                 return Err(std::io::Error::new(
//                     std::io::ErrorKind::Other,
//                     err.to_string(),
//                 ))
//             }
//         }
//     }
// }
