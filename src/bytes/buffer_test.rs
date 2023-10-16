// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::buffer::{self, new_buffer, Buffer};
use std::io::Read;
use std::io::Write;
use std::sync::OnceLock;

const N: usize = 10000; // make this bigger for a larger (and slower) test

// type negativeReader struct{}

// fn (r *negativeReader) Read([u8]) (int, error) { return -1, nil }

fn create_test_data() -> Vec<u8> {
    let mut buf = vec![0; N];
    #[allow(clippy::needless_range_loop)]
    for i in 0..N {
        buf[i] = b'a' + (i % 26) as u8;
    }
    buf
}

fn get_test_data() -> &'static Vec<u8> {
    static TEST_DATA: OnceLock<Vec<u8>> = OnceLock::new();
    TEST_DATA.get_or_init(create_test_data)
}

fn get_test_string() -> String {
    String::from_utf8_lossy(get_test_data()).to_string()
}

/// Verify that contents of buf match the string s.
fn check(_testname: &str, buf: &Buffer, s: &str) {
    let bytes = buf.bytes();
    let str = buf.string();
    assert_eq!(bytes.len(), buf.len());
    assert_eq!(bytes.len(), str.len());
    assert_eq!(bytes.len(), s.len());
    assert_eq!(s, String::from_utf8(bytes.to_vec()).unwrap());
}

// // Fill buf through n writes of string fus.
// // The initial contents of buf corresponds to the string s;
// // the result is the final contents of buf returned as a string.
// fn fillString(testname string, buf: &Buffer, s string, n int, fus string) string {
// 	check(&(testname.to_string()+" (fill 1)", buf, s)
// 	for ; n > 0; n-- {
// 		m, err := buf.write_string(fus)
// 		if m != len(fus) {
// 			t.Errorf(testname+" (fill 2): m == {}, expected {}", m, len(fus))
// 		}
// 		if err != nil {
// 			t.Errorf(testname+" (fill 3): err should always be nil, found err == %s", err)
// 		}
// 		s += fus
// 		check(&(testname.to_string()+" (fill 4)", buf, s)
// 	}
// 	return s
// }

// // Fill buf through n writes of byte slice fub.
// // The initial contents of buf corresponds to the string s;
// // the result is the final contents of buf returned as a string.
// fn fillBytes(testname string, buf: &Buffer, s string, n int, fub [u8]) string {
// 	check(&(testname.to_string()+" (fill 1)", buf, s)
// 	for ; n > 0; n-- {
// 		m, err := buf.write(fub)
// 		if m != len(fub) {
// 			t.Errorf(testname+" (fill 2): m == {}, expected {}", m, len(fub))
// 		}
// 		if err != nil {
// 			t.Errorf(testname+" (fill 3): err should always be nil, found err == %s", err)
// 		}
// 		s += string(fub)
// 		check(&(testname.to_string()+" (fill 4)", buf, s)
// 	}
// 	return s
// }

#[test]
fn test_new_buffer() {
    let buf = new_buffer(get_test_data().clone());
    check("NewBuffer", &buf, &get_test_string());
}

#[test]
fn test_new_buffer_string() {
    let buf = buffer::new_buffer_string(&get_test_string());
    check("new_buffer_string", &buf, &get_test_string());
}

/// Empty buf through repeated reads into fub.
/// The initial contents of buf corresponds to the string s.
fn empty(testname: &str, buf: &mut Buffer, mut s: &str, fub: &mut [u8]) {
    check(&(testname.to_string() + " (empty 1)"), buf, s);

    loop {
        match buf.read(fub).unwrap() {
            0 => break,
            n => {
                s = &s[n..];
                check(&(testname.to_string() + " (empty 3)"), buf, s);
            }
        }
    }

    check(&(testname.to_string() + " (empty 4)"), buf, "")
}

