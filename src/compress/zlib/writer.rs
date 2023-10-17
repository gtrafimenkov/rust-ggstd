// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compress::flate;
use crate::encoding::binary::{self, ByteOrder};
use crate::errors;
use crate::hash::{self, adler32};
use std::io::Write;

// These constants are copied from the flate package, so that code that imports
// "compress/zlib" does not also have to import "compress/flate".
pub const NO_COMPRESSION: isize = flate::NO_COMPRESSION;
pub const BEST_SPEED: isize = flate::BEST_SPEED;
pub const BEST_COMPRESSION: isize = flate::BEST_COMPRESSION;
pub const DEFAULT_COMPRESSION: isize = flate::DEFAULT_COMPRESSION;
pub const HUFFMAN_ONLY: isize = flate::HUFFMAN_ONLY;

/// A Writer takes data written to it and writes the compressed
/// form of that data to an underlying writer (see new_writer).
pub struct Writer<'a, Output: std::io::Write> {
    level: isize,
    dict: &'a [u8],
    compressor: flate::Writer<'a, Output>,
    digest: Box<dyn hash::Hash32>,
    scratch: [u8; 4],
    wrote_header: bool,
}

/// new_writer creates a new Writer.
/// Writes to the returned Writer are compressed and written to w.
///
/// It is the caller's responsibility to call close on the Writer when done.
/// Writes may be buffered and not flushed until close.
pub fn new_writer<Output: std::io::Write>(w: &mut Output) -> Writer<Output> {
    new_writer_level_dict(w, DEFAULT_COMPRESSION, &[]).unwrap()
}

/// new_writer_level is like new_writer but specifies the compression level instead
/// of assuming DEFAULT_COMPRESSION.
///
/// The compression level can be DEFAULT_COMPRESSION, NO_COMPRESSION, HUFFMAN_ONLY
/// or any integer value between BEST_SPEED and BEST_COMPRESSION inclusive.
/// The error returned will be nil if the level is valid.
pub fn new_writer_level<Output: std::io::Write>(w: &mut Output, level: isize) -> Writer<Output> {
    return new_writer_level_dict(w, level, &[]).unwrap();
}

/// new_writer_level_dict is like new_writer_level but specifies a dictionary to
/// compress with.
///
/// The dictionary may be nil. If not, its contents should not be modified until
/// the Writer is closed.
pub fn new_writer_level_dict<'a, Output: std::io::Write>(
    w: &'a mut Output,
    level: isize,
    dict: &'a [u8],
) -> std::io::Result<Writer<'a, Output>> {
    if !(HUFFMAN_ONLY..=BEST_COMPRESSION).contains(&level) {
        return Err(errors::new_stdio_other_error(format!(
            "zlib: invalid compression level: {}",
            level
        )));
    }
    Ok(Writer {
        level,
        dict,
        compressor: flate::Writer::new_dict(w, level, dict).unwrap(),
        digest: Box::new(adler32::new()),
        scratch: [0; 4],
        wrote_header: false,
    })
}

impl<'a, Output: std::io::Write> Writer<'a, Output> {
    /// reset clears the state of the Writer z such that it is equivalent to its
    /// initial state from new_writer_level or new_writer_level_dict, but instead writing
    /// to w.
    pub fn reset(&mut self, w: &'a mut Output) {
        // self.w = w;
        self.compressor.reset(w);
        self.digest.reset();
        self.scratch = [0; 4];
        self.wrote_header = false;
    }

    // write_header writes the ZLIB header and initializes compression state.
    fn write_header(&mut self) -> std::io::Result<()> {
        if self.wrote_header {
            return Ok(());
        }
        self.wrote_header = true;
        // ZLIB has a two-u8 header (as documented in RFC 1950).
        // The first four bits is the CINFO (compression info), which is 7 for the default deflate window size.
        // The next four bits is the CM (compression method), which is 8 for deflate.
        self.scratch[0] = 0x78;
        // The next two bits is the FLEVEL (compression level). The four values are:
        // 0=fastest, 1=fast, 2=default, 3=best.
        // The next bit, FDICT, is set if a dictionary is given.
        // The final five FCHECK bits form a mod-31 checksum.
        self.scratch[1] = match self.level {
            -2 | 0 | 1 => 0 << 6,
            2..=5 => 1 << 6,
            6 | -1 => 2 << 6,
            7..=9 => 3 << 6,
            _ => {
                panic!("unreachable");
            }
        };
        if !self.dict.is_empty() {
            self.scratch[1] |= 1 << 5;
        }
        self.scratch[1] +=
            (31 - (((self.scratch[0] as u16) << 8) + (self.scratch[1] as u16)) % 31) as u8;
        self.compressor.output().write_all(&self.scratch[0..2])?;
        if !self.dict.is_empty() {
            // The next four bytes are the Adler-32 checksum of the dictionary.
            binary::BIG_ENDIAN.put_uint32(&mut self.scratch[..], adler32::checksum(self.dict));
            self.compressor.output().write_all(&self.scratch[0..4])?;
        }
        Ok(())
    }

    /// write writes a compressed form of p to the underlying writer. The
    /// compressed bytes are not necessarily flushed until the Writer is closed or
    /// explicitly flushed.
    pub fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        self.write_header()?;
        if p.is_empty() {
            return Ok(0);
        }
        let written = self.compressor.write(p)?;
        if written != p.len() {
            panic!(
                "not all data were written: written {}, all {}",
                written,
                p.len()
            );
        }
        let written = self.digest.write(p)?;
        if written != p.len() {
            panic!(
                "not all data were written to digest: written {}, all {}",
                written,
                p.len()
            );
        }
        Ok(written)
    }

    /// flush flushes the Writer to its underlying ggio::Writer.
    pub fn flush(&'a mut self) -> std::io::Result<()> {
        self.write_header()?;
        self.compressor.flush()?;
        Ok(())
    }

    /// close closes the Writer, flushing any unwritten data to the underlying
    /// writer, but does not close the underlying writer.
    pub fn close(&mut self) -> std::io::Result<()> {
        self.write_header()?;
        self.compressor.close()?;
        let checksum = self.digest.sum32();
        // ZLIB (RFC 1950) is big-endian, unlike GZIP (RFC 1952).
        binary::BIG_ENDIAN.put_uint32(&mut self.scratch[..], checksum);
        self.compressor.output().write_all(&self.scratch[0..4])?;
        Ok(())
    }
}
