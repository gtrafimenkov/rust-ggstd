// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// // The conditions RUNE_ERROR==unicode:REPLACEMENT_CHAR and
// // MAX_RUNE==unicode.MAX_RUNE are verified in the tests.
// // Defining them locally avoids this package depending on package unicode.

/// Numbers fundamental to the encoding.
pub const RUNE_ERROR: char = '\u{FFFD}'; // the "error" Rune or "Unicode replacement character"

/// Characters below RUNE_SELF are represented as themselves in a single byte.
pub const RUNE_SELF: char = 0x80 as char;

/// Maximum valid Unicode code point.
pub const MAX_RUNE: char = '\u{10FFFF}';

/// Maximum number of bytes of a UTF-8 encoded Unicode character.
pub const UTFMAX: usize = 4;

/// Code points in the surrogate range are not valid for UTF-8.
pub(crate) const SURROGATE_MIN: u32 = 0xD800;
pub(crate) const SURROGATE_MAX: u32 = 0xDFFF;

#[allow(unused)]
const T1: u8 = 0b00000000;
const TX: u8 = 0b10000000;
const T2: u8 = 0b11000000;
const T3: u8 = 0b11100000;
const T4: u8 = 0b11110000;
#[allow(unused)]
const T5: u8 = 0b11111000;

const MASKX: u8 = 0b00111111;
const MASK2: u8 = 0b00011111;
const MASK3: u8 = 0b00001111;
const MASK4: u8 = 0b00000111;

pub(crate) const RUNE1_MAX: u32 = (1 << 7) - 1;
pub(crate) const RUNE2_MAX: u32 = (1 << 11) - 1;
pub(crate) const RUNE3_MAX: u32 = 0xffff;

// The default lowest and highest continuation byte.
const LOCB: u8 = 0b10000000;
const HICB: u8 = 0b10111111;

// These names of these constants are chosen to give nice alignment in the
// table below. The first nibble is an index into acceptRanges or F for
// special one-byte cases. The second nibble is the Rune length or the
// Status for the special one-byte case.
/// invalid: size 1
const XX: u8 = 0xF1;
/// ASCII: size 1
const AS: u8 = 0xF0;
/// accept 0, size 2
const S1: u8 = 0x02;
/// accept 1, size 3
const S2: u8 = 0x13;
/// accept 0, size 3
const S3: u8 = 0x03;
/// accept 2, size 3
const S4: u8 = 0x23;
/// accept 3, size 4
const S5: u8 = 0x34;
/// accept 0, size 4
const S6: u8 = 0x04;
/// accept 4, size 4
const S7: u8 = 0x44;

/// first is information about the first byte in a UTF-8 sequence.
const FIRST: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x00-0x0F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x10-0x1F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x20-0x2F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x30-0x3F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x40-0x4F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x50-0x5F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x60-0x6F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x70-0x7F
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0x80-0x8F
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0x90-0x9F
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0xA0-0xAF
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0xB0-0xBF
    XX, XX, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, // 0xC0-0xCF
    S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, // 0xD0-0xDF
    S2, S3, S3, S3, S3, S3, S3, S3, S3, S3, S3, S3, S3, S4, S3, S3, // 0xE0-0xEF
    S5, S6, S6, S6, S7, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0xF0-0xFF
];

/// acceptRange gives the range of valid values for the second byte in a UTF-8
/// sequence.
struct AcceptRange {
    lo: u8, // lowest value for second byte.
    hi: u8, // highest value for second byte.
}

/// acceptRanges has size 16 to avoid bounds checks in the code that uses it.
const ACCEPT_RANGES: [AcceptRange; 16] = [
    AcceptRange { lo: LOCB, hi: HICB },
    AcceptRange { lo: 0xA0, hi: HICB },
    AcceptRange { lo: LOCB, hi: 0x9F },
    AcceptRange { lo: 0x90, hi: HICB },
    AcceptRange { lo: LOCB, hi: 0x8F },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
    AcceptRange { lo: 0, hi: 0 },
];

/// full_rune reports whether the bytes in p begin with a full UTF-8 encoding of a rune.
/// An invalid encoding is considered a full Rune since it will convert as a width-1 error rune.
pub fn full_rune(p: &[u8]) -> bool {
    let n = p.len();
    if n == 0 {
        return false;
    }
    let x = FIRST[p[0] as usize];
    if n >= (x & 7) as usize {
        return true; // ASCII, invalid or valid.
    }
    // Must be short or invalid.
    let accept = &ACCEPT_RANGES[(x >> 4) as usize];
    #[allow(clippy::if_same_then_else)]
    if n > 1 && (p[1] < accept.lo || accept.hi < p[1]) {
        return true;
    } else if n > 2 && (p[2] < LOCB || HICB < p[2]) {
        return true;
    }
    false
}