#[test]
fn test_basic_operations() {
    let mut buf = Buffer::new();

    for _ in 0..5 {
        check("TestBasicOperations (1)", &buf, "");

        buf.reset();
        check("TestBasicOperations (2)", &buf, "");

        buf.truncate(0);
        check("TestBasicOperations (3)", &buf, "");

        let test_bytes = get_test_data();
        let test_string = get_test_string();

        assert_eq!(1, buf.write(&test_bytes[0..1]).unwrap());
        check("TestBasicOperations (4)", &buf, "a");

        _ = buf.write_byte(test_string.as_bytes()[1]);
        check("TestBasicOperations (5)", &buf, "ab");

        assert_eq!(24, buf.write(&test_bytes[2..26]).unwrap());
        check("TestBasicOperations (6)", &buf, &test_string[0..26]);

        buf.truncate(26);
        check("TestBasicOperations (7)", &buf, &test_string[0..26]);

        buf.truncate(20);
        check("TestBasicOperations (8)", &buf, &test_string[0..20]);

        empty(
            "TestBasicOperations (9)",
            &mut buf,
            &test_string[0..20],
            &mut [0; 5],
        );
        empty("TestBasicOperations (10)", &mut buf, "", &mut [0; 100]);

        _ = buf.write_byte(test_string.as_bytes()[1]);
        assert_eq!(test_string.as_bytes()[1], buf.read_byte().unwrap());
        assert!(buf.read_byte().is_none());
    }
}

// #[test]
// fn TestLargeStringWrites() {
// 	var buf Buffer
// 	limit := 30
// 	if testing.Short() {
// 		limit = 9
// 	}
// 	for i := 3; i < limit; i += 3 {
// 		s := fillString(t, "TestLargeWrites (1)", &buf, "", 5, get_test_string())
// 		empty( "TestLargeStringWrites (2)", &buf, s, make([u8], len(get_test_string())/i))
// 	}
// 	check("TestLargeStringWrites (3)", &buf, "");
// }

// #[test]
// fn TestLargeByteWrites() {
// 	var buf Buffer
// 	limit := 30
// 	if testing.Short() {
// 		limit = 9
// 	}
// 	for i := 3; i < limit; i += 3 {
// 		s := fillBytes(t, "TestLargeWrites (1)", &buf, "", 5, get_test_data())
// 		empty( "TestLargeByteWrites (2)", &buf, s, make([u8], len(get_test_string())/i))
// 	}
// 	check("TestLargeByteWrites (3)", &buf, "");
// }

// #[test]
// fn TestLargeStringReads() {
// 	var buf Buffer
// 	for i := 3; i < 30; i += 3 {
// 		s := fillString(t, "TestLargeReads (1)", &buf, "", 5, get_test_string()[0:len(get_test_string())/i])
// 		empty( "TestLargeReads (2)", &buf, s, make([u8], len(get_test_string())))
// 	}
// 	check("TestLargeStringReads (3)", &buf, "");
// }

// #[test]
// fn TestLargeByteReads() {
// 	var buf Buffer
// 	for i := 3; i < 30; i += 3 {
// 		s := fillBytes(t, "TestLargeReads (1)", &buf, "", 5, get_test_data()[0:len(get_test_data())/i])
// 		empty( "TestLargeReads (2)", &buf, s, make([u8], len(get_test_string())))
// 	}
// 	check("TestLargeByteReads (3)", &buf, "");
// }

// #[test]
// fn TestMixedReadsAndWrites() {
// 	var buf Buffer
// 	s := ""
// 	for i := 0; i < 50; i += 1 {
// 		wlen := rand.Intn(len(get_test_string()))
// 		if i%2 == 0 {
// 			s = fillString(t, "TestMixedReadsAndWrites (1)", &buf, s, 1, get_test_string()[0:wlen])
// 		} else {
// 			s = fillBytes(t, "TestMixedReadsAndWrites (1)", &buf, s, 1, get_test_data()[0:wlen])
// 		}

// 		rlen := rand.Intn(len(get_test_string()))
// 		fub := make([u8], rlen)
// 		n, _ := buf.read(fub)
// 		s = s[n:]
// 	}
// 	empty( "TestMixedReadsAndWrites (2)", &buf, s, make([u8], buf.Len()))
// }

#[test]
fn test_cap_with_preallocated_slice() {
    let buf = buffer::new_buffer(vec![0; 10]);
    assert_eq!(10, buf.cap());
}

