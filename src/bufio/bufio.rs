// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package bufio implements buffered I/O. It wraps an std::io::Read or std::io::Write
//! object, creating another object (Reader or Writer) that also implements
//! the interface but provides buffering and some help for textual I/O.

use crate::errors;
use crate::io as ggio;

// import (
// 	"bytes"
// 	"errors"
// 	"io"
// 	"strings"
// 	"unicode/utf8"
// )

const DEFAULT_BUF_SIZE: usize = 4096;

// 	ErrInvalidUnreadByte = errors.New("bufio: invalid use of UnreadByte")
// 	ErrInvalidUnreadRune = errors.New("bufio: invalid use of UnreadRune")
static ERR_BUFFER_FULL: errors::ErrorStaticString = errors::new_static("bufio: buffer full");
// static ERR_NEGATIVE_COUNT: errors::ErrorStaticString = errors::new_static("bufio: negative count");

// buffered input.

/// Reader implements buffering for an std::io::Read object.
pub struct Reader<'a> {
    buf: Vec<u8>,
    rd: &'a mut dyn std::io::Read, // reader provided by the client
    r: usize,                      // buf read  positions
    w: usize,                      // buf write positions
    err: Option<Box<dyn std::error::Error>>,
    last_byte: isize,      // last byte read for UnreadByte; -1 means invalid
    last_rune_size: isize, // size of last rune read for UnreadRune; -1 means invalid
}

const MIN_READ_BUFFER_SIZE: usize = 16;
const MAX_CONSECUTIVE_EMPTY_READS: usize = 100;

/// new_reader_size returns a new Reader whose buffer has at least the specified
/// size.
// not implemented: can it be implemented?
// If the argument std::io::Read is already a Reader with large enough
// size, it returns the underlying Reader.
pub fn new_reader_size(rd: &mut dyn std::io::Read, size: usize) -> Reader {
    // Is it already a Reader?
    // 	b, ok := rd.(*Reader)
    // 	if ok && b.buf.len() >= size {
    // 		return b
    // 	}
    let size = size.max(MIN_READ_BUFFER_SIZE);
    Reader {
        buf: vec![0; size],
        rd,
        r: 0,
        w: 0,
        err: None,
        last_byte: -1,
        last_rune_size: -1,
    }
}

/// new_reader returns a new Reader whose buffer has the default size.
pub fn new_reader(rd: &mut dyn std::io::Read) -> Reader {
    new_reader_size(rd, DEFAULT_BUF_SIZE)
}

impl<'a> Reader<'a> {
    /// new creates a reader with the default size buffer
    pub fn new(r: &'a mut dyn std::io::Read) -> Self {
        Self {
            buf: vec![0; DEFAULT_BUF_SIZE],
            rd: r,
            r: 0,
            w: 0,
            err: None,
            last_byte: -1,
            last_rune_size: -1,
        }
    }

    /// size returns the size of the underlying buffer in bytes.
    pub fn size(&mut self) -> usize {
        return self.buf.len();
    }

    /// reset discards any buffered data, resets all state, and switches
    /// the buffered reader to read from r.
    pub fn reset(&mut self, r: &'a mut dyn std::io::Read) {
        self.buf.resize(DEFAULT_BUF_SIZE, 0);
        self.rd = r;
        self.r = 0;
        self.w = 0;
        self.last_byte = -1;
        self.last_rune_size = -1;
    }

    // var errNegativeRead = errors.New("bufio: reader returned negative count from Read")

    /// fill reads a new chunk into the buffer.
    fn fill(&mut self) {
        // Slide existing data to beginning.
        if self.r > 0 {
            self.buf.copy_within(self.r..self.w, 0);
            self.w -= self.r;
            self.r = 0
        }

        if self.w >= self.buf.len() {
            panic!("bufio: tried to fill full buffer");
        }

        // Read new data: try a limited number of times.
        for _i in 0..MAX_CONSECUTIVE_EMPTY_READS {
            // let (n, err) = self.rd.read(&mut self.buf[self.w..]);
            let res = self.rd.read(&mut self.buf[self.w..]);
            match res {
                Err(err) => {
                    self.err = Some(Box::new(err));
                    return;
                }
                Ok(n) => {
                    self.w += n;
                    if n > 0 {
                        return;
                    }
                }
            }
        }
        self.err = Some(Box::new(ggio::ERR_NO_PROGRESS));
    }