// // FullRuneInString is like full_rune but its input is a string.
// fn FullRuneInString(s string) -> bool {
// 	let n = len(s);
// 	if n == 0 {
// 		return false
// 	}
// 	x := first[s[0]]
// 	if n >= int(x&7) {
// 		return true // ASCII, invalid, or valid.
// 	}
// 	// Must be short or invalid.
// 	accept := acceptRanges[x>>4]
// 	if n > 1 && (s[1] < accept.lo || accept.hi < s[1]) {
// 		return true
// 	} else if n > 2 && (s[2] < locb || hicb < s[2]) {
// 		return true
// 	}
// 	return false
// }

/// decode_rune unpacks the first UTF-8 encoding in p and returns the rune and
/// its width in bytes. If p is empty it returns (RUNE_ERROR, 0). Otherwise, if
/// the encoding is invalid, it returns (RUNE_ERROR, 1). Both are impossible
/// results for correct, non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
pub fn decode_rune(p: &[u8]) -> (char, usize) {
    // fn decode_rune(p: &[u8]) (r rune, size int) {
    let n = p.len();
    if n < 1 {
        return (RUNE_ERROR, 0);
    }
    let p0 = p[0];
    let x = FIRST[p0 as usize];
    if x >= AS {
        // The following code simulates an additional check for x == XX and
        // handling the ASCII and invalid cases accordingly. This mask-and-or
        // approach prevents an additional branch.
        let mask = (((x as i32) << 31) >> 31) as u32; // Create 0x0000 or 0xFFFF.
        unsafe {
            return (
                char::from_u32_unchecked(((p[0] as u32) & !mask) | (RUNE_ERROR as u32 & mask)),
                1,
            );
        }
    }
    let sz = (x & 7) as usize;
    let accept = &ACCEPT_RANGES[(x >> 4) as usize];
    if n < sz {
        return (RUNE_ERROR, 1);
    }
    let b1 = p[1];
    if b1 < accept.lo || accept.hi < b1 {
        return (RUNE_ERROR, 1);
    }
    if sz <= 2 {
        // <= instead of == to help the compiler eliminate some bounds checks
        unsafe {
            return (
                char::from_u32_unchecked((((p0 & MASK2) as u32) << 6) | (b1 & MASKX) as u32),
                2,
            );
        }
    }
    let b2 = p[2];
    if !(LOCB..=HICB).contains(&b2) {
        return (RUNE_ERROR, 1);
    }
    if sz <= 3 {
        unsafe {
            return (
                char::from_u32_unchecked(
                    (((p0 & MASK3) as u32) << 12)
                        | (((b1 & MASKX) as u32) << 6)
                        | (b2 & MASKX) as u32,
                ),
                3,
            );
        }
    }
    let b3 = p[3];
    if !(LOCB..=HICB).contains(&b3) {
        return (RUNE_ERROR, 1);
    }
    unsafe {
        (
            char::from_u32_unchecked(
                (((p0 & MASK4) as u32) << 18)
                    | (((b1 & MASKX) as u32) << 12)
                    | (((b2 & MASKX) as u32) << 6)
                    | ((b3 & MASKX) as u32),
            ),
            4,
        )
    }
}

