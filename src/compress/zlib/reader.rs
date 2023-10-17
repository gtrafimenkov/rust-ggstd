// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compress::flate;
use crate::errors;
use crate::hash::{self, adler32, Hash32};
use std::io::Write;

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
    StdIo(std::io::Error),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::ErrChecksum => matches!(other, Error::ErrChecksum),
            Error::ErrDictionary => matches!(other, Error::ErrDictionary),
            Error::ErrHeader => matches!(other, Error::ErrHeader),
            Error::StdIo(e1) => match other {
                Error::StdIo(e2) => e1.kind() == e2.kind(),
                _ => false,
            },
        }
    }
}

impl Error {
    pub fn to_stdio_error(&self) -> std::io::Error {
        match self {
            Error::StdIo(e) => errors::copy_stdio_error(e),
            _ => std::io::Error::new(std::io::ErrorKind::Other, self.to_string()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ErrHeader => write!(f, "zlib: invalid header"),
            Error::ErrDictionary => write!(f, "zlib: invalid dictionary"),
            Error::ErrChecksum => write!(f, "zlib: invalid checksum"),
            Error::StdIo(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::StdIo(error)
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(kind: std::io::ErrorKind) -> Self {
        Error::StdIo(std::io::Error::from(kind))
    }
}

/// Reader reads and decompress data from the input reader.
// If r does not implement io.ByteReader, the decompressor may read more
// data than necessary from r.
// It is the caller's responsibility to call close on the Reader when done.
pub struct Reader<'a, Input: std::io::BufRead> {
    decompressor: flate::Reader<&'a mut Input>,
    digest: adler32::Digest,
}

impl<'a, Input: std::io::BufRead> Reader<'a, Input> {
    pub fn new(r: &'a mut Input) -> Result<Self, Error> {
        Self::new_dict(r, &[])
    }

    /// new_dict is like new but uses a preset dictionary.
    /// new_dict ignores the dictionary if the compressed data does not refer to it.
    /// If the compressed data refers to a different dictionary, new_dict returns ErrDictionary.
    //
    // ggstd not implemented:
    // The Reader returned by new_reader_dict also implements Resetter.
    pub fn new_dict(r: &'a mut Input, dict: &'a [u8]) -> Result<Self, Error> {
        let have_dict = read_dictionary_config(r, dict)?;
        Ok(Self {
            decompressor: if have_dict {
                flate::Reader::new_dict(r, dict)
            } else {
                flate::Reader::new(r)
            },
            digest: hash::adler32::new(),
        })
    }

    pub fn read(&mut self, p: &mut [u8]) -> Result<usize, Error> {
        let (n, err) = crate::io::Reader::read(&mut self.decompressor, p);
        self.digest.write_all(&p[0..n])?;

        if n > 0 {
            return Ok(n);
        }

        if let Some(err) = err {
            return Err(Error::StdIo(err));
        }

        // n == 0 or an error happened and it is EOF

        // Finished file; check checksum.
        let mut scratch = [0; 4];

        std::io::Read::read_exact(self.decompressor.input_reader(), &mut scratch)?;

        // ZLIB (RFC 1950) is big-endian, unlike GZIP (RFC 1952).
        let checksum = (scratch[0] as u32) << 24
            | (scratch[1] as u32) << 16
            | (scratch[2] as u32) << 8
            | (scratch[3] as u32);
        if checksum != self.digest.sum32() {
            return Err(Error::ErrChecksum);
        }
        Ok(n)
    }

    /// Calling close does not close the wrapped io.Reader originally passed to new_reader.
    /// In order for the ZLIB checksum to be verified, the reader must be
    /// fully consumed until the io.EOF.
    pub fn close(&mut self) -> Result<(), std::io::Error> {
        self.decompressor.close()
    }

    /// reset discards any buffered data and resets the Reader as if it was
    /// newly initialized with the given reader.
    /// This permits reusing a Reader instead of allocating a new one.
    pub fn reset(&mut self, r: &'a mut Input, dict: &'a [u8]) -> Result<(), Error> {
        let have_dict = read_dictionary_config(r, dict)?;
        self.decompressor = if have_dict {
            flate::Reader::new_dict(r, dict)
        } else {
            flate::Reader::new(r)
        };
        self.digest = hash::adler32::new();
        Ok(())
    }
}

impl<Input: std::io::BufRead> std::io::Read for Reader<'_, Input> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = Reader::read(self, buf);
        match res {
            Ok(n) => Ok(n),
            Err(err) => Err(err.to_stdio_error()),
        }
    }
}

/// Read the header, validate it and return true if the dictionary is used.
fn read_dictionary_config<Input: std::io::BufRead>(
    r: &mut Input,
    dict: &[u8],
) -> Result<bool, Error> {
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
    Ok(have_dict)
}
