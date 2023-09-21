// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::utf8::MAX_RUNE;
use super::{decode_rune, full_rune, RUNE_ERROR};
use crate::unicode;

/// Validate the constants redefined from unicode.
#[test]
fn test_constants() {
    assert_eq!(
        super::MAX_RUNE,
        unicode::MAX_RUNE,
        "utf8::MAX_RUNE is wrong"
    );
    assert_eq!(
        super::RUNE_ERROR,
        unicode::REPLACEMENT_CHAR,
        "utf8.RUNE_ERROR is wrong"
    );
}

#[derive(Debug)]
struct Utf8Map {
    r: char,
    encoded: &'static [u8],
}

impl Utf8Map {
    const fn new(r: char, encoded: &'static [u8]) -> Self {
        Self { r, encoded }
    }
}

static UTF8MAP: &[Utf8Map] = &[
    Utf8Map::new('\u{0000}', b"\x00"),
    Utf8Map::new('\u{0001}', b"\x01"),
    Utf8Map::new('\u{007e}', b"\x7e"),
    Utf8Map::new('\u{007f}', b"\x7f"),
    Utf8Map::new('\u{0080}', b"\xc2\x80"),
    Utf8Map::new('\u{0081}', b"\xc2\x81"),
    Utf8Map::new('\u{00bf}', b"\xc2\xbf"),
    Utf8Map::new('\u{00c0}', b"\xc3\x80"),
    Utf8Map::new('\u{00c1}', b"\xc3\x81"),
    Utf8Map::new('\u{00c8}', b"\xc3\x88"),
    Utf8Map::new('\u{00d0}', b"\xc3\x90"),
    Utf8Map::new('\u{00e0}', b"\xc3\xa0"),
    Utf8Map::new('\u{00f0}', b"\xc3\xb0"),
    Utf8Map::new('\u{00f8}', b"\xc3\xb8"),
    Utf8Map::new('\u{00ff}', b"\xc3\xbf"),
    Utf8Map::new('\u{0100}', b"\xc4\x80"),
    Utf8Map::new('\u{07ff}', b"\xdf\xbf"),
    Utf8Map::new('\u{0400}', b"\xd0\x80"),
    Utf8Map::new('\u{0800}', b"\xe0\xa0\x80"),
    Utf8Map::new('\u{0801}', b"\xe0\xa0\x81"),
    Utf8Map::new('\u{1000}', b"\xe1\x80\x80"),
    Utf8Map::new('\u{d000}', b"\xed\x80\x80"),
    // last code point before surrogate half.
    Utf8Map::new('\u{d7ff}', b"\xed\x9f\xbf"),
    // first code point after surrogate half.
    Utf8Map::new('\u{e000}', b"\xee\x80\x80"),
    Utf8Map::new('\u{fffe}', b"\xef\xbf\xbe"),
    Utf8Map::new('\u{ffff}', b"\xef\xbf\xbf"),
    Utf8Map::new('\u{10000}', b"\xf0\x90\x80\x80"),
    Utf8Map::new('\u{10001}', b"\xf0\x90\x80\x81"),
    Utf8Map::new('\u{40000}', b"\xf1\x80\x80\x80"),
    Utf8Map::new('\u{10fffe}', b"\xf4\x8f\xbf\xbe"),
    Utf8Map::new('\u{10ffff}', b"\xf4\x8f\xbf\xbf"),
    Utf8Map::new('\u{FFFD}', b"\xef\xbf\xbd"),
];

struct InvalidUtf8Map {
    _r: u32,
    encoded: &'static [u8],
}
static SURROGATE_MAP: [InvalidUtf8Map; 2] = [
    // surrogate min decodes to (RUNE_ERROR, 1)
    InvalidUtf8Map {
        _r: 0xd800,
        encoded: b"\xed\xa0\x80",
    },
    // surrogate max decodes to (RUNE_ERROR, 1)
    InvalidUtf8Map {
        _r: 0xdfff,
        encoded: b"\xed\xbf\xbf",
    },
];

