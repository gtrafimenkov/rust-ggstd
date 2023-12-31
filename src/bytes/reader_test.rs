// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::reader::Reader;
use crate::io::{self as ggio, ByteReader};
use std::io::Read;

#[test]
fn test_reader() {
    let mut r = Reader::new("0123456789".as_bytes());
    struct Test {
        off: i64,
        seek: ggio::Seek,
        n: usize,
        want: &'static [u8],
        wantpos: u64,
        readerr: Option<&'static str>,
        seekerr: Option<&'static str>,
    }

    let tests = [
        Test {
            seek: ggio::Seek::Start,
            off: 0,
            n: 20,
            want: "0123456789".as_bytes(),
            wantpos: 0,
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::Start,
            off: 1,
            n: 1,
            want: "1".as_bytes(),
            wantpos: 0,
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::Current,
            off: 1,
            wantpos: 3,
            n: 2,
            want: "34".as_bytes(),
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::Start,
            off: -1,
            n: 0,
            want: &[],
            wantpos: 0,
            readerr: None,
            seekerr: Some("bytes::Reader::seek: negative position"),
        },
        Test {
            seek: ggio::Seek::Start,
            off: 1 << 33,
            wantpos: 1 << 33,
            n: 0,
            want: &[],
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::Current,
            off: 1,
            wantpos: (1 << 33) + 1,
            n: 0,
            want: &[],
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::Start,
            n: 5,
            want: "01234".as_bytes(),
            off: 0,
            wantpos: 0,
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::Current,
            n: 5,
            want: "56789".as_bytes(),
            off: 0,
            wantpos: 0,
            readerr: None,
            seekerr: None,
        },
        Test {
            seek: ggio::Seek::End,
            off: -1,
            n: 1,
            wantpos: 9,
            want: "9".as_bytes(),
            readerr: None,
            seekerr: None,
        },
    ];

    for (i, tt) in tests.iter().enumerate() {
        let res = r.seek(tt.off, tt.seek);
        if res.is_ok() && tt.seekerr.is_some() {
            panic!("{}. want seek error {}", i, tt.seekerr.as_ref().unwrap());
        }
        match res {
            Err(err) => {
                let errstr = err.to_string();
                assert_eq!(
                    tt.seekerr.as_ref().unwrap(),
                    &errstr,
                    "{}. seek error = '{}'; want '{}'",
                    i,
                    errstr,
                    tt.seekerr.as_ref().unwrap()
                );
            }
            Ok(pos) => {
                if tt.wantpos != 0 {
                    assert_eq!(tt.wantpos, pos, "{}. pos = {}, want {}", i, pos, tt.wantpos);
                }
            }
        }
        let mut buf = vec![0; tt.n];
        let res = r.read(&mut buf);
        if res.is_ok() {
            if tt.readerr.is_some() {
                panic!("{}. read = no error; want {:?}", i, tt.readerr);
            }
        } else if tt.readerr.is_none() {
            panic!("{}. read = {:?}; want = no error", i, res);
        } else {
            let errstr = res.as_ref().unwrap_err().to_string();
            assert_eq!(
                tt.readerr.as_ref().unwrap(),
                &errstr,
                "{}. read error = '{}'; want '{}'",
                i,
                errstr,
                tt.readerr.as_ref().unwrap()
            );
        }
        let n = res.unwrap();
        let got = &buf[..n];
        assert_eq!(tt.want, got, "{}. got {:?}; want {:?}", i, got, tt.want);
    }
}

#[test]
fn test_read_after_big_seek() {
    let mut r = Reader::new("0123456789".as_bytes());
    r.seek((1 << 31) + 5, ggio::Seek::Start).unwrap();
    let mut buf = [0; 10];
    let res = r.read(&mut buf);
    assert!(res.is_ok() && res.unwrap() == 0);
}

#[test]
fn test_reader_at() {
    let mut r = Reader::new("0123456789".as_bytes());
    struct Test {
        off: u64,
        n: usize,
        want: &'static [u8],
        wanterr: Option<&'static str>,
    }
    let tests = &[
        Test {
            off: 0,
            n: 10,
            want: "0123456789".as_bytes(),
            wanterr: None,
        },
        Test {
            off: 1,
            n: 10,
            want: "123456789".as_bytes(),
            wanterr: None,
        },
        Test {
            off: 1,
            n: 9,
            want: "123456789".as_bytes(),
            wanterr: None,
        },
        Test {
            off: 11,
            n: 10,
            want: "".as_bytes(),
            wanterr: None,
        },
        Test {
            off: 0,
            n: 0,
            want: "".as_bytes(),
            wanterr: None,
        },
    ];
    for (i, tt) in tests.iter().enumerate() {
        let mut b = vec![0; tt.n];
        let res = r.read_at(&mut b, tt.off);
        match res {
            Ok(rn) => {
                let got = &b[..rn];
                assert_eq!(tt.want, got, "{}. got {:?}; want {:?}", i, got, tt.want);
            }
            Err(err) => {
                let got = err.to_string();
                let wanterr = tt.wanterr.unwrap();
                assert_eq!(
                    wanterr, got,
                    "{}. got error = '{:?}'; want '{:?}'",
                    i, got, wanterr
                );
            }
        }
    }
}

