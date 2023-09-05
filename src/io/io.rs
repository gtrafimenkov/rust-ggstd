// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package io provides basic interfaces to I/O primitives.
//! Its primary job is to wrap existing implementations of such primitives,
//! such as those in package os, into shared public interfaces that
//! abstract the functionality, plus some other related primitives.
//!
//! Because these interfaces and primitives wrap lower-level operations with
//! various implementations, unless otherwise informed clients should not
//! assume they are safe for parallel execution.

use crate::errors;

// import (
// 	"errors"
// 	"sync"
// )

/// Seek whence values.
#[derive(Debug, Clone, Copy)]
pub enum Seek {
    Start = 0,   // seek relative to the origin of the file
    Current = 1, // seek relative to the current offset
    End = 2,     // seek relative to the end
}

// /// lossy_copy_to_stdio_err makes a copy of an error as good as possible.
// /// If it is not possibly to copy the error, an error of ErrorKind::Other will be returned instead.
// pub fn lossy_copy_to_stdio_err(&self) -> std::io::Error {
//     match self {
//         Error::StdIo(err) => std::io::Error::new(err.kind(), err.to_string()),
//         _ => std::io::Error::new(std::io::ErrorKind::Other, self.to_string()),
//     }
// }

// ErrNoProgress
pub static ERR_NO_PROGRESS: errors::ErrorStaticString =
    errors::new_static("multiple read calls return no data or error");

// Instead of using Reader, use std::io::Read.  That will make the
// code more idiomatic and make it easier to interoperate with the
// rest of Rust ecosystem.
//
// The main difference between std::io::Read and ggio::Reader is
// that std::io::Read returns 0 to indicate the end of file.
//
// /// Reader is the interface that wraps the basic read method.
// ///
// /// read reads up to len(p) bytes into p. It returns the number of bytes
// /// read (0 <= n <= len(p)) and any error encountered. Even if read
// /// returns n < len(p), it may use all of p as scratch space during the call.
// /// If some data is available but not len(p) bytes, read conventionally
// /// returns what is available instead of waiting for more.
// ///
// /// When read encounters an error or end-of-file condition after
// /// successfully reading n > 0 bytes, it returns the number of
// /// bytes read. It may return the (non-nil) error from the same call
// /// or return the error (and n == 0) from a subsequent call.
// /// An instance of this general case is that a Reader returning
// /// a non-zero number of bytes at the end of the input stream may
// /// return either err == EOF or err == nil. The next read should
// /// return 0, EOF.
// ///
// /// Callers should always process the n > 0 bytes returned before
// /// considering the error err. Doing so correctly handles I/O errors
// /// that happen after reading some bytes and also both of the
// /// allowed EOF behaviors.
// ///
// /// Implementations of read are discouraged from returning a
// /// zero byte count with a nil error, except when len(p) == 0.
// /// Callers should treat a return of 0 and nil as indicating that
// /// nothing happened; in particular it does not indicate EOF.
// ///
// /// Implementations must not retain p.
// pub trait Reader {
//     fn read(&mut self, p: &mut [u8]) -> (usize, Option<Error>);
// }

// Instead of using Writer, use std::io::Write.  That will make the
// code more idiomatic and make it easier to interoperate with the
// rest of Rust ecosystem.
// /// Writer is the interface that wraps the basic write method.
// ///
// /// write writes len(p) bytes from p to the underlying data stream.
// /// It returns the number of bytes written from p (0 <= n <= len(p))
// /// and any error encountered that caused the write to stop early.
// /// write must return a non-nil error if it returns n < len(p).
// /// write must not modify the slice data, even temporarily.
// ///
// /// Implementations must not retain p.
// pub trait Writer {
//     fn write(&mut self, p: &[u8]) -> (usize, Option<Error>);
// }

/// Closer is the interface that wraps the basic Close method.
///
/// The behavior of Close after the first call is undefined.
/// Specific implementations may document their own behavior.
pub trait Closer {
    fn close(&mut self) -> std::io::Result<()>;
}

// // Seeker is the interface that wraps the basic Seek method.
// //
// // Seek sets the offset for the next read or write to offset,
// // interpreted according to whence:
// // SeekStart means relative to the start of the file,
// // SeekCurrent means relative to the current offset, and
// // SeekEnd means relative to the end
// // (for example, offset = -2 specifies the penultimate byte of the file).
// // Seek returns the new offset relative to the start of the
// // file or an error, if any.
// //
// // Seeking to an offset before the start of the file is an error.
// // Seeking to any positive offset may be allowed, but if the new offset exceeds
// // the size of the underlying object the behavior of subsequent I/O operations
// // is implementation-dependent.
// type Seeker interface {
// 	Seek(offset int64, whence isize) (int64, error)
// }