const TEST_STRINGS: &[&str] = &[
    "",
    "abcd",
    "☺☻☹",
    "日a本b語ç日ð本Ê語þ日¥本¼語i日©",
    "日a本b語ç日ð本Ê語þ日¥本¼語i日©日a本b語ç日ð本Ê語þ日¥本¼語i日©日a本b語ç日ð本Ê語þ日¥本¼語i日©",
    // ggstd:: binary data can't be encoded to string
    // "\x80\x80\x80\x80",
];

#[test]
fn test_full_rune() {
    for m in UTF8MAP {
        let b = m.encoded;
        assert!(
            full_rune(b),
            "full_rune({:?}) ({:?}) = false, want true",
            b,
            m.r
        );
        // 		s := m.str
        // 		if !FullRuneInString(s) {
        // 			t.Errorf("FullRuneInString({:?}) (U+{:X}) = false, want true", s, m.r)
        // 		}
        let b1 = &b[0..b.len() - 1];
        assert!(!full_rune(b1), "full_rune({:?}) = true, want false", b1);
        // 		s1 := string(b1)
        // 		if FullRuneInString(s1) {
        // 			t.Errorf("full_rune({:?}) = true, want false", s1)
        // 		}
        // 	}
        for s in [b"\xc0", b"\xc1"] {
            let b = &s[..];
            assert!(full_rune(b), "full_rune({:?}) = false, want true", s);
            // 		if !FullRuneInString(s) {
            // 			t.Errorf("FullRuneInString({:?}) = false, want true", s)
            // 		}
        }
    }
}

#[test]
fn test_encode_rune() {
    for m in UTF8MAP {
        let mut buf = [0_u8; 10];
        let n = super::encode_rune(&mut buf, m.r as u32);
        let got = &buf[..n];
        assert_eq!(
            m.encoded, got,
            "encode_rune(%{:?}) = {:?} want {:?}",
            m.r, got, m.encoded
        );
    }
}

// #[test]
// fn TestAppendRune() {
// 	for _, m := range utf8map {
// 		if buf := AppendRune(nil, m.r); string(buf) != m.str {
// 			t.Errorf("AppendRune(nil, %{:04X}) = %s, want %s", m.r, buf, m.str)
// 		}
// 		if buf := AppendRune([]byte("init"), m.r); string(buf) != "init"+m.str {
// 			t.Errorf("AppendRune(init, %{:04X}) = %s, want %s", m.r, buf, "init"+m.str)
// 		}
// 	}
// }

#[test]
fn test_decode_rune() {
    for m in UTF8MAP {
        let b = m.encoded;
        let (r, size) = decode_rune(b);
        assert!(
            r == m.r && size == b.len(),
            "decode_rune({:?}) = {:?}, {} want {:?}, {}",
            b,
            r,
            size,
            m.r,
            b.len()
        );
        // 		s := m.str
        // 		let (r, size) = decode_rune_in_string(s)
        // 		if r != m.r || size != b.len() {
        // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X}, {} want %{:04X}, {}", s, r, size, m.r, b.len())
        // 		}

        // there's an extra byte that bytes left behind - make sure trailing byte works
        {
            let original_size = b.len();
            let mut b = b.to_vec();
            b.push(0);
            let (r, size) = decode_rune(&b);
            assert!(
                r == m.r && size == original_size,
                "decode_rune({:?}) = {:?}, {} want {:?}, {}",
                b,
                r,
                size,
                m.r,
                original_size
            );
            // 		s = m.str + "\x00"
            // 		let (r, size) = decode_rune_in_string(s)
            // 		if r != m.r || size != b.len() {
            // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X}, {} want %{:04X}, {}", s, r, size, m.r, b.len())
            // 		}
        }

        // make sure missing bytes fail
        let wantsize = if b.len() <= 1 { 0 } else { 1 };
        let (r, size) = decode_rune(&b[0..b.len() - 1]);
        assert!(
            r == RUNE_ERROR && size == wantsize,
            "decode_rune({:?}) = %{:?}, {} want %{:?}, {}",
            &b[0..b.len() - 1],
            r,
            size,
            RUNE_ERROR,
            wantsize
        );
        // 		s = m.str[0..len(m.str)-1]
        // 		let (r, size) = decode_rune_in_string(s)
        // 		if r != RUNE_ERROR || size != wantsize {
        // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X}, {} want %{:04X}, {}", s, r, size, RUNE_ERROR, wantsize)
        // 		}

        // make sure bad sequences fail
        let mut b = b.to_vec();
        {
            let index = b.len() - 1;
            let value = if b.len() == 1 { 0x80 } else { 0x7F };
            b[index] = value;
            let (r, size) = decode_rune(&b);
            assert!(
                r == RUNE_ERROR && size == 1,
                "decode_rune({:?}) = %{:?}, {} want %{:?}, {}",
                b,
                r,
                size,
                RUNE_ERROR,
                1
            );
            // 		s = string(b)
            // 		let (r, size) = decode_rune_in_string(s)
            // 		if r != RUNE_ERROR || size != 1 {
            // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X}, {} want %{:04X}, {}", s, r, size, RUNE_ERROR, 1)
            // 		}
        }
    }
}

