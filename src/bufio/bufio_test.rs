// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io::Write;

use super::bufio::{self, new_writer, new_writer_size};
use crate::bytes;
use crate::io as ggio;

// import (
// 	. "bufio"
// 	"bytes"
// 	"errors"
// 	"fmt"
// 	"io"
// 	"math/rand"
// 	"strconv"
// 	"strings"
// 	"testing"
// 	"testing/iotest"
// 	"time"
// 	"unicode/utf8"
// )

// // Reads from a reader and rot13s the result.
// type rot13Reader struct {
// 	r io.Reader
// }

// fn newRot13Reader(r io.Reader) *rot13Reader {
// 	r13 := new(rot13Reader)
// 	r13.r = r
// 	return r13
// }

// fn (r13 *rot13Reader) Read(p [u8]) (isize, error) {
// 	n, err := r13.r.Read(p)
// 	for i := 0; i < n; i += 1 {
// 		c := p[i] | 0x20 // lowercase byte
// 		if 'a' <= c && c <= 'm' {
// 			p[i] += 13
// 		} else if 'n' <= c && c <= 'z' {
// 			p[i] -= 13
// 		}
// 	}
// 	return n, err
// }

// // Call ReadByte to accumulate the text of a file
// fn readBytes(buf *Reader) string {
// 	var b [1000]byte
// 	nb := 0
// 	for {
// 		c, err := buf.ReadByte()
// 		if err == io.EOF {
// 			break
// 		}
// 		if err == nil {
// 			b[nb] = c
// 			nb++
// 		} else if err != iotest.ErrTimeout {
// 			panic("Data: " + err.Error())
// 		}
// 	}
// 	return string(b[0..nb])
// }

// fn TestReaderSimple() {
// 	data := "hello world"
// 	b := new_reader(strings.new_reader(data))
// 	if s := readBytes(b); s != "hello world" {
// 		t.Errorf("simple hello world test failed: got %q", s)
// 	}

// 	b = new_reader(newRot13Reader(strings.new_reader(data)))
// 	if s := readBytes(b); s != "uryyb jbeyq" {
// 		t.Errorf("rot13 hello world test failed: got %q", s)
// 	}
// }

// type readMaker struct {
// 	name string
// 	fn   fn(io.Reader) io.Reader
// }

// var readMakers = []readMaker{
// 	{"full", fn(r io.Reader) io.Reader { return r }},
// 	{"byte", iotest.OneByteReader},
// 	{"half", iotest.HalfReader},
// 	{"data+err", iotest.DataErrReader},
// 	{"timeout", iotest.TimeoutReader},
// }

// // Call ReadString (which ends up calling everything else)
// // to accumulate the text of a file.
// fn readLines(b *Reader) string {
// 	s := ""
// 	for {
// 		s1, err := b.ReadString('\n')
// 		if err == io.EOF {
// 			break
// 		}
// 		if err != nil && err != iotest.ErrTimeout {
// 			panic("GetLines: " + err.Error())
// 		}
// 		s += s1
// 	}
// 	return s
// }

// // Call Read to accumulate the text of a file
// fn reads(buf *Reader, m isize) string {
// 	var b [1000]byte
// 	nb := 0
// 	for {
// 		n, err := buf.Read(b[nb : nb+m])
// 		nb += n
// 		if err == io.EOF {
// 			break
// 		}
// 	}
// 	return string(b[0..nb])
// }

// type bufReader struct {
// 	name string
// 	fn   fn(*Reader) string
// }

// var bufreaders = []bufReader{
// 	{"1", fn(b *Reader) string { return reads(b, 1) }},
// 	{"2", fn(b *Reader) string { return reads(b, 2) }},
// 	{"3", fn(b *Reader) string { return reads(b, 3) }},
// 	{"4", fn(b *Reader) string { return reads(b, 4) }},
// 	{"5", fn(b *Reader) string { return reads(b, 5) }},
// 	{"7", fn(b *Reader) string { return reads(b, 7) }},
// 	{"bytes", readBytes},
// 	{"lines", readLines},
// }

const MIN_READ_BUFFER_SIZE: usize = 16;

const BUFSIZES: [usize; 10] = [0, MIN_READ_BUFFER_SIZE, 23, 32, 46, 64, 93, 128, 1024, 4096];

// fn TestReader() {
// 	var texts [31]string
// 	str := ""
// 	all := ""
// 	for i := 0; i < len(texts)-1; i += 1 {
// 		texts[i] = str + "\n"
// 		all += texts[i]
// 		str += string(rune(i%26 + 'a'))
// 	}
// 	texts[len(texts)-1] = all

// 	for h := 0; h < len(texts); h++ {
// 		text := texts[h]
// 		for i := 0; i < len(readMakers); i += 1 {
// 			for j := 0; j < len(bufreaders); j++ {
// 				for k := 0; k < BUFSIZES.len(); k++ {
// 					readmaker := readMakers[i]
// 					bufreader := bufreaders[j]
// 					bufsize := BUFSIZES[k]
// 					read := readmaker.fn(strings.new_reader(text))
// 					buf := new_reader_size(read, bufsize)
// 					s := bufreader.fn(buf)
// 					if s != text {
// 						t.Errorf("reader={} fn={} bufsize={} want=%q got=%q",
// 							readmaker.name, bufreader.name, bufsize, text, s)
// 					}
// 				}
// 			}
// 		}
// 	}
// }

// type zeroReader struct{}

// fn (zeroReader) Read(p [u8]) (isize, error) {
// 	return 0, nil
// }

// fn TestZeroReader() {
// 	var z zeroReader
// 	r := new_reader(z)

// 	c := make(chan error)
// 	go fn() {
// 		_, err := r.ReadByte()
// 		c <- err
// 	}()

// 	select {
// 	case err := <-c:
// 		if err == nil {
// 			t.Error("error expected")
// 		} else if err != io.ErrNoProgress {
// 			t.Error("unexpected error:", err)
// 		}
// 	case <-time.After(time.Second):
// 		t.Error("test timed out (endless loop in ReadByte?)")
// 	}
// }

// // A StringReader delivers its data one string segment at a time via Read.
// type StringReader struct {
// 	data []string
// 	step isize
// }

// fn (r *StringReader) Read(p [u8]) (n isize, err error) {
// 	if r.step < len(r.data) {
// 		s := r.data[r.step]
// 		n = copy(p, s)
// 		r.step++
// 	} else {
// 		err = io.EOF
// 	}
// 	return
// }

// fn readRuneSegments(, segments []string) {
// 	got := ""
// 	want := strings.Join(segments, "")
// 	r := new_reader(&StringReader{data: segments})
// 	for {
// 		r, _, err := r.ReadRune()
// 		if err != nil {
// 			if err != io.EOF {
// 				return
// 			}
// 			break
// 		}
// 		got += string(r)
// 	}
// 	if got != want {
// 		t.Errorf("segments={} got={} want={}", segments, got, want)
// 	}
// }

// var segmentList = [][]string{
// 	{},
// 	{""},
// 	{"日", "本語"},
// 	{"\u65e5", "\u672c", "\u8a9e"},
// 	{"\U000065e5", "\U0000672c", "\U00008a9e"},
// 	{"\xe6", "\x97\xa5\xe6", "\x9c\xac\xe8\xaa\x9e"},
// 	{"Hello", ", ", "World", "!"},
// 	{"Hello", ", ", "", "World", "!"},
// }

// fn TestReadRune() {
// 	for _, s := range segmentList {
// 		readRuneSegments(t, s)
// 	}
// }

// fn TestUnreadRune() {
// 	segments := []string{"Hello, world:", "日本語"}
// 	r := new_reader(&StringReader{data: segments})
// 	got := ""
// 	want := strings.Join(segments, "")
// 	// Normal execution.
// 	for {
// 		r1, _, err := r.ReadRune()
// 		if err != nil {
// 			if err != io.EOF {
// 				t.Error("unexpected error on ReadRune:", err)
// 			}
// 			break
// 		}
// 		got += string(r1)
// 		// Put it back and read it again.
// 		if err = r.UnreadRune(); err != nil {
// 			t.Fatal("unexpected error on UnreadRune:", err)
// 		}
// 		r2, _, err := r.ReadRune()
// 		if err != nil {
// 			t.Fatal("unexpected error reading after unreading:", err)
// 		}
// 		if r1 != r2 {
// 			t.Fatalf("incorrect rune after unread: got %c, want %c", r1, r2)
// 		}
// 	}
// 	if got != want {
// 		t.Errorf("got %q, want %q", got, want)
// 	}
// }

// fn TestNoUnreadRuneAfterPeek() {
// 	br := new_reader(strings.new_reader("example"))
// 	br.ReadRune()
// 	br.Peek(1)
// 	if err := br.UnreadRune(); err == nil {
// 		t.Error("UnreadRune didn't fail after Peek")
// 	}
// }

// fn TestNoUnreadByteAfterPeek() {
// 	br := new_reader(strings.new_reader("example"))
// 	br.ReadByte()
// 	br.Peek(1)
// 	if err := br.UnreadByte(); err == nil {
// 		t.Error("UnreadByte didn't fail after Peek")
// 	}
// }

// fn TestNoUnreadRuneAfterDiscard() {
// 	br := new_reader(strings.new_reader("example"))
// 	br.ReadRune()
// 	br.Discard(1)
// 	if err := br.UnreadRune(); err == nil {
// 		t.Error("UnreadRune didn't fail after Discard")
// 	}
// }