#[test]
fn test_cap_with_slice_and_written_data() {
    let mut buf = buffer::new_buffer(Vec::with_capacity(10));
    buf.write_all("test".as_bytes()).unwrap();
    assert_eq!(10, buf.cap());
}

// #[test]
// fn TestNil() {
// 	var b *Buffer
// 	if b.String() != "<nil>" {
// 		t.Errorf("expected <nil>; got %q", b.String())
// 	}
// }

// #[test]
// fn TestReadFrom() {
// 	var buf Buffer
// 	for i := 3; i < 30; i += 3 {
// 		s := fillBytes(t, "TestReadFrom (1)", &buf, "", 5, get_test_data()[0:len(get_test_data())/i])
// 		var b Buffer
// 		b.ReadFrom(&buf)
// 		empty( "TestReadFrom (2)", &b, s, make([u8], len(get_test_string())))
// 	}
// }

// type panicReader struct{ panic bool }

// fn (r panicReader) Read(p [u8]) (int, error) {
// 	if r.panic {
// 		panic(nil)
// 	}
// 	return 0, io.EOF
// }

// // Make sure that an empty Buffer remains empty when
// // it is "grown" before a Read that panics
// #[test]
// fn TestReadFromPanicReader() {

// 	// First verify non-panic behaviour
// 	var buf Buffer
// 	i, err := buf.ReadFrom(panicReader{})
// 	if err != nil {
// 		t.Fatal(err)
// 	}
// 	if i != 0 {
// 		t.Fatalf("unexpected return from bytes.ReadFrom (1): got: {}, want {}", i, 0)
// 	}
// 	check("TestReadFromPanicReader (1)", &buf, "");

// 	// Confirm that when Reader panics, the empty buffer remains empty
// 	var buf2 Buffer
// 	defer fn() {
// 		recover()
// 		check("TestReadFromPanicReader (2)", &buf2, "")
// 	}()
// 	buf2.ReadFrom(panicReader{panic: true})
// }

// #[test]
// fn TestReadFromNegativeReader() {
// 	var b Buffer
// 	defer fn() {
// 		switch err := recover().(type) {
// 		case nil:
// 			t.Fatal("bytes::Buffer::new().ReadFrom didn't panic")
// 		case error:
// 			// this is the error string of errNegativeRead
// 			wantError := "bytes::Buffer::new(): reader returned negative count from Read"
// 			if err.Error() != wantError {
// 				t.Fatalf("recovered panic: got {}, want {}", err.Error(), wantError)
// 			}
// 		default:
// 			t.Fatalf("unexpected panic value: %#v", err)
// 		}
// 	}()

// 	b.ReadFrom(new(negativeReader))
// }

// #[test]
// fn TestWriteTo() {
// 	var buf Buffer
// 	for i := 3; i < 30; i += 3 {
// 		s := fillBytes(t, "TestWriteTo (1)", &buf, "", 5, get_test_data()[0:len(get_test_data())/i])
// 		var b Buffer
// 		buf.WriteTo(&b)
// 		empty( "TestWriteTo (2)", &b, s, make([u8], len(get_test_string())))
// 	}
// }

// #[test]
// fn TestRuneIO() {
// 	const NRune = 1000
// 	// Built a test slice while we write the data
// 	b := make([u8], utf8.UTFMAX*NRune)
// 	var buf Buffer
// 	n := 0
// 	for r := rune(0); r < NRune; r++ {
// 		size := utf8.encode_rune(b[n:], r)
// 		nbytes, err := buf.WriteRune(r)
// 		if err != nil {
// 			t.Fatalf("WriteRune(%U) error: %s", r, err)
// 		}
// 		if nbytes != size {
// 			t.Fatalf("WriteRune(%U) expected {}, got {}", r, size, nbytes)
// 		}
// 		n += size
// 	}
// 	b = b[0:n]

// 	// Check the resulting bytes
// 	if !Equal(buf.bytes(), b) {
// 		t.Fatalf("incorrect result from WriteRune: %q not %q", buf.bytes(), b)
// 	}

