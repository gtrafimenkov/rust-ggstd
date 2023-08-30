// // Copyright 2010 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package io_test

// import (
// 	"bytes"
// 	"crypto/sha1"
// 	"errors"
// 	"fmt"
// 	. "io"
// 	"runtime"
// 	"strings"
// 	"testing"
// 	"time"
// )

// func TestMultiReader(t *testing.T) {
// 	var mr Reader
// 	var buf [u8]
// 	nread := 0
// 	withFooBar := func(tests func()) {
// 		r1 := strings.new_reader("foo ")
// 		r2 := strings.new_reader("")
// 		r3 := strings.new_reader("bar")
// 		mr = MultiReader(r1, r2, r3)
// 		buf = make([u8], 20)
// 		tests()
// 	}
// 	expectRead := func(size int, expected string, eerr error) {
// 		nread++
// 		n, gerr := mr.Read(buf[0:size])
// 		if n != len(expected) {
// 			t.Errorf("#{}, expected {} bytes; got {}",
// 				nread, len(expected), n)
// 		}
// 		got := string(buf[0:n])
// 		if got != expected {
// 			t.Errorf("#{}, expected %q; got %q",
// 				nread, expected, got)
// 		}
// 		if gerr != eerr {
// 			t.Errorf("#{}, expected error {}; got {}",
// 				nread, eerr, gerr)
// 		}
// 		buf = buf[n:]
// 	}
// 	withFooBar(func() {
// 		expectRead(2, "fo", nil)
// 		expectRead(5, "o ", nil)
// 		expectRead(5, "bar", nil)
// 		expectRead(5, "", EOF)
// 	})
// 	withFooBar(func() {
// 		expectRead(4, "foo ", nil)
// 		expectRead(1, "b", nil)
// 		expectRead(3, "ar", nil)
// 		expectRead(1, "", EOF)
// 	})
// 	withFooBar(func() {
// 		expectRead(5, "foo ", nil)
// 	})
// }

// func TestMultiReaderAsWriterTo(t *testing.T) {
// 	mr := MultiReader(
// 		strings.new_reader("foo "),
// 		MultiReader( // Tickle the buffer reusing codepath
// 			strings.new_reader(""),
// 			strings.new_reader("bar"),
// 		),
// 	)
// 	mrAsWriterTo, ok := mr.(WriterTo)
// 	if !ok {
// 		t.Fatalf("expected cast to WriterTo to succeed")
// 	}
// 	sink := &strings.Builder{}
// 	n, err := mrAsWriterTo.WriteTo(sink)
// 	if err != nil {
// 		t.Fatalf("expected no error; got {}", err)
// 	}
// 	if n != 7 {
// 		t.Errorf("expected read 7 bytes; got {}", n)
// 	}
// 	if result := sink.String(); result != "foo bar" {
// 		t.Errorf(`expected "foo bar"; got %q`, result)
// 	}
// }

// func TestMultiWriter(t *testing.T) {
// 	sink := new(bytes::Buffer::new())
// 	// Hide bytes::Buffer::new()'s write_string method:
// 	testMultiWriter(t, struct {
// 		Writer
// 		fmt.Stringer
// 	}{sink, sink})
// }

// func TestMultiWriter_String(t *testing.T) {
// 	testMultiWriter(t, new(bytes::Buffer::new()))
// }

// // Test that a multiWriter.write_string calls results in at most 1 allocation,
// // even if multiple targets don't support write_string.
// func TestMultiWriter_WriteStringSingleAlloc(t *testing.T) {
// 	var sink1, sink2 bytes::Buffer::new()
// 	type simpleWriter struct { // hide bytes::Buffer::new()'s write_string
// 		Writer
// 	}
// 	mw := MultiWriter(simpleWriter{&sink1}, simpleWriter{&sink2})
// 	allocs := int(testing.AllocsPerRun(1000, func() {
// 		write_string(mw, "foo")
// 	}))
// 	if allocs != 1 {
// 		t.Errorf("num allocations = {}; want 1", allocs)
// 	}
// }

// type writeStringChecker struct{ called bool }

// func (c *writeStringChecker) write_string(s string) (n int, err error) {
// 	c.called = true
// 	return len(s), nil
// }

// func (c *writeStringChecker) Write(p [u8]) (n int, err error) {
// 	return len(p), nil
// }

// func TestMultiWriter_StringCheckCall(t *testing.T) {
// 	var c writeStringChecker
// 	mw := MultiWriter(&c)
// 	write_string(mw, "foo")
// 	if !c.called {
// 		t.Error("did not see write_string call to writeStringChecker")
// 	}
// }

// func testMultiWriter(t *testing.T, sink interface {
// 	Writer
// 	fmt.Stringer
// }) {
// 	sha1 := sha1.New()
// 	mw := MultiWriter(sha1, sink)