// fn TestNoUnreadByteAfterDiscard() {
// 	br := new_reader(strings.new_reader("example"))
// 	br.ReadByte()
// 	br.Discard(1)
// 	if err := br.UnreadByte(); err == nil {
// 		t.Error("UnreadByte didn't fail after Discard")
// 	}
// }

// fn TestNoUnreadRuneAfterWriteTo() {
// 	br := new_reader(strings.new_reader("example"))
// 	br.WriteTo(ggio::Discard::new())
// 	if err := br.UnreadRune(); err == nil {
// 		t.Error("UnreadRune didn't fail after WriteTo")
// 	}
// }

// fn TestNoUnreadByteAfterWriteTo() {
// 	br := new_reader(strings.new_reader("example"))
// 	br.WriteTo(ggio::Discard::new())
// 	if err := br.UnreadByte(); err == nil {
// 		t.Error("UnreadByte didn't fail after WriteTo")
// 	}
// }

// fn TestUnreadByte() {
// 	segments := []string{"Hello, ", "world"}
// 	r := new_reader(&StringReader{data: segments})
// 	got := ""
// 	want := strings.Join(segments, "")
// 	// Normal execution.
// 	for {
// 		b1, err := r.ReadByte()
// 		if err != nil {
// 			if err != io.EOF {
// 				t.Error("unexpected error on ReadByte:", err)
// 			}
// 			break
// 		}
// 		got += string(b1)
// 		// Put it back and read it again.
// 		if err = r.UnreadByte(); err != nil {
// 			t.Fatal("unexpected error on UnreadByte:", err)
// 		}
// 		b2, err := r.ReadByte()
// 		if err != nil {
// 			t.Fatal("unexpected error reading after unreading:", err)
// 		}
// 		if b1 != b2 {
// 			t.Fatalf("incorrect byte after unread: got %q, want %q", b1, b2)
// 		}
// 	}
// 	if got != want {
// 		t.Errorf("got %q, want %q", got, want)
// 	}
// }

// fn TestUnreadByteMultiple() {
// 	segments := []string{"Hello, ", "world"}
// 	data := strings.Join(segments, "")
// 	for n := 0; n <= len(data); n++ {
// 		r := new_reader(&StringReader{data: segments})
// 		// Read n bytes.
// 		for i := 0; i < n; i += 1 {
// 			b, err := r.ReadByte()
// 			if err != nil {
// 				t.Fatalf("n = {}: unexpected error on ReadByte: {}", n, err)
// 			}
// 			if b != data[i] {
// 				t.Fatalf("n = {}: incorrect byte returned from ReadByte: got %q, want %q", n, b, data[i])
// 			}
// 		}
// 		// Unread one byte if there is one.
// 		if n > 0 {
// 			if err := r.UnreadByte(); err != nil {
// 				t.Errorf("n = {}: unexpected error on UnreadByte: {}", n, err)
// 			}
// 		}
// 		// Test that we cannot unread any further.
// 		if err := r.UnreadByte(); err == nil {
// 			t.Errorf("n = {}: expected error on UnreadByte", n)
// 		}
// 	}
// }

// fn TestUnreadByteOthers() {
// 	// A list of readers to use in conjunction with UnreadByte.
// 	var readers = []fn(*Reader, byte) ([u8], error){
// 		(*Reader).ReadBytes,
// 		(*Reader).ReadSlice,
// 		fn(r *Reader, delim byte) ([u8], error) {
// 			data, err := r.ReadString(delim)
// 			return [u8](data), err
// 		},
// 		// ReadLine doesn't fit the data/pattern easily
// 		// so we leave it out. It should be covered via
// 		// the ReadSlice test since ReadLine simply calls
// 		// ReadSlice, and it's that function that handles
// 		// the last byte.
// 	}

// 	// Try all readers with UnreadByte.
// 	for rno, read := range readers {
// 		// Some input data that is longer than the minimum reader buffer size.
// 		const n = 10
// 		let mut buf = bytes::Buffer::new();
// 		for i := 0; i < n; i += 1 {
// 			buf.write_string("abcdefg")
// 		}

// 		r := new_reader_size(&buf, MIN_READ_BUFFER_SIZE)
// 		readTo := fn(delim byte, want string) {
// 			data, err := read(r, delim)
// 			if err != nil {
// 				t.Fatalf("#{}: unexpected error reading to %c: {}", rno, delim, err)
// 			}
// 			if got := string(data); got != want {
// 				t.Fatalf("#{}: got %q, want %q", rno, got, want)
// 			}
// 		}

// 		// Read the data with occasional UnreadByte calls.
// 		for i := 0; i < n; i += 1 {
// 			readTo('d', "abcd")
// 			for j := 0; j < 3; j++ {
// 				if err := r.UnreadByte(); err != nil {
// 					t.Fatalf("#{}: unexpected error on UnreadByte: {}", rno, err)
// 				}
// 				readTo('d', "d")
// 			}
// 			readTo('g', "efg")
// 		}

// 		// All data should have been read.
// 		_, err := r.ReadByte()
// 		if err != io.EOF {
// 			t.Errorf("#{}: got error {}; want EOF", rno, err)
// 		}
// 	}
// }

// // Test that UnreadRune fails if the preceding operation was not a ReadRune.
// fn TestUnreadRuneError() {
// 	buf := make([u8], 3) // All runes in this test are 3 bytes long
// 	r := new_reader(&StringReader{data: []string{"日本語日本語日本語"}})
// 	if r.UnreadRune() == nil {
// 		t.Error("expected error on UnreadRune from fresh buffer")
// 	}
// 	_, _, err := r.ReadRune()
// 	if err != nil {
// 		t.Error("unexpected error on ReadRune (1):", err)
// 	}
// 	if err = r.UnreadRune(); err != nil {
// 		t.Error("unexpected error on UnreadRune (1):", err)
// 	}
// 	if r.UnreadRune() == nil {
// 		t.Error("expected error after UnreadRune (1)")
// 	}
// 	// Test error after Read.
// 	_, _, err = r.ReadRune() // reset state
// 	if err != nil {
// 		t.Error("unexpected error on ReadRune (2):", err)
// 	}
// 	_, err = r.Read(buf)
// 	if err != nil {
// 		t.Error("unexpected error on Read (2):", err)
// 	}
// 	if r.UnreadRune() == nil {
// 		t.Error("expected error after Read (2)")
// 	}
// 	// Test error after ReadByte.
// 	_, _, err = r.ReadRune() // reset state
// 	if err != nil {
// 		t.Error("unexpected error on ReadRune (2):", err)
// 	}
// 	for range buf {
// 		_, err = r.ReadByte()
// 		if err != nil {
// 			t.Error("unexpected error on ReadByte (2):", err)
// 		}
// 	}
// 	if r.UnreadRune() == nil {
// 		t.Error("expected error after ReadByte")
// 	}
// 	// Test error after UnreadByte.
// 	_, _, err = r.ReadRune() // reset state
// 	if err != nil {
// 		t.Error("unexpected error on ReadRune (3):", err)
// 	}
// 	_, err = r.ReadByte()
// 	if err != nil {
// 		t.Error("unexpected error on ReadByte (3):", err)
// 	}
// 	err = r.UnreadByte()
// 	if err != nil {
// 		t.Error("unexpected error on UnreadByte (3):", err)
// 	}
// 	if r.UnreadRune() == nil {
// 		t.Error("expected error after UnreadByte (3)")
// 	}
// 	// Test error after ReadSlice.
// 	_, _, err = r.ReadRune() // reset state
// 	if err != nil {
// 		t.Error("unexpected error on ReadRune (4):", err)
// 	}
// 	_, err = r.ReadSlice(0)
// 	if err != io.EOF {
// 		t.Error("unexpected error on ReadSlice (4):", err)
// 	}
// 	if r.UnreadRune() == nil {
// 		t.Error("expected error after ReadSlice (4)")
// 	}
// }

// fn TestUnreadRuneAtEOF() {
// 	// UnreadRune/ReadRune should error at EOF (was a bug; used to panic)
// 	r := new_reader(strings.new_reader("x"))
// 	r.ReadRune()
// 	r.ReadRune()
// 	r.UnreadRune()
// 	_, _, err := r.ReadRune()
// 	if err == nil {
// 		t.Error("expected error at EOF")
// 	} else if err != io.EOF {
// 		t.Error("expected EOF; got", err)
// 	}
// }

// fn TestReadWriteRune() {
// 	const NRune = 1000
// 	byteBuf := new(bytes::Buffer::new())
// 	w := new_writer(byteBuf)
// 	// Write the runes out using WriteRune
// 	buf := make([u8], utf8.UTFMAX)
// 	for r := rune(0); r < NRune; r++ {
// 		size := utf8.encode_rune(buf, r)
// 		nbytes, err := w.WriteRune(r)
// 		if err != nil {
// 			t.Fatalf("WriteRune(0x%x) error: {}", r, err)
// 		}
// 		if nbytes != size {
// 			t.Fatalf("WriteRune(0x%x) expected {}, got {}", r, size, nbytes)
// 		}
// 	}
// 	w.flush()

// 	r := new_reader(byteBuf)
// 	// Read them back with ReadRune
// 	for r1 := rune(0); r1 < NRune; r1++ {
// 		size := utf8.encode_rune(buf, r1)
// 		nr, nbytes, err := r.ReadRune()
// 		if nr != r1 || nbytes != size || err != nil {
// 			t.Fatalf("ReadRune(0x%x) got 0x%x,{} not 0x%x,{} (err={})", r1, nr, nbytes, r1, size, err)
// 		}
// 	}
// }