#[test]
fn test_decode_surrogate_rune() {
    for m in &SURROGATE_MAP {
        let b = m.encoded;
        let (r, size) = decode_rune(b);
        assert!(
            r == RUNE_ERROR && size == 1,
            "decode_rune({:?}) = {}, {} want {}, {}",
            b,
            r,
            size,
            RUNE_ERROR,
            1,
        );
        // s := m.str
        // let (r, size) = decode_rune_in_string(s)
        // if r != RUNE_ERROR || size != 1 {
        // 	t.Errorf("decode_rune_in_string({:?}) = %x, {} want %x, {}", b, r, size, RUNE_ERROR, 1)
        // }
    }
}

/// Check that decode_rune and DecodeLastRune correspond to
/// the equivalent range loop.
#[test]
fn test_sequencing() {
    for ts in TEST_STRINGS {
        for m in UTF8MAP {
            for s in [
                ts.to_string() + &String::from_utf8_lossy(m.encoded),
                String::from_utf8_lossy(m.encoded).to_string() + ts,
                ts.to_string() + &String::from_utf8_lossy(m.encoded) + ts,
            ] {
                test_sequence(&s);
            }
        }
    }
}

// fn runtimeRuneCount(s string) int {
// 	return len([]Rune(s)) // Replaced by gc with call to runtime.countrunes(s).
// }

// // Check that a range loop, len([]Rune(string)) optimization and
// // []Rune conversions visit the same runes.
// // Not really a test of this package, but the assumption is used here and
// // it's good to verify.
// #[test]
// fn TestRuntimeConversion() {
// 	for _, ts := range TEST_STRINGS {
// 		count := RuneCountInString(ts)
// 		if n := runtimeRuneCount(ts); n != count {
// 			t.Errorf("{:?}: len([]Rune()) counted {} runes; got {} from RuneCountInString", ts, n, count)
// 			break
// 		}

// 		runes := []Rune(ts)
// 		if n := len(runes); n != count {
// 			t.Errorf("{:?}: []Rune() has length {}; got {} from RuneCountInString", ts, n, count)
// 			break
// 		}
// 		i := 0
// 		for _, r := range ts {
// 			if r != runes[i] {
// 				t.Errorf("{:?}[{}]: expected %c (U+{:X}); got %c (U+{:X})", ts, i, runes[i], runes[i], r, r)
// 			}
// 			i++
// 		}
// 	}
// }

