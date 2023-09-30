// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::io as ggio;
use crate::bytes;
use std::io::Write;

struct UnexpectedEOFErrorReader {}

impl std::io::Read for UnexpectedEOFErrorReader {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
    }
}

#[test]
fn test_read_all() {
    const MESSAGE: &str = "hello there";

    let mut r = bytes::new_buffer_string(MESSAGE);
    let (data, err) = ggio::read_all(&mut r);
    assert_eq!(data.as_slice(), MESSAGE.as_bytes());
    assert!(err.is_none(), "not expecting an error");

    let mut r = UnexpectedEOFErrorReader {};
    let (data, err) = ggio::read_all(&mut r);
    assert_eq!(0, data.len());
    assert!(
        err.is_some_and(|e| e.kind() == std::io::ErrorKind::UnexpectedEof),
        "expecting ErrUnexpectedEOF"
    );
}

// ggrust: ReadFrom and WriteTo optimizations are not implemented
// // A version of bytes::Buffer::new() without ReadFrom and WriteTo
// type Buffer struct {
// 	bytes::Buffer::new()
// 	ReaderFrom // conflicts with and hides bytes::Buffer::new()'s ReaderFrom.
// 	WriterTo   // conflicts with and hides bytes::Buffer::new()'s WriterTo.
// }

// Simple tests, primarily to verify the ReadFrom and WriteTo callouts inside Copy, CopyBuffer and copy_n.

#[test]
fn test_copy() {
    let mut rb = bytes::Buffer::new();
    let mut wb = bytes::Buffer::new();
    rb.write_string("hello, world.").unwrap();
    ggio::copy(&mut wb, &mut rb);
    assert_eq!("hello, world.", wb.string(), "Copy did not work properly");
}

#[test]
fn test_copy_buffer() {
    let mut rb = bytes::Buffer::new();
    let mut wb = bytes::Buffer::new();
    rb.write_string("hello, world.").unwrap();
    let mut buf = [0_u8];
    ggio::copy_buffer(&mut wb, &mut rb, Some(&mut buf)); // Tiny buffer to keep it honest.
    assert_eq!(
        "hello, world.",
        wb.string(),
        "copy_buffer did not work properly"
    );
}

#[test]
fn test_copy_buffer_nil() {
    let mut rb = bytes::Buffer::new();
    let mut wb = bytes::Buffer::new();
    rb.write_string("hello, world.").unwrap();
    ggio::copy_buffer(&mut wb, &mut rb, None);
    assert_eq!(
        "hello, world.",
        wb.string(),
        "copy_buffer did not work properly"
    );
}
// ggrust: ReadFrom and WriteTo optimizations are not implemented
// fn TestCopyReadFrom() {
// 	rb := new(Buffer)
// 	wb := new(bytes::Buffer::new()) // implements ReadFrom.
// 	rb.write_string("hello, world.")
// 	Copy(wb, rb)
// 	if wb.String() != "hello, world." {
// 		t.Errorf("Copy did not work properly")
// 	}
// }

// ggrust: ReadFrom and WriteTo optimizations are not implemented
// fn TestCopyWriteTo() {
// 	rb := new(bytes::Buffer::new()) // implements WriteTo.
// 	wb := new(Buffer)
// 	rb.write_string("hello, world.")
// 	Copy(wb, rb)
// 	if wb.String() != "hello, world." {
// 		t.Errorf("Copy did not work properly")
// 	}
// }

// // Version of bytes::Buffer::new() that checks whether WriteTo was called or not
// type writeToChecker struct {
// 	bytes::Buffer::new()
// 	writeToCalled bool
// }

// fn (wt *writeToChecker) WriteTo(w Writer) (int64, error) {
// 	wt.writeToCalled = true
// 	return wt.Buffer.WriteTo(w)
// }