// /// ReadWriter is the interface that groups the basic read and write methods.
// pub trait ReadWriter: Reader + Writer {}

// /// ReadCloser is the interface that groups the basic read and Close methods.
// pub trait ReadCloser: Reader + Closer {}

// // WriteCloser is the interface that groups the basic write and Close methods.
// type WriteCloser interface {
// 	Writer
// 	Closer
// }

// // ReadWriteCloser is the interface that groups the basic read, write and Close methods.
// type ReadWriteCloser interface {
// 	Reader
// 	Writer
// 	Closer
// }

// // ReadSeeker is the interface that groups the basic read and Seek methods.
// type ReadSeeker interface {
// 	Reader
// 	Seeker
// }

// // ReadSeekCloser is the interface that groups the basic read, Seek and Close
// // methods.
// type ReadSeekCloser interface {
// 	Reader
// 	Seeker
// 	Closer
// }

// // WriteSeeker is the interface that groups the basic write and Seek methods.
// type WriteSeeker interface {
// 	Writer
// 	Seeker
// }

// // ReadWriteSeeker is the interface that groups the basic read, write and Seek methods.
// type ReadWriteSeeker interface {
// 	Reader
// 	Writer
// 	Seeker
// }

// // ReaderFrom is the interface that wraps the ReadFrom method.
// //
// // ReadFrom reads data from r until EOF or error.
// // The return value n is the number of bytes read.
// // Any error except EOF encountered during the read is also returned.
// //
// // The copy function uses ReaderFrom if available.
// type ReaderFrom interface {
// 	ReadFrom(r Reader) (n int64, err error)
// }

// /// WriterTo is the interface that wraps the WriteTo method.
// ///
// /// WriteTo writes data to w until there's no more data to write or
// /// when an error occurs. The return value n is the number of bytes
// /// written. Any error encountered during the write is also returned.
// ///
// /// The copy function uses WriterTo if available.
// pub trait WriterTo {
//     fn write_to(&mut self, w: &mut dyn Writer) -> std::io::Result<usize>;
// }

// // ReaderAt is the interface that wraps the basic ReadAt method.
// //
// // ReadAt reads len(p) bytes into p starting at offset off in the
// // underlying input source. It returns the number of bytes
// // read (0 <= n <= len(p)) and any error encountered.
// //
// // When ReadAt returns n < len(p), it returns a non-nil error
// // explaining why more bytes were not returned. In this respect,
// // ReadAt is stricter than read.
// //
// // Even if ReadAt returns n < len(p), it may use all of p as scratch
// // space during the call. If some data is available but not len(p) bytes,
// // ReadAt blocks until either all the data is available or an error occurs.
// // In this respect ReadAt is different from read.
// //
// // If the n = len(p) bytes returned by ReadAt are at the end of the
// // input source, ReadAt may return either err == EOF or err == nil.
// //
// // If ReadAt is reading from an input source with a seek offset,
// // ReadAt should not affect nor be affected by the underlying
// // seek offset.
// //
// // Clients of ReadAt can execute parallel ReadAt calls on the
// // same input source.
// //
// // Implementations must not retain p.
// type ReaderAt interface {
// 	ReadAt(p [u8], off int64) -> IOResult
// }

// // WriterAt is the interface that wraps the basic WriteAt method.
// //
// // WriteAt writes len(p) bytes from p to the underlying data stream
// // at offset off. It returns the number of bytes written from p (0 <= n <= len(p))
// // and any error encountered that caused the write to stop early.
// // WriteAt must return a non-nil error if it returns n < len(p).
// //
// // If WriteAt is writing to a destination with a seek offset,
// // WriteAt should not affect nor be affected by the underlying
// // seek offset.
// //
// // Clients of WriteAt can execute parallel WriteAt calls on the same
// // destination if the ranges do not overlap.
// //
// // Implementations must not retain p.
// type WriterAt interface {
// 	WriteAt(p [u8], off int64) -> IOResult
// }