// fn TestReaderAtConcurrent() {
// 	// Test for the race detector, to verify read_at doesn't mutate
// 	// any state.
// 	r := Reader::new([u8]("0123456789"))
// 	var wg sync.WaitGroup
// 	for i := 0; i < 5; i += 1 {
// 		wg.Add(1)
// 		go fn(i int) {
// 			defer wg.Done()
// 			var buf [1]byte
// 			r.read_at(buf[..], int64(i))
// 		}(i)
// 	}
// 	wg.Wait()
// }

// fn TestEmptyReaderConcurrent() {
// 	// Test for the race detector, to verify a read that doesn't yield any bytes
// 	// is okay to use from multiple goroutines. This was our historic behavior.
// 	// See golang.org/issue/7856
// 	r := Reader::new([u8]{})
// 	var wg sync.WaitGroup
// 	for i := 0; i < 5; i += 1 {
// 		wg.Add(2)
// 		go fn() {
// 			defer wg.Done()
// 			var buf [1]byte
// 			r.read(buf[..])
// 		}()
// 		go fn() {
// 			defer wg.Done()
// 			r.read(nil)
// 		}()
// 	}
// 	wg.Wait()
// }

// fn TestReaderWriteTo() {
// 	for i := 0; i < 30; i += 3 {
// 		var l int
// 		if i > 0 {
// 			l = len(testString) / i
// 		}
// 		s := testString[..l]
// 		r := Reader::new(testBytes[..l])
// 		var b Buffer
// 		n, err := r.write_to(&b)
// 		if expect := int64(len(s)); n != expect {
// 			t.Errorf("got '{}'; want '{}'", n, expect)
// 		}
// 		if err != nil {
// 			t.Errorf("for length {}: got error = '{}'; want nil", l, err)
// 		}
// 		if b.String() != s {
// 			t.Errorf("got string %q; want %q", b.String(), s)
// 		}
// 		if r.len() != 0 {
// 			t.Errorf("reader contains '{}' bytes; want 0", r.len())
// 		}
// 	}
// }

#[test]
fn test_reader_len() {
    let data = "hello world";
    let mut r = Reader::new(data.as_bytes());

    let want = 11;
    let got = r.len();
    assert_eq!(want, got, "r.len(): got {}, want {}", got, want);

    let mut buf = [0; 10];
    let res = r.read(&mut buf);
    assert!(res.is_ok() && res.unwrap() == 10);

    let want = 1;
    let got = r.len();
    assert_eq!(want, got, "r.len(): got {}, want {}", got, want);

    let mut buf = [0; 1];
    let res = r.read(&mut buf);
    assert!(res.is_ok() && res.unwrap() == 1);

    let want = 0;
    let got = r.len();
    assert_eq!(want, got, "r.len(): got {}, want {}", got, want);
}

// var UnreadRuneErrorTests = []struct {
// 	name string
// 	f    fn(*Reader)
// }{
// 	{"read", fn(r *Reader) { r.read([u8]{0}) }},
// 	{"read_byte", fn(r *Reader) { r.read_byte() }},
// 	{"UnreadRune", fn(r *Reader) { r.UnreadRune() }},
// 	{"Seek", fn(r *Reader) { r.seek(0, ggio::Seek::Current) }},
// 	{"write_to", fn(r *Reader) { r.write_to(&Buffer{}) }},
// }

// fn TestUnreadRuneError() {
// 	for _, tt := range UnreadRuneErrorTests {
// 		reader := Reader::new([u8]("0123456789"))
// 		if _, _, err := reader.ReadRune(); err != nil {
// 			// should not happen
// 			t.Fatal(err)
// 		}
// 		tt.f(reader)
// 		err := reader.UnreadRune()
// 		if err == nil {
// 			t.Errorf("Unreading after %s: expected error", tt.name)
// 		}
// 	}
// }

