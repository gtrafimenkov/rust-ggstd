// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::letter::{is, RangeTable, MAX_LATIN1};
use super::tables::{self, PROPERTIES};

/// Bit masks for each code point under U+0100, for fast lookup.
pub(super) const P_C: u8 = 1; // a control character.
pub(super) const P_P: u8 = 2; // a punctuation character.
pub(super) const P_N: u8 = 3; // a numeral.
pub(super) const P_S: u8 = 4; // a symbolic character.
pub(super) const P_Z: u8 = 5; // a spacing character.
pub(super) const P_LU: u8 = 6; // an upper-case letter.
pub(super) const P_LL: u8 = 7; // a lower-case letter.
pub(super) const PP: u8 = 8; // a printable character according to Go's definition.
#[allow(unused)]
pub(super) const PG: u8 = PP | P_Z; // a graphical character according to the Unicode definition.
pub(super) const P_LO: u8 = P_LL | P_LU; // a letter that is neither upper nor lower case.
#[allow(unused)]
pub(super) const P_LMASK: u8 = P_LO;

// // GraphicRanges defines the set of graphic characters according to Unicode.
// var GraphicRanges = []*RangeTable{
// 	L, M, N, P, S, Zs,
// }

/// PRINT_RANGES defines the set of printable characters according to Go.
/// ASCII space, U+0020, is handled separately.
static PRINT_RANGES: &[&RangeTable] = &[tables::L, tables::M, tables::N, tables::P, tables::S];

// // is_graphic reports whether the rune is defined as a Graphic by Unicode.
// // Such characters include letters, marks, numbers, punctuation, symbols, and
// // spaces, from categories L, M, N, P, S, Zs.
// fn is_graphic(r rune) bool {
// 	// We convert to uint32 to avoid the extra test for negative,
// 	// and in the index we convert to uint8 to avoid the range check.
// 	if uint32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pg != 0
// 	}
// 	return In(r, GraphicRanges...)
// }

/// is_print reports whether the rune is defined as printable by Go. Such
/// characters include letters, marks, numbers, punctuation, symbols, and the
/// ASCII space character, from categories L, M, N, P, S and the ASCII space
/// character. This categorization is the same as is_graphic except that the
/// only spacing character is ASCII space, U+0020.
#[allow(dead_code)]
pub fn is_print(r: char) -> bool {
    if r <= MAX_LATIN1 {
        return PROPERTIES[(r as u8) as usize] & PP != 0;
    }
    in_range(r, PRINT_RANGES)
}

// // IsOneOf reports whether the rune is a member of one of the ranges.
// // The function "In" provides a nicer signature and should be used in preference to IsOneOf.
// fn IsOneOf(ranges []*RangeTable, r rune) bool {
// 	for _, inside := range ranges {
// 		if Is(inside, r) {
// 			return true
// 		}
// 	}
// 	return false
// }

/// in_range reports whether the rune is a member of one of the ranges.
#[allow(dead_code)]
pub fn in_range(r: char, ranges: &[&RangeTable]) -> bool {
    for inside in ranges {
        if is(inside, r) {
            return true;
        }
    }
    false
}

// // IsControl reports whether the rune is a control character.
// // The C (Other) Unicode category includes more code points
// // such as surrogates; use Is(C, r) to test for them.
// fn IsControl(r rune) bool {
// 	if uint32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pC != 0
// 	}
// 	// All control characters are < MaxLatin1.
// 	return false
// }

// // IsLetter reports whether the rune is a letter (category L).
// fn IsLetter(r rune) bool {
// 	if uint32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&(pLmask) != 0
// 	}
// 	return isExcludingLatin(Letter, r)
// }

// // IsMark reports whether the rune is a mark character (category M).
// fn IsMark(r rune) bool {
// 	// There are no mark characters in Latin-1.
// 	return isExcludingLatin(Mark, r)
// }

// // IsNumber reports whether the rune is a number (category N).
// fn IsNumber(r rune) bool {
// 	if uint32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pN != 0
// 	}
// 	return isExcludingLatin(Number, r)
// }

// // IsPunct reports whether the rune is a Unicode punctuation character
// // (category P).
// fn IsPunct(r rune) bool {
// 	if uint32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pP != 0
// 	}
// 	return Is(Punct, r)
// }

// // IsSpace reports whether the rune is a space character as defined
// // by Unicode's White Space property; in the Latin-1 space
// // this is
// //
// //	'\t', '\n', '\v', '\f', '\r', ' ', U+0085 (NEL), U+00A0 (NBSP).
// //
// // Other definitions of spacing characters are set by category
// // Z and property Pattern_White_Space.
// fn IsSpace(r rune) bool {
// 	// This property isn't the same as Z; special-case it.
// 	if uint32(r) <= MaxLatin1 {
// 		switch r {
// 		case '\t', '\n', '\v', '\f', '\r', ' ', 0x85, 0xA0:
// 			return true
// 		}
// 		return false
// 	}
// 	return isExcludingLatin(White_Space, r)
// }

// // IsSymbol reports whether the rune is a symbolic character.
// fn IsSymbol(r rune) bool {
// 	if uint32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pS != 0
// 	}
// 	return isExcludingLatin(Symbol, r)
// }