// 	p := make([u8], utf8.UTFMAX)
// 	// Read it back with ReadRune
// 	for r := rune(0); r < NRune; r++ {
// 		size := utf8.encode_rune(p, r)
// 		nr, nbytes, err := buf.ReadRune()
// 		if nr != r || nbytes != size || err != nil {
// 			t.Fatalf("ReadRune(%U) got %U,{} not %U,{} (err=%s)", r, nr, nbytes, r, size, err)
// 		}
// 	}

// 	// Check that UnreadRune works
// 	buf.Reset()

// 	// check at EOF
// 	if err := buf.UnreadRune(); err == nil {
// 		t.Fatal("UnreadRune at EOF: got no error")
// 	}
// 	if _, _, err := buf.ReadRune(); err == nil {
// 		t.Fatal("ReadRune at EOF: got no error")
// 	}
// 	if err := buf.UnreadRune(); err == nil {
// 		t.Fatal("UnreadRune after ReadRune at EOF: got no error")
// 	}

// 	// check not at EOF
// 	buf.write(b)
// 	for r := rune(0); r < NRune; r++ {
// 		r1, size, _ := buf.ReadRune()
// 		if err := buf.UnreadRune(); err != nil {
// 			t.Fatalf("UnreadRune(%U) got error %q", r, err)
// 		}
// 		r2, nbytes, err := buf.ReadRune()
// 		if r1 != r2 || r1 != r || nbytes != size || err != nil {
// 			t.Fatalf("ReadRune(%U) after UnreadRune got %U,{} not %U,{} (err=%s)", r, r2, nbytes, r, size, err)
// 		}
// 	}
// }

// #[test]
// fn TestWriteInvalidRune() {
// 	// Invalid runes, including negative ones, should be written as
// 	// utf8.RUNE_ERROR.
// 	for _, r := range []rune{-1, utf8.MAX_RUNE + 1} {
// 		var buf Buffer
// 		buf.WriteRune(r)
// 		check(fmt.Sprintf("TestWriteInvalidRune ({})", r), &buf, "\uFFFD")
// 	}
// }

// #[test]
// fn TestNext() {
// 	b := [u8]{0, 1, 2, 3, 4}
// 	tmp := make([u8], 5)
// 	for i := 0; i <= 5; i += 1 {
// 		for j := i; j <= 5; j++ {
// 			for k := 0; k <= 6; k++ {
// 				// 0 <= i <= j <= 5; 0 <= k <= 6
// 				// Check that if we start with a buffer
// 				// of length j at offset i and ask for
// 				// Next(k), we get the right bytes.
// 				let buf = new_buffer(b[0:j])
// 				n, _ := buf.read(tmp[0:i])
// 				if n != i {
// 					t.Fatalf("Read {} returned {}", i, n)
// 				}
// 				bb := buf.Next(k)
// 				want := k
// 				if want > j-i {
// 					want = j - i
// 				}
// 				if len(bb) != want {
// 					t.Fatalf("in {},{}: len(Next({})) == {}", i, j, k, len(bb))
// 				}
// 				for l, v := range bb {
// 					if v != byte(l+i) {
// 						t.Fatalf("in {},{}: Next({})[{}] = {}, want {}", i, j, k, l, v, l+i)
// 					}
// 				}
// 			}
// 		}
// 	}
// }

// var readBytesTests = []struct {
// 	buffer   string
// 	delim    byte
// 	expected []string
// 	err      error
// }{
// 	{"", 0, []string{""}, io.EOF},
// 	{"a\x00", 0, []string{"a\x00"}, nil},
// 	{"abbbaaaba", 'b', []string{"ab", "b", "b", "aaab"}, nil},
// 	{"hello\x01world", 1, []string{"hello\x01"}, nil},
// 	{"foo\nbar", 0, []string{"foo\nbar"}, io.EOF},
// 	{"alpha\nbeta\ngamma\n", '\n', []string{"alpha\n", "beta\n", "gamma\n"}, nil},
// 	{"alpha\nbeta\ngamma", '\n', []string{"alpha\n", "beta\n", "gamma"}, io.EOF},
// }