    fn read_err(&mut self) -> Option<Box<dyn std::error::Error>> {
        let err = std::mem::replace(&mut self.err, None);
        return err;
    }

    /// peek returns the next n bytes without advancing the reader. The bytes stop
    /// being valid at the next read call. If peek returns fewer than n bytes, it
    /// also returns an error explaining why the read is short. The error is
    /// ERR_BUFFER_FULL if n is larger than b's buffer size.
    ///
    /// Calling peek prevents a UnreadByte or UnreadRune call from succeeding
    /// until the next read operation.
    pub fn peek(&mut self, n: usize) -> (&[u8], Option<Box<dyn std::error::Error>>) {
        self.last_byte = -1;
        self.last_rune_size = -1;

        while self.w - self.r < n && self.w - self.r < self.buf.len() && self.err.is_none() {
            self.fill(); // self.w-self.r < self.buf.len() => buffer is not full
        }

        if n > self.buf.len() {
            return (&self.buf[self.r..self.w], Some(Box::new(ERR_BUFFER_FULL)));
        }

        // 0 <= n <= self.buf.len()
        let mut err = None;
        let avail = self.w - self.r;
        let mut n = n;
        if avail < n {
            // not enough data in buffer
            n = avail;
            err = self.read_err();
            if err.is_none() {
                err = Some(Box::new(ERR_BUFFER_FULL));
            }
        }
        return (&self.buf[self.r..self.r + n], err);
    }

    // // Discard skips the next n bytes, returning the number of bytes discarded.
    // //
    // // If Discard skips fewer than n bytes, it also returns an error.
    // // If 0 <= n <= b.buffered(), Discard is guaranteed to succeed without
    // // reading from the underlying std::io::Read.
    // fn Discard(&mut self, n int) (discarded int, err error) {
    // 	if n < 0 {
    // 		return 0, ERR_NEGATIVE_COUNT
    // 	}
    // 	if n == 0 {
    // 		return
    // 	}

    // 	b.lastByte = -1
    // 	b.lastRuneSize = -1

    // 	remain := n
    // 	for {
    // 		skip := b.buffered()
    // 		if skip == 0 {
    // 			b.fill()
    // 			skip = b.buffered()
    // 		}
    // 		if skip > remain {
    // 			skip = remain
    // 		}
    // 		b.r += skip
    // 		remain -= skip
    // 		if remain == 0 {
    // 			return n, nil
    // 		}
    // 		if b.err.is_some() {
    // 			return n - remain, b.read_err()
    // 		}
    // 	}
    // }

    // // Read reads data into p.
    // // It returns the number of bytes read into p.
    // // The bytes are taken from at most one Read on the underlying Reader,
    // // hence n may be less than len(p).
    // // To read exactly len(p) bytes, use io.ReadFull(b, p).
    // // If the underlying Reader can return a non-zero count with io.EOF,
    // // then this Read method can do so as well; see the [std::io::Read] docs.
    // fn Read(&mut self, p [u8]) (n int, err error) {
    // 	n = len(p)
    // 	if n == 0 {
    // 		if b.buffered() > 0 {
    // 			return 0, nil
    // 		}
    // 		return 0, b.read_err()
    // 	}
    // 	if b.r == b.w {
    // 		if b.err.is_some() {
    // 			return 0, b.read_err()
    // 		}
    // 		if len(p) >= b.buf.len() {
    // 			// Large read, empty buffer.
    // 			// Read directly into p to avoid copy.
    // 			n, b.err = b.rd.Read(p)
    // 			if n < 0 {
    // 				panic(errNegativeRead)
    // 			}
    // 			if n > 0 {
    // 				b.lastByte = int(p[n-1])
    // 				b.lastRuneSize = -1
    // 			}
    // 			return n, b.read_err()
    // 		}
    // 		// One read.
    // 		// Do not use b.fill, which will loop.
    // 		b.r = 0
    // 		b.w = 0
    // 		n, b.err = b.rd.Read(b.buf)
    // 		if n < 0 {
    // 			panic(errNegativeRead)
    // 		}
    // 		if n == 0 {
    // 			return 0, b.read_err()
    // 		}
    // 		b.w += n
    // 	}