// // It's preferable to choose WriterTo over ReaderFrom, since a WriterTo can issue one large write,
// // while the ReaderFrom must read until EOF, potentially allocating when running out of buffer.
// // Make sure that we choose WriterTo when both are implemented.
// fn TestCopyPriority() {
// 	rb := new(writeToChecker)
// 	wb := new(bytes::Buffer::new())
// 	rb.write_string("hello, world.")
// 	Copy(wb, rb)
// 	if wb.String() != "hello, world." {
// 		t.Errorf("Copy did not work properly")
// 	} else if !rb.writeToCalled {
// 		t.Errorf("WriteTo was not prioritized over ReadFrom")
// 	}
// }

// type zeroErrReader struct {
// 	err error
// }

// fn (r zeroErrReader) Read(p [u8]) (int, error) {
// 	return copy(p, [u8]{0}), r.err
// }

// type errWriter struct {
// 	err error
// }

// fn (w errWriter) Write([u8]) (int, error) {
// 	return 0, w.err
// }

// // In case a Read results in an error with non-zero bytes read, and
// // the subsequent Write also results in an error, the error from Write
// // is returned, as it is the one that prevented progressing further.
// fn TestCopyReadErrWriteErr() {
// 	er, ew := errors.New("readError"), errors.New("writeError")
// 	r, w := zeroErrReader{err: er}, errWriter{err: ew}
// 	n, err := Copy(w, r)
// 	if n != 0 || err != ew {
// 		t.Errorf("Copy(zeroErrReader, errWriter) = {}, {}; want 0, writeError", n, err)
// 	}
// }

#[test]
fn test_copy_n() {
    let mut rb = bytes::Buffer::new();
    let mut wb = bytes::Buffer::new();
    rb.write_string("hello, world.").unwrap();
    ggio::copy_n(&mut wb, &mut rb, 5);
    assert_eq!("hello", wb.string(), "copy_n did not work properly");
}

// fn TestCopyNReadFrom() {
// 	rb := new(Buffer)
// 	wb := new(bytes::Buffer::new()) // implements ReadFrom.
// 	rb.write_string("hello")
// 	copy_n(wb, rb, 5)
// 	if wb.String() != "hello" {
// 		t.Errorf("copy_n did not work properly")
// 	}
// }

// fn TestCopyNWriteTo() {
// 	rb := new(bytes::Buffer::new()) // implements WriteTo.
// 	wb := new(Buffer)
// 	rb.write_string("hello, world.")
// 	copy_n(wb, rb, 5)
// 	if wb.String() != "hello" {
// 		t.Errorf("copy_n did not work properly")
// 	}
// }