const INVALID_SEQUENCE_TESTS: &[&[u8]] = &[
    b"\xed\xa0\x80\x80", // surrogate min
    b"\xed\xbf\xbf\x80", // surrogate max
    // xx
    b"\x91\x80\x80\x80",
    // s1
    b"\xC2\x7F\x80\x80",
    b"\xC2\xC0\x80\x80",
    b"\xDF\x7F\x80\x80",
    b"\xDF\xC0\x80\x80",
    // s2
    b"\xE0\x9F\xBF\x80",
    b"\xE0\xA0\x7F\x80",
    b"\xE0\xBF\xC0\x80",
    b"\xE0\xC0\x80\x80",
    // s3
    b"\xE1\x7F\xBF\x80",
    b"\xE1\x80\x7F\x80",
    b"\xE1\xBF\xC0\x80",
    b"\xE1\xC0\x80\x80",
    // s4
    b"\xED\x7F\xBF\x80",
    b"\xED\x80\x7F\x80",
    b"\xED\x9F\xC0\x80",
    b"\xED\xA0\x80\x80",
    // s5
    b"\xF0\x8F\xBF\xBF",
    b"\xF0\x90\x7F\xBF",
    b"\xF0\x90\x80\x7F",
    b"\xF0\xBF\xBF\xC0",
    b"\xF0\xBF\xC0\x80",
    b"\xF0\xC0\x80\x80",
    // s6
    b"\xF1\x7F\xBF\xBF",
    b"\xF1\x80\x7F\xBF",
    b"\xF1\x80\x80\x7F",
    b"\xF1\xBF\xBF\xC0",
    b"\xF1\xBF\xC0\x80",
    b"\xF1\xC0\x80\x80",
    // s7
    b"\xF4\x7F\xBF\xBF",
    b"\xF4\x80\x7F\xBF",
    b"\xF4\x80\x80\x7F",
    b"\xF4\x8F\xBF\xC0",
    b"\xF4\x8F\xC0\x80",
    b"\xF4\x90\x80\x80",
];

// fn runtimeDecodeRune(s string) Rune {
// 	for _, r := range s {
// 		return r
// 	}
// 	return -1
// }

#[test]
fn test_decode_invalid_sequence() {
    for s in INVALID_SEQUENCE_TESTS {
        let (r1, _) = decode_rune(s);
        assert_eq!(
            r1, RUNE_ERROR,
            "decode_rune({:?}) = {}, want {}",
            s, r1, RUNE_ERROR
        );
        // 		r2, _ := decode_rune_in_string(s)
        // 		if want := RUNE_ERROR; r2 != want {
        // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X}, want %{:04X}", s, r2, want)
        // 			return
        // 		}
        // 		if r1 != r2 {
        // 			t.Errorf("decode_rune(%#x) = %{:04X} mismatch with decode_rune_in_string({:?}) = %{:04X}", s, r1, s, r2)
        // 			return
        // 		}
        // 		r3 := runtimeDecodeRune(s)
        // 		if r2 != r3 {
        // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X} mismatch with runtime.decoderune({:?}) = %{:04X}", s, r2, s, r3)
        // 			return
        // 		}
    }
}