    // 	// copy as much as we can
    // 	// Note: if the slice panics here, it is probably because
    // 	// the underlying reader returned a bad count. See issue 49795.
    // 	n = copy(p, b.buf[b.r:b.w])
    // 	b.r += n
    // 	b.lastByte = int(b.buf[b.r-1])
    // 	b.lastRuneSize = -1
    // 	return n, nil
    // }

    // // ReadByte reads and returns a single byte.
    // // If no byte is available, returns an error.
    // fn ReadByte (&mut self) (byte, error) {
    // 	b.lastRuneSize = -1
    // 	for b.r == b.w {
    // 		if b.err.is_some() {
    // 			return 0, b.read_err()
    // 		}
    // 		b.fill() // buffer is empty
    // 	}
    // 	c := b.buf[b.r]
    // 	b.r++
    // 	b.lastByte = int(c)
    // 	return c, nil
    // }

    // // UnreadByte unreads the last byte. Only the most recently read byte can be unread.
    // //
    // // UnreadByte returns an error if the most recent method called on the
    // // Reader was not a read operation. Notably, peek, Discard, and WriteTo are not
    // // considered read operations.
    // fn UnreadByte (&mut self) error {
    // 	if b.lastByte < 0 || b.r == 0 && b.w > 0 {
    // 		return ErrInvalidUnreadByte
    // 	}
    // 	// b.r > 0 || b.w == 0
    // 	if b.r > 0 {
    // 		b.r--
    // 	} else {
    // 		// b.r == 0 && b.w == 0
    // 		b.w = 1
    // 	}
    // 	b.buf[b.r] = byte(b.lastByte)
    // 	b.lastByte = -1
    // 	b.lastRuneSize = -1
    // 	return nil
    // }

    // // ReadRune reads a single UTF-8 encoded Unicode character and returns the
    // // rune and its size in bytes. If the encoded rune is invalid, it consumes one byte
    // // and returns unicode.ReplacementChar (U+FFFD) with a size of 1.
    // fn ReadRune (&mut self) (r rune, size int, err error) {
    // 	for b.r+utf8.UTFMax > b.w && !utf8.FullRune(b.buf[b.r:b.w]) && b.err.is_none() && b.w-b.r < b.buf.len() {
    // 		b.fill() // b.w-b.r < len(buf) => buffer is not full
    // 	}
    // 	b.lastRuneSize = -1
    // 	if b.r == b.w {
    // 		return 0, 0, b.read_err()
    // 	}
    // 	r, size = rune(b.buf[b.r]), 1
    // 	if r >= utf8.RuneSelf {
    // 		r, size = utf8.DecodeRune(b.buf[b.r:b.w])
    // 	}
    // 	b.r += size
    // 	b.lastByte = int(b.buf[b.r-1])
    // 	b.lastRuneSize = size
    // 	return r, size, nil
    // }

    // // UnreadRune unreads the last rune. If the most recent method called on
    // // the Reader was not a ReadRune, UnreadRune returns an error. (In this
    // // regard it is stricter than UnreadByte, which will unread the last byte
    // // from any read operation.)
    // fn UnreadRune (&mut self) error {
    // 	if b.lastRuneSize < 0 || b.r < b.lastRuneSize {
    // 		return ErrInvalidUnreadRune
    // 	}
    // 	b.r -= b.lastRuneSize
    // 	b.lastByte = -1
    // 	b.lastRuneSize = -1
    // 	return nil
    // }

    /// buffered returns the number of bytes that can be read from the current buffer.
    pub fn buffered(&mut self) -> usize {
        return self.w - self.r;
    }

    // // ReadSlice reads until the first occurrence of delim in the input,
    // // returning a slice pointing at the bytes in the buffer.
    // // The bytes stop being valid at the next read.
    // // If ReadSlice encounters an error before finding a delimiter,
    // // it returns all the data in the buffer and the error itself (often io.EOF).
    // // ReadSlice fails with error ERR_BUFFER_FULL if the buffer fills without a delim.
    // // Because the data returned from ReadSlice will be overwritten
    // // by the next I/O operation, most clients should use
    // // ReadBytes or ReadString instead.
    // // ReadSlice returns err.is_some() if and only if line does not end in delim.
    // fn ReadSlice(&mut self, delim byte) (line [u8], err error) {
    // 	s := 0 // search start index
    // 	for {
    // 		// Search buffer.
    // 		if i := bytes.IndexByte(b.buf[b.r+s:b.w], delim); i >= 0 {
    // 			i += s
    // 			line = b.buf[b.r : b.r+i+1]
    // 			b.r += i + 1
    // 			break
    // 		}

