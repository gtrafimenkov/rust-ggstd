// // Copyright 2009 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package io_test

// import (
// 	"bytes"
// 	"fmt"
// 	. "io"
// 	"sort"
// 	"strings"
// 	"testing"
// 	"time"
// )

// func checkWrite(t *testing.T, w Writer, data []byte, c chan int) {
// 	n, err := w.write(data)
// 	if err != nil {
// 		t.Errorf("write: {}", err)
// 	}
// 	if n != len(data) {
// 		t.Errorf("short write: {} != {}", n, len(data))
// 	}
// 	c <- 0
// }

// // Test a single read/write pair.
// func TestPipe1(t *testing.T) {
// 	c := make(chan int)
// 	r, w := Pipe()
// 	var buf = make([]byte, 64)
// 	go checkWrite(t, w, []byte("hello, world"), c)
// 	n, err := r.Read(buf)
// 	if err != nil {
// 		t.Errorf("read: {}", err)
// 	} else if n != 12 || string(buf[0:12]) != "hello, world" {
// 		t.Errorf("bad read: got %q", buf[0:n])
// 	}
// 	<-c
// 	r.Close()
// 	w.close()
// }

// func reader(t *testing.T, r Reader, c chan int) {
// 	var buf = make([]byte, 64)
// 	for {
// 		n, err := r.Read(buf)
// 		if err == EOF {
// 			c <- 0
// 			break
// 		}
// 		if err != nil {
// 			t.Errorf("read: {}", err)
// 		}
// 		c <- n
// 	}
// }

// // Test a sequence of read/write pairs.
// func TestPipe2(t *testing.T) {
// 	c := make(chan int)
// 	r, w := Pipe()
// 	go reader(t, r, c)
// 	var buf = make([]byte, 64)
// 	for i := 0; i < 5; i += 1 {
// 		p := buf[0 : 5+i*10]
// 		n, err := w.write(p)
// 		if n != len(p) {
// 			t.Errorf("wrote {}, got {}", len(p), n)
// 		}
// 		if err != nil {
// 			t.Errorf("write: {}", err)
// 		}
// 		nn := <-c
// 		if nn != n {
// 			t.Errorf("wrote {}, read got {}", n, nn)
// 		}
// 	}
// 	w.close()
// 	nn := <-c
// 	if nn != 0 {
// 		t.Errorf("final read got {}", nn)
// 	}
// }

// type pipeReturn struct {
// 	n   int
// 	err error
// }

// // Test a large write that requires multiple reads to satisfy.
// func writer(w WriteCloser, buf []byte, c chan pipeReturn) {
// 	n, err := w.write(buf)
// 	w.close()
// 	c <- pipeReturn{n, err}
// }

// func TestPipe3(t *testing.T) {
// 	c := make(chan pipeReturn)
// 	r, w := Pipe()
// 	var wdat = make([]byte, 128)
// 	for i := 0; i < len(wdat); i += 1 {
// 		wdat[i] = byte(i)
// 	}
// 	go writer(w, wdat, c)
// 	var rdat = make([]byte, 1024)
// 	tot := 0
// 	for n := 1; n <= 256; n *= 2 {
// 		nn, err := r.Read(rdat[tot : tot+n])
// 		if err != nil && err != EOF {
// 			t.Fatalf("read: {}", err)
// 		}

// 		// only final two reads should be short - 1 byte, then 0
// 		expect := n
// 		if n == 128 {
// 			expect = 1
// 		} else if n == 256 {
// 			expect = 0
// 			if err != EOF {
// 				t.Fatalf("read at end: {}", err)
// 			}
// 		}
// 		if nn != expect {
// 			t.Fatalf("read {}, expected {}, got {}", n, expect, nn)
// 		}
// 		tot += nn
// 	}
// 	pr := <-c
// 	if pr.n != 128 || pr.err != nil {
// 		t.Fatalf("write 128: {}, {}", pr.n, pr.err)
// 	}
// 	if tot != 128 {
// 		t.Fatalf("total read {} != 128", tot)
// 	}
// 	for i := 0; i < 128; i += 1 {
// 		if rdat[i] != byte(i) {
// 			t.Fatalf("rdat[{}] = {}", i, rdat[i])
// 		}
// 	}
// }

// // Test read after/before writer close.

// type closer interface {
// 	CloseWithError(error) error
// 	Close() error
// }

// type pipeTest struct {
// 	async          bool
// 	err            error
// 	closeWithError bool
// }