/// ByteReader is the interface that wraps the read_byte method.
///
/// read_byte reads and returns the next byte from the input or
/// any error encountered. If read_byte returns an error, no input
/// byte was consumed, and the returned byte value is undefined.
///
/// read_byte provides an efficient interface for byte-at-time
/// processing. A Reader that does not implement  ByteReader
/// can be wrapped using bufio.new_reader to add this method.
pub trait ByteReader {
    fn read_byte(&mut self) -> std::io::Result<u8>;
}

// // ByteScanner is the interface that adds the UnreadByte method to the
// // basic read_byte method.
// //
// // UnreadByte causes the next call to read_byte to return the last byte read.
// // If the last operation was not a successful call to read_byte, UnreadByte may
// // return an error, unread the last byte read (or the byte prior to the
// // last-unread byte), or (in implementations that support the Seeker interface)
// // seek to one byte before the current offset.
// type ByteScanner interface {
// 	ByteReader
// 	UnreadByte() error
// }

// // ByteWriter is the interface that wraps the write_byte method.
// type ByteWriter interface {
// 	write_byte(c byte) error
// }

// // RuneReader is the interface that wraps the ReadRune method.
// //
// // ReadRune reads a single encoded Unicode character
// // and returns the rune and its size in bytes. If no character is
// // available, err will be set.
// type RuneReader interface {
// 	ReadRune() (r rune, size isize, err error)
// }

// // RuneScanner is the interface that adds the UnreadRune method to the
// // basic ReadRune method.
// //
// // UnreadRune causes the next call to ReadRune to return the last rune read.
// // If the last operation was not a successful call to ReadRune, UnreadRune may
// // return an error, unread the last rune read (or the rune prior to the
// // last-unread rune), or (in implementations that support the Seeker interface)
// // seek to the start of the rune before the current offset.
// type RuneScanner interface {
// 	RuneReader
// 	UnreadRune() error
// }

// // StringWriter is the interface that wraps the write_string method.
// type StringWriter interface {
// 	write_string(s string) -> (usize, Option<Error>)
// }

/// write_string writes the contents of the string s to w, which accepts a slice of bytes.
/// w.write is called exactly once.
// // If w implements StringWriter, its write_string method is invoked directly.
// // Otherwise, w.write is called exactly once.
pub fn write_string(w: &mut dyn std::io::Write, s: &str) -> std::io::Result<usize> {
    // 	if sw, ok := w.(StringWriter); ok {
    // 		return sw.write_string(s)
    // 	}
    return w.write(s.as_bytes());
}

/// read_at_least reads from r into buf until it has read at least min bytes.
/// It returns the number of bytes copied and an error if fewer bytes were read.
/// If fewer than min bytes were read, read_at_least returns an error.
/// If min is greater than the length of buf, read_at_least returns ErrorKind::InvalidInput.
pub fn read_at_least(
    r: &mut dyn std::io::Read,
    buf: &mut [u8],
    min: usize,
) -> (usize, Option<std::io::Error>) {
    if buf.len() < min {
        return (
            0,
            Some(std::io::Error::from(std::io::ErrorKind::InvalidInput)),
        );
    }
    let mut n = 0;
    while n < min {
        match r.read(&mut buf[n..]) {
            Ok(nr) => {
                if nr == 0 {
                    return (
                        n,
                        Some(std::io::Error::from(std::io::ErrorKind::UnexpectedEof)),
                    );
                }
                n += nr;
            }
            Err(e) => return (n, Some(e)),
        }
    }
    return (n, None);
}

/// read_full reads exactly buf.len() bytes from r into buf.
/// The difference from Rust's std::io::Read::read_exact, it that read_full returns both
/// number of bytes read and a possible error, when std::io::Read::read_exact returns one or another.
pub fn read_full(r: &mut dyn std::io::Read, buf: &mut [u8]) -> (usize, Option<std::io::Error>) {
    // r.read_exact(buf)
    read_at_least(r, buf, buf.len())
}

/// copy_n copies n bytes (or until an error) from src to dst.
/// It returns the number of bytes copied and the earliest
/// error encountered while copying.
/// On return, written == n if and only if err == nil.
///
/// If dst implements the ReaderFrom interface,
/// the copy is implemented using it.
pub fn copy_n(
    dst: &mut dyn std::io::Write,
    src: &mut dyn std::io::Read,
    n: usize,
) -> (usize, Option<std::io::Error>) {
    // -> (written int64, err error) {
    let (written, err) = copy(dst, &mut LimitedReader::new(src, n));
    if written == n as u64 {
        return (n, None);
    }
    if written < n as u64 && err.is_none() {
        // src stopped early; must have been EOF.
        return (
            written as usize,
            Some(std::io::Error::from(std::io::ErrorKind::UnexpectedEof)),
        );
    }
    return (written as usize, err);
}