    // 		// Pending error?
    // 		if b.err.is_some() {
    // 			line = b.buf[b.r:b.w]
    // 			b.r = b.w
    // 			err = b.read_err()
    // 			break
    // 		}

    // 		// Buffer full?
    // 		if b.buffered() >= b.buf.len() {
    // 			b.r = b.w
    // 			line = b.buf
    // 			err = ERR_BUFFER_FULL
    // 			break
    // 		}

    // 		s = b.w - b.r // do not rescan area we scanned before

    // 		b.fill() // buffer is not full
    // 	}

    // 	// Handle last byte, if any.
    // 	if i := len(line) - 1; i >= 0 {
    // 		b.lastByte = int(line[i])
    // 		b.lastRuneSize = -1
    // 	}

    // 	return
    // }

    // // ReadLine is a low-level line-reading primitive. Most callers should use
    // // ReadBytes('\n') or ReadString('\n') instead or use a Scanner.
    // //
    // // ReadLine tries to return a single line, not including the end-of-line bytes.
    // // If the line was too long for the buffer then isPrefix is set and the
    // // beginning of the line is returned. The rest of the line will be returned
    // // from future calls. isPrefix will be false when returning the last fragment
    // // of the line. The returned buffer is only valid until the next call to
    // // ReadLine. ReadLine either returns a non-nil line or it returns an error,
    // // never both.
    // //
    // // The text returned from ReadLine does not include the line end ("\r\n" or "\n").
    // // No indication or error is given if the input ends without a final line end.
    // // Calling UnreadByte after ReadLine will always unread the last byte read
    // // (possibly a character belonging to the line end) even if that byte is not
    // // part of the line returned by ReadLine.
    // fn ReadLine (&mut self) (line [u8], isPrefix bool, err error) {
    // 	line, err = b.ReadSlice('\n')
    // 	if err == ERR_BUFFER_FULL {
    // 		// Handle the case where "\r\n" straddles the buffer.
    // 		if len(line) > 0 && line[len(line)-1] == '\r' {
    // 			// Put the '\r' back on buf and drop it from line.
    // 			// Let the next call to ReadLine check for "\r\n".
    // 			if b.r == 0 {
    // 				// should be unreachable
    // 				panic("bufio: tried to rewind past start of buffer")
    // 			}
    // 			b.r--
    // 			line = line[..len(line)-1]
    // 		}
    // 		return line, true, nil
    // 	}

    // 	if len(line) == 0 {
    // 		if err.is_some() {
    // 			line = nil
    // 		}
    // 		return
    // 	}
    // 	err = nil

    // 	if line[len(line)-1] == '\n' {
    // 		drop := 1
    // 		if len(line) > 1 && line[len(line)-2] == '\r' {
    // 			drop = 2
    // 		}
    // 		line = line[..len(line)-drop]
    // 	}
    // 	return
    // }

    // // collectFragments reads until the first occurrence of delim in the input. It
    // // returns (slice of full buffers, remaining bytes before delim, total number
    // // of bytes in the combined first two elements, error).
    // // The complete result is equal to
    // // `bytes.Join(append(fullBuffers, finalFragment), nil)`, which has a
    // // length of `totalLen`. The result is structured in this way to allow callers
    // // to minimize allocations and copies.
    // fn collectFragments(&mut self, delim byte) (fullBuffers [][u8], finalFragment [u8], totalLen int, err error) {
    // 	var frag [u8]
    // 	// Use ReadSlice to look for delim, accumulating full buffers.
    // 	for {
    // 		var e error
    // 		frag, e = b.ReadSlice(delim)
    // 		if e == nil { // got final fragment
    // 			break
    // 		}
    // 		if e != ERR_BUFFER_FULL { // unexpected error
    // 			err = e
    // 			break
    // 		}