// fn TestReaderDoubleUnreadRune() {
// 	buf := NewBuffer([u8]("groucho"))
// 	if _, _, err := buf.ReadRune(); err != nil {
// 		// should not happen
// 		t.Fatal(err)
// 	}
// 	if err := buf.unread_byte(); err != nil {
// 		// should not happen
// 		t.Fatal(err)
// 	}
// 	if err := buf.unread_byte(); err == nil {
// 		t.Fatal("unread_byte: expected error, got nil")
// 	}
// }

// // verify that copying from an empty reader always has the same results,
// // regardless of the presence of a write_to method.
// fn TestReaderCopyNothing() {
// 	type nErr struct {
// 		n   int64
// 		err error
// 	}
// 	type justReader struct {
// 		io.Reader
// 	}
// 	type justWriter struct {
// 		ggio::Writer
// 	}
// 	discard := justWriter{ggio::Discard::new()} // hide ReadFrom

// 	var with, withOut nErr
// 	with.n, with.err = io.Copy(discard, Reader::new(nil))
// 	withOut.n, withOut.err = io.Copy(discard, justReader{Reader::new(nil)})
// 	if with != withOut {
// 		t.Errorf("behavior differs: with = %#v; without: %#v", with, withOut)
// 	}
// }

/// tests that len is affected by reads, but size is not.
#[test]
fn test_reader_len_size() {
    let mut r = Reader::new("abc".as_bytes());
    ggio::copy_n(&mut ggio::Discard::new(), &mut r, 1);
    assert_eq!(2, r.len(), "len = {}; want 2", r.len());
    assert_eq!(3, r.size(), "size = {}; want 3", r.size());
}

// fn TestReaderReset() {
// 	r := Reader::new([u8]("世界"))
// 	if _, _, err := r.ReadRune(); err != nil {
// 		t.Errorf("ReadRune: unexpected error: '{}'", err)
// 	}

// 	const want = "abcdef"
// 	r.Reset([u8](want))
// 	if err := r.UnreadRune(); err == nil {
// 		t.Errorf("UnreadRune: expected error, got nil")
// 	}
// 	buf, err := ggio::read_all(r)
// 	if err != nil {
// 		t.Errorf("read_all: unexpected error: '{}'", err)
// 	}
// 	if got := string(buf); got != want {
// 		t.Errorf("read_all: got %q, want %q", got, want)
// 	}
// }

#[test]
fn test_reader_zero() {
    let l = Reader::new(&[]).len();
    assert_eq!(0, l, "len: got {}, want 0", l);

    let mut buf = [];
    let res = Reader::new(&[]).read(&mut buf);
    assert!(res.is_ok() && res.unwrap() == 0);

    let res = Reader::new(&[]).read_at(&mut buf, 11);
    assert!(res.is_ok_and(|x| x == 0));

    let res = Reader::new(&[]).read_byte();
    assert!(res.is_err());
    assert!(res.err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof);

    // 	if ch, size, err := Reader::new(&[]).ReadRune(); ch != 0 || size != 0 || err != io.EOF {
    // 		t.Errorf("ReadRune: got {}, {}, '{}'; want 0, 0, io.EOF", ch, size, err)
    // 	}

    let offset = Reader::new(&[]).seek(11, ggio::Seek::Start).unwrap();
    assert!(offset == 11, "Seek: got {}; want 11", offset);

    let s = Reader::new(&[]).size();
    assert_eq!(0, s, "size: got {}, want 0", s);

    assert!(
        Reader::new(&[]).unread_byte().is_err(),
        "unread_byte: got nil, want error"
    );

    // 	if Reader::new(&[]).UnreadRune() == nil {
    // 		t.Errorf("UnreadRune: got nil, want error")
    // 	}

    let res = Reader::new(&[]).write_to(&mut ggio::Discard::new());
    assert!(
        res.as_ref().is_ok_and(|x| *x == 0),
        "write_to: got {:?}; want 0, nil",
        res
    );
}

#[test]
fn test_reader_std_io_read() {
    let mut r = Reader::new(b"hello".as_slice());
    let mut buf = [0; 2];
    assert_eq!(2, std::io::Read::read(&mut r, &mut buf).unwrap());
    assert_eq!(b"he", &buf);
    assert_eq!(2, std::io::Read::read(&mut r, &mut buf).unwrap());
    assert_eq!(b"ll", &buf);
    assert_eq!(1, std::io::Read::read(&mut r, &mut buf).unwrap());
    assert_eq!(b"o", &buf[..1]);
}