/// decode_rune_in_string is like decode_rune but its input is a string. If s is
/// empty it returns (RUNE_ERROR, 0). Otherwise, if the encoding is invalid, it
/// returns (RUNE_ERROR, 1). Both are impossible results for correct, non-empty
/// UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
pub fn decode_rune_in_string(s: &str) -> (char, usize) {
    decode_rune(s.as_bytes())
    // 	let n = len(s);
    // 	if n < 1 {
    // 		return RUNE_ERROR, 0
    // 	}
    // 	s0 := s[0]
    // 	x := first[s0]
    // 	if x >= as {
    // 		// The following code simulates an additional check for x == XX and
    // 		// handling the ASCII and invalid cases accordingly. This mask-and-or
    // 		// approach prevents an additional branch.
    // 		mask := rune(x) << 31 >> 31 // Create 0x0000 or 0xFFFF.
    // 		return rune(s[0])&^mask | RUNE_ERROR&mask, 1
    // 	}
    // 	sz := int(x & 7)
    // 	accept := acceptRanges[x>>4]
    // 	if n < sz {
    // 		return RUNE_ERROR, 1
    // 	}
    // 	s1 := s[1]
    // 	if s1 < accept.lo || accept.hi < s1 {
    // 		return RUNE_ERROR, 1
    // 	}
    // 	if sz <= 2 { // <= instead of == to help the compiler eliminate some bounds checks
    // 		return rune(s0&mask2)<<6 | rune(s1&maskx), 2
    // 	}
    // 	s2 := s[2]
    // 	if s2 < locb || hicb < s2 {
    // 		return RUNE_ERROR, 1
    // 	}
    // 	if sz <= 3 {
    // 		return rune(s0&mask3)<<12 | rune(s1&maskx)<<6 | rune(s2&maskx), 3
    // 	}
    // 	s3 := s[3]
    // 	if s3 < locb || hicb < s3 {
    // 		return RUNE_ERROR, 1
    // 	}
    // 	return rune(s0&mask4)<<18 | rune(s1&maskx)<<12 | rune(s2&maskx)<<6 | rune(s3&maskx), 4
}

// // DecodeLastRune unpacks the last UTF-8 encoding in p and returns the rune and
// // its width in bytes. If p is empty it returns (RUNE_ERROR, 0). Otherwise, if
// // the encoding is invalid, it returns (RUNE_ERROR, 1). Both are impossible
// // results for correct, non-empty UTF-8.
// //
// // An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
// // out of range, or is not the shortest possible UTF-8 encoding for the
// // value. No other validation is performed.
// fn DecodeLastRune(p: &[u8]) (r rune, size int) {
// 	end := p.len()
// 	if end == 0 {
// 		return RUNE_ERROR, 0
// 	}
// 	start := end - 1
// 	r = rune(p[start])
// 	if r < RUNE_SELF {
// 		return r, 1
// 	}
// 	// guard against O(n^2) behavior when traversing
// 	// backwards through strings with long sequences of
// 	// invalid UTF-8.
// 	lim := end - UTFMAX
// 	if lim < 0 {
// 		lim = 0
// 	}
// 	for start--; start >= lim; start-- {
// 		if RuneStart(p[start]) {
// 			break
// 		}
// 	}
// 	if start < 0 {
// 		start = 0
// 	}
// 	r, size = decode_rune(p[start:end])
// 	if start+size != end {
// 		return RUNE_ERROR, 1
// 	}
// 	return r, size
// }

// // DecodeLastRuneInString is like DecodeLastRune but its input is a string. If
// // s is empty it returns (RUNE_ERROR, 0). Otherwise, if the encoding is invalid,
// // it returns (RUNE_ERROR, 1). Both are impossible results for correct,
// // non-empty UTF-8.
// //
// // An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
// // out of range, or is not the shortest possible UTF-8 encoding for the
// // value. No other validation is performed.
// fn DecodeLastRuneInString(s string) (r rune, size int) {
// 	end := len(s)
// 	if end == 0 {
// 		return RUNE_ERROR, 0
// 	}
// 	start := end - 1
// 	r = rune(s[start])
// 	if r < RUNE_SELF {
// 		return r, 1
// 	}
// 	// guard against O(n^2) behavior when traversing
// 	// backwards through strings with long sequences of
// 	// invalid UTF-8.
// 	lim := end - UTFMAX
// 	if lim < 0 {
// 		lim = 0
// 	}
// 	for start--; start >= lim; start-- {
// 		if RuneStart(s[start]) {
// 			break
// 		}
// 	}
// 	if start < 0 {
// 		start = 0
// 	}
// 	r, size = decode_rune_in_string(s[start:end])
// 	if start+size != end {
// 		return RUNE_ERROR, 1
// 	}
// 	return r, size
// }

/// rune_len returns the number of bytes required to encode the rune.
/// It returns -1 if the rune is not a valid value to encode in UTF-8.
pub fn rune_len(r: u32) -> isize {
    if r <= RUNE1_MAX {
        return 1;
    } else if r <= RUNE2_MAX {
        return 2;
    } else if (SURROGATE_MIN..=SURROGATE_MAX).contains(&r) {
        return -1;
    } else if r <= RUNE3_MAX {
        return 3;
    } else if r <= MAX_RUNE as u32 {
        return 4;
    }
    -1
}

