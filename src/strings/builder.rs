// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2017 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// package strings

// import (
// 	"unicode/utf8"
// 	"unsafe"
// )

/// A Builder is used to efficiently build a string using Write methods.
/// It minimizes memory copying.
// The zero value is ready to use.
// Do not copy a non-zero Builder.
pub struct Builder {
    // buf: std::io::BufWriter,
    buf: Vec<u8>,
    // 	addr *Builder // of receiver, to detect copies by value
    // 	buf  []byte
}

// // noescape hides a pointer from escape analysis. It is the identity function
// // but escape analysis doesn't think the output depends on the input.
// // noescape is inlined and currently compiles down to zero instructions.
// // USE CAREFULLY!
// // This was copied from the runtime; see issues 23382 and 7921.
// //
// //go:nosplit
// //go:nocheckptr
// fn noescape(p unsafe.Pointer) unsafe.Pointer {
// 	x := uintptr(p)
// 	return unsafe.Pointer(x ^ 0)
// }

impl Builder {
    /// new creates a new instance of Builder.
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }
    /// data returns all the accumulated data as bytes.
    pub fn data(&self) -> &[u8] {
        &self.buf
    }

    // fn copyCheck(&self) {
    // 	if b.addr == nil {
    // 		// This hack works around a failing of Go's escape analysis
    // 		// that was causing b to escape and be heap allocated.
    // 		// See issue 23382.
    // 		// TODO: once issue 7921 is fixed, this should be reverted to
    // 		// just "b.addr = b".
    // 		b.addr = (*Builder)(noescape(unsafe.Pointer(b)))
    // 	} else if b.addr != b {
    // 		panic("strings: illegal use of non-zero Builder copied by value")
    // 	}
    // }

    /// string returns the accumulated string.
    pub fn string(&self) -> String {
        String::from_utf8_lossy(&self.buf).to_string()
    }

    // // Len returns the number of accumulated bytes; b.Len() == len(b.String()).
    // fn Len(&self) int { return len(b.buf) }

    // // Cap returns the capacity of the builder's underlying byte slice. It is the
    // // total space allocated for the string being built and includes any bytes
    // // already written.
    // fn Cap(&self) int { return cap(b.buf) }

    // // Reset resets the Builder to be empty.
    // fn Reset(&self) {
    // 	b.addr = nil
    // 	b.buf = nil
    // }

    // // grow copies the buffer to a new, larger buffer so that there are at least n
    // // bytes of capacity beyond len(b.buf).
    // fn grow(&selfn int) {
    // 	buf := make([]byte, len(b.buf), 2*cap(b.buf)+n)
    // 	copy(buf, b.buf)
    // 	b.buf = buf
    // }

    // // Grow grows b's capacity, if necessary, to guarantee space for
    // // another n bytes. After Grow(n), at least n bytes can be written to b
    // // without another allocation. If n is negative, Grow panics.
    // fn Grow(&selfn int) {
    // 	b.copyCheck()
    // 	if n < 0 {
    // 		panic("strings.Builder.Grow: negative count")
    // 	}
    // 	if cap(b.buf)-len(b.buf) < n {
    // 		b.grow(n)
    // 	}
    // }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::io::Write for Builder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.write(buf)
    }

    // // Write appends the contents of p to b's buffer.
    // // Write always returns len(p), nil.
    // fn Write(&selfp []byte) (int, error) {
    // 	b.copyCheck()
    // 	b.buf = append(b.buf, p...)
    // 	return len(p), nil
    // }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buf.flush()
    }
}

impl Builder {
    // // WriteByte appends the byte c to b's buffer.
    // // The returned error is always nil.
    // fn WriteByte(&selfc byte) error {
    // 	b.copyCheck()
    // 	b.buf = append(b.buf, c)
    // 	return nil
    // }

    // // WriteRune appends the UTF-8 encoding of Unicode code point r to b's buffer.
    // // It returns the length of r and a nil error.
    // fn WriteRune(&selfr rune) (int, error) {
    // 	b.copyCheck()
    // 	n := len(b.buf)
    // 	b.buf = utf8.AppendRune(b.buf, r)
    // 	return len(b.buf) - n, nil
    // }

    // // WriteString appends the contents of s to b's buffer.
    // // It returns the length of s and a nil error.
    // fn WriteString(&selfs string) (int, error) {
    // 	b.copyCheck()
    // 	b.buf = append(b.buf, s...)
    // 	return len(s), nil
    // }
}
