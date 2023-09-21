// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;
use crate::errors;
use crate::io as ggio;

/// A Reader implements the crate::io::Reader, crate::io::ReaderAt, crate::io::WriterTo, crate::io::Seeker,
/// crate::io::ByteScanner, and io.RuneScanner interfaces by reading from
/// a u8 slice.
/// Unlike a Buffer, a Reader is read-only and supports seeking.
/// The zero value for Reader operates like a Reader of an empty slice.
pub struct Reader<'a> {
    s: &'a [u8],
    i: u64, // current reading index
            // 	prevRune int   // index of previous rune; or < 0
}

impl<'a> Reader<'a> {
    // pub fn new() -> Self {
    //     Self { s: &[], i: 0 }
    // }
    /// new returns a new Reader reading from b.
    pub fn new(b: &'a [u8]) -> Self {
        Self {
            s: b,
            i: 0,
            // prevRune:-1,
        }
    }

    /// len returns the number of bytes of the unread portion of the
    /// slice.
    pub fn len(&self) -> usize {
        if self.i >= self.s.len() as u64 {
            return 0;
        }
        (self.s.len() as u64 - self.i) as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// size returns the original length of the underlying u8 slice.
    /// size is the number of bytes available for reading via read_at.
    /// The result is unaffected by any method calls except reset.
    pub fn size(&mut self) -> usize {
        self.s.len()
    }
}

impl std::io::Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.i >= self.s.len() as u64 {
            return Ok(0);
        }
        // 	self.prevRune = -1
        let n = compat::copy(buf, &self.s[self.i as usize..]);
        self.i += n as u64;
        Ok(n)
    }
}

impl Reader<'_> {
    /// read_at implements the io.ReaderAt interface.
    pub fn read_at(&mut self, b: &mut [u8], off: u64) -> std::io::Result<usize> {
        if off >= self.s.len() as u64 {
            return Ok(0);
        }
        let n = compat::copy(b, &self.s[off as usize..]);
        Ok(n)
    }
}

impl ggio::ByteReader for Reader<'_> {
    /// read_byte implements the io.ByteReader interface.
    fn read_byte(&mut self) -> std::io::Result<u8> {
        // 	self.prevRune = -1
        if self.i >= self.s.len() as u64 {
            return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
        }
        let b = self.s[self.i as usize];
        self.i += 1;
        Ok(b)
    }
}

impl<'a> Reader<'a> {
    /// unread_byte complements read_byte in implementing the io.ByteScanner interface.
    pub fn unread_byte(&mut self) -> std::io::Result<()> {
        if self.i == 0 {
            return Err(errors::new_stdio_other_error(
                "bytes::Reader::unread_byte: at beginning of slice".to_string(),
            ));
        }
        // 	self.prevRune = -1
        self.i -= 1;
        Ok(())
    }

    // // ReadRune implements the io.RuneReader interface.
    // fn ReadRune(&mut self) (ch rune, size int, err error) {
    // 	if self.i >= i64(self.s.len()) {
    // 		self.prevRune = -1
    // 		return 0, 0, io.EOF
    // 	}
    // 	self.prevRune = int(self.i)
    // 	if c := self.s[self.i]; c < utf8::RUNE_SELF {
    // 		self.i += 1
    // 		return rune(c), 1, nil
    // 	}
    // 	ch, size = utf8.decode_rune(self.s[self.i..])
    // 	self.i += i64(size)
    // 	return
    // }

    // // UnreadRune complements ReadRune in implementing the io.RuneScanner interface.
    // fn UnreadRune(&mut self) error {
    // 	if self.i <= 0 {
    // 		return errors.New("bytes::Reader::UnreadRune: at beginning of slice")
    // 	}
    // 	if self.prevRune < 0 {
    // 		return errors.New("bytes::Reader::UnreadRune: previous operation was not ReadRune")
    // 	}
    // 	self.i = i64(self.prevRune)
    // 	self.prevRune = -1
    // 	return nil
    // }

    /// seek implements the io.Seeker interface.
    pub fn seek(&mut self, offset: i64, whence: ggio::Seek) -> std::io::Result<u64> {
        // 	self.prevRune = -1
        let abs = match whence {
            ggio::Seek::Start => offset,
            ggio::Seek::Current => self.i as i64 + offset,
            ggio::Seek::End => self.s.len() as i64 + offset,
        };
        if abs < 0 {
            return Err(errors::new_stdio_other_error(
                "bytes::Reader::seek: negative position".to_string(),
            ));
        }
        self.i = abs as u64;
        Ok(self.i)
    }

    /// write_to implements the io.WriterTo interface.
    pub fn write_to(&mut self, w: &mut dyn std::io::Write) -> std::io::Result<usize> {
        // 	self.prevRune = -1
        if self.i >= self.s.len() as u64 {
            return Ok(0);
        }
        let b = &self.s[self.i as usize..];
        // let (m, err) = w.write(b);
        match w.write(b) {
            Ok(m) => {
                if m > b.len() {
                    panic!("bytes::Reader::write_to: invalid Write count");
                }
                self.i += m as u64;
                if m != b.len() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "short write",
                    ));
                }
                Ok(m)
            }
            Err(err) => Err(err),
        }
    }

    /// reset resets the Reader to be reading from b.
    pub fn reset(&mut self, b: &'a [u8]) {
        self.s = b;
        self.i = 0;
        // self.prevRune = -1;
    }
}