// fn TestWriteInvalidRune() {
// 	// Invalid runes, including negative ones, should be written as the
// 	// replacement character.
// 	for _, r := range []rune{-1, utf8.MAX_RUNE + 1} {
// 		var buf strings.Builder
// 		w := new_writer(&buf)
// 		w.WriteRune(r)
// 		w.flush()
// 		if s := buf.String(); s != "\uFFFD" {
// 			t.Errorf("WriteRune({}) wrote %q, not replacement character", r, s)
// 		}
// 	}
// }

// fn TestReadStringAllocs() {
// 	r := strings.new_reader("       foo       foo        42        42        42        42        42        42        42        42       4.2       4.2       4.2       4.2\n")
// 	buf := new_reader(r)
// 	allocs := testing.AllocsPerRun(100, fn() {
// 		r.Seek(0, ggio::Seek::Start)
// 		buf.Reset(r)

// 		_, err := buf.ReadString('\n')
// 		if err != nil {
// 			t.Fatal(err)
// 		}
// 	})
// 	if allocs != 1 {
// 		t.Errorf("Unexpected number of allocations, got %f, want 1", allocs)
// 	}
// }

#[test]
fn test_writer() {
    let mut data = [0_u8; 8192];

    for i in 0..data.len() {
        data[i] = b' ' + (i % (b'~' - b' ') as usize) as u8;
    }
    let mut w = bytes::Buffer::new();
    for i in 0..BUFSIZES.len() {
        for j in 0..BUFSIZES.len() {
            let nwrite = BUFSIZES[i];
            let bs = BUFSIZES[j];

            // Write nwrite bytes using buffer size bs.
            // Check that the right amount makes it out
            // and that the data is correct.

            w.reset();
            let mut buf = new_writer_size(&mut w, bs);
            let context = format!("nwrite={} bufsize={}", nwrite, bs);
            let res = buf.write(&data[0..nwrite]);
            assert!(
                res.as_ref().is_ok_and(|x| *x == nwrite),
                "{}: buf.write {} = {:?}",
                context,
                nwrite,
                res
            );
            let res = buf.flush();
            assert!(res.as_ref().is_ok(), "{}: buf.flush = {:?}", context, res);

            let written = w.bytes();
            assert!(
                written.len() == nwrite,
                "{}: {} bytes written",
                context,
                written.len()
            );
            for l in 0..written.len() {
                assert!(
                    written[l] == data[l],
                    "wrong bytes written, want={:?}, have={:?}",
                    &data[0..written.len()],
                    written
                );
            }
        }
    }
}

// fn TestWriterAppend() {
// 	got := new(bytes::Buffer::new())
// 	var want [u8]
// 	rn := rand.New(rand.NewSource(0))
// 	w := new_writer_size(got, 64)
// 	for i := 0; i < 100; i += 1 {
// 		// Obtain a buffer to append to.
// 		b := w.AvailableBuffer()
// 		if w.Available() != b.capacity() {
// 			t.Fatalf("Available() = {}, want {}", w.Available(), b.capacity())
// 		}

// 		// While not recommended, it is valid to append to a shifted buffer.
// 		// This forces Write to copy the input.
// 		if rn.Intn(8) == 0 && b.capacity() > 0 {
// 			b = b[1:1:b.capacity()]
// 		}

// 		// Append a random integer of varying width.
// 		n := int64(rn.Intn(1 << rn.Intn(30)))
// 		want = append(strconv.AppendInt(want, n, 10), ' ')
// 		b = append(strconv.AppendInt(b, n, 10), ' ')
// 		w.write(b)
// 	}
// 	w.flush()

// 	if !bytes::equal(got.bytes(), want) {
// 		t.Errorf("output mismatch:\ngot  {}\nwant {}", got.bytes(), want)
// 	}
// }

// Check that write errors are returned properly.

struct ShortWriter {
    /// how much data this writer can take
    capacity: usize,
    /// error to report after the capacity is exhausted
    errkind: Option<std::io::ErrorKind>,
}