fn test_sequence(s: &str) {
    #[derive(Clone)]
    struct Info {
        _index: usize,
        _r: u32, //Rune
    }
    let mut index = Vec::new();
    let b = s.as_bytes();
    let mut si = 0;
    // let mut j = 0;
    for r in s.chars() {
        index.push(Info {
            _index: si,
            _r: r as u32,
        });
        // j += 1;
        let (r1, size1) = super::decode_rune(&b[si..]);
        assert_eq!(
            r,
            r1,
            "decode_rune({:?}) = %{:?}, want %{:?}",
            &s[si..],
            r1,
            r
        );
        // 		r2, size2 := decode_rune_in_string(s[i..])
        // 		if r != r2 {
        // 			t.Errorf("decode_rune_in_string({:?}) = %{:04X}, want %{:04X}", s[i..], r2, r)
        // 			return
        // 		}
        // 		if size1 != size2 {
        // 			t.Errorf("decode_rune/decode_rune_in_string({:?}) size mismatch {}/{}", s[i..], size1, size2)
        // 			return
        // 		}
        si += size1;
    }
    // j = index.len() - 1;
    // 	for si = len(s); si > 0; {
    // 		r1, size1 := DecodeLastRune(b[0..si])
    // 		r2, size2 := DecodeLastRuneInString(s[0..si])
    // 		if size1 != size2 {
    // 			t.Errorf("DecodeLastRune/DecodeLastRuneInString({:?}, {}) size mismatch {}/{}", s, si, size1, size2)
    // 			return
    // 		}
    // 		if r1 != index[j].r {
    // 			t.Errorf("DecodeLastRune({:?}, {}) = %{:04X}, want %{:04X}", s, si, r1, index[j].r)
    // 			return
    // 		}
    // 		if r2 != index[j].r {
    // 			t.Errorf("DecodeLastRuneInString({:?}, {}) = %{:04X}, want %{:04X}", s, si, r2, index[j].r)
    // 			return
    // 		}
    // 		si -= size1
    // 		if si != index[j].index {
    // 			t.Errorf("DecodeLastRune({:?}) index mismatch at {}, want {}", s, si, index[j].index)
    // 			return
    // 		}
    // 		j--
    // 	}
    // 	if si != 0 {
    // 		t.Errorf("DecodeLastRune({:?}) finished at {}, not 0", s, si)
    // 	}
}

// ggstd: no need to check this
// // Check that negative runes encode as U+FFFD.
// #[test]
// fn TestNegativeRune() {
//     let mut errorbuf = [0; UTFMAX];
//     // 	errorbuf = errorbuf[0..encode_rune(errorbuf, RUNE_ERROR)]
//     let mut buf = [0; UTFMAX];
//     // 	buf = buf[0..encode_rune(buf, -1)]
//     // 	if !bytes.Equal(buf, errorbuf) {
//     // 		t.Errorf("incorrect encoding [% x] for -1; expected [% x]", buf, errorbuf)
//     // 	}
// }

struct RuneCountTest {
    input: &'static [u8],
    out: usize,
}

impl RuneCountTest {
    const fn new(input: &'static [u8], out: usize) -> Self {
        Self { input, out }
    }
}

static RUNE_COUNT_TESTS: &[RuneCountTest] = &[
    RuneCountTest::new(b"abcd", 4),
    RuneCountTest::new("☺☻☹".as_bytes(), 3),
    RuneCountTest::new(b"1,2,3,4", 7),
    RuneCountTest::new(b"\xe2\x00", 2),
    RuneCountTest::new(b"\xe2\x80", 2),
    RuneCountTest::new(b"a\xe2\x80", 3),
];

#[test]
fn test_rune_count() {
    for tt in RUNE_COUNT_TESTS {
        // 		if out := RuneCountInString(tt.input); out != tt.out {
        // 			t.Errorf("RuneCountInString({:?}) = {}, want {}", tt.input, out, tt.out)
        // 		}
        let out = super::rune_count(tt.input);
        assert_eq!(
            out, tt.out,
            "rune_count({:?}) = {:?}, want {:?}",
            tt.input, out, tt.out
        );
    }
}

struct RuneLenTest {
    r: u32,
    size: isize,
}

impl RuneLenTest {
    const fn new(r: u32, size: isize) -> Self {
        Self { r, size }
    }
}

static RUN_ELEN_TESTS: &[RuneLenTest] = &[
    RuneLenTest::new(0, 1),
    RuneLenTest::new('e' as u32, 1),
    RuneLenTest::new('é' as u32, 2),
    RuneLenTest::new('☺' as u32, 3),
    RuneLenTest::new(RUNE_ERROR as u32, 3),
    RuneLenTest::new(MAX_RUNE as u32, 4),
    RuneLenTest::new(0xD800, -1),
    RuneLenTest::new(0xDFFF, -1),
    RuneLenTest::new(MAX_RUNE as u32 + 1, -1),
    // RuneLenTest::new(-1, -1),
];