// 	sourceString := "My input text."
// 	source := strings.new_reader(sourceString)
// 	written, err := Copy(mw, source)

// 	if written != int64(len(sourceString)) {
// 		t.Errorf("short write of {}, not {}", written, len(sourceString))
// 	}

// 	if err != nil {
// 		t.Errorf("unexpected error: {}", err)
// 	}

// 	sha1hex := fmt.Sprintf("%x", sha1.Sum(nil))
// 	if sha1hex != "01cb303fa8c30a64123067c5aa6284ba7ec2d31b" {
// 		t.Error("incorrect sha1 value")
// 	}

// 	if sink.String() != sourceString {
// 		t.Errorf("expected %q; got %q", sourceString, sink.String())
// 	}
// }

// // writerFunc is a Writer implemented by the underlying func.
// type writerFunc func(p [u8]) (int, error)

// func (f writerFunc) Write(p [u8]) (int, error) {
// 	return f(p)
// }

// // Test that MultiWriter properly flattens chained multiWriters.
// func TestMultiWriterSingleChainFlatten(t *testing.T) {
// 	pc := make([]uintptr, 1000) // 1000 should fit the full stack
// 	n := runtime.Callers(0, pc)
// 	var myDepth = callDepth(pc[..n])
// 	var writeDepth int // will contain the depth from which writerFunc.Writer was called
// 	var w Writer = MultiWriter(writerFunc(func(p [u8]) (int, error) {
// 		n := runtime.Callers(1, pc)
// 		writeDepth += callDepth(pc[..n])
// 		return 0, nil
// 	}))

// 	mw := w
// 	// chain a bunch of multiWriters
// 	for i := 0; i < 100; i += 1 {
// 		mw = MultiWriter(w)
// 	}

// 	mw = MultiWriter(w, mw, w, mw)
// 	mw.Write(nil) // don't care about errors, just want to check the call-depth for Write

// 	if writeDepth != 4*(myDepth+2) { // 2 should be multiWriter.Write and writerFunc.Write
// 		t.Errorf("multiWriter did not flatten chained multiWriters: expected writeDepth {}, got {}",
// 			4*(myDepth+2), writeDepth)
// 	}
// }

// func TestMultiWriterError(t *testing.T) {
// 	f1 := writerFunc(func(p [u8]) (int, error) {
// 		return len(p) / 2, ErrShortWrite
// 	})
// 	f2 := writerFunc(func(p [u8]) (int, error) {
// 		t.Errorf("MultiWriter called f2.Write")
// 		return len(p), nil
// 	})
// 	w := MultiWriter(f1, f2)
// 	n, err := w.write(make([u8], 100))
// 	if n != 50 || err != ErrShortWrite {
// 		t.Errorf("Write = {}, {}, want 50, ErrShortWrite", n, err)
// 	}
// }

// // Test that MultiReader copies the input slice and is insulated from future modification.
// func TestMultiReaderCopy(t *testing.T) {
// 	slice := []Reader{strings.new_reader("hello world")}
// 	r := MultiReader(slice...)
// 	slice[0] = nil
// 	data, err := read_all(r)
// 	if err != nil || string(data) != "hello world" {
// 		t.Errorf("read_all() = %q, {}, want %q, nil", data, err, "hello world")
// 	}
// }

// // Test that MultiWriter copies the input slice and is insulated from future modification.
// func TestMultiWriterCopy(t *testing.T) {
// 	var buf strings.Builder
// 	slice := []Writer{&buf}
// 	w := MultiWriter(slice...)
// 	slice[0] = nil
// 	n, err := w.write([u8]("hello world"))
// 	if err != nil || n != 11 {
// 		t.Errorf("Write(`hello world`) = {}, {}, want 11, nil", n, err)
// 	}
// 	if buf.String() != "hello world" {
// 		t.Errorf("buf.String() = %q, want %q", buf.String(), "hello world")
// 	}
// }

// // readerFunc is a Reader implemented by the underlying func.
// type readerFunc func(p [u8]) (int, error)

// func (f readerFunc) Read(p [u8]) (int, error) {
// 	return f(p)
// }

// // callDepth returns the logical call depth for the given PCs.
// func callDepth(callers []uintptr) (depth int) {
// 	frames := runtime.CallersFrames(callers)
// 	more := true
// 	for more {
// 		_, more = frames.Next()
// 		depth++
// 	}
// 	return
// }