/// encode_rune writes into p (which must be large enough) the UTF-8 encoding of the rune.
/// If the rune is out of range, it writes the encoding of RUNE_ERROR.
/// It returns the number of bytes written.
pub fn encode_rune(p: &mut [u8], r: u32) -> usize {
    let mut r = r;
    if r <= RUNE1_MAX {
        p[0] = r as u8;
        1
    } else if r <= RUNE2_MAX {
        // 		_ = p[1] // eliminate bounds checks
        p[0] = T2 | (r >> 6) as u8;
        p[1] = TX | (r as u8) & MASKX;
        return 2;
    } else {
        if (r > (MAX_RUNE as u32)) || (SURROGATE_MIN..=SURROGATE_MAX).contains(&r) {
            r = RUNE_ERROR as u32;
        }
        if r <= RUNE3_MAX {
            // 		_ = p[2] // eliminate bounds checks
            p[0] = T3 | (r >> 12) as u8;
            p[1] = TX | (r >> 6) as u8 & MASKX;
            p[2] = TX | (r as u8) & MASKX;
            return 3;
        } else {
            // 		_ = p[3] // eliminate bounds checks
            p[0] = T4 | (r >> 18) as u8;
            p[1] = TX | (r >> 12) as u8 & MASKX;
            p[2] = TX | (r >> 6) as u8 & MASKX;
            p[3] = TX | (r as u8) & MASKX;
            return 4;
        }
    }
}

// // AppendRune appends the UTF-8 encoding of r to the end of p and
// // returns the extended buffer. If the rune is out of range,
// // it appends the encoding of RUNE_ERROR.
// fn AppendRune(p []byte, r rune) []byte {
// 	// This function is inlineable for fast handling of ASCII.
// 	if uint32(r) <= RUNE1_MAX {
// 		return append(p, byte(r))
// 	}
// 	return appendRuneNonASCII(p, r)
// }

// fn appendRuneNonASCII(p []byte, r rune) []byte {
// 	// Negative values are erroneous. Making it unsigned addresses the problem.
// 	switch i := uint32(r); {
// 	case i <= RUNE2_MAX:
// 		return append(p, t2|byte(r>>6), tx|byte(r)&maskx)
// 	case i > MAX_RUNE, SURROGATE_MIN <= i && i <= SURROGATE_MAX:
// 		r = RUNE_ERROR
// 		fallthrough
// 	case i <= rune3Max:
// 		return append(p, t3|byte(r>>12), tx|byte(r>>6)&maskx, tx|byte(r)&maskx)
// 	default:
// 		return append(p, t4|byte(r>>18), tx|byte(r>>12)&maskx, tx|byte(r>>6)&maskx, tx|byte(r)&maskx)
// 	}
// }

/// rune_count returns the number of runes in p. Erroneous and short
/// encodings are treated as single runes of width 1 byte.
pub fn rune_count(p: &[u8]) -> usize {
    let np = p.len();
    let mut n = 0;
    let mut i = 0;
    while i < np {
        n += 1;
        let c = p[i];
        if (c as u32) < (RUNE_SELF as u32) {
            // ASCII fast path
            i += 1;
            continue;
        }
        let x = FIRST[c as usize];
        if x == XX {
            i += 1; // invalid.
            continue;
        }
        let mut size = (x & 7) as usize;
        if i + size > np {
            i += 1; // Short or invalid.
            continue;
        }
        let accept = &ACCEPT_RANGES[(x >> 4) as usize];
        let c = p[i + 1];
        if c < accept.lo || accept.hi < c {
            size = 1
        } else if size == 2 {
        } else {
            let c = p[i + 2];
            if !(LOCB..=HICB).contains(&c) {
                size = 1
            } else if size == 3 {
            } else {
                let c = p[i + 3];
                if !(LOCB..=HICB).contains(&c) {
                    size = 1;
                }
            }
        }
        i += size;
    }
    n
}

// // RuneCountInString is like rune_count but its input is a string.
// fn RuneCountInString(s string) (n int) {
// 	ns := len(s)
// 	for i := 0; i < ns; n++ {
// 		c := s[i]
// 		if c < RUNE_SELF {
// 			// ASCII fast path
// 			i++
// 			continue
// 		}
// 		x := FIRST[c]
// 		if x == XX {
// 			i++ // invalid.
// 			continue
// 		}
// 		size := int(x & 7)
// 		if i+size > ns {
// 			i++ // Short or invalid.
// 			continue
// 		}
// 		accept := acceptRanges[x>>4]
// 		if c := s[i+1]; c < accept.lo || accept.hi < c {
// 			size = 1
// 		} else if size == 2 {
// 		} else if c := s[i+2]; c < locb || hicb < c {
// 			size = 1
// 		} else if size == 3 {
// 		} else if c := s[i+3]; c < locb || hicb < c {
// 			size = 1
// 		}
// 		i += size
// 	}
// 	return n
// }