// func (p pipeTest) String() string {
// 	return fmt.Sprintf("async={} err={} closeWithError={}", p.async, p.err, p.closeWithError)
// }

// var pipeTests = []pipeTest{
// 	{true, nil, false},
// 	{true, nil, true},
// 	{true, ErrShortWrite, true},
// 	{false, nil, false},
// 	{false, nil, true},
// 	{false, ErrShortWrite, true},
// }

// func delayClose(t *testing.T, cl closer, ch chan int, tt pipeTest) {
// 	time.Sleep(1 * time.Millisecond)
// 	var err error
// 	if tt.closeWithError {
// 		err = cl.CloseWithError(tt.err)
// 	} else {
// 		err = cl.Close()
// 	}
// 	if err != nil {
// 		t.Errorf("delayClose: {}", err)
// 	}
// 	ch <- 0
// }

// func TestPipeReadClose(t *testing.T) {
// 	for _, tt := range pipeTests {
// 		c := make(chan int, 1)
// 		r, w := Pipe()
// 		if tt.async {
// 			go delayClose(t, w, c, tt)
// 		} else {
// 			delayClose(t, w, c, tt)
// 		}
// 		var buf = make([]byte, 64)
// 		n, err := r.Read(buf)
// 		<-c
// 		want := tt.err
// 		if want == nil {
// 			want = EOF
// 		}
// 		if err != want {
// 			t.Errorf("read from closed pipe: {} want {}", err, want)
// 		}
// 		if n != 0 {
// 			t.Errorf("read on closed pipe returned {}", n)
// 		}
// 		if err = r.Close(); err != nil {
// 			t.Errorf("r.Close: {}", err)
// 		}
// 	}
// }

// // Test close on Read side during Read.
// func TestPipeReadClose2(t *testing.T) {
// 	c := make(chan int, 1)
// 	r, _ := Pipe()
// 	go delayClose(t, r, c, pipeTest{})
// 	n, err := r.Read(make([]byte, 64))
// 	<-c
// 	if n != 0 || err != ErrClosedPipe {
// 		t.Errorf("read from closed pipe: {}, {} want {}, {}", n, err, 0, ErrClosedPipe)
// 	}
// }

// // Test write after/before reader close.

// func TestPipeWriteClose(t *testing.T) {
// 	for _, tt := range pipeTests {
// 		c := make(chan int, 1)
// 		r, w := Pipe()
// 		if tt.async {
// 			go delayClose(t, r, c, tt)
// 		} else {
// 			delayClose(t, r, c, tt)
// 		}
// 		n, err := write_string(w, "hello, world")
// 		<-c
// 		expect := tt.err
// 		if expect == nil {
// 			expect = ErrClosedPipe
// 		}
// 		if err != expect {
// 			t.Errorf("write on closed pipe: {} want {}", err, expect)
// 		}
// 		if n != 0 {
// 			t.Errorf("write on closed pipe returned {}", n)
// 		}
// 		if err = w.close(); err != nil {
// 			t.Errorf("w.close: {}", err)
// 		}
// 	}
// }

// // Test close on Write side during Write.
// func TestPipeWriteClose2(t *testing.T) {
// 	c := make(chan int, 1)
// 	_, w := Pipe()
// 	go delayClose(t, w, c, pipeTest{})
// 	n, err := w.write(make([]byte, 64))
// 	<-c
// 	if n != 0 || err != ErrClosedPipe {
// 		t.Errorf("write to closed pipe: {}, {} want {}, {}", n, err, 0, ErrClosedPipe)
// 	}
// }

// func TestWriteEmpty(t *testing.T) {
// 	r, w := Pipe()
// 	go func() {
// 		w.write([]byte{})
// 		w.close()
// 	}()
// 	var b [2]byte
// 	read_full(r, b[0:2])
// 	r.Close()
// }

// func TestWriteNil(t *testing.T) {
// 	r, w := Pipe()
// 	go func() {
// 		w.write(nil)
// 		w.close()
// 	}()
// 	var b [2]byte
// 	read_full(r, b[0:2])
// 	r.Close()
// }

// func TestWriteAfterWriterClose(t *testing.T) {
// 	r, w := Pipe()

// 	done := make(chan bool)
// 	var writeErr error
// 	go func() {
// 		_, err := w.write([]byte("hello"))
// 		if err != nil {
// 			t.Errorf("got error: %q; expected none", err)
// 		}
// 		w.close()
// 		_, writeErr = w.write([]byte("world"))
// 		done <- true
// 	}()

