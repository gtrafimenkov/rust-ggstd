// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compress::flate;
use crate::hash::{self, adler32, Hash32};
use crate::io as ggio;
use std::io::{Read, Write};

const ZLIB_DEFLATE: u8 = 8;
const ZLIB_MAX_WINDOW: u8 = 7;

#[derive(Debug)]
pub enum Error {
    /// ErrChecksum is returned when reading ZLIB data that has an invalid checksum.
    ErrChecksum,
    /// ErrDictionary is returned when reading ZLIB data that has an invalid dictionary.
    ErrDictionary,
    /// ErrHeader is returned when reading ZLIB data that has an invalid header.
    ErrHeader,
    GGIo(ggio::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::ErrChecksum => match other {
                Error::ErrChecksum => true,
                _ => false,
            },
            Error::ErrDictionary => match other {
                Error::ErrDictionary => true,
                _ => false,
            },
            Error::ErrHeader => match other {
                Error::ErrHeader => true,
                _ => false,
            },
            Error::GGIo(e1) => match other {
                Error::GGIo(e2) => e1 == e2,
                _ => false,
            },
        }
    }
}

impl Error {
    fn to_stdio_error(e: Self) -> std::io::Error {
        match e {
            Error::GGIo(e) => match e {
                ggio::Error::StdIo(e) => return std::io::Error::new(e.kind(), e.to_string()),
                _ => return std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
            },
            _ => return std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ErrHeader => write!(f, "zlib: invalid header"),
            Error::ErrDictionary => write!(f, "zlib: invalid dictionary"),
            Error::ErrChecksum => write!(f, "zlib: invalid checksum"),
            Error::GGIo(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::GGIo(ggio::Error::from(error))
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(kind: std::io::ErrorKind) -> Self {
        Error::GGIo(ggio::Error::from(std::io::Error::from(kind)))
    }
}

impl From<ggio::Error> for Error {
    fn from(error: ggio::Error) -> Self {
        Error::GGIo(error)
    }
}

pub struct Reader<'a> {
    /// Input source.
    r: std::io::BufReader<&'a mut dyn std::io::Read>,
    decompressor: flate::DecompressorFilter,
    digest: adler32::Digest,
}

/// new_reader creates a new Reader.
/// Reads from the returned Reader read and decompress data from r.
/// If r does not implement io.ByteReader, the decompressor may read more
/// data than necessary from r.
/// It is the caller's responsibility to call close on the Reader when done.
//
// ggstd not implemented:
// The Reader returned by new_reader also implements Resetter.
pub fn new_reader(r: &mut dyn std::io::Read) -> Result<Reader, Error> {
    new_reader_dict(r, &[])
}

/// new_reader_dict is like new_reader but uses a preset dictionary.
/// new_reader_dict ignores the dictionary if the compressed data does not refer to it.
/// If the compressed data refers to a different dictionary, new_reader_dict returns ErrDictionary.
//
// ggstd not implemented:
// The Reader returned by new_reader_dict also implements Resetter.
pub fn new_reader_dict<'a>(
    r: &'a mut dyn std::io::Read,
    dict: &'a [u8],
) -> Result<Reader<'a>, Error> {
    Reader::new_dict(r, dict)
}

impl<'a> Reader<'a> {
    pub fn new(r: &'a mut dyn std::io::Read) -> Result<Self, Error> {
        Self::new_dict(r, &[])
    }

    pub fn new_dict(r: &'a mut dyn std::io::Read, dict: &'a [u8]) -> Result<Self, Error> {
        let (decompressor, digest) = init_stream(r, dict)?;
        Ok(Self {
            r: std::io::BufReader::new(r),
            decompressor,
            digest,
        })
    }

    pub fn read(&mut self, p: &mut [u8]) -> Result<usize, Error> {
        let (n, err) = self.decompressor.read(&mut self.r, p);
        self.digest.write(&p[0..n])?;

        if n > 0 {
            return Ok(n);
        }

        if !err.as_ref().is_some_and(|e| e.is_eof()) {
            // an error happened and it is not EOF
            return Err(Error::GGIo(err.unwrap()));
        }

        // n == 0 or an error happened and it is EOF

        // Finished file; check checksum.
        let mut scratch = [0; 4];

        self.r.read_exact(&mut scratch)?;

        // ZLIB (RFC 1950) is big-endian, unlike GZIP (RFC 1952).
        let checksum = (scratch[0] as u32) << 24
            | (scratch[1] as u32) << 16
            | (scratch[2] as u32) << 8
            | (scratch[3] as u32);
        if checksum != self.digest.sum32() {
            return Err(Error::ErrChecksum);
        }
        return Ok(n);
    }

    /// Calling close does not close the wrapped io.Reader originally passed to new_reader.
    /// In order for the ZLIB checksum to be verified, the reader must be
    /// fully consumed until the io.EOF.
    pub fn close(&mut self) -> Result<(), flate::Error> {
        self.decompressor.close()
    }

    /// reset discards any buffered data and resets the Reader as if it was
    /// newly initialized with the given reader.
    /// This permits reusing a Reader instead of allocating a new one.
    pub fn reset(&mut self, r: &'a mut dyn std::io::Read, dict: &'a [u8]) -> Result<(), Error> {
        let (decompressor, digest) = init_stream(r, dict)?;
        self.decompressor = decompressor;
        self.digest = digest;
        Ok(())
    }
}

impl std::io::Read for Reader<'_> {
    /// When using this fuction, error reporting will not be precise
    /// because any error will be converted to std::io::Error.
    /// If you want precise error reporting, use Reader::Read method.
    //
    // ggrust TODO: ??? save the exact error to make it possible to get it with a separate
    // function call (get_last_read_error)?
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = Reader::read(self, buf);
        match res {
            Ok(n) => Ok(n),
            Err(err) => Err(Error::to_stdio_error(err)),
        }
    }
}

fn init_stream<'a>(
    r: &'a mut dyn std::io::Read,
    dict: &'a [u8],
) -> Result<(flate::DecompressorFilter, adler32::Digest), Error> {
    let mut scratch = [0; 4];
    r.read_exact(&mut scratch[0..2])?;
    let h = ((scratch[0] as usize) << 8) | (scratch[1] as usize);
    if (scratch[0] & 0x0f != ZLIB_DEFLATE) || (scratch[0] >> 4 > ZLIB_MAX_WINDOW) || (h % 31 != 0) {
        return Err(Error::ErrHeader);
    }
    let have_dict = scratch[1] & 0x20 != 0;
    if have_dict {
        r.read_exact(&mut scratch[0..4])?;
        let checksum = (scratch[0] as u32) << 24
            | (scratch[1] as u32) << 16
            | (scratch[2] as u32) << 8
            | (scratch[3] as u32);
        if checksum != hash::adler32::checksum(dict) {
            return Err(Error::ErrDictionary);
        }
    }

    // 	if self.decompressor == nil {
    let decompressor = if have_dict {
        flate::DecompressorFilter::new_dict(dict)
    } else {
        flate::DecompressorFilter::new()
    };
    let digest = hash::adler32::new();
    Ok((decompressor, digest))
}