/// copy copies from src to dst until either EOF is reached
/// on src or an error occurs. It returns the number of bytes
/// copied and the first error encountered while copying, if any.
///
/// A successful copy returns err == nil, not err == EOF.
/// Because copy is defined to read from src until EOF, it does
/// not treat an EOF from read as an error to be reported.
//
// not implemented:
// If src implements the WriterTo interface,
// the copy is implemented by calling src.WriteTo(dst).
// Otherwise, if dst implements the ReaderFrom interface,
// the copy is implemented by calling dst.ReadFrom(src).
pub fn copy(
    dst: &mut dyn std::io::Write,
    src: &mut dyn std::io::Read,
) -> (u64, Option<std::io::Error>) {
    return copy_buffer_int(dst, src, None);
}

/// copy_buffer is identical to copy except that it stages through the
/// provided buffer (if one is required) rather than allocating a
/// temporary one. If buf is nil, one is allocated; otherwise if it has
/// zero length, copy_buffer panics.
//
// not implemented:
// If either src implements WriterTo or dst implements ReaderFrom,
// buf will not be used to perform the copy.
pub fn copy_buffer(
    dst: &mut dyn std::io::Write,
    src: &mut dyn std::io::Read,
    buf: Option<&mut [u8]>,
) -> (u64, Option<std::io::Error>) {
    if buf.is_some() && buf.as_ref().unwrap().len() == 0 {
        panic!("empty buffer in copy_buffer");
    }
    return copy_buffer_int(dst, src, buf);
}

/// copy_buffer_int is the actual implementation of copy and copy_buffer.
/// if buf is nil, one is allocated.
fn copy_buffer_int(
    dst: &mut dyn std::io::Write,
    src: &mut dyn std::io::Read,
    buf: Option<&mut [u8]>,
) -> (u64, Option<std::io::Error>) {
    // If the reader has a WriteTo method, use it to do the copy.
    // Avoids an allocation and a copy.
    // 	if wt, ok := src.(WriterTo); ok {
    // 		return wt.WriteTo(dst)
    // 	}
    // 	// Similarly, if the writer has a ReadFrom method, use it to do the copy.
    // 	if rt, ok := dst.(ReaderFrom); ok {
    // 		return rt.ReadFrom(src)
    // 	}
    fn copy_buf(
        dst: &mut dyn std::io::Write,
        src: &mut dyn std::io::Read,
        buf: &mut [u8],
    ) -> (u64, Option<std::io::Error>) {
        let mut written: u64 = 0;
        loop {
            // println!("   copy_buf, written = {}, buf len= {}", written, buf.len());
            // let (nr, er) = src.read(buf);
            let res = src.read(buf);
            match res {
                Ok(nr) => {
                    if nr > 0 {
                        // let (nw, ew) = dst.write(&buf[0..nr]);
                        match dst.write(&buf[0..nr]) {
                            Ok(nw) => {
                                if nr < nw {
                                    return (written, Some(new_error_invalid_write()));
                                }
                                written += nw as u64;
                                if nr != nw {
                                    return (written, Some(new_error_short_write()));
                                }
                            }
                            Err(ew) => return (written, Some(ew)),
                        }
                    } else {
                        // consider this to be end of file
                        return (written, None);
                    }
                }
                Err(er) => {
                    return (written, Some(er));
                }
            }
        }
    }
    if buf.is_none() {
        let mut buf = vec![0; 32 * 1024];
        return copy_buf(dst, src, &mut buf);
    } else {
        return copy_buf(dst, src, buf.unwrap());
    }
    // 	if buf == nil {
    // 		size := 32 * 1024
    // 		if l, ok := src.(*LimitedReader); ok && int64(size) > l.N {
    // 			if l.N < 1 {
    // 				size = 1
    // 			} else {
    // 				size = isize(l.N)
    // 			}
    // 		}
    // 		buf = make([u8], size)
    // 	}
}

/// limit_reader returns a Reader that reads from r
/// but stops with EOF after n bytes.
/// The underlying implementation is a *LimitedReader.
pub fn limit_reader(r: &mut dyn std::io::Read, n: usize) -> impl std::io::Read + '_ {
    LimitedReader::new(r, n)
}

