// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// package strings

// import (
// 	"errors"
// 	"io"
// 	"unicode/utf8"
// )

/// A Reader implements the std::io::Read trait.
// A Reader implements the io.Reader, io.ReaderAt, io.ByteReader, io.ByteScanner,
// io.RuneReader, io.RuneScanner, io.Seeker, and io.WriterTo interfaces by reading
// from a string.
// The zero value for Reader operates like a Reader of an empty string.
pub struct Reader<'a> {
    cursor: std::io::Cursor<&'a [u8]>,
    // 	s        string
    // 	i        int64 // current reading index
    // 	prevRune int   // index of previous rune; or < 0
}

impl<'a> Reader<'a> {
    /// new returns a new Reader reading from s.
    // // It is similar to bytes.NewBufferString but more efficient and read-only.
    pub fn new(s: &'a str) -> Self {
        // fn NewReader(s string) *Reader { return &Reader{s, 0, -1} }
        Self {
            cursor: std::io::Cursor::new(s.as_bytes()),
        }
    }
    /// len returns the number of bytes of the unread portion of the
    /// string.
    pub fn len(&self) -> u64 {
        let pos = self.cursor.position();
        let len = self.cursor.get_ref().len() as u64;
        if pos > len { 0 } else { len - pos }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// size returns the original length of the underlying string.
    /// size is the number of bytes available for reading via ReadAt.
    /// The returned value is always the same and is not affected by calls
    /// to any other method.
    pub fn size(&self) -> u64 {
        self.cursor.get_ref().len() as u64
    }
}

impl std::io::Read for Reader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}

// // Read implements the io.Reader interface.
// fn (r *Reader) Read(b []byte) (n int, err error) {
// 	if self.i >= int64(len(self.s)) {
// 		return 0, io.EOF
// 	}
// 	self.prevRune = -1
// 	n = copy(b, self.s[self.i:])
// 	self.i += int64(n)
// 	return
// }

// // ReadAt implements the io.ReaderAt interface.
// fn (r *Reader) ReadAt(b []byte, off int64) (n int, err error) {
// 	// cannot modify state - see io.ReaderAt
// 	if off < 0 {
// 		return 0, errors.New("strings.Reader.ReadAt: negative offset")
// 	}
// 	if off >= int64(len(self.s)) {
// 		return 0, io.EOF
// 	}
// 	n = copy(b, self.s[off:])
// 	if n < len(b) {
// 		err = io.EOF
// 	}
// 	return
// }

// // ReadByte implements the io.ByteReader interface.
// fn (r *Reader) ReadByte() (byte, error) {
// 	self.prevRune = -1
// 	if self.i >= int64(len(self.s)) {
// 		return 0, io.EOF
// 	}
// 	b := self.s[self.i]
// 	self.i++
// 	return b, nil
// }

// // UnreadByte implements the io.ByteScanner interface.
// fn (r *Reader) UnreadByte() error {
// 	if self.i <= 0 {
// 		return errors.New("strings.Reader.UnreadByte: at beginning of string")
// 	}
// 	self.prevRune = -1
// 	self.i--
// 	return nil
// }

// // ReadRune implements the io.RuneReader interface.
// fn (r *Reader) ReadRune() (ch rune, size int, err error) {
// 	if self.i >= int64(len(self.s)) {
// 		self.prevRune = -1
// 		return 0, 0, io.EOF
// 	}
// 	self.prevRune = int(self.i)
// 	if c := self.s[self.i]; c < utf8::RUNE_SELF {
// 		self.i++
// 		return rune(c), 1, nil
// 	}
// 	ch, size = utf8.decode_rune_in_string(self.s[self.i:])
// 	self.i += int64(size)
// 	return
// }

// // UnreadRune implements the io.RuneScanner interface.
// fn (r *Reader) UnreadRune() error {
// 	if self.i <= 0 {
// 		return errors.New("strings.Reader.UnreadRune: at beginning of string")
// 	}
// 	if self.prevRune < 0 {
// 		return errors.New("strings.Reader.UnreadRune: previous operation was not ReadRune")
// 	}
// 	self.i = int64(self.prevRune)
// 	self.prevRune = -1
// 	return nil
// }

// // Seek implements the io.Seeker interface.
// fn (r *Reader) Seek(offset int64, whence int) (int64, error) {
// 	self.prevRune = -1
// 	var abs int64
// 	switch whence {
// 	case io.SeekStart:
// 		abs = offset
// 	case io.SeekCurrent:
// 		abs = self.i + offset
// 	case io.SeekEnd:
// 		abs = int64(len(self.s)) + offset
// 	default:
// 		return 0, errors.New("strings.Reader.Seek: invalid whence")
// 	}
// 	if abs < 0 {
// 		return 0, errors.New("strings.Reader.Seek: negative position")
// 	}
// 	self.i = abs
// 	return abs, nil
// }

// // WriteTo implements the io.WriterTo interface.
// fn (r *Reader) WriteTo(w io.Writer) (n int64, err error) {
// 	self.prevRune = -1
// 	if self.i >= int64(len(self.s)) {
// 		return 0, nil
// 	}
// 	s := self.s[self.i:]
// 	m, err := io.WriteString(w, s)
// 	if m > len(s) {
// 		panic("strings.Reader.WriteTo: invalid WriteString count")
// 	}
// 	self.i += int64(m)
// 	n = int64(m)
// 	if m != len(s) && err == nil {
// 		err = io.ErrShortWrite
// 	}
// 	return
// }

// // Reset resets the Reader to be reading from s.
// fn (r *Reader) Reset(s string) { *r = Reader{s, 0, -1} }