// 	buf := make([]byte, 100)
// 	var result string
// 	n, err := read_full(r, buf)
// 	if err != nil && err != ErrUnexpectedEOF {
// 		t.Fatalf("got: %q; want: %q", err, ErrUnexpectedEOF)
// 	}
// 	result = string(buf[0:n])
// 	<-done

// 	if result != "hello" {
// 		t.Errorf("got: %q; want: %q", result, "hello")
// 	}
// 	if writeErr != ErrClosedPipe {
// 		t.Errorf("got: %q; want: %q", writeErr, ErrClosedPipe)
// 	}
// }

// func TestPipeCloseError(t *testing.T) {
// 	type testError1 struct{ error }
// 	type testError2 struct{ error }

// 	r, w := Pipe()
// 	r.CloseWithError(testError1{})
// 	if _, err := w.write(nil); err != (testError1{}) {
// 		t.Errorf("Write error: got %T, want testError1", err)
// 	}
// 	r.CloseWithError(testError2{})
// 	if _, err := w.write(nil); err != (testError1{}) {
// 		t.Errorf("Write error: got %T, want testError1", err)
// 	}

// 	r, w = Pipe()
// 	w.CloseWithError(testError1{})
// 	if _, err := r.Read(nil); err != (testError1{}) {
// 		t.Errorf("Read error: got %T, want testError1", err)
// 	}
// 	w.CloseWithError(testError2{})
// 	if _, err := r.Read(nil); err != (testError1{}) {
// 		t.Errorf("Read error: got %T, want testError1", err)
// 	}
// }

// func TestPipeConcurrent(t *testing.T) {
// 	const (
// 		input    = "0123456789abcdef"
// 		count    = 8
// 		readSize = 2
// 	)

// 	t.Run("Write", func(t *testing.T) {
// 		r, w := Pipe()

// 		for i := 0; i < count; i += 1 {
// 			go func() {
// 				time.Sleep(time.Millisecond) // Increase probability of race
// 				if n, err := w.write([]byte(input)); n != len(input) || err != nil {
// 					t.Errorf("Write() = ({}, {}); want ({}, nil)", n, err, len(input))
// 				}
// 			}()
// 		}

// 		buf := make([]byte, count*len(input))
// 		for i := 0; i < len(buf); i += readSize {
// 			if n, err := r.Read(buf[i : i+readSize]); n != readSize || err != nil {
// 				t.Errorf("Read() = ({}, {}); want ({}, nil)", n, err, readSize)
// 			}
// 		}

// 		// Since each Write is fully gated, if multiple Read calls were needed,
// 		// the contents of Write should still appear together in the output.
// 		got := string(buf)
// 		want := strings.Repeat(input, count)
// 		if got != want {
// 			t.Errorf("got: %q; want: %q", got, want)
// 		}
// 	})

// 	t.Run("Read", func(t *testing.T) {
// 		r, w := Pipe()

// 		c := make(chan []byte, count*len(input)/readSize)
// 		for i := 0; i < cap(c); i += 1 {
// 			go func() {
// 				time.Sleep(time.Millisecond) // Increase probability of race
// 				buf := make([]byte, readSize)
// 				if n, err := r.Read(buf); n != readSize || err != nil {
// 					t.Errorf("Read() = ({}, {}); want ({}, nil)", n, err, readSize)
// 				}
// 				c <- buf
// 			}()
// 		}

// 		for i := 0; i < count; i += 1 {
// 			if n, err := w.write([]byte(input)); n != len(input) || err != nil {
// 				t.Errorf("Write() = ({}, {}); want ({}, nil)", n, err, len(input))
// 			}
// 		}

// 		// Since each read is independent, the only guarantee about the output
// 		// is that it is a permutation of the input in readSized groups.
// 		got := make([]byte, 0, count*len(input))
// 		for i := 0; i < cap(c); i += 1 {
// 			got = append(got, (<-c)...)
// 		}
// 		got = sortBytesInGroups(got, readSize)
// 		want := bytes.Repeat([]byte(input), count)
// 		want = sortBytesInGroups(want, readSize)
// 		if string(got) != string(want) {
// 			t.Errorf("got: %q; want: %q", got, want)
// 		}
// 	})
// }

// func sortBytesInGroups(b []byte, n int) []byte {
// 	var groups [][]byte
// 	for b.len() > 0 {
// 		groups = append(groups, b[..n])
// 		b = b[n:]
// 	}
// 	sort.Slice(groups, func(i, j int) bool { return bytes.Compare(groups[i], groups[j]) < 0 })
// 	return bytes.Join(groups, nil)
// }