#[test]
fn test_rune_len() {
    for tt in RUN_ELEN_TESTS {
        let size = super::rune_len(tt.r);
        assert_eq!(
            size, tt.size,
            "rune_len({:?}) = {}, want {}",
            tt.r, size, tt.size
        );
    }
}

// type ValidTest struct {
// 	in  string
// 	out bool
// }

// var validTests = []ValidTest{
// 	{"", true},
// 	{"a", true},
// 	{"abc", true},
// 	{"Ж", true},
// 	{"ЖЖ", true},
// 	{"брэд-ЛГТМ", true},
// 	{"☺☻☹", true},
// 	{"aa\xe2", false},
// 	{string([]byte{66, 250}), false},
// 	{string([]byte{66, 250, 67}), false},
// 	{"a\uFFFDb", true},
// 	{string("\xF4\x8F\xBF\xBF"), true},      // U+10FFFF
// 	{string("\xF4\x90\x80\x80"), false},     // U+10FFFF+1; out of range
// 	{string("\xF7\xBF\xBF\xBF"), false},     // 0x1FFFFF; out of range
// 	{string("\xFB\xBF\xBF\xBF\xBF"), false}, // 0x3FFFFFF; out of range
// 	{string("\xc0\x80"), false},             // U+0000 encoded in two bytes: incorrect
// 	{string("\xed\xa0\x80"), false},         // U+D800 high surrogate (sic)
// 	{string("\xed\xbf\xbf"), false},         // U+DFFF low surrogate (sic)
// }

// #[test]
// fn TestValid() {
// 	for _, tt := range validTests {
// 		if Valid([]byte(tt.in)) != tt.out {
// 			t.Errorf("Valid({:?}) = %v; want %v", tt.in, !tt.out, tt.out)
// 		}
// 		if ValidString(tt.in) != tt.out {
// 			t.Errorf("ValidString({:?}) = %v; want %v", tt.in, !tt.out, tt.out)
// 		}
// 	}
// }

// type ValidRuneTest struct {
// 	r  Rune
// 	ok bool
// }

// var validrunetests = []ValidRuneTest{
// 	{0, true},
// 	{'e', true},
// 	{'é', true},
// 	{'☺', true},
// 	{RUNE_ERROR, true},
// 	{MAX_RUNE, true},
// 	{0xD7FF, true},
// 	{0xD800, false},
// 	{0xDFFF, false},
// 	{0xE000, true},
// 	{MAX_RUNE + 1, false},
// 	{-1, false},
// }

// #[test]
// fn TestValidRune() {
// 	for _, tt := range validrunetests {
// 		if ok := ValidRune(tt.r); ok != tt.ok {
// 			t.Errorf("ValidRune(%#U) = %t, want %t", tt.r, ok, tt.ok)
// 		}
// 	}
// }

// fn BenchmarkRuneCountTenASCIIChars(b *testing.B) {
// 	s := []byte("0123456789")
// 	for i := 0; i < b.N; i++ {
// 		rune_count(s)
// 	}
// }

// fn BenchmarkRuneCountTenJapaneseChars(b *testing.B) {
// 	s := []byte("日本語日本語日本語日")
// 	for i := 0; i < b.N; i++ {
// 		rune_count(s)
// 	}
// }

// fn BenchmarkRuneCountInStringTenASCIIChars(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		RuneCountInString("0123456789")
// 	}
// }

// fn BenchmarkRuneCountInStringTenJapaneseChars(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		RuneCountInString("日本語日本語日本語日")
// 	}
// }

// var ascii100000 = strings.Repeat("0123456789", 10000)

// fn BenchmarkValidTenASCIIChars(b *testing.B) {
// 	s := []byte("0123456789")
// 	for i := 0; i < b.N; i++ {
// 		Valid(s)
// 	}
// }

// fn BenchmarkValid100KASCIIChars(b *testing.B) {
// 	s := []byte(ascii100000)
// 	for i := 0; i < b.N; i++ {
// 		Valid(s)
// 	}
// }