/// A LimitedReader reads from R but limits the amount of
/// data returned to just N bytes. Each call to read
/// updates N to reflect the new amount remaining.
/// read returns EOF when N <= 0 or when the underlying R returns EOF.
pub struct LimitedReader<'a> {
    r: &'a mut dyn std::io::Read, // underlying reader
    n: usize,                     // max bytes remaining
}

impl<'a> LimitedReader<'a> {
    pub fn new(r: &'a mut dyn std::io::Read, n: usize) -> Self {
        Self { r, n }
    }
}

impl<'a> std::io::Read for LimitedReader<'a> {
    fn read(&mut self, p: &mut [u8]) -> std::io::Result<usize> {
        if self.n == 0 {
            return Ok(0);
        }
        let size = p.len().min(self.n);
        match self.r.read(&mut p[..size]) {
            Ok(n) => {
                self.n -= n;
                return Ok(n);
            }
            Err(err) => Err(err),
        }
    }
}

// // NewSectionReader returns a SectionReader that reads from r
// // starting at offset off and stops with EOF after n bytes.
// fn NewSectionReader(r ReaderAt, off int64, n int64) *SectionReader {
// 	var remaining int64
// 	const maxint64 = 1<<63 - 1
// 	if off <= maxint64-n {
// 		remaining = n + off
// 	} else {
// 		// Overflow, with no way to return error.
// 		// Assume we can read up to an offset of 1<<63 - 1.
// 		remaining = maxint64
// 	}
// 	return &SectionReader{r, off, off, remaining}
// }

// // SectionReader implements read, Seek, and ReadAt on a section
// // of an underlying ReaderAt.
// type SectionReader struct {
// 	r     ReaderAt
// 	base  int64
// 	off   int64
// 	limit int64
// }

// fn (s *SectionReader) read(p : &[u8]) -> IOResult {
// 	if s.off >= s.limit {
// 		return 0, EOF
// 	}
// 	if max := s.limit - s.off; int64(len(p)) > max {
// 		p = p[0:max]
// 	}
// 	n, err = s.r.ReadAt(p, s.off)
// 	s.off += int64(n)
// 	return
// }

// var errWhence = errors.New("Seek: invalid whence")
// var errOffset = errors.New("Seek: invalid offset")

// fn (s *SectionReader) Seek(offset int64, whence isize) (int64, error) {
// 	switch whence {
// 	default:
// 		return 0, errWhence
// 	case SeekStart:
// 		offset += s.base
// 	case SeekCurrent:
// 		offset += s.off
// 	case SeekEnd:
// 		offset += s.limit
// 	}
// 	if offset < s.base {
// 		return 0, errOffset
// 	}
// 	s.off = offset
// 	return offset - s.base, nil
// }

// fn (s *SectionReader) ReadAt(p [u8], off int64) -> IOResult {
// 	if off < 0 || off >= s.limit-s.base {
// 		return 0, EOF
// 	}
// 	off += s.base
// 	if max := s.limit - off; int64(len(p)) > max {
// 		p = p[0:max]
// 		n, err = s.r.ReadAt(p, off)
// 		if err == nil {
// 			err = EOF
// 		}
// 		return n, err
// 	}
// 	return s.r.ReadAt(p, off)
// }

// // Size returns the size of the section in bytes.
// fn (s *SectionReader) Size() int64 { return s.limit - s.base }

// // An OffsetWriter maps writes at offset base to offset base+off in the underlying writer.
// type OffsetWriter struct {
// 	w    WriterAt
// 	base int64 // the original offset
// 	off  int64 // the current offset
// }

// // NewOffsetWriter returns an OffsetWriter that writes to w
// // starting at offset off.
// fn NewOffsetWriter(w WriterAt, off int64) *OffsetWriter {
// 	return &OffsetWriter{w, off, off}
// }

// fn (o *OffsetWriter) write(p : &[u8]) -> IOResult {
// 	n, err = o.w.WriteAt(p, o.off)
// 	o.off += int64(n)
// 	return
// }

// fn (o *OffsetWriter) WriteAt(p [u8], off int64) -> IOResult {
// 	off += o.base
// 	return o.w.WriteAt(p, off)
// }

