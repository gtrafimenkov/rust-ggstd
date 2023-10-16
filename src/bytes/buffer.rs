// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Simple byte buffer for marshaling data.

use crate::compat;
use std::io::Write;

const SMALL_BUFFER_SIZE: usize = 64;

/// A Buffer is a variable-sized buffer of bytes with read and write methods.
// The zero value for Buffer is an empty buffer ready to use.
pub struct Buffer {
    // ggstd TODO: use std::io::Cursor instead?

    // rust impl: it is much simpler then Go since we are using Vec,
    // which has support for growing.
    // buf[off..buf.len()] - unread data in the buffer
    buf: Vec<u8>,
    off: usize,
    // 	lastRead readOp // last read operation, so that Unread* can work correctly.
}

// // The readOp constants describe the last action performed on
// // the buffer, so that UnreadRune and UnreadByte can check for
// // invalid usage. opReadRuneX constants are chosen such that
// // converted to int they correspond to the rune size that was read.
// type readOp int8

// // Don't use iota for these, as the values need to correspond with the
// // names and comments, which is easier to see when being explicit.
// const (
// 	opRead      readOp = -1 // Any other read operation.
// 	opInvalid   readOp = 0  // Non-read operation.
// 	opReadRune1 readOp = 1  // Read rune of size 1.
// 	opReadRune2 readOp = 2  // Read rune of size 2.
// 	opReadRune3 readOp = 3  // Read rune of size 3.
// 	opReadRune4 readOp = 4  // Read rune of size 4.
// )

// // ErrTooLarge is passed to panic if memory cannot be allocated to store data in a buffer.
// var ErrTooLarge = errors.New("bytes::Buffer::new(): too large")
// var errNegativeRead = errors.New("bytes::Buffer::new(): reader returned negative count from Read")

// const maxInt = int(^uint(0) >> 1)

