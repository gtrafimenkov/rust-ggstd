// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compress::flate;
use crate::encoding::binary::{ByteOrder, LITTLE_ENDIAN};
use crate::errors;
use crate::hash::crc32;
use crate::io as ggio;
use crate::time;

// import (
// 	"bufio"
// 	"compress/flate"
// 	"encoding/binary"
// 	"errors"
// 	"hash/crc32"
// 	"io"
// 	"time"
// )

pub(super) const GZIP_ID1: u8 = 0x1f;
const GZIP_ID2: u8 = 0x8b;
const GZIP_DEFLATE: u8 = 8;
// const FLAG_TEXT: u8 = 1 << 0;
const FLAG_HDR_CRC: u8 = 1 << 1;
const FLAG_EXTRA: u8 = 1 << 2;
const FLAG_NAME: u8 = 1 << 3;
const FLAG_COMMENT: u8 = 1 << 4;

// /// Error describes possible errors.
// #[derive(Debug)]
// pub enum Error {
//     /// ErrChecksum is returned when reading GZIP data that has an invalid checksum.
//     ErrChecksum,
//     // ErrHeader is returned when reading GZIP data that has an invalid header.
//     ErrHeader,
//     Io(std::io::Error),
// }

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::ErrChecksum => write!(f, "{}", ERR_CHECKSUM_MSG),
//             Error::ErrHeader => write!(f, "gzip: invalid header"),
//             Error::Io(e) => write!(f, "{}", e),
//         }
//     }
// }

// impl From<std::io::Error> for Error {
//     fn from(error: std::io::Error) -> Self {
//         Error::Io(error)
//     }
// }

/// ERR_CHECKSUM_MSG is an error message that will be return as std::io::Error
/// when the checksum is wrong.
pub const ERR_CHECKSUM_MSG: &str = "gzip: invalid checksum";
/// ERR_CHECKSUM_MSG is an error message that will be return as std::io::Error
/// when the header is wrong.
pub const ERR_INVALID_HEADER: &str = "gzip: invalid header";

fn get_err_checksum() -> std::io::Error {
    errors::new_stdio_other_error(ERR_CHECKSUM_MSG.to_string())
}

fn get_err_invalid_header() -> std::io::Error {
    errors::new_stdio_other_error(ERR_INVALID_HEADER.to_string())
}
// var le = binary.LittleEndian

// // noEOF converts io.EOF to io.ErrUnexpectedEOF.
// fn noEOF(err error) error {
// 	if err == io.EOF {
// 		return io.ErrUnexpectedEOF
// 	}
// 	return err
// }

/// The gzip file stores a header giving metadata about the compressed file.
/// That header is exposed as the fields of the Writer and Reader structs.
///
/// Strings must be UTF-8 encoded and may only contain Unicode code points
/// U+0001 through U+00FF, due to limitations of the GZIP file format.
#[derive(Debug)]
pub struct Header {
    pub comment: Option<String>, // comment
    pub extra: Option<Vec<u8>>,  // "extra data"
    pub mod_time: time::Time,    // modification time
    pub name: Option<String>,    // file name
    pub os: u8,                  // operating system type
}

struct ReadState {
    digest: u32, // CRC-32, IEEE polynomial (section 8)
    buf: Vec<u8>,
}

/// A Reader implements io::Reader and (TODO std::io::Read) that can be used to retrieve
/// uncompressed data from a gzip-format compressed file.
///
/// In general, a gzip file can be a concatenation of gzip files,
/// each with its own header. Reads from the Reader
/// return the concatenation of the uncompressed data of each.
/// Only the first header is recorded in the Reader fields.
///
/// Gzip files store a length and checksum of the uncompressed data.
/// The Reader will return an ErrChecksum when Read
/// reaches the end of the uncompressed data if it does not
/// have the expected length or checksum. Clients should treat data
/// returned by Read as tentative until they receive the io.EOF
/// marking the end of the data.
pub struct Reader<'a> {
    pub header: Option<Header>, // valid after Reader::new or Reader.reset
    read_state: ReadState,
    decompressor: flate::Reader<'a>,
    size: u32, // Uncompressed size (section 2.3.1)
    err: Option<std::io::Error>,
    multistream: bool,
}

impl<'a> Reader<'a> {
    /// new creates a new Reader reading the given reader.
    ///
    /// Make sure that the reader implements buffering otherwise the performance
    /// can be low.  You can use std::io::BufReader to add buffering to any reader.
    ///
    ///
    // If r does not also implement io.ByteReader,
    // the decompressor may read more data than necessary from r.
    //
    // It is the caller's responsibility to call close on the Reader when done.
    //
    /// The Reader.header fields will be valid in the Reader returned.
    /// If Reader.header is None, then there is no stream available.
    pub fn new(r: &'a mut dyn std::io::Read) -> std::io::Result<Self> {
        // 	z := new(Reader)
        // 	if err := self.reset(r); err != nil {
        // 		return nil, err
        // 	}
        // 	return z, nil

        let mut read_state = ReadState::new();
        let header = read_state.read_header(r)?;
        let decompressor = flate::Reader::new(r);

        Ok(Self {
            header,
            decompressor,
            read_state,
            size: 0,
            err: None,
            multistream: true,
        })
    }