// fn BenchmarkValidTenJapaneseChars(b *testing.B) {
// 	s := []byte("日本語日本語日本語日")
// 	for i := 0; i < b.N; i++ {
// 		Valid(s)
// 	}
// }
// fn BenchmarkValidLongMostlyASCII(b *testing.B) {
// 	longMostlyASCII := []byte(longStringMostlyASCII)
// 	for i := 0; i < b.N; i++ {
// 		Valid(longMostlyASCII)
// 	}
// }

// fn BenchmarkValidLongJapanese(b *testing.B) {
// 	longJapanese := []byte(longStringJapanese)
// 	for i := 0; i < b.N; i++ {
// 		Valid(longJapanese)
// 	}
// }

// fn BenchmarkValidStringTenASCIIChars(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		ValidString("0123456789")
// 	}
// }

// fn BenchmarkValidString100KASCIIChars(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		ValidString(ascii100000)
// 	}
// }

// fn BenchmarkValidStringTenJapaneseChars(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		ValidString("日本語日本語日本語日")
// 	}
// }

// fn BenchmarkValidStringLongMostlyASCII(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		ValidString(longStringMostlyASCII)
// 	}
// }

// fn BenchmarkValidStringLongJapanese(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		ValidString(longStringJapanese)
// 	}
// }

// var longStringMostlyASCII string // ~100KB, ~97% ASCII
// var longStringJapanese string    // ~100KB, non-ASCII

// fn init() {
// 	const japanese = "日本語日本語日本語日"
// 	var b strings.Builder
// 	for i := 0; b.Len() < 100_000; i++ {
// 		if i%100 == 0 {
// 			b.WriteString(japanese)
// 		} else {
// 			b.WriteString("0123456789")
// 		}
// 	}
// 	longStringMostlyASCII = b.String()
// 	longStringJapanese = strings.Repeat(japanese, 100_000/len(japanese))
// }

// fn BenchmarkEncodeASCIIRune(b *testing.B) {
// 	buf := make([]byte, UTFMAX)
// 	for i := 0; i < b.N; i++ {
// 		encode_rune(buf, 'a')
// 	}
// }

// fn BenchmarkEncodeJapaneseRune(b *testing.B) {
// 	buf := make([]byte, UTFMAX)
// 	for i := 0; i < b.N; i++ {
// 		encode_rune(buf, '本')
// 	}
// }

// fn BenchmarkAppendASCIIRune(b *testing.B) {
// 	buf := make([]byte, UTFMAX)
// 	for i := 0; i < b.N; i++ {
// 		AppendRune(buf[:0], 'a')
// 	}
// }

// fn BenchmarkAppendJapaneseRune(b *testing.B) {
// 	buf := make([]byte, UTFMAX)
// 	for i := 0; i < b.N; i++ {
// 		AppendRune(buf[:0], '本')
// 	}
// }

// fn BenchmarkDecodeASCIIRune(b *testing.B) {
// 	a := []byte{'a'}
// 	for i := 0; i < b.N; i++ {
// 		decode_rune(a)
// 	}
// }

// fn BenchmarkDecodeJapaneseRune(b *testing.B) {
// 	nihon := []byte("本")
// 	for i := 0; i < b.N; i++ {
// 		decode_rune(nihon)
// 	}
// }

// // boolSink is used to reference the return value of benchmarked
// // functions to avoid dead code elimination.
// var boolSink bool

// fn BenchmarkFullRune(b *testing.B) {
// 	benchmarks := []struct {
// 		name string
// 		data []byte
// 	}{
// 		{"ASCII", []byte("a")},
// 		{"Incomplete", []byte("\xf0\x90\x80")},
// 		{"Japanese", []byte("本")},
// 	}
// 	for _, bm := range benchmarks {
// 		b.Run(bm.name, fn(b *testing.B) {
// 			for i := 0; i < b.N; i++ {
// 				boolSink = full_rune(bm.data)
// 			}
// 		})
// 	}
// }