impl std::io::Write for ShortWriter {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        if self.capacity > 0 {
            let n = self.capacity;
            self.capacity = 0;
            return Ok(n);
        }
        match self.errkind {
            Some(errkind) => Err(std::io::Error::from(errkind)),
            None => Ok(0),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_short_write() {
    // testing situation when the underlying writer cannot take
    // all the data written to bufio::Writer
    let data = b"hello";
    for capacity in [0, 4] {
        let mut buf = ShortWriter {
            capacity,
            errkind: None,
        };
        let mut w = new_writer(&mut buf);

        // writing 5 bytes
        // no error because the writer has big buffer
        let res = w.write(data);
        assert!(res.as_ref().is_ok_and(|x| *x == 5), "write res: {:?}", res);

        // flush should fail because the underlying buffer can take only 4 bytes
        let res = w.flush();
        assert!(
            res.as_ref().is_err_and(ggio::is_short_write_error),
            "flush res: {:?}",
            res
        );
    }
}

// type errorWriterTest struct {
// 	n, m   isize
// 	err    error
// 	expect error
// }

// fn (w errorWriterTest) Write(p [u8]) (isize, error) {
// 	return len(p) * w.n / w.m, w.err
// }

// var errorWriterTests = []errorWriterTest{
// 	{0, 1, nil, io.ErrShortWrite},
// 	{1, 2, nil, io.ErrShortWrite},
// 	{1, 1, nil, nil},
// 	{0, 1, io.ErrClosedPipe, io.ErrClosedPipe},
// 	{1, 2, io.ErrClosedPipe, io.ErrClosedPipe},
// 	{1, 1, io.ErrClosedPipe, io.ErrClosedPipe},
// }

// fn TestWriteErrors() {
// 	for _, w := range errorWriterTests {
// 		buf := new_writer(w)
// 		_, e := buf.write([u8]("hello world"))
// 		if e != nil {
// 			t.Errorf("Write hello to {}: {}", w, e)
// 			continue
// 		}
// 		// Two flushes, to verify the error is sticky.
// 		for i := 0; i < 2; i += 1 {
// 			e = buf.flush()
// 			if e != w.expect {
// 				t.Errorf("Flush {}/2 {}: got {}, wanted {}", i+1, w, e, w.expect)
// 			}
// 		}
// 	}
// }

// fn Testnew_reader_sizeIdempotent() {
// 	const BufSize = 1000
// 	b := new_reader_size(strings.new_reader("hello world"), BufSize)
// 	// Does it recognize itself?
// 	b1 := new_reader_size(b, BufSize)
// 	if b1 != b {
// 		t.Error("new_reader_size did not detect underlying Reader")
// 	}
// 	// Does it wrap if existing buffer is too small?
// 	b2 := new_reader_size(b, 2*BufSize)
// 	if b2 == b {
// 		t.Error("new_reader_size did not enlarge buffer")
// 	}
// }

// fn TestNewWriterSizeIdempotent() {
// 	const BufSize = 1000
// 	b := new_writer_size(new(bytes::Buffer::new()), BufSize)
// 	// Does it recognize itself?
// 	b1 := new_writer_size(b, BufSize)
// 	if b1 != b {
// 		t.Error("new_writer_size did not detect underlying Writer")
// 	}
// 	// Does it wrap if existing buffer is too small?
// 	b2 := new_writer_size(b, 2*BufSize)
// 	if b2 == b {
// 		t.Error("new_writer_size did not enlarge buffer")
// 	}
// }

#[test]
fn test_write_string() {
    const BUF_SIZE: usize = 8;
    // 	buf := new(strings.Builder)
    let mut buf = Vec::<u8>::new();
    let mut b = new_writer_size(&mut buf, BUF_SIZE);
    b.write_string("0").unwrap(); // easy
    b.write_string("123456").unwrap(); // still easy
    b.write_string("7890").unwrap(); // easy after flush
    b.write_string("abcdefghijklmnopqrstuvwxy").unwrap(); // hard
    b.write_string("z").unwrap();
    b.flush().unwrap();
    // 	s := "01234567890abcdefghijklmnopqrstuvwxyz"
    // 	if buf.String() != s {
    // 		t.Errorf("write_string wants %q gets %q", s, buf.String())
    // 	}
    assert_eq!(b"01234567890abcdefghijklmnopqrstuvwxyz", buf.as_slice());
}

// fn TestWriteStringStringWriter() {
// 	const BufSize = 8
// 	{
// 		tw := &teststringwriter{}
// 		b := new_writer_size(tw, BufSize)
// 		b.write_string("1234")
// 		tw.check(t, "", "")
// 		b.write_string("56789012")   // longer than BufSize
// 		tw.check(t, "12345678", "") // but not enough (after filling the partially-filled buffer)
// 		b.flush()
// 		tw.check(t, "123456789012", "")
// 	}
// 	{
// 		tw := &teststringwriter{}
// 		b := new_writer_size(tw, BufSize)
// 		b.write_string("123456789")   // long string, empty buffer:
// 		tw.check(t, "", "123456789") // use write_string
// 	}
// 	{
// 		tw := &teststringwriter{}
// 		b := new_writer_size(tw, BufSize)
// 		b.write_string("abc")
// 		tw.check(t, "", "")
// 		b.write_string("123456789012345")      // long string, non-empty buffer
// 		tw.check(t, "abc12345", "6789012345") // use Write and then write_string since the remaining part is still longer than BufSize
// 	}
// 	{
// 		tw := &teststringwriter{}
// 		b := new_writer_size(tw, BufSize)
// 		b.Write([]byte("abc")) // same as above, but use Write instead of write_string
// 		tw.check(t, "", "")
// 		b.write_string("123456789012345")
// 		tw.check(t, "abc12345", "6789012345") // same as above
// 	}
// }

// type teststringwriter struct {
// 	write       string
// 	writeString string
// }

// fn (w *teststringwriter) Write(b [u8]) (isize, error) {
// 	w.write += string(b)
// 	return b.len(), nil
// }

// fn (w *teststringwriter) write_string(s string) (isize, error) {
// 	w.writeString += s
// 	return len(s), nil
// }

// fn (w *teststringwriter) check(, write, writeString string) {
// 	t.Helper()
// 	if w.write != write {
// 		t.Errorf("write: expected %q, got %q", write, w.write)
// 	}
// 	if w.writeString != writeString {
// 		t.Errorf("writeString: expected %q, got %q", writeString, w.writeString)
// 	}
// }

// fn TestBufferFull() {
// 	const longString = "And now, hello, world! It is the time for all good men to come to the aid of their party"
// 	buf := new_reader_size(strings.new_reader(longString), MIN_READ_BUFFER_SIZE)
// 	line, err := buf.ReadSlice('!')
// 	if string(line) != "And now, hello, " || err != ErrBufferFull {
// 		t.Errorf("first ReadSlice(,) = %q, {}", line, err)
// 	}
// 	line, err = buf.ReadSlice('!')
// 	if string(line) != "world!" || err != nil {
// 		t.Errorf("second ReadSlice(,) = %q, {}", line, err)
// 	}
// }

// fn TestPeek() {
// 	p := make([u8], 10)
// 	// string is 16 (MIN_READ_BUFFER_SIZE) long.
// 	buf := new_reader_size(strings.new_reader("abcdefghijklmnop"), MIN_READ_BUFFER_SIZE)
// 	if s, err := buf.Peek(1); string(s) != "a" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "a", string(s), err)
// 	}
// 	if s, err := buf.Peek(4); string(s) != "abcd" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "abcd", string(s), err)
// 	}
// 	if _, err := buf.Peek(-1); err != ErrNegativeCount {
// 		t.Fatalf("want ErrNegativeCount got {}", err)
// 	}
// 	if s, err := buf.Peek(32); string(s) != "abcdefghijklmnop" || err != ErrBufferFull {
// 		t.Fatalf("want %q, ErrBufFull got %q, err={}", "abcdefghijklmnop", string(s), err)
// 	}
// 	if _, err := buf.Read(p[0..3]); string(p[0..3]) != "abc" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "abc", string(p[0..3]), err)
// 	}
// 	if s, err := buf.Peek(1); string(s) != "d" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "d", string(s), err)
// 	}
// 	if s, err := buf.Peek(2); string(s) != "de" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "de", string(s), err)
// 	}
// 	if _, err := buf.Read(p[0..3]); string(p[0..3]) != "def" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "def", string(p[0..3]), err)
// 	}
// 	if s, err := buf.Peek(4); string(s) != "ghij" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "ghij", string(s), err)
// 	}
// 	if _, err := buf.Read(p[0..]); string(p[0..]) != "ghijklmnop" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "ghijklmnop", string(p[0..MIN_READ_BUFFER_SIZE]), err)
// 	}
// 	if s, err := buf.Peek(0); string(s) != "" || err != nil {
// 		t.Fatalf("want %q got %q, err={}", "", string(s), err)
// 	}
// 	if _, err := buf.Peek(1); err != io.EOF {
// 		t.Fatalf("want EOF got {}", err)
// 	}

// 	// Test for issue 3022, not exposing a reader's error on a successful Peek.
// 	buf = new_reader_size(dataAndEOFReader("abcd"), 32)
// 	if s, err := buf.Peek(2); string(s) != "ab" || err != nil {
// 		t.Errorf(`Peek(2) on "abcd", EOF = %q, {}; want "ab", nil`, string(s), err)
// 	}
// 	if s, err := buf.Peek(4); string(s) != "abcd" || err != nil {
// 		t.Errorf(`Peek(4) on "abcd", EOF = %q, {}; want "abcd", nil`, string(s), err)
// 	}
// 	if n, err := buf.Read(p[0..5]); string(p[0..n]) != "abcd" || err != nil {
// 		t.Fatalf("Read after peek = %q, {}; want abcd, EOF", p[0..n], err)
// 	}
// 	if n, err := buf.Read(p[0..1]); string(p[0..n]) != "" || err != io.EOF {
// 		t.Fatalf(`second Read after peek = %q, {}; want "", EOF`, p[0..n], err)
// 	}
// }

// type dataAndEOFReader string

// fn (r dataAndEOFReader) Read(p [u8]) (isize, error) {
// 	return copy(p, r), io.EOF
// }

// fn TestPeekThenUnreadRune() {
// 	// This sequence used to cause a crash.
// 	r := new_reader(strings.new_reader("x"))
// 	r.ReadRune()
// 	r.Peek(1)
// 	r.UnreadRune()
// 	r.ReadRune() // Used to panic here
// }

// var testOutput = [u8]("0123456789abcdefghijklmnopqrstuvwxy")
// var testInput = [u8]("012\n345\n678\n9ab\ncde\nfgh\nijk\nlmn\nopq\nrst\nuvw\nxy")
// var testInputrn = [u8]("012\r\n345\r\n678\r\n9ab\r\ncde\r\nfgh\r\nijk\r\nlmn\r\nopq\r\nrst\r\nuvw\r\nxy\r\n\n\r\n")

// // TestReader wraps a [u8] and returns reads of a specific length.
// type testReader struct {
// 	data   [u8]
// 	stride isize
// }

// fn (t *testReader) Read(buf [u8]) (n isize, err error) {
// 	n = t.stride
// 	if n > len(t.data) {
// 		n = len(t.data)
// 	}
// 	if n > len(buf) {
// 		n = len(buf)
// 	}
// 	copy(buf, t.data)
// 	t.data = t.data[n:]
// 	if len(t.data) == 0 {
// 		err = io.EOF
// 	}
// 	return
// }

// fn testReadLine(, input [u8]) {
// 	//for stride := 1; stride < len(input); stride++ {
// 	for stride := 1; stride < 2; stride++ {
// 		done := 0
// 		reader := testReader{input, stride}
// 		l := new_reader_size(&reader, len(input)+1)
// 		for {
// 			line, isPrefix, err := l.ReadLine()
// 			if len(line) > 0 && err != nil {
// 				t.Errorf("ReadLine returned both data and error: {}", err)
// 			}
// 			if isPrefix {
// 				t.Errorf("ReadLine returned prefix")
// 			}
// 			if err != nil {
// 				if err != io.EOF {
// 					t.Fatalf("Got unknown error: {}", err)
// 				}
// 				break
// 			}
// 			if want := testOutput[done : done+len(line)]; !bytes::equal(want, line) {
// 				t.Errorf("Bad line at stride {}: want: %x got: %x", stride, want, line)
// 			}
// 			done += len(line)
// 		}
// 		if done != len(testOutput) {
// 			t.Errorf("ReadLine didn't return everything: got: {}, want: {} (stride: {})", done, len(testOutput), stride)
// 		}
// 	}
// }

// fn TestReadLine() {
// 	testReadLine(t, testInput)
// 	testReadLine(t, testInputrn)
// }

// fn TestLineTooLong() {
// 	data := make([u8], 0)
// 	for i := 0; i < MIN_READ_BUFFER_SIZE*5/2; i += 1 {
// 		data = append(data, '0'+byte(i%10))
// 	}
// 	buf := bytes::Reader::new(data)
// 	l := new_reader_size(buf, MIN_READ_BUFFER_SIZE)
// 	line, isPrefix, err := l.ReadLine()
// 	if !isPrefix || !bytes::equal(line, data[..MIN_READ_BUFFER_SIZE]) || err != nil {
// 		t.Errorf("bad result for first line: got %q want %q {}", line, data[..MIN_READ_BUFFER_SIZE], err)
// 	}
// 	data = data[len(line):]
// 	line, isPrefix, err = l.ReadLine()
// 	if !isPrefix || !bytes::equal(line, data[..MIN_READ_BUFFER_SIZE]) || err != nil {
// 		t.Errorf("bad result for second line: got %q want %q {}", line, data[..MIN_READ_BUFFER_SIZE], err)
// 	}
// 	data = data[len(line):]
// 	line, isPrefix, err = l.ReadLine()
// 	if isPrefix || !bytes::equal(line, data[..MIN_READ_BUFFER_SIZE/2]) || err != nil {
// 		t.Errorf("bad result for third line: got %q want %q {}", line, data[..MIN_READ_BUFFER_SIZE/2], err)
// 	}
// 	line, isPrefix, err = l.ReadLine()
// 	if isPrefix || err == nil {
// 		t.Errorf("expected no more lines: %x {}", line, err)
// 	}
// }

// fn TestReadAfterLines() {
// 	line1 := "this is line1"
// 	restData := "this is line2\nthis is line 3\n"
// 	inbuf := bytes::Reader::new([u8](line1 + "\n" + restData))
// 	outbuf := new(strings.Builder)
// 	maxLineLength := len(line1) + len(restData)/2
// 	l := new_reader_size(inbuf, maxLineLength)
// 	line, isPrefix, err := l.ReadLine()
// 	if isPrefix || err != nil || string(line) != line1 {
// 		t.Errorf("bad result for first line: isPrefix={} err={} line=%q", isPrefix, err, string(line))
// 	}
// 	n, err := io.Copy(outbuf, l)
// 	if isize(n) != len(restData) || err != nil {
// 		t.Errorf("bad result for Read: n={} err={}", n, err)
// 	}
// 	if outbuf.String() != restData {
// 		t.Errorf("bad result for Read: got %q; expected %q", outbuf.String(), restData)
// 	}
// }

// fn TestReadEmptyBuffer() {
// 	l := new_reader_size(new(bytes::Buffer::new()), MIN_READ_BUFFER_SIZE)
// 	line, isPrefix, err := l.ReadLine()
// 	if err != io.EOF {
// 		t.Errorf("expected EOF from ReadLine, got '{}' %t {}", line, isPrefix, err)
// 	}
// }

// fn TestLinesAfterRead() {
// 	l := new_reader_size(bytes::Reader::new([u8]("foo")), MIN_READ_BUFFER_SIZE)
// 	_, err := ggio::read_all(l)
// 	if err != nil {
// 		t.Error(err)
// 		return
// 	}

// 	line, isPrefix, err := l.ReadLine()
// 	if err != io.EOF {
// 		t.Errorf("expected EOF from ReadLine, got '{}' %t {}", line, isPrefix, err)
// 	}
// }

// fn TestReadLineNonNilLineOrError() {
// 	r := new_reader(strings.new_reader("line 1\n"))
// 	for i := 0; i < 2; i += 1 {
// 		l, _, err := r.ReadLine()
// 		if l != nil && err != nil {
// 			t.Fatalf("on line {}/2; ReadLine=%#v, {}; want non-nil line or Error, but not both",
// 				i+1, l, err)
// 		}
// 	}
// }

// type readLineResult struct {
// 	line     [u8]
// 	isPrefix bool
// 	err      error
// }

// var readLineNewlinesTests = []struct {
// 	input  string
// 	expect []readLineResult
// }{
// 	{"012345678901234\r\n012345678901234\r\n", []readLineResult{
// 		{[u8]("012345678901234"), true, nil},
// 		{nil, false, nil},
// 		{[u8]("012345678901234"), true, nil},
// 		{nil, false, nil},
// 		{nil, false, io.EOF},
// 	}},
// 	{"0123456789012345\r012345678901234\r", []readLineResult{
// 		{[u8]("0123456789012345"), true, nil},
// 		{[u8]("\r012345678901234"), true, nil},
// 		{[u8]("\r"), false, nil},
// 		{nil, false, io.EOF},
// 	}},
// }

// fn TestReadLineNewlines() {
// 	for _, e := range readLineNewlinesTests {
// 		testReadLineNewlines(t, e.input, e.expect)
// 	}
// }

// fn testReadLineNewlines(, input string, expect []readLineResult) {
// 	b := new_reader_size(strings.new_reader(input), MIN_READ_BUFFER_SIZE)
// 	for i, e := range expect {
// 		line, isPrefix, err := b.ReadLine()
// 		if !bytes::equal(line, e.line) {
// 			t.Errorf("%q call {}, line == %q, want %q", input, i, line, e.line)
// 			return
// 		}
// 		if isPrefix != e.isPrefix {
// 			t.Errorf("%q call {}, isPrefix == {}, want {}", input, i, isPrefix, e.isPrefix)
// 			return
// 		}
// 		if err != e.err {
// 			t.Errorf("%q call {}, err == {}, want {}", input, i, err, e.err)
// 			return
// 		}
// 	}
// }

// fn createTestInput(n isize) [u8] {
// 	input := make([u8], n)
// 	for i := range input {
// 		// 101 and 251 are arbitrary prime numbers.
// 		// The idea is to create an input sequence
// 		// which doesn't repeat too frequently.
// 		input[i] = byte(i % 251)
// 		if i%101 == 0 {
// 			input[i] ^= byte(i / 101)
// 		}
// 	}
// 	return input
// }

// fn TestReaderWriteTo() {
// 	input := createTestInput(8192)
// 	r := new_reader(onlyReader{bytes::Reader::new(input)})
// 	w := new(bytes::Buffer::new())
// 	if n, err := r.WriteTo(w); err != nil || n != int64(len(input)) {
// 		t.Fatalf("r.WriteTo(w) = {}, {}, want {}, nil", n, err, len(input))
// 	}

// 	for i, val := range w.bytes() {
// 		if val != input[i] {
// 			t.Errorf("after write: out[{}] = %#x, want %#x", i, val, input[i])
// 		}
// 	}
// }

// type errorWriterToTest struct {
// 	rn, wn     isize
// 	rerr, werr error
// 	expected   error
// }

// fn (r errorWriterToTest) Read(p [u8]) (isize, error) {
// 	return len(p) * r.rn, r.rerr
// }

// fn (w errorWriterToTest) Write(p [u8]) (isize, error) {
// 	return len(p) * w.wn, w.werr
// }

// var errorWriterToTests = []errorWriterToTest{
// 	{1, 0, nil, io.ErrClosedPipe, io.ErrClosedPipe},
// 	{0, 1, io.ErrClosedPipe, nil, io.ErrClosedPipe},
// 	{0, 0, std::io::Error::ErrUnexpectedEOF, io.ErrClosedPipe, io.ErrClosedPipe},
// 	{0, 1, io.EOF, nil, nil},
// }

// fn TestReaderWriteToErrors() {
// 	for i, rw := range errorWriterToTests {
// 		r := new_reader(rw)
// 		if _, err := r.WriteTo(rw); err != rw.expected {
// 			t.Errorf("r.WriteTo(errorWriterToTests[{}]) = _, {}, want _,{}", i, err, rw.expected)
// 		}
// 	}
// }

// fn TestWriterReadFrom() {
// 	ws := []fn(ggio::Writer) ggio::Writer{
// 		fn(w: &mut dyn ggio::Writer) ggio::Writer { return onlyWriter{w} },
// 		fn(w: &mut dyn ggio::Writer) ggio::Writer { return w },
// 	}

// 	rs := []fn(io.Reader) io.Reader{
// 		iotest.DataErrReader,
// 		fn(r io.Reader) io.Reader { return r },
// 	}

// 	for ri, rfunc := range rs {
// 		for wi, wfunc := range ws {
// 			input := createTestInput(8192)
// 			b := new(strings.Builder)
// 			w := new_writer(wfunc(b))
// 			r := rfunc(bytes::Reader::new(input))
// 			if n, err := w.ReadFrom(r); err != nil || n != int64(len(input)) {
// 				t.Errorf("ws[{}],rs[{}]: w.ReadFrom(r) = {}, {}, want {}, nil", wi, ri, n, err, len(input))
// 				continue
// 			}
// 			if err := w.flush(); err != nil {
// 				t.Errorf("Flush returned {}", err)
// 				continue
// 			}
// 			if got, want := b.String(), string(input); got != want {
// 				t.Errorf("ws[{}], rs[{}]:\ngot  %q\nwant %q\n", wi, ri, got, want)
// 			}
// 		}
// 	}
// }

// type errorReaderFromTest struct {
// 	rn, wn     isize
// 	rerr, werr error
// 	expected   error
// }

// fn (r errorReaderFromTest) Read(p [u8]) (isize, error) {
// 	return len(p) * r.rn, r.rerr
// }

// fn (w errorReaderFromTest) Write(p [u8]) (isize, error) {
// 	return len(p) * w.wn, w.werr
// }

// var errorReaderFromTests = []errorReaderFromTest{
// 	{0, 1, io.EOF, nil, nil},
// 	{1, 1, io.EOF, nil, nil},
// 	{0, 1, io.ErrClosedPipe, nil, io.ErrClosedPipe},
// 	{0, 0, io.ErrClosedPipe, io.ErrShortWrite, io.ErrClosedPipe},
// 	{1, 0, nil, io.ErrShortWrite, io.ErrShortWrite},
// }

// fn TestWriterReadFromErrors() {
// 	for i, rw := range errorReaderFromTests {
// 		w := new_writer(rw)
// 		if _, err := w.ReadFrom(rw); err != rw.expected {
// 			t.Errorf("w.ReadFrom(errorReaderFromTests[{}]) = _, {}, want _,{}", i, err, rw.expected)
// 		}
// 	}
// }

// // TestWriterReadFromCounts tests that using io.Copy to copy into a
// // bufio::Writer does not prematurely flush the buffer. For example, when
// // buffering writes to a network socket, excessive network writes should be
// // avoided.
// fn TestWriterReadFromCounts() {
// 	var w0 writeCountingDiscard
// 	b0 := new_writer_size(&w0, 1234)
// 	b0.write_string(strings.Repeat("x", 1000))
// 	if w0 != 0 {
// 		t.Fatalf("write 1000 'x's: got {} writes, want 0", w0)
// 	}
// 	b0.write_string(strings.Repeat("x", 200))
// 	if w0 != 0 {
// 		t.Fatalf("write 1200 'x's: got {} writes, want 0", w0)
// 	}
// 	io.Copy(b0, onlyReader{strings.new_reader(strings.Repeat("x", 30))})
// 	if w0 != 0 {
// 		t.Fatalf("write 1230 'x's: got {} writes, want 0", w0)
// 	}
// 	io.Copy(b0, onlyReader{strings.new_reader(strings.Repeat("x", 9))})
// 	if w0 != 1 {
// 		t.Fatalf("write 1239 'x's: got {} writes, want 1", w0)
// 	}

// 	var w1 writeCountingDiscard
// 	b1 := new_writer_size(&w1, 1234)
// 	b1.write_string(strings.Repeat("x", 1200))
// 	b1.flush()
// 	if w1 != 1 {
// 		t.Fatalf("flush 1200 'x's: got {} writes, want 1", w1)
// 	}
// 	b1.write_string(strings.Repeat("x", 89))
// 	if w1 != 1 {
// 		t.Fatalf("write 1200 + 89 'x's: got {} writes, want 1", w1)
// 	}
// 	io.Copy(b1, onlyReader{strings.new_reader(strings.Repeat("x", 700))})
// 	if w1 != 1 {
// 		t.Fatalf("write 1200 + 789 'x's: got {} writes, want 1", w1)
// 	}
// 	io.Copy(b1, onlyReader{strings.new_reader(strings.Repeat("x", 600))})
// 	if w1 != 2 {
// 		t.Fatalf("write 1200 + 1389 'x's: got {} writes, want 2", w1)
// 	}
// 	b1.flush()
// 	if w1 != 3 {
// 		t.Fatalf("flush 1200 + 1389 'x's: got {} writes, want 3", w1)
// 	}
// }

// // A writeCountingDiscard is like ggio::Discard::new() and counts the number of times
// // Write is called on it.
// type writeCountingDiscard isize

// fn (w *writeCountingDiscard) Write(p [u8]) (isize, error) {
// 	*w++
// 	return len(p), nil
// }

// type negativeReader isize

// fn (r *negativeReader) Read([u8]) (isize, error) { return -1, nil }

// fn TestNegativeRead() {
// 	// should panic with a description pointing at the reader, not at itself.
// 	// (should NOT panic with slice index error, for example.)
// 	b := new_reader(new(negativeReader))
// 	defer fn() {
// 		switch err := recover().(type) {
// 		case nil:
// 			t.Fatal("read did not panic")
// 		case error:
// 			if !strings.Contains(err.Error(), "reader returned negative count from Read") {
// 				t.Fatalf("wrong panic: {}", err)
// 			}
// 		default:
// 			t.Fatalf("unexpected panic value: %T({})", err, err)
// 		}
// 	}()
// 	b.Read(make([u8], 100))
// }

// var errFake = errors.New("fake error")

// type errorThenGoodReader struct {
// 	didErr bool
// 	nread  isize
// }

// fn (r *errorThenGoodReader) Read(p [u8]) (isize, error) {
// 	r.nread++
// 	if !r.didErr {
// 		r.didErr = true
// 		return 0, errFake
// 	}
// 	return len(p), nil
// }

// fn TestReaderClearError() {
// 	r := &errorThenGoodReader{}
// 	b := new_reader(r)
// 	buf := make([u8], 1)
// 	if _, err := b.Read(nil); err != nil {
// 		t.Fatalf("1st nil Read = {}; want nil", err)
// 	}
// 	if _, err := b.Read(buf); err != errFake {
// 		t.Fatalf("1st Read = {}; want errFake", err)
// 	}
// 	if _, err := b.Read(nil); err != nil {
// 		t.Fatalf("2nd nil Read = {}; want nil", err)
// 	}
// 	if _, err := b.Read(buf); err != nil {
// 		t.Fatalf("3rd Read with buffer = {}; want nil", err)
// 	}
// 	if r.nread != 2 {
// 		t.Errorf("num reads = {}; want 2", r.nread)
// 	}
// }

// // Test for golang.org/issue/5947
// fn TestWriterReadFromWhileFull() {
// 	let mut buf = bytes::Buffer::new();
// 	w := new_writer_size(buf, 10)

// 	// Fill buffer exactly.
// 	n, err := w.write([u8]("0123456789"))
// 	if n != 10 || err != nil {
// 		t.Fatalf("Write returned ({}, {}), want (10, nil)", n, err)
// 	}

// 	// Use ReadFrom to read in some data.
// 	n2, err := w.ReadFrom(strings.new_reader("abcdef"))
// 	if n2 != 6 || err != nil {
// 		t.Fatalf("ReadFrom returned ({}, {}), want (6, nil)", n2, err)
// 	}
// }

// type emptyThenNonEmptyReader struct {
// 	r io.Reader
// 	n isize
// }

// fn (r *emptyThenNonEmptyReader) Read(p [u8]) (isize, error) {
// 	if r.n <= 0 {
// 		return r.r.Read(p)
// 	}
// 	r.n--
// 	return 0, nil
// }

// // Test for golang.org/issue/7611
// fn TestWriterReadFromUntilEOF() {
// 	let mut buf = bytes::Buffer::new();
// 	w := new_writer_size(buf, 5)

// 	// Partially fill buffer
// 	n, err := w.write([u8]("0123"))
// 	if n != 4 || err != nil {
// 		t.Fatalf("Write returned ({}, {}), want (4, nil)", n, err)
// 	}

// 	// Use ReadFrom to read in some data.
// 	r := &emptyThenNonEmptyReader{r: strings.new_reader("abcd"), n: 3}
// 	n2, err := w.ReadFrom(r)
// 	if n2 != 4 || err != nil {
// 		t.Fatalf("ReadFrom returned ({}, {}), want (4, nil)", n2, err)
// 	}
// 	w.flush()
// 	if got, want := buf.String(), "0123abcd"; got != want {
// 		t.Fatalf("buf.bytes() returned %q, want %q", got, want)
// 	}
// }

// fn TestWriterReadFromErrNoProgress() {
// 	let mut buf = bytes::Buffer::new();
// 	w := new_writer_size(buf, 5)

// 	// Partially fill buffer
// 	n, err := w.write([u8]("0123"))
// 	if n != 4 || err != nil {
// 		t.Fatalf("Write returned ({}, {}), want (4, nil)", n, err)
// 	}

// 	// Use ReadFrom to read in some data.
// 	r := &emptyThenNonEmptyReader{r: strings.new_reader("abcd"), n: 100}
// 	n2, err := w.ReadFrom(r)
// 	if n2 != 0 || err != io.ErrNoProgress {
// 		t.Fatalf("buf.bytes() returned ({}, {}), want (0, io.ErrNoProgress)", n2, err)
// 	}
// }

// type readFromWriter struct {
// 	buf           [u8]
// 	writeBytes    isize
// 	readFromBytes isize
// }

// fn (w *readFromWriter) Write(p [u8]) (isize, error) {
// 	w.buf = append(w.buf, p...)
// 	w.writeBytes += len(p)
// 	return len(p), nil
// }

// fn (w *readFromWriter) ReadFrom(r io.Reader) (int64, error) {
// 	b, err := ggio::read_all(r)
// 	w.buf = append(w.buf, b...)
// 	w.readFromBytes += b.len()
// 	return int64(b.len()), err
// }

// // Test that calling (*Writer).ReadFrom with a partially-filled buffer
// // fills the buffer before switching over to ReadFrom.
// fn TestWriterReadFromWithBufferedData() {
// 	const bufsize = 16

// 	input := createTestInput(64)
// 	rfw := &readFromWriter{}
// 	w := new_writer_size(rfw, bufsize)

// 	const writeSize = 8
// 	if n, err := w.write(input[..writeSize]); n != writeSize || err != nil {
// 		t.Errorf("w.write({} bytes) = {}, {}; want {}, nil", writeSize, n, err, writeSize)
// 	}
// 	n, err := w.ReadFrom(bytes::Reader::new(input[writeSize:]))
// 	if wantn := len(input[writeSize:]); isize(n) != wantn || err != nil {
// 		t.Errorf("io.Copy(w, {} bytes) = {}, {}; want {}, nil", wantn, n, err, wantn)
// 	}
// 	if err := w.flush(); err != nil {
// 		t.Errorf("w.flush() = {}, want nil", err)
// 	}

// 	if got, want := rfw.writeBytes, bufsize; got != want {
// 		t.Errorf("wrote {} bytes with Write, want {}", got, want)
// 	}
// 	if got, want := rfw.readFromBytes, len(input)-bufsize; got != want {
// 		t.Errorf("wrote {} bytes with ReadFrom, want {}", got, want)
// 	}
// }

// fn TestReadZero() {
// 	for _, size := range []isize{100, 2} {
// 		t.Run(fmt.Sprintf("bufsize={}", size), fn() {
// 			r := io.MultiReader(strings.new_reader("abc"), &emptyThenNonEmptyReader{r: strings.new_reader("def"), n: 1})
// 			br := new_reader_size(r, size)
// 			want := fn(s string, wantErr error) {
// 				p := make([u8], 50)
// 				n, err := br.Read(p)
// 				if err != wantErr || n != len(s) || string(p[..n]) != s {
// 					t.Fatalf("read({}) = %q, {}, want %q, {}", len(p), string(p[..n]), err, s, wantErr)
// 				}
// 				t.Logf("read({}) = %q, {}", len(p), string(p[..n]), err)
// 			}
// 			want("abc", nil)
// 			want("", nil)
// 			want("def", nil)
// 			want("", io.EOF)
// 		})
// 	}
// }

// fn TestReaderReset() {
// 	checkAll := fn(r *Reader, want string) {
// 		t.Helper()
// 		all, err := ggio::read_all(r)
// 		if err != nil {
// 			t.Fatal(err)
// 		}
// 		if string(all) != want {
// 			t.Errorf("read_all returned %q, want %q", all, want)
// 		}
// 	}

// 	r := new_reader(strings.new_reader("foo foo"))
// 	buf := make([u8], 3)
// 	r.Read(buf)
// 	if string(buf) != "foo" {
// 		t.Errorf("buf = %q; want foo", buf)
// 	}

// 	r.Reset(strings.new_reader("bar bar"))
// 	checkAll(r, "bar bar")

// 	*r = Reader{} // zero out the Reader
// 	r.Reset(strings.new_reader("bar bar"))
// 	checkAll(r, "bar bar")

// 	// Wrap a reader and then Reset to that reader.
// 	r.Reset(strings.new_reader("recur"))
// 	r2 := new_reader(r)
// 	checkAll(r2, "recur")
// 	r.Reset(strings.new_reader("recur2"))
// 	r2.Reset(r)
// 	checkAll(r2, "recur2")
// }

#[test]
fn test_writer_reset() {
    let mut buf1 = bytes::Buffer::new();
    // 	var buf1, buf2, buf3, buf4, buf5 strings.Builder
    let mut w = new_writer(&mut buf1);
    w.write_string("foo").unwrap();

    let mut buf2 = bytes::Buffer::new();
    w.reset(&mut buf2); // and not flushed
    w.write_string("bar").unwrap();
    w.flush().unwrap();
    assert!(buf1.bytes() == b"");
    assert!(buf2.bytes() == b"bar");

    // 	*w = Writer{}  // zero out the Writer
    // 	w.reset(&buf3) // and not flushed
    // 	w.write_string("bar")
    // 	w.flush()
    // 	if buf1.String() != "" {
    // 		t.Errorf("buf1 = %q; want empty", buf1.String())
    // 	}
    // 	if buf3.String() != "bar" {
    // 		t.Errorf("buf3 = %q; want bar", buf3.String())
    // 	}

    // 	// Wrap a writer and then Reset to that writer.
    // 	w.reset(&buf4)
    // 	w2 := new_writer(w)
    // 	w2.write_string("recur")
    // 	w2.flush()
    // 	if buf4.String() != "recur" {
    // 		t.Errorf("buf4 = %q, want %q", buf4.String(), "recur")
    // 	}
    // 	w.reset(&buf5)
    // 	w2.Reset(w)
    // 	w2.write_string("recur2")
    // 	w2.flush()
    // 	if buf5.String() != "recur2" {
    // 		t.Errorf("buf5 = %q, want %q", buf5.String(), "recur2")
    // 	}
}

// fn TestReaderDiscard() {
// 	tests := []struct {
// 		name     string
// 		r        io.Reader
// 		bufSize  isize // 0 means 16
// 		peekSize isize

// 		n isize // input to Discard

// 		want    isize   // from Discard
// 		wantErr error // from Discard

// 		wantBuffered isize
// 	}{
// 		{
// 			name:         "normal case",
// 			r:            strings.new_reader("abcdefghijklmnopqrstuvwxyz"),
// 			peekSize:     16,
// 			n:            6,
// 			want:         6,
// 			wantBuffered: 10,
// 		},
// 		{
// 			name:         "discard causing read",
// 			r:            strings.new_reader("abcdefghijklmnopqrstuvwxyz"),
// 			n:            6,
// 			want:         6,
// 			wantBuffered: 10,
// 		},
// 		{
// 			name:         "discard all without peek",
// 			r:            strings.new_reader("abcdefghijklmnopqrstuvwxyz"),
// 			n:            26,
// 			want:         26,
// 			wantBuffered: 0,
// 		},
// 		{
// 			name:         "discard more than end",
// 			r:            strings.new_reader("abcdefghijklmnopqrstuvwxyz"),
// 			n:            27,
// 			want:         26,
// 			wantErr:      io.EOF,
// 			wantBuffered: 0,
// 		},
// 		// Any error from filling shouldn't show up until we
// 		// get past the valid bytes. Here we return 5 valid bytes at the same time
// 		// as an error, but test that we don't see the error from Discard.
// 		{
// 			name: "fill error, discard less",
// 			r: newScriptedReader(fn(p [u8]) (n isize, err error) {
// 				if len(p) < 5 {
// 					panic("unexpected small read")
// 				}
// 				return 5, errors.New("5-then-error")
// 			}),
// 			n:            4,
// 			want:         4,
// 			wantErr:      nil,
// 			wantBuffered: 1,
// 		},
// 		{
// 			name: "fill error, discard equal",
// 			r: newScriptedReader(fn(p [u8]) (n isize, err error) {
// 				if len(p) < 5 {
// 					panic("unexpected small read")
// 				}
// 				return 5, errors.New("5-then-error")
// 			}),
// 			n:            5,
// 			want:         5,
// 			wantErr:      nil,
// 			wantBuffered: 0,
// 		},
// 		{
// 			name: "fill error, discard more",
// 			r: newScriptedReader(fn(p [u8]) (n isize, err error) {
// 				if len(p) < 5 {
// 					panic("unexpected small read")
// 				}
// 				return 5, errors.New("5-then-error")
// 			}),
// 			n:            6,
// 			want:         5,
// 			wantErr:      errors.New("5-then-error"),
// 			wantBuffered: 0,
// 		},
// 		// Discard of 0 shouldn't cause a read:
// 		{
// 			name:         "discard zero",
// 			r:            newScriptedReader(), // will panic on Read
// 			n:            0,
// 			want:         0,
// 			wantErr:      nil,
// 			wantBuffered: 0,
// 		},
// 		{
// 			name:         "discard negative",
// 			r:            newScriptedReader(), // will panic on Read
// 			n:            -1,
// 			want:         0,
// 			wantErr:      ErrNegativeCount,
// 			wantBuffered: 0,
// 		},
// 	}
// 	for _, tt := range tests {
// 		br := new_reader_size(tt.r, tt.bufSize)
// 		if tt.peekSize > 0 {
// 			peekBuf, err := br.Peek(tt.peekSize)
// 			if err != nil {
// 				t.Errorf("{}: Peek({}): {}", tt.name, tt.peekSize, err)
// 				continue
// 			}
// 			if len(peekBuf) != tt.peekSize {
// 				t.Errorf("{}: len(Peek({})) = {}; want {}", tt.name, tt.peekSize, len(peekBuf), tt.peekSize)
// 				continue
// 			}
// 		}
// 		discarded, err := br.Discard(tt.n)
// 		if ge, we := fmt.Sprint(err), fmt.Sprint(tt.wantErr); discarded != tt.want || ge != we {
// 			t.Errorf("{}: Discard({}) = ({}, {}); want ({}, {})", tt.name, tt.n, discarded, ge, tt.want, we)
// 			continue
// 		}
// 		if bn := br.Buffered(); bn != tt.wantBuffered {
// 			t.Errorf("{}: after Discard, Buffered = {}; want {}", tt.name, bn, tt.wantBuffered)
// 		}
// 	}

// }

// fn TestReaderSize() {
// 	if got, want := new_reader(nil).Size(), DefaultBufSize; got != want {
// 		t.Errorf("new_reader's Reader.Size = {}; want {}", got, want)
// 	}
// 	if got, want := new_reader_size(nil, 1234).Size(), 1234; got != want {
// 		t.Errorf("new_reader_size's Reader.Size = {}; want {}", got, want)
// 	}
// }

#[test]
fn test_writer_size() {
    let mut w = Vec::<u8>::new();
    {
        assert_eq!(bufio::DEFAULT_BUF_SIZE, bufio::new_writer(&mut w).size());
    }
    {
        assert_eq!(1234, bufio::new_writer_size(&mut w, 1234).size());
    }
}

// // An onlyReader only implements io.Reader, no matter what other methods the underlying implementation may have.
// type onlyReader struct {
// 	io.Reader
// }

// // An onlyWriter only implements ggio::Writer, no matter what other methods the underlying implementation may have.
// type onlyWriter struct {
// 	ggio::Writer
// }

// // A scriptedReader is an io.Reader that executes its steps sequentially.
// type scriptedReader []fn(p [u8]) (n isize, err error)

// fn (sr *scriptedReader) Read(p [u8]) (n isize, err error) {
// 	if len(*sr) == 0 {
// 		panic("too many Read calls on scripted Reader. No steps remain.")
// 	}
// 	step := (*sr)[0]
// 	*sr = (*sr)[1:]
// 	return step(p)
// }

// fn newScriptedReader(steps ...fn(p [u8]) (n isize, err error)) io.Reader {
// 	sr := scriptedReader(steps)
// 	return &sr
// }

// // eofReader returns the number of bytes read and io.EOF for the read that consumes the last of the content.
// type eofReader struct {
// 	buf [u8]
// }

// fn (r *eofReader) Read(p [u8]) (isize, error) {
// 	read := copy(p, r.buf)
// 	r.buf = r.buf[read:]

// 	switch read {
// 	case 0, len(r.buf):
// 		// As allowed in the documentation, this will return io.EOF
// 		// in the same call that consumes the last of the data.
// 		// https://godoc.org/io#Reader
// 		return read, io.EOF
// 	}

// 	return read, nil
// }

// fn TestPartialReadEOF() {
// 	src := make([u8], 10)
// 	eofR := &eofReader{buf: src}
// 	r := new_reader(eofR)

// 	// Start by reading 5 of the 10 available bytes.
// 	dest := make([u8], 5)
// 	read, err := r.Read(dest)
// 	if err != nil {
// 		t.Fatalf("unexpected error: {}", err)
// 	}
// 	if n := len(dest); read != n {
// 		t.Fatalf("read {} bytes; wanted {} bytes", read, n)
// 	}

// 	// The Reader should have buffered all the content from the io.Reader.
// 	if n := len(eofR.buf); n != 0 {
// 		t.Fatalf("got {} bytes left in bufio.Reader source; want 0 bytes", n)
// 	}
// 	// To prove the point, check that there are still 5 bytes available to read.
// 	if n := r.Buffered(); n != 5 {
// 		t.Fatalf("got {} bytes buffered in bufio.Reader; want 5 bytes", n)
// 	}

// 	// This is the second read of 0 bytes.
// 	read, err = r.Read([u8]{})
// 	if err != nil {
// 		t.Fatalf("unexpected error: {}", err)
// 	}
// 	if read != 0 {
// 		t.Fatalf("read {} bytes; want 0 bytes", read)
// 	}
// }

// type writerWithReadFromError struct{}

// fn (w writerWithReadFromError) ReadFrom(r io.Reader) (int64, error) {
// 	return 0, errors.New("writerWithReadFromError error")
// }

// fn (w writerWithReadFromError) Write(b [u8]) (n isize, err error) {
// 	return 10, nil
// }

// fn TestWriterReadFromMustSetUnderlyingError() {
// 	var wr = new_writer(writerWithReadFromError{})
// 	if _, err := wr.ReadFrom(strings.new_reader("test2")); err == nil {
// 		t.Fatal("expected ReadFrom returns error, got nil")
// 	}
// 	if _, err := wr.Write([u8]("123")); err == nil {
// 		t.Fatal("expected Write returns error, got nil")
// 	}
// }

// type writeErrorOnlyWriter struct{}

// fn (w writeErrorOnlyWriter) Write(p [u8]) (n isize, err error) {
// 	return 0, errors.New("writeErrorOnlyWriter error")
// }

// // Ensure that previous Write errors are immediately returned
// // on any ReadFrom. See golang.org/issue/35194.
// fn TestWriterReadFromMustReturnUnderlyingError() {
// 	var wr = new_writer(writeErrorOnlyWriter{})
// 	s := "test1"
// 	wantBuffered := len(s)
// 	if _, err := wr.write_string(s); err != nil {
// 		t.Fatalf("unexpected error: {}", err)
// 	}
// 	if err := wr.flush(); err == nil {
// 		t.Error("expected flush error, got nil")
// 	}
// 	if _, err := wr.ReadFrom(strings.new_reader("test2")); err == nil {
// 		t.Fatal("expected error, got nil")
// 	}
// 	if buffered := wr.Buffered(); buffered != wantBuffered {
// 		t.Fatalf("Buffered = {}; want {}", buffered, wantBuffered)
// 	}
// }

// fn BenchmarkReaderCopyOptimal(b *testing.B) {
// 	// Optimal case is where the underlying reader implements io.WriterTo
// 	srcBuf := bytes::new_buffer(make([u8], 8192))
// 	src := new_reader(srcBuf)
// 	dstBuf := new(bytes::Buffer::new())
// 	dst := onlyWriter{dstBuf}
// 	for i := 0; i < b.N; i += 1 {
// 		srcBuf.Reset()
// 		src.Reset(srcBuf)
// 		dstBuf.Reset()
// 		io.Copy(dst, src)
// 	}
// }

// fn BenchmarkReaderCopyUnoptimal(b *testing.B) {
// 	// Unoptimal case is where the underlying reader doesn't implement io.WriterTo
// 	srcBuf := bytes::new_buffer(make([u8], 8192))
// 	src := new_reader(onlyReader{srcBuf})
// 	dstBuf := new(bytes::Buffer::new())
// 	dst := onlyWriter{dstBuf}
// 	for i := 0; i < b.N; i += 1 {
// 		srcBuf.Reset()
// 		src.Reset(onlyReader{srcBuf})
// 		dstBuf.Reset()
// 		io.Copy(dst, src)
// 	}
// }

// fn BenchmarkReaderCopyNoWriteTo(b *testing.B) {
// 	srcBuf := bytes::new_buffer(make([u8], 8192))
// 	srcReader := new_reader(srcBuf)
// 	src := onlyReader{srcReader}
// 	dstBuf := new(bytes::Buffer::new())
// 	dst := onlyWriter{dstBuf}
// 	for i := 0; i < b.N; i += 1 {
// 		srcBuf.Reset()
// 		srcReader.Reset(srcBuf)
// 		dstBuf.Reset()
// 		io.Copy(dst, src)
// 	}
// }

// fn BenchmarkReaderWriteToOptimal(b *testing.B) {
// 	const bufSize = 16 << 10
// 	buf := make([u8], bufSize)
// 	r := bytes::Reader::new(buf)
// 	srcReader := new_reader_size(onlyReader{r}, 1<<10)
// 	if _, ok := ggio::Discard::new().(io.ReaderFrom); !ok {
// 		b.Fatal("ggio::Discard::new() doesn't support ReaderFrom")
// 	}
// 	for i := 0; i < b.N; i += 1 {
// 		r.Seek(0, ggio::Seek::Start)
// 		srcReader.Reset(onlyReader{r})
// 		n, err := srcReader.WriteTo(ggio::Discard::new())
// 		if err != nil {
// 			b.Fatal(err)
// 		}
// 		if n != bufSize {
// 			b.Fatalf("n = {}; want {}", n, bufSize)
// 		}
// 	}
// }

// fn BenchmarkReaderReadString(b *testing.B) {
// 	r := strings.new_reader("       foo       foo        42        42        42        42        42        42        42        42       4.2       4.2       4.2       4.2\n")
// 	buf := new_reader(r)
// 	b.ReportAllocs()
// 	for i := 0; i < b.N; i += 1 {
// 		r.Seek(0, ggio::Seek::Start)
// 		buf.Reset(r)

// 		_, err := buf.ReadString('\n')
// 		if err != nil {
// 			b.Fatal(err)
// 		}
// 	}
// }

// fn BenchmarkWriterCopyOptimal(b *testing.B) {
// 	// Optimal case is where the underlying writer implements io.ReaderFrom
// 	srcBuf := bytes::new_buffer(make([u8], 8192))
// 	src := onlyReader{srcBuf}
// 	dstBuf := new(bytes::Buffer::new())
// 	dst := new_writer(dstBuf)
// 	for i := 0; i < b.N; i += 1 {
// 		srcBuf.Reset()
// 		dstBuf.Reset()
// 		dst.Reset(dstBuf)
// 		io.Copy(dst, src)
// 	}
// }

// fn BenchmarkWriterCopyUnoptimal(b *testing.B) {
// 	srcBuf := bytes::new_buffer(make([u8], 8192))
// 	src := onlyReader{srcBuf}
// 	dstBuf := new(bytes::Buffer::new())
// 	dst := new_writer(onlyWriter{dstBuf})
// 	for i := 0; i < b.N; i += 1 {
// 		srcBuf.Reset()
// 		dstBuf.Reset()
// 		dst.Reset(onlyWriter{dstBuf})
// 		io.Copy(dst, src)
// 	}
// }

// fn BenchmarkWriterCopyNoReadFrom(b *testing.B) {
// 	srcBuf := bytes::new_buffer(make([u8], 8192))
// 	src := onlyReader{srcBuf}
// 	dstBuf := new(bytes::Buffer::new())
// 	dstWriter := new_writer(dstBuf)
// 	dst := onlyWriter{dstWriter}
// 	for i := 0; i < b.N; i += 1 {
// 		srcBuf.Reset()
// 		dstBuf.Reset()
// 		dstWriter.Reset(dstBuf)
// 		io.Copy(dst, src)
// 	}
// }

// fn BenchmarkReaderEmpty(b *testing.B) {
// 	b.ReportAllocs()
// 	str := strings.Repeat("x", 16<<10)
// 	for i := 0; i < b.N; i += 1 {
// 		br := new_reader(strings.new_reader(str))
// 		n, err := io.Copy(ggio::Discard::new(), br)
// 		if err != nil {
// 			b.Fatal(err)
// 		}
// 		if n != int64(len(str)) {
// 			b.Fatal("wrong length")
// 		}
// 	}
// }

// fn BenchmarkWriterEmpty(b *testing.B) {
// 	b.ReportAllocs()
// 	str := strings.Repeat("x", 1<<10)
// 	bs := [u8](str)
// 	for i := 0; i < b.N; i += 1 {
// 		bw := new_writer(ggio::Discard::new())
// 		bw.flush()
// 		bw.write_byte('a')
// 		bw.flush()
// 		bw.WriteRune('B')
// 		bw.flush()
// 		bw.Write(bs)
// 		bw.flush()
// 		bw.write_string(str)
// 		bw.flush()
// 	}
// }

// fn BenchmarkWriterFlush(b *testing.B) {
// 	b.ReportAllocs()
// 	bw := new_writer(ggio::Discard::new())
// 	str := strings.Repeat("x", 50)
// 	for i := 0; i < b.N; i += 1 {
// 		bw.write_string(str)
// 		bw.flush()
// 	}
// }