// // RuneStart reports whether the byte could be the first byte of an encoded,
// // possibly invalid rune. Second and subsequent bytes always have the top two
// // bits set to 10.
// fn RuneStart(b byte) -> bool { return b&0xC0 != 0x80 }

// // Valid reports whether p consists entirely of valid UTF-8-encoded runes.
// fn Valid(p: &[u8]) -> bool {
// 	// This optimization avoids the need to recompute the capacity
// 	// when generating code for p[8:], bringing it to parity with
// 	// ValidString, which was 20% faster on long ASCII strings.
// 	p = p[:p.len():p.len()]

// 	// Fast path. Check for and skip 8 bytes of ASCII characters per iteration.
// 	for p.len() >= 8 {
// 		// Combining two 32 bit loads allows the same code to be used
// 		// for 32 and 64 bit platforms.
// 		// The compiler can generate a 32bit load for first32 and second32
// 		// on many platforms. See test/codegen/memcombine.go.
// 		first32 := uint32(p[0]) | uint32(p[1])<<8 | uint32(p[2])<<16 | uint32(p[3])<<24
// 		second32 := uint32(p[4]) | uint32(p[5])<<8 | uint32(p[6])<<16 | uint32(p[7])<<24
// 		if (first32|second32)&0x80808080 != 0 {
// 			// Found a non ASCII byte (>= RUNE_SELF).
// 			break
// 		}
// 		p = p[8:]
// 	}
// 	let n = p.len();
// 	for i := 0; i < n; {
// 		pi := p[i]
// 		if pi < RUNE_SELF {
// 			i++
// 			continue
// 		}
// 		x := first[pi]
// 		if x == XX {
// 			return false // Illegal starter byte.
// 		}
// 		size := int(x & 7)
// 		if i+size > n {
// 			return false // Short or invalid.
// 		}
// 		accept := acceptRanges[x>>4]
// 		if c := p[i+1]; c < accept.lo || accept.hi < c {
// 			return false
// 		} else if size == 2 {
// 		} else if c := p[i+2]; c < locb || hicb < c {
// 			return false
// 		} else if size == 3 {
// 		} else if c := p[i+3]; c < locb || hicb < c {
// 			return false
// 		}
// 		i += size
// 	}
// 	return true
// }

// // ValidString reports whether s consists entirely of valid UTF-8-encoded runes.
// fn ValidString(s string) -> bool {
// 	// Fast path. Check for and skip 8 bytes of ASCII characters per iteration.
// 	for len(s) >= 8 {
// 		// Combining two 32 bit loads allows the same code to be used
// 		// for 32 and 64 bit platforms.
// 		// The compiler can generate a 32bit load for first32 and second32
// 		// on many platforms. See test/codegen/memcombine.go.
// 		first32 := uint32(s[0]) | uint32(s[1])<<8 | uint32(s[2])<<16 | uint32(s[3])<<24
// 		second32 := uint32(s[4]) | uint32(s[5])<<8 | uint32(s[6])<<16 | uint32(s[7])<<24
// 		if (first32|second32)&0x80808080 != 0 {
// 			// Found a non ASCII byte (>= RUNE_SELF).
// 			break
// 		}
// 		s = s[8:]
// 	}
// 	let n = len(s);
// 	for i := 0; i < n; {
// 		si := s[i]
// 		if si < RUNE_SELF {
// 			i++
// 			continue
// 		}
// 		x := first[si]
// 		if x == XX {
// 			return false // Illegal starter byte.
// 		}
// 		size := int(x & 7)
// 		if i+size > n {
// 			return false // Short or invalid.
// 		}
// 		accept := acceptRanges[x>>4]
// 		if c := s[i+1]; c < accept.lo || accept.hi < c {
// 			return false
// 		} else if size == 2 {
// 		} else if c := s[i+2]; c < locb || hicb < c {
// 			return false
// 		} else if size == 3 {
// 		} else if c := s[i+3]; c < locb || hicb < c {
// 			return false
// 		}
// 		i += size
// 	}
// 	return true
// }

/// ValidRune reports whether r can be legally encoded as UTF-8.
/// Code points that are out of range or a surrogate half are illegal.
pub fn valid_rune(r: u32) -> bool {
    (r < SURROGATE_MIN) || (SURROGATE_MAX < r && r <= (MAX_RUNE as u32))
}