    /// reset discards the Reader self's state and makes it equivalent to the
    /// result of its original state from Reader::new, but reading from r instead.
    /// This permits reusing a Reader rather than allocating a new one.
    pub fn reset(&mut self, r: &'a mut dyn std::io::Read) -> std::io::Result<()> {
        self.read_state = ReadState::new();
        self.header = self.read_state.read_header(r)?;
        self.decompressor.reset(r, &[]);
        self.multistream = true;
        self.size = 0;
        self.err = None;
        Ok(())
    }

    /// reset_state is similar to reset, but reuses the underlying reader.
    pub fn reset_state(&mut self) -> std::io::Result<()> {
        self.read_state = ReadState::new();
        self.header = self
            .read_state
            .read_header(self.decompressor.underlying_reader())?;
        self.decompressor.reset_state(&[]);
        self.multistream = true;
        self.size = 0;
        self.err = None;
        Ok(())
    }

    /// multistream controls whether the reader supports multistream files.
    ///
    /// If enabled (the default), the Reader expects the input to be a sequence
    /// of individually gzipped data streams, each with its own header and
    /// trailer, ending at EOF. The effect is that the concatenation of a sequence
    /// of gzipped files is treated as equivalent to the gzip of the concatenation
    /// of the sequence. This is standard behavior for gzip readers.
    ///
    /// Calling Multistream(false) disables this behavior; disabling the behavior
    /// can be useful when reading file formats that distinguish individual gzip
    /// data streams or mix gzip data streams with other data streams.
    /// In this mode, when the Reader reaches the end of the data stream,
    /// Read returns io.EOF. The underlying reader must implement io.ByteReader
    /// in order to be left positioned just after the gzip stream.
    /// To start the next stream, call self.reset(r) followed by self.Multistream(false).
    /// If there is no next stream, self.reset(r) will return io.EOF.
    pub fn multistream(&mut self, ok: bool) {
        self.multistream = ok;
    }

    /// close closes the Reader. It does not close the underlying io.Reader.
    /// In order for the GZIP checksum to be verified, the reader must be
    /// fully consumed until the io.EOF.
    pub fn close(&mut self) -> std::io::Result<()> {
        self.decompressor.close()
    }

    /// is_eof returns true if there are no more gzip steams available.
    pub fn is_eof(&self) -> bool {
        self.header.is_none()
    }
}

impl crate::io::Reader for Reader<'_> {
    /// read implements io.Reader, reading uncompressed bytes from its underlying Reader.
    fn read(&mut self, p: &mut [u8]) -> ggio::IoRes {
        let mut n = 0;
        if self.err.is_some() {
            return (0, errors::copy_stdio_option_error(&self.err));
        }

        while n == 0 {
            if self.is_eof() {
                return ggio::EOF;
            }
            let res = crate::io::Reader::read(&mut self.decompressor, p);
            self.read_state.digest =
                crc32::update(self.read_state.digest, &crc32::IEEE_TABLE, &p[..res.0]);
            self.size += res.0 as u32;
            if !ggio::is_eof(&res) {
                return res;
            }
            (n, self.err) = res;

            // Finished file; check checksum and size.
            {
                let mut buf = [0; 8];
                if let Err(err) = self.decompressor.underlying_reader().read_exact(&mut buf) {
                    self.err = Some(err);
                    return (n, errors::copy_stdio_option_error(&self.err));
                }
                let digest = LITTLE_ENDIAN.uint32(&buf[..4]);
                let size = LITTLE_ENDIAN.uint32(&buf[4..8]);
                if digest != self.read_state.digest || size != self.size {
                    self.err = Some(get_err_checksum());
                    return (n, errors::copy_stdio_option_error(&self.err));
                }
            }
            self.read_state.digest = 0;
            self.size = 0;

            // File is ok; check if there is another.
            if !self.multistream {
                return (n, None);
            }
            self.err = None; // Remove io.EOF

            self.read_state = ReadState::new();
            self.header = match self
                .read_state
                .read_header(self.decompressor.underlying_reader())
            {
                Ok(header) => header,
                Err(err) => {
                    return (n, Some(err));
                }
            };
            self.decompressor.reset_state(&[]);
        }

        (n, None)
    }
}