// fn BenchmarkCopyNSmall(b *testing.B) {
// 	bs := bytes.Repeat([u8]{0}, 512+1)
// 	rd := bytes::Reader::new(bs)
// 	buf := new(Buffer)
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i += 1 {
// 		copy_n(buf, rd, 512)
// 		rd.Reset(bs)
// 	}
// }

// fn BenchmarkCopyNLarge(b *testing.B) {
// 	bs := bytes.Repeat([u8]{0}, (32*1024)+1)
// 	rd := bytes::Reader::new(bs)
// 	buf := new(Buffer)
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i += 1 {
// 		copy_n(buf, rd, 32*1024)
// 		rd.Reset(bs)
// 	}
// }

// type noReadFrom struct {
// 	w Writer
// }

// fn (w *noReadFrom) Write(p [u8]) (n int, err error) {
// 	return w.w.write(p)
// }

// type wantedAndErrReader struct{}

// fn (wantedAndErrReader) Read(p [u8]) (int, error) {
// 	return len(p), errors.New("wantedAndErrReader error")
// }

// fn TestCopyNEOF() {
// 	// Test that EOF behavior is the same regardless of whether
// 	// argument to copy_n has ReadFrom.

// 	b := new(bytes::Buffer::new())

// 	n, err := copy_n(&noReadFrom{b}, strings::Reader::new("foo"), 3)
// 	if n != 3 || err != nil {
// 		t.Errorf("copy_n(noReadFrom, foo, 3) = {}, {}; want 3, nil", n, err)
// 	}

// 	n, err = copy_n(&noReadFrom{b}, strings::Reader::new("foo"), 4)
// 	if n != 3 || err != EOF {
// 		t.Errorf("copy_n(noReadFrom, foo, 4) = {}, {}; want 3, EOF", n, err)
// 	}

// 	n, err = copy_n(b, strings::Reader::new("foo"), 3) // b has read from
// 	if n != 3 || err != nil {
// 		t.Errorf("copy_n(bytes::Buffer::new(), foo, 3) = {}, {}; want 3, nil", n, err)
// 	}

// 	n, err = copy_n(b, strings::Reader::new("foo"), 4) // b has read from
// 	if n != 3 || err != EOF {
// 		t.Errorf("copy_n(bytes::Buffer::new(), foo, 4) = {}, {}; want 3, EOF", n, err)
// 	}

// 	n, err = copy_n(b, wantedAndErrReader{}, 5)
// 	if n != 5 || err != nil {
// 		t.Errorf("copy_n(bytes::Buffer::new(), wantedAndErrReader, 5) = {}, {}; want 5, nil", n, err)
// 	}

// 	n, err = copy_n(&noReadFrom{b}, wantedAndErrReader{}, 5)
// 	if n != 5 || err != nil {
// 		t.Errorf("copy_n(noReadFrom, wantedAndErrReader, 5) = {}, {}; want 5, nil", n, err)
// 	}
// }

#[test]
fn test_read_at_least() {
    let mut rb = bytes::Buffer::new();
    test_read_at_least_int(&mut rb);
}

// // A version of bytes::Buffer::new() that returns n > 0, err on Read
// // when the input is exhausted.
// type dataAndErrorBuffer struct {
// 	err error
// 	bytes::Buffer::new()
// }

// fn (r *dataAndErrorBuffer) Read(p [u8]) (n int, err error) {
// 	n, err = r.Buffer.Read(p)
// 	if n > 0 && r.Buffer.Len() == 0 && err == nil {
// 		err = r.err
// 	}
// 	return
// }

// fn TestReadAtLeastWithDataAndEOF() {
// 	var rb dataAndErrorBuffer
// 	rb.err = EOF
// 	test_read_at_least_int(t, &rb)
// }

// fn TestReadAtLeastWithDataAndError() {
// 	var rb dataAndErrorBuffer
// 	rb.err = fmt.Errorf("fake error")
// 	test_read_at_least_int(t, &rb)
// }

fn test_read_at_least_int(rb: &mut bytes::Buffer) {
    // fn test_read_at_least_int(rb: &mut dyn ReadWriter) {
    rb.write("0123".as_bytes()).unwrap();
    let buf = &mut [0; 2];

    let (n, err) = ggio::read_at_least(rb, buf, 2);
    assert!(err.is_none());
    assert_eq!(2, n, "expected to have read 2 bytes, got {}", n);

    // ErrorKind::InvalidInput if buffer is too small
    let (n, err) = ggio::read_at_least(rb, buf, 4);
    assert!(
        err.as_ref().unwrap().kind() == std::io::ErrorKind::InvalidInput,
        "expected ErrorKind::InvalidInput got {:?}",
        err
    );
    assert_eq!(0, n, "expected to have read 0 bytes, got {}", n);

    let (n, err) = ggio::read_at_least(rb, buf, 1);
    assert!(err.is_none());
    assert_eq!(2, n, "expected to have read 2 bytes, got {}", n);

    let (n, err) = ggio::read_at_least(rb, buf, 2);
    assert!(
        err.as_ref().unwrap().kind() == std::io::ErrorKind::UnexpectedEof,
        "expected EOF got {:?}",
        err
    );
    assert_eq!(0, n, "expected to have read 0 bytes, got {}", n);

    rb.write("4".as_bytes()).unwrap();
    let (n, err) = ggio::read_at_least(rb, buf, 2);
    assert!(err.as_ref().unwrap().kind() == std::io::ErrorKind::UnexpectedEof);
    assert_eq!(1, n, "expected to have read 1 bytes, got {}", n);
}

// fn TestTeeReader() {
// 	src := [u8]("hello, world")
// 	dst := make([u8], len(src))
// 	rb := bytes::new_buffer(src)
// 	wb := new(bytes::Buffer::new())
// 	r := TeeReader(rb, wb)
// 	if n, err := read_full(r, dst); err != nil || n != len(src) {
// 		t.Fatalf("read_full(r, dst) = {}, {}; want {}, nil", n, err, len(src))
// 	}
// 	if !bytes::equal(dst, src) {
// 		t.Errorf("bytes read = %q want %q", dst, src)
// 	}
// 	if !bytes::equal(wb.bytes(), src) {
// 		t.Errorf("bytes written = %q want %q", wb.bytes(), src)
// 	}
// 	if n, err := r.Read(dst); n != 0 || err != EOF {
// 		t.Errorf("r.Read at EOF = {}, {} want 0, EOF", n, err)
// 	}
// 	rb = bytes::new_buffer(src)
// 	pr, pw := Pipe()
// 	pr.Close()
// 	r = TeeReader(rb, pw)
// 	if n, err := read_full(r, dst); n != 0 || err != ErrClosedPipe {
// 		t.Errorf("closed tee: read_full(r, dst) = {}, {}; want 0, EPIPE", n, err)
// 	}
// }

// fn TestSectionReader_ReadAt() {
// 	dat := "a long sample data, 1234567890"
// 	tests := []struct {
// 		data   string
// 		off    int
// 		n      int
// 		bufLen int
// 		at     int
// 		exp    string
// 		err    error
// 	}{
// 		{data: "", off: 0, n: 10, bufLen: 2, at: 0, exp: "", err: EOF},
// 		{data: dat, off: 0, n: len(dat), bufLen: 0, at: 0, exp: "", err: nil},
// 		{data: dat, off: len(dat), n: 1, bufLen: 1, at: 0, exp: "", err: EOF},
// 		{data: dat, off: 0, n: len(dat) + 2, bufLen: len(dat), at: 0, exp: dat, err: nil},
// 		{data: dat, off: 0, n: len(dat), bufLen: len(dat) / 2, at: 0, exp: dat[..len(dat)/2], err: nil},
// 		{data: dat, off: 0, n: len(dat), bufLen: len(dat), at: 0, exp: dat, err: nil},
// 		{data: dat, off: 0, n: len(dat), bufLen: len(dat) / 2, at: 2, exp: dat[2 : 2+len(dat)/2], err: nil},
// 		{data: dat, off: 3, n: len(dat), bufLen: len(dat) / 2, at: 2, exp: dat[5 : 5+len(dat)/2], err: nil},
// 		{data: dat, off: 3, n: len(dat) / 2, bufLen: len(dat)/2 - 2, at: 2, exp: dat[5 : 5+len(dat)/2-2], err: nil},
// 		{data: dat, off: 3, n: len(dat) / 2, bufLen: len(dat)/2 + 2, at: 2, exp: dat[5 : 5+len(dat)/2-2], err: EOF},
// 		{data: dat, off: 0, n: 0, bufLen: 0, at: -1, exp: "", err: EOF},
// 		{data: dat, off: 0, n: 0, bufLen: 0, at: 1, exp: "", err: EOF},
// 	}
// 	for i, tt := range tests {
// 		r := strings::Reader::new(tt.data)
// 		s := NewSectionReader(r, int64(tt.off), int64(tt.n))
// 		buf := make([u8], tt.bufLen)
// 		if n, err := s.ReadAt(buf, int64(tt.at)); n != len(tt.exp) || string(buf[..n]) != tt.exp || err != tt.err {
// 			t.Fatalf("{}: ReadAt({}) = %q, {}; expected %q, {}", i, tt.at, buf[..n], err, tt.exp, tt.err)
// 		}
// 	}
// }

// fn TestSectionReader_Seek() {
// 	// Verifies that NewSectionReader's Seeker behaves like bytes::new_reader (which is like strings::Reader::new)
// 	br := bytes::Reader::new([u8]("foo"))
// 	sr := NewSectionReader(br, 0, int64(len("foo")))

// 	for _, whence := range []int{SeekStart, SeekCurrent, SeekEnd} {
// 		for offset := int64(-3); offset <= 4; offset++ {
// 			brOff, brErr := br.Seek(offset, whence)
// 			srOff, srErr := sr.Seek(offset, whence)
// 			if (brErr != nil) != (srErr != nil) || brOff != srOff {
// 				t.Errorf("For whence {}, offset {}: bytes.Reader.Seek = ({}, {}) != SectionReader.Seek = ({}, {})",
// 					whence, offset, brOff, brErr, srErr, srOff)
// 			}
// 		}
// 	}

// 	// And verify we can just seek past the end and get an EOF
// 	got, err := sr.Seek(100, SeekStart)
// 	if err != nil || got != 100 {
// 		t.Errorf("Seek = {}, {}; want 100, nil", got, err)
// 	}

// 	n, err := sr.Read(make([u8], 10))
// 	if n != 0 || err != EOF {
// 		t.Errorf("Read = {}, {}; want 0, EOF", n, err)
// 	}
// }

// fn TestSectionReader_Size() {
// 	tests := []struct {
// 		data string
// 		want int64
// 	}{
// 		{"a long sample data, 1234567890", 30},
// 		{"", 0},
// 	}

// 	for _, tt := range tests {
// 		r := strings::Reader::new(tt.data)
// 		sr := NewSectionReader(r, 0, int64(len(tt.data)))
// 		if got := sr.size(); got != tt.want {
// 			t.Errorf("Size = {}; want {}", got, tt.want)
// 		}
// 	}
// }

// fn TestSectionReader_Max() {
// 	r := strings::Reader::new("abcdef")
// 	const maxint64 = 1<<63 - 1
// 	sr := NewSectionReader(r, 3, maxint64)
// 	n, err := sr.Read(make([u8], 3))
// 	if n != 3 || err != nil {
// 		t.Errorf("Read = {} {}, want 3, nil", n, err)
// 	}
// 	n, err = sr.Read(make([u8], 3))
// 	if n != 0 || err != EOF {
// 		t.Errorf("Read = {}, {}, want 0, EOF", n, err)
// 	}
// }

// // largeWriter returns an invalid count that is larger than the number
// // of bytes provided (issue 39978).
// type largeWriter struct {
// 	err error
// }

// fn (w largeWriter) Write(p [u8]) (int, error) {
// 	return len(p) + 1, w.err
// }

// fn TestCopyLargeWriter() {
// 	want := ErrInvalidWrite
// 	rb := new(Buffer)
// 	wb := largeWriter{}
// 	rb.write_string("hello, world.")
// 	if _, err := Copy(wb, rb); err != want {
// 		t.Errorf("Copy error: got {}, want {}", err, want)
// 	}

// 	want = errors.New("largeWriterError")
// 	rb = new(Buffer)
// 	wb = largeWriter{err: want}
// 	rb.write_string("hello, world.")
// 	if _, err := Copy(wb, rb); err != want {
// 		t.Errorf("Copy error: got {}, want {}", err, want)
// 	}
// }

// fn TestNopCloserWriterToForwarding() {
// 	for _, tc := range [...]struct {
// 		Name string
// 		r    Reader
// 	}{
// 		{"not a WriterTo", Reader(nil)},
// 		{"a WriterTo", struct {
// 			Reader
// 			WriterTo
// 		}{}},
// 	} {
// 		nc := NopCloser(tc.r)

// 		_, expected := tc.r.(WriterTo)
// 		_, got := nc.(WriterTo)
// 		if expected != got {
// 			t.Errorf("NopCloser incorrectly forwards WriterTo for %s, got %t want %t", tc.Name, got, expected)
// 		}
// 	}
// }

// fn TestOffsetWriter_Seek() {
// 	tmpfilename := "TestOffsetWriter_Seek"
// 	tmpfile, err := os.CreateTemp(t.TempDir(), tmpfilename)
// 	if err != nil || tmpfile == nil {
// 		t.Fatalf("CreateTemp(%s) failed: {}", tmpfilename, err)
// 	}
// 	defer tmpfile.Close()
// 	w := NewOffsetWriter(tmpfile, 0)

// 	// Should throw error errWhence if whence is not valid
// 	t.Run("errWhence", fn() {
// 		for _, whence := range []int{-3, -2, -1, 3, 4, 5} {
// 			var offset int64 = 0
// 			gotOff, gotErr := w.Seek(offset, whence)
// 			if gotOff != 0 || gotErr != ErrWhence {
// 				t.Errorf("For whence {}, offset {}, OffsetWriter.Seek got: ({}, {}), want: ({}, {})",
// 					whence, offset, gotOff, gotErr, 0, ErrWhence)
// 			}
// 		}
// 	})

// 	// Should throw error errOffset if offset is negative
// 	t.Run("errOffset", fn() {
// 		for _, whence := range []int{SeekStart, SeekCurrent} {
// 			for offset := int64(-3); offset < 0; offset++ {
// 				gotOff, gotErr := w.Seek(offset, whence)
// 				if gotOff != 0 || gotErr != ErrOffset {
// 					t.Errorf("For whence {}, offset {}, OffsetWriter.Seek got: ({}, {}), want: ({}, {})",
// 						whence, offset, gotOff, gotErr, 0, ErrOffset)
// 				}
// 			}
// 		}
// 	})

// 	// Normal tests
// 	t.Run("normal", fn() {
// 		tests := []struct {
// 			offset    int64
// 			whence    int
// 			returnOff int64
// 		}{
// 			// keep in order
// 			{whence: SeekStart, offset: 1, returnOff: 1},
// 			{whence: SeekStart, offset: 2, returnOff: 2},
// 			{whence: SeekStart, offset: 3, returnOff: 3},
// 			{whence: SeekCurrent, offset: 1, returnOff: 4},
// 			{whence: SeekCurrent, offset: 2, returnOff: 6},
// 			{whence: SeekCurrent, offset: 3, returnOff: 9},
// 		}
// 		for idx, tt := range tests {
// 			gotOff, gotErr := w.Seek(tt.offset, tt.whence)
// 			if gotOff != tt.returnOff || gotErr != nil {
// 				t.Errorf("{}:: For whence {}, offset {}, OffsetWriter.Seek got: ({}, {}), want: ({}, <nil>)",
// 					idx+1, tt.whence, tt.offset, gotOff, gotErr, tt.returnOff)
// 			}
// 		}
// 	})
// }

// fn TestOffsetWriter_WriteAt() {
// 	const content = "0123456789ABCDEF"
// 	contentSize := int64(len(content))
// 	tmpdir, err := os.MkdirTemp(t.TempDir(), "TestOffsetWriter_WriteAt")
// 	if err != nil {
// 		t.Fatal(err)
// 	}

// 	work := fn(off, at int64) {
// 		position := fmt.Sprintf("off_%d_at_{}", off, at)
// 		tmpfile, err := os.CreateTemp(tmpdir, position)
// 		if err != nil || tmpfile == nil {
// 			t.Fatalf("CreateTemp(%s) failed: {}", position, err)
// 		}
// 		defer tmpfile.Close()

// 		var writeN int64
// 		var wg sync.WaitGroup
// 		// Concurrent writes, one byte at a time
// 		for step, value := range [u8](content) {
// 			wg.Add(1)
// 			go fn(wg *sync.WaitGroup, tmpfile *os.File, value byte, off, at int64, step int) {
// 				defer wg.Done()

// 				w := NewOffsetWriter(tmpfile, off)
// 				n, e := w.WriteAt([u8]{value}, at+int64(step))
// 				if e != nil {
// 					t.Errorf("WriteAt failed. off: {}, at: {}, step: {}\n error: {}", off, at, step, e)
// 				}
// 				atomic.AddInt64(&writeN, int64(n))
// 			}(&wg, tmpfile, value, off, at, step)
// 		}
// 		wg.Wait()

// 		// Read one more byte to reach EOF
// 		buf := make([u8], contentSize+1)
// 		readN, err := tmpfile.ReadAt(buf, off+at)
// 		if err != EOF {
// 			t.Fatalf("ReadAt failed: {}", err)
// 		}
// 		readContent := string(buf[..contentSize])
// 		if writeN != int64(readN) || writeN != contentSize || readContent != content {
// 			t.Fatalf("%s:: WriteAt(%s, {}) error. \ngot n: {}, content: %s \nexpected n: {}, content: {}",
// 				position, content, at, readN, readContent, contentSize, content)
// 		}
// 	}
// 	for off := int64(0); off < 2; off++ {
// 		for at := int64(0); at < 2; at++ {
// 			work(off, at)
// 		}
// 	}
// }

// fn TestOffsetWriter_Write() {
// 	const content = "0123456789ABCDEF"
// 	contentSize := len(content)
// 	tmpdir := t.TempDir()

// 	makeOffsetWriter := fn(name string) (*OffsetWriter, *os.File) {
// 		tmpfilename := "TestOffsetWriter_Write_" + name
// 		tmpfile, err := os.CreateTemp(tmpdir, tmpfilename)
// 		if err != nil || tmpfile == nil {
// 			t.Fatalf("CreateTemp(%s) failed: {}", tmpfilename, err)
// 		}
// 		return NewOffsetWriter(tmpfile, 0), tmpfile
// 	}
// 	checkContent := fn(name string, f *os.File) {
// 		// Read one more byte to reach EOF
// 		buf := make([u8], contentSize+1)
// 		readN, err := f.ReadAt(buf, 0)
// 		if err != EOF {
// 			t.Fatalf("ReadAt failed, err: {}", err)
// 		}
// 		readContent := string(buf[..contentSize])
// 		if readN != contentSize || readContent != content {
// 			t.Fatalf("%s error. \ngot n: {}, content: %s \nexpected n: {}, content: {}",
// 				name, readN, readContent, contentSize, content)
// 		}
// 	}

// 	var name string
// 	name = "Write"
// 	t.Run(name, fn() {
// 		// Write directly (off: 0, at: 0)
// 		// Write content to file
// 		w, f := makeOffsetWriter(name)
// 		defer f.Close()
// 		for _, value := range [u8](content) {
// 			n, err := w.write([u8]{value})
// 			if err != nil {
// 				t.Fatalf("Write failed, n: {}, err: {}", n, err)
// 			}
// 		}
// 		checkContent(name, f)

// 		// Copy -> Write
// 		// Copy file f to file f2
// 		name = "Copy"
// 		w2, f2 := makeOffsetWriter(name)
// 		defer f2.Close()
// 		Copy(w2, f)
// 		checkContent(name, f2)
// 	})

// 	// Copy -> WriteTo -> Write
// 	// Note: strings.Reader implements the io.WriterTo interface.
// 	name = "Write_Of_Copy_WriteTo"
// 	t.Run(name, fn() {
// 		w, f := makeOffsetWriter(name)
// 		defer f.Close()
// 		Copy(w, strings::Reader::new(content))
// 		checkContent(name, f)
// 	})
// }