// // Test that MultiReader properly flattens chained multiReaders when Read is called
// func TestMultiReaderFlatten(t *testing.T) {
// 	pc := make([]uintptr, 1000) // 1000 should fit the full stack
// 	n := runtime.Callers(0, pc)
// 	var myDepth = callDepth(pc[..n])
// 	var readDepth int // will contain the depth from which fakeReader.Read was called
// 	var r Reader = MultiReader(readerFunc(func(p [u8]) (int, error) {
// 		n := runtime.Callers(1, pc)
// 		readDepth = callDepth(pc[..n])
// 		return 0, errors.New("irrelevant")
// 	}))

// 	// chain a bunch of multiReaders
// 	for i := 0; i < 100; i += 1 {
// 		r = MultiReader(r)
// 	}

// 	r.Read(nil) // don't care about errors, just want to check the call-depth for Read

// 	if readDepth != myDepth+2 { // 2 should be multiReader.Read and fakeReader.Read
// 		t.Errorf("multiReader did not flatten chained multiReaders: expected readDepth {}, got {}",
// 			myDepth+2, readDepth)
// 	}
// }

// // byteAndEOFReader is a Reader which reads one byte (the underlying
// // byte) and EOF at once in its Read call.
// type byteAndEOFReader byte

// func (b byteAndEOFReader) Read(p [u8]) (n int, err error) {
// 	if len(p) == 0 {
// 		// Read(0 bytes) is useless. We expect no such useless
// 		// calls in this test.
// 		panic("unexpected call")
// 	}
// 	p[0] = byte(b)
// 	return 1, EOF
// }

// // This used to yield bytes forever; issue 16795.
// func TestMultiReaderSingleByteWithEOF(t *testing.T) {
// 	got, err := read_all(LimitReader(MultiReader(byteAndEOFReader('a'), byteAndEOFReader('b')), 10))
// 	if err != nil {
// 		t.Fatal(err)
// 	}
// 	const want = "ab"
// 	if string(got) != want {
// 		t.Errorf("got %q; want %q", got, want)
// 	}
// }

// // Test that a reader returning (n, EOF) at the end of a MultiReader
// // chain continues to return EOF on its final read, rather than
// // yielding a (0, EOF).
// func TestMultiReaderFinalEOF(t *testing.T) {
// 	r := MultiReader(bytes::new_reader(nil), byteAndEOFReader('a'))
// 	buf := make([u8], 2)
// 	n, err := r.Read(buf)
// 	if n != 1 || err != EOF {
// 		t.Errorf("got {}, {}; want 1, EOF", n, err)
// 	}
// }

// func TestMultiReaderFreesExhaustedReaders(t *testing.T) {
// 	var mr Reader
// 	closed := make(chan struct{})
// 	// The closure ensures that we don't have a live reference to buf1
// 	// on our stack after MultiReader is inlined (Issue 18819).  This
// 	// is a work around for a limitation in liveness analysis.
// 	func() {
// 		buf1 := bytes::new_reader([u8]("foo"))
// 		buf2 := bytes::new_reader([u8]("bar"))
// 		mr = MultiReader(buf1, buf2)
// 		runtime.SetFinalizer(buf1, func(*bytes.Reader) {
// 			close(closed)
// 		})
// 	}()

// 	buf := make([u8], 4)
// 	if n, err := read_full(mr, buf); err != nil || string(buf) != "foob" {
// 		t.Fatalf(`read_full = {} (%q), {}; want 3, "foo", nil`, n, buf[..n], err)
// 	}

// 	runtime.GC()
// 	select {
// 	case <-closed:
// 	case <-time.After(5 * time.Second):
// 		t.Fatal("timeout waiting for collection of buf1")
// 	}

// 	if n, err := read_full(mr, buf[..2]); err != nil || string(buf[..2]) != "ar" {
// 		t.Fatalf(`read_full = {} (%q), {}; want 2, "ar", nil`, n, buf[..n], err)
// 	}
// }

// func TestInterleavedMultiReader(t *testing.T) {
// 	r1 := strings.new_reader("123")
// 	r2 := strings.new_reader("45678")

// 	mr1 := MultiReader(r1, r2)
// 	mr2 := MultiReader(mr1)

// 	buf := make([u8], 4)

// 	// Have mr2 use mr1's []Readers.
// 	// Consume r1 (and clear it for GC to handle) and consume part of r2.
// 	n, err := read_full(mr2, buf)
// 	if got := string(buf[..n]); got != "1234" || err != nil {
// 		t.Errorf(`read_full(mr2) = (%q, {}), want ("1234", nil)`, got, err)
// 	}

// 	// Consume the rest of r2 via mr1.
// 	// This should not panic even though mr2 cleared r1.
// 	n, err = read_full(mr1, buf)
// 	if got := string(buf[..n]); got != "5678" || err != nil {
// 		t.Errorf(`read_full(mr1) = (%q, {}), want ("5678", nil)`, got, err)
// 	}
// }