impl std::io::Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = crate::io::Reader::read(self, buf);
        if res.0 > 0 {
            Ok(res.0)
        } else if ggio::is_eof(&res) {
            return Ok(0);
        } else {
            return Err(res.1.unwrap());
        }
    }
}

impl ReadState {
    fn new() -> Self {
        Self {
            digest: 0,
            buf: vec![0; 512],
        }
    }

    /// read_header reads the GZIP header according to section 2.3.1.
    /// This method does not set self.err.
    fn read_header(&mut self, r: &mut dyn std::io::Read) -> Result<Option<Header>, std::io::Error> {
        // 	if _, err = io.ReadFull(r, self.buf[..10]); err != nil {
        // 		// RFC 1952, section 2.2, says the following:
        // 		//	A gzip file consists of a series of "members" (compressed data sets).
        // 		//
        // 		// Other than this, the specification does not clarify whether a
        // 		// "series" is defined as "one or more" or "zero or more". To err on the
        // 		// side of caution, Go interprets this to mean "zero or more".
        // 		// Thus, it is okay to return io.EOF here.
        // 		return hdr, err
        // 	}
        let res = ggio::read_full(r, &mut self.buf[..10]);
        if res.0 == 0 && ggio::is_unexpected_eof(&res) {
            // nothing was read, there is no header
            return Ok(None);
        }
        if let Some(err) = res.1 {
            return Err(err);
        }
        if self.buf[0] != GZIP_ID1 || self.buf[1] != GZIP_ID2 || self.buf[2] != GZIP_DEFLATE {
            return Err(get_err_invalid_header());
        }
        let flg = self.buf[3];
        let t = LITTLE_ENDIAN.uint32(&self.buf[4..8]) as i64;
        let mod_time = if t > 0 {
            // Section 2.3.1, the zero value for MTIME means that the
            // modified time is not set.
            time::unix(t, 0)
        } else {
            time::Time::default()
        };
        // self.buf[8] is XFL and is currently ignored.
        let os = self.buf[9];
        self.digest = crc32::checksum_ieee(&self.buf[..10]);

        let extra_data: Option<Vec<u8>> = if flg & FLAG_EXTRA != 0 {
            let mut buf = [0; 2];
            r.read_exact(&mut buf)?;
            self.digest = crc32::update(self.digest, &crc32::IEEE_TABLE, &buf[..2]);
            let mut data = vec![0; LITTLE_ENDIAN.uint16(&buf[..2]) as usize];
            r.read_exact(&mut data)?;
            self.digest = crc32::update(self.digest, &crc32::IEEE_TABLE, &data);
            Some(data)
        } else {
            None
        };

        let name = if flg & FLAG_NAME != 0 {
            Some(self.read_string(r)?)
        } else {
            None
        };

        let comment = if flg & FLAG_COMMENT != 0 {
            Some(self.read_string(r)?)
        } else {
            None
        };

        if flg & FLAG_HDR_CRC != 0 {
            r.read_exact(&mut self.buf[..2])?;
            let digest = LITTLE_ENDIAN.uint16(&self.buf[..2]);
            if digest != self.digest as u16 {
                return Err(get_err_invalid_header());
            }
        }

        self.digest = 0;
        // 	if self.decompressor == nil {
        // 		self.decompressor = flate.Reader::new(r)
        // 	} else {
        // 		self.decompressor.(flate.Resetter).reset(r, nil)
        // 	}
        // 	return hdr, nil

        Ok(Some(Header {
            comment,
            extra: extra_data,
            mod_time,
            name,
            os,
        }))
    }

    /// read_string reads a NUL-terminated string from self.r.
    /// It treats the bytes read as being encoded as ISO 8859-1 (Latin-1) and
    /// will output a string encoded using UTF-8.
    /// This method always updates self.digest with the data read.
    fn read_string(&mut self, r: &mut dyn std::io::Read) -> std::io::Result<String> {
        let mut need_conv = false;
        let mut i = 0;
        loop {
            if i >= self.buf.len() {
                return Err(get_err_invalid_header());
            }
            r.read_exact(&mut self.buf[i..i + 1])?;
            if self.buf[i] > 0x7f {
                need_conv = true;
            }
            if self.buf[i] == 0 {
                // Digest covers the NUL terminator.
                self.digest = crc32::update(self.digest, &crc32::IEEE_TABLE, &self.buf[..i + 1]);

                // Strings are ISO 8859-1, Latin-1 (RFC 1952, section 2.3.1).
                if need_conv {
                    let mut s = String::new();
                    for ch in &self.buf[..i] {
                        s.push(*ch as char);
                    }
                    return Ok(s);
                }
                return Ok(String::from_utf8_lossy(&self.buf[..i]).to_string());
            }
            i += 1;
        }
    }
}