    // 		// Make a copy of the buffer.
    // 		buf := bytes.Clone(frag)
    // 		fullBuffers = append(fullBuffers, buf)
    // 		totalLen += len(buf)
    // 	}

    // 	totalLen += len(frag)
    // 	return fullBuffers, frag, totalLen, err
    // }

    // // ReadBytes reads until the first occurrence of delim in the input,
    // // returning a slice containing the data up to and including the delimiter.
    // // If ReadBytes encounters an error before finding a delimiter,
    // // it returns the data read before the error and the error itself (often io.EOF).
    // // ReadBytes returns err.is_some() if and only if the returned data does not end in
    // // delim.
    // // For simple uses, a Scanner may be more convenient.
    // fn ReadBytes(&mut self, delim byte) ([u8], error) {
    // 	full, frag, n, err := b.collectFragments(delim)
    // 	// Allocate new buffer to hold the full pieces and the fragment.
    // 	buf := make([u8], n)
    // 	n = 0
    // 	// Copy full pieces and fragment in.
    // 	for i := range full {
    // 		n += copy(buf[n..], full[i])
    // 	}
    // 	copy(buf[n..], frag)
    // 	return buf, err
    // }

    // // ReadString reads until the first occurrence of delim in the input,
    // // returning a string containing the data up to and including the delimiter.
    // // If ReadString encounters an error before finding a delimiter,
    // // it returns the data read before the error and the error itself (often io.EOF).
    // // ReadString returns err.is_some() if and only if the returned data does not end in
    // // delim.
    // // For simple uses, a Scanner may be more convenient.
    // fn ReadString(&mut self, delim byte) (string, error) {
    // 	full, frag, n, err := b.collectFragments(delim)
    // 	// Allocate new buffer to hold the full pieces and the fragment.
    // 	var buf strings.Builder
    // 	buf.Grow(n)
    // 	// Copy full pieces and fragment in.
    // 	for _, fb := range full {
    // 		buf.Write(fb)
    // 	}
    // 	buf.Write(frag)
    // 	return buf.String(), err
    // }

    // // WriteTo implements io.WriterTo.
    // // This may make multiple calls to the Read method of the underlying Reader.
    // // If the underlying reader supports the WriteTo method,
    // // this calls the underlying WriteTo without buffering.
    // fn WriteTo(&mut self, w ggio::Writer) (n int64, err error) {
    // 	b.lastByte = -1
    // 	b.lastRuneSize = -1

    // 	n, err = b.writeBuf(w)
    // 	if err.is_some() {
    // 		return
    // 	}

    // 	if r, ok := b.rd.(io.WriterTo); ok {
    // 		m, err := r.WriteTo(w)
    // 		n += m
    // 		return n, err
    // 	}

    // 	if w, ok := w.(io.ReaderFrom); ok {
    // 		m, err := w.ReadFrom(b.rd)
    // 		n += m
    // 		return n, err
    // 	}

    // 	if b.w-b.r < b.buf.len() {
    // 		b.fill() // buffer not full
    // 	}

    // 	for b.r < b.w {
    // 		// b.r < b.w => buffer is not empty
    // 		m, err := b.writeBuf(w)
    // 		n += m
    // 		if err.is_some() {
    // 			return n, err
    // 		}
    // 		b.fill() // buffer is empty
    // 	}

    // 	if b.err == io.EOF {
    // 		b.err = nil
    // 	}

    // 	return n, b.read_err()
    // }

    // var errNegativeWrite = errors.New("bufio: writer returned negative count from Write")

    // // writeBuf writes the Reader's buffer to the writer.
    // fn writeBuf(&mut self, w ggio::Writer) (int64, error) {
    // 	n, err := w.write(b.buf[b.r:b.w])
    // 	if n < 0 {
    // 		panic(errNegativeWrite)
    // 	}
    // 	b.r += n
    // 	return int64(n), err
    // }
}

// // buffered output

// // Writer implements buffering for an ggio::Writer object.
// // If an error occurs writing to a Writer, no more data will be
// // accepted and all subsequent writes, and Flush, will return the error.
// // After all data has been written, the client should call the
// // Flush method to guarantee all data has been forwarded to
// // the underlying ggio::Writer.
// type Writer struct {
// 	err error
// 	buf [u8]
// 	n   int
// 	wr  ggio::Writer
// }