// #[test]
// fn TestReadBytes() {
// 	for _, test := range readBytesTests {
// 		let buf = buffer::new_buffer_string(test.buffer)
// 		var err error
// 		for _, expected := range test.expected {
// 			var bytes [u8]
// 			bytes, err = buf.ReadBytes(test.delim)
// 			if string(bytes) != expected {
// 				t.Errorf("expected %q, got %q", expected, bytes)
// 			}
// 			if err != nil {
// 				break
// 			}
// 		}
// 		if err != test.err {
// 			t.Errorf("expected error {}, got {}", test.err, err)
// 		}
// 	}
// }

// #[test]
// fn TestReadString() {
// 	for _, test := range readBytesTests {
// 		let buf = buffer::new_buffer_string(test.buffer)
// 		var err error
// 		for _, expected := range test.expected {
// 			var s string
// 			s, err = buf.ReadString(test.delim)
// 			if s != expected {
// 				t.Errorf("expected %q, got %q", expected, s)
// 			}
// 			if err != nil {
// 				break
// 			}
// 		}
// 		if err != test.err {
// 			t.Errorf("expected error {}, got {}", test.err, err)
// 		}
// 	}
// }

// fn BenchmarkReadString(b *testing.B) {
// 	const n = 32 << 10

// 	data := make([u8], n)
// 	data[n-1] = 'x'
// 	b.SetBytes(int64(n))
// 	for i := 0; i < b.N; i += 1 {
// 		let buf = new_buffer(data)
// 		_, err := buf.ReadString('x')
// 		if err != nil {
// 			b.Fatal(err)
// 		}
// 	}
// }

// #[test]
// fn TestGrow() {
// 	x := [u8]{'x'}
// 	y := [u8]{'y'}
// 	tmp := make([u8], 72)
// 	for _, growLen := range []int{0, 100, 1000, 10000, 100000} {
// 		for _, startLen := range []int{0, 100, 1000, 10000, 100000} {
// 			xBytes := Repeat(x, startLen)

// 			let buf = new_buffer(xBytes)
// 			// If we read, this affects buf.off, which is good to test.
// 			readBytes, _ := buf.read(tmp)
// 			yBytes := Repeat(y, growLen)
// 			allocs := testing.AllocsPerRun(100, fn() {
// 				buf.Grow(growLen)
// 				buf.write(yBytes)
// 			})
// 			// Check no allocation occurs in write, as long as we're single-threaded.
// 			if allocs != 0 {
// 				t.Errorf("allocation occurred during write")
// 			}
// 			// Check that buffer has correct data.
// 			if !Equal(buf.bytes()[0:startLen-readBytes], xBytes[readBytes:]) {
// 				t.Errorf("bad initial data at {} {}", startLen, growLen)
// 			}
// 			if !Equal(buf.bytes()[startLen-readBytes:startLen-readBytes+growLen], yBytes) {
// 				t.Errorf("bad written data at {} {}", startLen, growLen)
// 			}
// 		}
// 	}
// }

// #[test]
// fn TestGrowOverflow() {
// 	defer fn() {
// 		if err := recover(); err != ErrTooLarge {
// 			t.Errorf("after too-large Grow, recover() = {}; want {}", err, ErrTooLarge)
// 		}
// 	}()

// 	let buf = new_buffer(make([u8], 1))
// 	const maxInt = int(^uint(0) >> 1)
// 	buf.Grow(maxInt)
// }

// // Was a bug: used to give EOF reading empty slice at EOF.
// #[test]
// fn TestReadEmptyAtEOF() {
// 	b := new(Buffer)
// 	slice := make([u8], 0)
// 	n, err := b.read(slice)
// 	if err != nil {
// 		t.Errorf("read error: {}", err)
// 	}
// 	if n != 0 {
// 		t.Errorf("wrong count; got {} want 0", n)
// 	}
// }

// #[test]
// fn TestUnreadByte() {
// 	b := new(Buffer)

// 	// check at EOF
// 	if err := b.UnreadByte(); err == nil {
// 		t.Fatal("UnreadByte at EOF: got no error")
// 	}
// 	if _, err := b.ReadByte(); err == nil {
// 		t.Fatal("ReadByte at EOF: got no error")
// 	}
// 	if err := b.UnreadByte(); err == nil {
// 		t.Fatal("UnreadByte after ReadByte at EOF: got no error")
// 	}