impl Buffer {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(SMALL_BUFFER_SIZE),
            off: 0,
        }
    }

    /// bytes returns a slice of length b.Len() holding the unread portion of the buffer.
    /// The slice is valid for use only until the next buffer modification (that is,
    /// only until the next call to a method like Read, Write, Reset, or Truncate).
    /// The slice aliases the buffer content at least until the next buffer modification,
    /// so immediate changes to the slice will affect the result of future reads.
    pub fn bytes(&self) -> &[u8] {
        &self.buf[self.off..]
    }

    /// string returns the contents of the unread portion of the buffer
    /// as a string.
    // If the Buffer is a nil pointer, it returns "<nil>".
    //
    // To build strings more efficiently, see the strings.Builder type.
    pub fn string(&self) -> String {
        // 	if b == nil {
        // 		// Special case, useful in debugging.
        // 		return "<nil>"
        // 	}
        String::from_utf8_lossy(self.bytes()).to_string()
    }

    /// empty reports whether the unread portion of the buffer is empty.
    fn empty(&self) -> bool {
        self.buf.len() <= self.off
    }

    /// len returns the number of bytes of the unread portion of the buffer;
    /// b.Len() == len(b.bytes()).
    pub fn len(&self) -> usize {
        self.buf.len() - self.off
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// cap returns the capacity of the buffer's underlying byte slice, that is, the
    /// total space allocated for the buffer's data.
    pub fn cap(&self) -> usize {
        self.buf.capacity()
    }

    // truncate discards all but the first n unread bytes from the buffer
    // but continues to use the same allocated storage.
    // It panics if n is greater than the length of the buffer.
    pub fn truncate(&mut self, n: usize) {
        if n == 0 {
            self.reset();
            return;
        }
        // self.lastRead = opInvalid
        if n > self.len() {
            panic!("bytes::Buffer::new(): truncation out of range");
        }
        self.buf.truncate(n + self.off);
    }

    /// reset resets the buffer to be empty,
    /// but it retains the underlying storage for use by future writes.
    /// Reset is the same as Truncate(0).
    pub fn reset(&mut self) {
        self.off = 0;
        self.buf.clear();
        // b.lastRead = opInvalid
    }

    /// guarantee_space guarantees space for n more bytes.
    fn guarantee_space(&mut self, n: usize) {
        // rust impl:
        //   buf can grow indefinitely if we allow it.
        //   We need to avoid that by moving off to the beginning of buf.
        //   We move off in two cases:
        //     - buf has no unread data
        //     - off is further than half of the buf capacity
        let data_size = self.buf.len() - self.off;
        if data_size == 0 {
            self.reset();
        } else if self.off > SMALL_BUFFER_SIZE && self.off > self.buf.capacity() / 2 {
            let end = self.buf.len();
            self.buf.copy_within(self.off..end, 0);
            self.buf.truncate(data_size);
            self.off = 0;
        }

        let space_left = self.buf.capacity() - data_size;
        if space_left < n {
            let missing_capacity = n - space_left;
            self.buf.reserve(missing_capacity);
        }
    }

    /// grow grows the buffer's capacity, if necessary, to guarantee space for
    /// another n bytes. After Grow(n), at least n bytes can be written to the
    /// buffer without another allocation.
    pub fn grow(&mut self, n: usize) {
        self.guarantee_space(n);
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::io::Read for Buffer {
    /// Read reads the next p.len() bytes from the buffer or until the buffer
    /// is drained. The return value n is the number of bytes read.
    fn read(&mut self, p: &mut [u8]) -> std::io::Result<usize> {
        // b.lastRead = opInvalid
        if self.empty() {
            // Buffer is empty, reset to recover space.
            self.reset();
            if p.is_empty() {
                return Ok(0);
            }
            return Ok(0);
        }
        let n = compat::copy(p, &self.buf[self.off..]);
        self.off += n;
        if n > 0 {
            // self.lastRead = opRead
        }
        Ok(n)
    }
}

impl std::io::Write for Buffer {
    /// write appends the contents of p to the buffer, growing the buffer as
    /// needed. The return value n is the length of p.
    fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        self.guarantee_space(p.len());
        self.buf.extend_from_slice(p);
        Ok(p.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Buffer {
    /// write_string appends the contents of s to the buffer, growing the buffer as
    /// needed. The return value n is the length of s.
    pub fn write_string(&mut self, s: &str) -> std::io::Result<usize> {
        // b.lastRead = opInvalid
        self.write(s.as_bytes())
    }

    // // MinRead is the minimum slice size passed to a Read call by
    // // Buffer.ReadFrom. As long as the Buffer has at least MinRead bytes beyond
    // // what is required to hold the contents of r, ReadFrom will not grow the
    // // underlying buffer.
    // const MinRead = 512

    // // ReadFrom reads data from r until EOF and appends it to the buffer, growing
    // // the buffer as needed. The return value n is the number of bytes read. Any
    // // error except io.EOF encountered during the read is also returned. If the
    // // buffer becomes too large, ReadFrom will panic with ErrTooLarge.
    // fn ReadFrom(&self, r io.Reader) (n int64, err error) {
    // 	b.lastRead = opInvalid
    // 	for {
    // 		i := b.guarantee_space(MinRead)
    // 		self.buf = self.buf[..i]
    // 		m, e := r.Read(self.buf[i:cap(self.buf)])
    // 		if m < 0 {
    // 			panic(errNegativeRead)
    // 		}

    // 		self.buf = self.buf[..i+m]
    // 		n += int64(m)
    // 		if e == io.EOF {
    // 			return n, nil // e is EOF, so return nil explicitly
    // 		}
    // 		if e != nil {
    // 			return n, e
    // 		}
    // 	}
    // }

    // // WriteTo writes data to w until the buffer is drained or an error occurs.
    // // The return value n is the number of bytes written; it always fits into an
    // // int, but it is int64 to match the io.WriterTo interface. Any error
    // // encountered during the write is also returned.
    // fn WriteTo(&self, w ggio::Writer) (n int64, err error) {
    // 	b.lastRead = opInvalid
    // 	if nBytes := b.Len(); nBytes > 0 {
    // 		m, e := w.write(self.buf[self.off..])
    // 		if m > nBytes {
    // 			panic("bytes::Buffer::new().WriteTo: invalid Write count")
    // 		}
    // 		self.off += m
    // 		n = int64(m)
    // 		if e != nil {
    // 			return n, e
    // 		}
    // 		// all bytes should have been written, by definition of
    // 		// Write method in ggio::Writer
    // 		if m != nBytes {
    // 			return n, io.ErrShortWrite
    // 		}
    // 	}
    // 	// Buffer is now empty; reset.
    // 	b.reset()
    // 	return n, nil
    // }

    /// write_byte appends the byte c to the buffer, growing the buffer as needed.
    pub fn write_byte(&mut self, c: u8) -> std::io::Result<()> {
        // b.lastRead = opInvalid
        self.guarantee_space(1);
        self.buf.push(c);
        Ok(())
    }

    // // WriteRune appends the UTF-8 encoding of Unicode code point r to the
    // // buffer, returning its length and an error, which is always nil but is
    // // included to match bufio::Writer's WriteRune. The buffer is grown as needed;
    // // if it becomes too large, WriteRune will panic with ErrTooLarge.
    // fn WriteRune(&self, r rune) (n: usize, err error) {
    // 	// Compare as uint32 to correctly handle negative runes.
    // 	if uint32(r) < utf8::RUNE_SELF {
    // 		b.write_byte(byte(r))
    // 		return 1, nil
    // 	}
    // 	b.lastRead = opInvalid
    // 	m, ok := b.tryGrowByReslice(utf8.UTFMAX)
    // 	if !ok {
    // 		m = b.guarantee_space(utf8.UTFMAX)
    // 	}
    // 	self.buf = utf8.AppendRune(self.buf[..m], r)
    // 	return self.buf.len() - m, nil
    // }

    // // Next returns a slice containing the next n bytes from the buffer,
    // // advancing the buffer as if the bytes had been returned by read.
    // // If there are fewer than n bytes in the buffer, Next returns the entire buffer.
    // // The slice is only valid until the next call to a read or write method.
    // pub fn Next(&self, n: usize) -> &[u8] {
    // 	b.lastRead = opInvalid
    // 	m := b.Len()
    // 	if n > m {
    // 		n = m
    // 	}
    // 	data := self.buf[self.off : self.off+n]
    // 	self.off += n
    // 	if n > 0 {
    // 		b.lastRead = opRead
    // 	}
    // 	return data
    // }

    /// read_byte reads and returns the next byte from the buffer.
    /// If no byte is available, it returns None.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.empty() {
            // Buffer is empty, reset to recover space.
            self.reset();
            return None;
        }
        let c = self.buf[self.off];
        self.off += 1;
        // 	self.lastRead = opRead
        Some(c)
    }

    // // ReadRune reads and returns the next UTF-8-encoded
    // // Unicode code point from the buffer.
    // // If no bytes are available, the error returned is io.EOF.
    // // If the bytes are an erroneous UTF-8 encoding, it
    // // consumes one byte and returns U+FFFD, 1.
    // pub fn ReadRune(&self) (r rune, size int, err error) {
    // 	if b.empty() {
    // 		// Buffer is empty, reset to recover space.
    // 		b.reset()
    // 		return 0, 0, io.EOF
    // 	}
    // 	c := self.buf[self.off]
    // 	if c < utf8::RUNE_SELF {
    // 		self.off++
    // 		b.lastRead = opReadRune1
    // 		return rune(c), 1, nil
    // 	}
    // 	r, n := utf8.decode_rune(self.buf[self.off..])
    // 	self.off += n
    // 	b.lastRead = readOp(n)
    // 	return r, n, nil
    // }

    // // UnreadRune unreads the last rune returned by ReadRune.
    // // If the most recent read or write operation on the buffer was
    // // not a successful ReadRune, UnreadRune returns an error.  (In this regard
    // // it is stricter than UnreadByte, which will unread the last byte
    // // from any read operation.)
    // pub fn UnreadRune(&self) error {
    // 	if b.lastRead <= opInvalid {
    // 		return errors.New("bytes::Buffer::new(): UnreadRune: previous operation was not a successful ReadRune")
    // 	}
    // 	if self.off >= int(b.lastRead) {
    // 		self.off -= int(b.lastRead)
    // 	}
    // 	b.lastRead = opInvalid
    // 	return nil
    // }

    // var errUnreadByte = errors.New("bytes::Buffer::new(): UnreadByte: previous operation was not a successful read")

    // // UnreadByte unreads the last byte returned by the most recent successful
    // // read operation that read at least one byte. If a write has happened since
    // // the last read, if the last read returned an error, or if the read read zero
    // // bytes, UnreadByte returns an error.
    // fn UnreadByte(&self) error {
    // 	if b.lastRead == opInvalid {
    // 		return errUnreadByte
    // 	}
    // 	b.lastRead = opInvalid
    // 	if self.off > 0 {
    // 		self.off--
    // 	}
    // 	return nil
    // }

    // // ReadBytes reads until the first occurrence of delim in the input,
    // // returning a slice containing the data up to and including the delimiter.
    // // If ReadBytes encounters an error before finding a delimiter,
    // // it returns the data read before the error and the error itself (often io.EOF).
    // // ReadBytes returns err != nil if and only if the returned data does not end in
    // // delim.
    // fn ReadBytes(&self, delim byte) (line [u8], err error) {
    // 	slice, err := b.readSlice(delim)
    // 	// return a copy of slice. The buffer's backing array may
    // 	// be overwritten by later calls.
    // 	line = append(line, slice...)
    // 	return line, err
    // }

    // // readSlice is like ReadBytes but returns a reference to internal buffer data.
    // fn readSlice(&self, delim byte) (line [u8], err error) {
    // 	i := index_byte(self.buf[self.off..], delim)
    // 	end := self.off + i + 1
    // 	if i < 0 {
    // 		end = self.buf.len()
    // 		err = io.EOF
    // 	}
    // 	line = self.buf[self.off:end]
    // 	self.off = end
    // 	b.lastRead = opRead
    // 	return line, err
    // }

    // // ReadString reads until the first occurrence of delim in the input,
    // // returning a string containing the data up to and including the delimiter.
    // // If ReadString encounters an error before finding a delimiter,
    // // it returns the data read before the error and the error itself (often io.EOF).
    // // ReadString returns err != nil if and only if the returned data does not end
    // // in delim.
    // pub fn ReadString(&self, delim byte) (line string, err error) {
    // 	slice, err := b.readSlice(delim)
    // 	return string(slice), err
    // }
}

/// new_buffer creates and initializes a new Buffer using buf as its
/// initial contents. The new Buffer takes ownership of buf, and the
/// caller should not use buf after this call. new_buffer is intended to
/// prepare a Buffer to read existing data. It can also be used to set
/// the initial size of the internal buffer for writing. To do that,
/// buf should have the desired capacity but a length of zero.
///
/// Buffer::new() also can be used to initialize a Buffer.
pub fn new_buffer(buf: Vec<u8>) -> Buffer {
    Buffer { buf, off: 0 }
}

/// new_buffer_string creates and initializes a new Buffer using string s as its
/// initial contents. It is intended to prepare a buffer to read an existing
/// string.
///
/// Buffer::new() also can be used to initialize a Buffer.
pub fn new_buffer_string(s: &str) -> Buffer {
    Buffer {
        buf: s.as_bytes().to_vec(),
        off: 0,
    }
}