// // NewWriterSize returns a new Writer whose buffer has at least the specified
// // size. If the argument ggio::Writer is already a Writer with large enough
// // size, it returns the underlying Writer.
// fn NewWriterSize(w: &mut dyn ggio::Writer, size int) -> Writer {
// 	// Is it already a Writer?
// 	b, ok := w.(*Writer)
// 	if ok && b.buf.len() >= size {
// 		return b
// 	}
// 	if size <= 0 {
// 		size = defaultBufSize
// 	}
// 	return &Writer{
// 		buf: make([u8], size),
// 		wr:  w,
// 	}
// }

// // new_writer returns a new Writer whose buffer has the default size.
// // If the argument ggio::Writer is already a Writer with large enough buffer size,
// // it returns the underlying Writer.
// fn new_writer(w: &mut dyn ggio::Writer) -> Writer {
// 	return NewWriterSize(w, defaultBufSize)
// }

// // size returns the size of the underlying buffer in bytes.
// fn (b *Writer) size() int { return b.buf.len() }

// // Reset discards any unflushed buffered data, clears any error, and
// // resets b to write its output to w.
// // Calling Reset on the zero value of Writer initializes the internal buffer
// // to the default size.
// // Calling w.reset(w) (that is, resetting a Writer to itself) does nothing.
// fn (b *Writer) Reset(w: &mut dyn ggio::Writer) {
// 	// If a Writer w is passed to new_writer, new_writer will return w.
// 	// Different layers of code may do that, and then later pass w
// 	// to Reset. Avoid infinite recursion in that case.
// 	if b == w {
// 		return
// 	}
// 	if b.buf == nil {
// 		b.buf = make([u8], defaultBufSize)
// 	}
// 	b.err = nil
// 	b.n = 0
// 	b.wr = w
// }

// // Flush writes any buffered data to the underlying ggio::Writer.
// fn (b *Writer) Flush() error {
// 	if b.err.is_some() {
// 		return b.err
// 	}
// 	if b.n == 0 {
// 		return nil
// 	}
// 	n, err := b.wr.Write(b.buf[0:b.n])
// 	if n < b.n && err.is_none() {
// 		err = io.ErrShortWrite
// 	}
// 	if err.is_some() {
// 		if n > 0 && n < b.n {
// 			copy(b.buf[0:b.n-n], b.buf[n:b.n])
// 		}
// 		b.n -= n
// 		b.err = err
// 		return err
// 	}
// 	b.n = 0
// 	return nil
// }

// // Available returns how many bytes are unused in the buffer.
// fn (b *Writer) Available() int { return b.buf.len() - b.n }

// // AvailableBuffer returns an empty buffer with b.Available() capacity.
// // This buffer is intended to be appended to and
// // passed to an immediately succeeding Write call.
// // The buffer is only valid until the next write operation on b.
// fn (b *Writer) AvailableBuffer() [u8] {
// 	return b.buf[b.n..][..0]
// }

// // buffered returns the number of bytes that have been written into the current buffer.
// fn (b *Writer) buffered() int { return b.n }

// // Write writes the contents of p into the buffer.
// // It returns the number of bytes written.
// // If nn < len(p), it also returns an error explaining
// // why the write is short.
// fn (b *Writer) Write(p [u8]) (nn int, err error) {
// 	for len(p) > b.Available() && b.err.is_none() {
// 		var n int
// 		if b.buffered() == 0 {
// 			// Large write, empty buffer.
// 			// Write directly from p to avoid copy.
// 			n, b.err = b.wr.Write(p)
// 		} else {
// 			n = copy(b.buf[b.n..], p)
// 			b.n += n
// 			b.Flush()
// 		}
// 		nn += n
// 		p = p[n..]
// 	}
// 	if b.err.is_some() {
// 		return nn, b.err
// 	}
// 	n := copy(b.buf[b.n..], p)
// 	b.n += n
// 	nn += n
// 	return nn, nil
// }

// // WriteByte writes a single byte.
// fn (b *Writer) WriteByte(c byte) error {
// 	if b.err.is_some() {
// 		return b.err
// 	}
// 	if b.Available() <= 0 && b.Flush() != nil {
// 		return b.err
// 	}
// 	b.buf[b.n] = c
// 	b.n++
// 	return nil
// }