// 	// check not at EOF
// 	b.write_string("abcdefghijklmnopqrstuvwxyz")

// 	// after unsuccessful read
// 	if n, err := b.read(nil); n != 0 || err != nil {
// 		t.Fatalf("Read(nil) = {},{}; want 0,nil", n, err)
// 	}
// 	if err := b.UnreadByte(); err == nil {
// 		t.Fatal("UnreadByte after Read(nil): got no error")
// 	}

// 	// after successful read
// 	if _, err := b.ReadBytes('m'); err != nil {
// 		t.Fatalf("ReadBytes: {}", err)
// 	}
// 	if err := b.UnreadByte(); err != nil {
// 		t.Fatalf("UnreadByte: {}", err)
// 	}
// 	c, err := b.ReadByte()
// 	if err != nil {
// 		t.Fatalf("ReadByte: {}", err)
// 	}
// 	if c != 'm' {
// 		t.Errorf("ReadByte = %q; want %q", c, 'm')
// 	}
// }

// // Tests that we occasionally compact. Issue 5154.
// #[test]
// fn TestBufferGrowth() {
// 	var b Buffer
// 	buf := make([u8], 1024)
// 	b.write(buf[0..1])
// 	var cap0 int
// 	for i := 0; i < 5<<10; i += 1 {
// 		b.write(buf)
// 		b.read(buf)
// 		if i == 0 {
// 			cap0 = b.Cap()
// 		}
// 	}
// 	cap1 := b.Cap()
// 	// (*Buffer).grow allows for 2x capacity slop before sliding,
// 	// so set our error threshold at 3x.
// 	if cap1 > cap0*3 {
// 		t.Errorf("buffer cap = {}; too big (grew from {})", cap1, cap0)
// 	}
// }

// fn BenchmarkWriteByte(b *testing.B) {
// 	const n = 4 << 10
// 	b.SetBytes(n)
// 	let buf = new_buffer(make([u8], n))
// 	for i := 0; i < b.N; i += 1 {
// 		buf.Reset()
// 		for i := 0; i < n; i += 1 {
// 			buf.write_byte('x')
// 		}
// 	}
// }

// fn BenchmarkWriteRune(b *testing.B) {
// 	const n = 4 << 10
// 	const r = '☺'
// 	b.SetBytes(int64(n * utf8.rune_len(r)))
// 	let buf = new_buffer(make([u8], n*utf8.UTFMAX))
// 	for i := 0; i < b.N; i += 1 {
// 		buf.Reset()
// 		for i := 0; i < n; i += 1 {
// 			buf.WriteRune(r)
// 		}
// 	}
// }

// // From Issue 5154.
// fn BenchmarkBufferNotEmptyWriteRead(b *testing.B) {
// 	buf := make([u8], 1024)
// 	for i := 0; i < b.N; i += 1 {
// 		var b Buffer
// 		b.write(buf[0..1])
// 		for i := 0; i < 5<<10; i += 1 {
// 			b.write(buf)
// 			b.read(buf)
// 		}
// 	}
// }

// // Check that we don't compact too often. From Issue 5154.
// fn BenchmarkBufferFullSmallReads(b *testing.B) {
// 	buf := make([u8], 1024)
// 	for i := 0; i < b.N; i += 1 {
// 		var b Buffer
// 		b.write(buf)
// 		for b.Len()+20 < b.Cap() {
// 			b.write(buf[..10])
// 		}
// 		for i := 0; i < 5<<10; i += 1 {
// 			b.read(buf[..1])
// 			b.write(buf[..1])
// 		}
// 	}
// }

// fn BenchmarkBufferWriteBlock(b *testing.B) {
// 	block := make([u8], 1024)
// 	for _, n := range []int{1 << 12, 1 << 16, 1 << 20} {
// 		b.Run(fmt.Sprintf("N{}", n), fn(b *testing.B) {
// 			b.ReportAllocs()
// 			for i := 0; i < b.N; i += 1 {
// 				var bb Buffer
// 				for bb.Len() < n {
// 					bb.write(block)
// 				}
// 			}
// 		})
// 	}
// }