// fn (o *OffsetWriter) Seek(offset int64, whence isize) (int64, error) {
// 	switch whence {
// 	default:
// 		return 0, errWhence
// 	case SeekStart:
// 		offset += o.base
// 	case SeekCurrent:
// 		offset += o.off
// 	}
// 	if offset < o.base {
// 		return 0, errOffset
// 	}
// 	o.off = offset
// 	return offset - o.base, nil
// }

// // TeeReader returns a Reader that writes to w what it reads from r.
// // All reads from r performed through it are matched with
// // corresponding writes to w. There is no internal buffering -
// // the write must complete before the read completes.
// // Any error encountered while writing is reported as a read error.
// fn TeeReader(r: &mut dyn Reader, w Writer) Reader {
// 	return &teeReader{r, w}
// }

// type teeReader struct {
// 	r Reader
// 	w Writer
// }

// fn (t *teeReader) read(p : &[u8]) -> IOResult {
// 	n, err = t.r.read(p)
// 	if n > 0 {
// 		if n, err := t.w.write(p[..n]); err != nil {
// 			return n, err
// 		}
// 	}
// 	return
// }

/// Discard is a Writer on which all write calls succeed
/// without doing anything.
pub struct Discard {}
// var Discard Writer = discard{}

// type discard struct{}

impl Discard {
    pub fn new() -> Self {
        Self {}
    }
}

impl std::io::Write for Discard {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// // discard implements ReaderFrom as an optimization so copy to
// // io.Discard can avoid doing unnecessary work.
// var _ ReaderFrom = discard{}

// fn (discard) write_string(s string) (isize, error) {
// 	return len(s), nil
// }

// var blackHolePool = sync.Pool{
// 	New: fn() any {
// 		b := make([u8], 8192)
// 		return &b
// 	},
// }

// fn (discard) ReadFrom(r Reader) (n int64, err error) {
// 	bufp := blackHolePool.Get().(*[u8])
// 	readSize := 0
// 	for {
// 		readSize, err = r.read(*bufp)
// 		n += int64(readSize)
// 		if err != nil {
// 			blackHolePool.Put(bufp)
// 			if err == EOF {
// 				return n, nil
// 			}
// 			return
// 		}
// 	}
// }

// // NopCloser returns a ReadCloser with a no-op Close method wrapping
// // the provided Reader r.
// // If r implements WriterTo, the returned ReadCloser will implement WriterTo
// // by forwarding calls to r.
// fn NopCloser(r Reader) ReadCloser {
// 	if _, ok := r.(WriterTo); ok {
// 		return nopCloserWriterTo{r}
// 	}
// 	return nopCloser{r}
// }

// type nopCloser struct {
// 	Reader
// }

// fn (nopCloser) Close() error { return nil }

// type nopCloserWriterTo struct {
// 	Reader
// }

// fn (nopCloserWriterTo) Close() error { return nil }

// fn (c nopCloserWriterTo) WriteTo(w Writer) (n int64, err error) {
// 	return c.Reader.(WriterTo).WriteTo(w)
// }

/// read_all reads from r until an error or EOF and returns the data it read.
/// This is effectively a wrapper around std::io::Read::read_to_end
pub fn read_all(r: &mut dyn std::io::Read) -> (Vec<u8>, Option<std::io::Error>) {
    let mut b = Vec::<u8>::with_capacity(512);
    loop {
        let len = b.len();
        let cap = b.capacity();
        if len == cap {
            // Add more capacity (let append pick how much).
            b.reserve(cap);
        }
        let free_space = b.spare_capacity_mut();
        let free_space = unsafe {
            std::slice::from_raw_parts_mut(free_space.as_mut_ptr() as *mut u8, free_space.len())
        };

        // let (n, err) =
        match r.read(free_space) {
            Ok(n) => {
                if n == 0 {
                    // consider this to be the end of file
                    return (b, None);
                } else {
                    unsafe {
                        b.set_len(len + n);
                    }
                }
            }
            Err(err) => {
                return (b, Some(err));
            }
        }
    }
}

const SHORT_WRITE_ERR_STR: &'static str = "ErrShortWrite: short write";

pub fn new_error_short_write() -> std::io::Error {
    errors::new_stdio_other_error(SHORT_WRITE_ERR_STR.to_string())
}

fn new_error_invalid_write() -> std::io::Error {
    errors::new_stdio_other_error("ErrInvalidWrite: invalid write result".to_string())
}

/// Check if the error was created by new_error_short_write.
pub fn is_short_write_error(err: &std::io::Error) -> bool {
    err.to_string() == SHORT_WRITE_ERR_STR
}