// // WriteRune writes a single Unicode code point, returning
// // the number of bytes written and any error.
// fn (b *Writer) WriteRune(r rune) (size int, err error) {
// 	// Compare as uint32 to correctly handle negative runes.
// 	if uint32(r) < utf8.RuneSelf {
// 		err = b.WriteByte(byte(r))
// 		if err.is_some() {
// 			return 0, err
// 		}
// 		return 1, nil
// 	}
// 	if b.err.is_some() {
// 		return 0, b.err
// 	}
// 	n := b.Available()
// 	if n < utf8.UTFMax {
// 		if b.Flush(); b.err.is_some() {
// 			return 0, b.err
// 		}
// 		n = b.Available()
// 		if n < utf8.UTFMax {
// 			// Can only happen if buffer is silly small.
// 			return b.write_string(string(r))
// 		}
// 	}
// 	size = utf8.EncodeRune(b.buf[b.n..], r)
// 	b.n += size
// 	return size, nil
// }

// // write_string writes a string.
// // It returns the number of bytes written.
// // If the count is less than len(s), it also returns an error explaining
// // why the write is short.
// fn (b *Writer) write_string(s string) (int, error) {
// 	var sw io.StringWriter
// 	tryStringWriter := true

// 	nn := 0
// 	for len(s) > b.Available() && b.err.is_none() {
// 		var n int
// 		if b.buffered() == 0 && sw == nil && tryStringWriter {
// 			// Check at most once whether b.wr is a StringWriter.
// 			sw, tryStringWriter = b.wr.(io.StringWriter)
// 		}
// 		if b.buffered() == 0 && tryStringWriter {
// 			// Large write, empty buffer, and the underlying writer supports
// 			// write_string: forward the write to the underlying StringWriter.
// 			// This avoids an extra copy.
// 			n, b.err = sw.write_string(s)
// 		} else {
// 			n = copy(b.buf[b.n..], s)
// 			b.n += n
// 			b.Flush()
// 		}
// 		nn += n
// 		s = s[n..]
// 	}
// 	if b.err.is_some() {
// 		return nn, b.err
// 	}
// 	n := copy(b.buf[b.n..], s)
// 	b.n += n
// 	nn += n
// 	return nn, nil
// }

// // ReadFrom implements io.ReaderFrom. If the underlying writer
// // supports the ReadFrom method, this calls the underlying ReadFrom.
// // If there is buffered data and an underlying ReadFrom, this fills
// // the buffer and writes it before calling ReadFrom.
// fn (b *Writer) ReadFrom(r std::io::Read) (n int64, err error) {
// 	if b.err.is_some() {
// 		return 0, b.err
// 	}
// 	readerFrom, readerFromOK := b.wr.(io.ReaderFrom)
// 	var m int
// 	for {
// 		if b.Available() == 0 {
// 			if err1 := b.Flush(); err1 != nil {
// 				return n, err1
// 			}
// 		}
// 		if readerFromOK && b.buffered() == 0 {
// 			nn, err := readerFrom.ReadFrom(r)
// 			b.err = err
// 			n += nn
// 			return n, err
// 		}
// 		nr := 0
// 		for nr < MAX_CONSECUTIVE_EMPTY_READS {
// 			m, err = r.Read(b.buf[b.n..])
// 			if m != 0 || err.is_some() {
// 				break
// 			}
// 			nr++
// 		}
// 		if nr == MAX_CONSECUTIVE_EMPTY_READS {
// 			return n, io.ErrNoProgress
// 		}
// 		b.n += m
// 		n += int64(m)
// 		if err.is_some() {
// 			break
// 		}
// 	}
// 	if err == io.EOF {
// 		// If we filled the buffer exactly, flush preemptively.
// 		if b.Available() == 0 {
// 			err = b.Flush()
// 		} else {
// 			err = nil
// 		}
// 	}
// 	return n, err
// }

// // buffered input and output

// // ReadWriter stores pointers to a Reader and a Writer.
// // It implements io.ReadWriter.
// type ReadWriter struct {
// 	*Reader
// 	*Writer
// }

// // NewReadWriter allocates a new ReadWriter that dispatches to r and w.
// fn NewReadWriter(r *Reader, w *Writer) *ReadWriter {
// 	return &ReadWriter{r, w}
// }
