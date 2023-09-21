// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// Maximum valid Unicode code point.
pub const MAX_RUNE: char = '\u{10FFFF}';
/// Represents invalid code points.
pub const REPLACEMENT_CHAR: char = '\u{FFFD}';
/// maximum ASCII value.
pub const MAX_ASCII: char = '\u{007F}';
/// maximum Latin-1 value.
pub const MAX_LATIN1: char = '\u{00FF}';

/// RangeTable defines a set of Unicode code points by listing the ranges of
/// code points within the set. The ranges are listed in two slices
/// to save space: a slice of 16-bit ranges and a slice of 32-bit ranges.
/// The two slices must be in sorted order and non-overlapping.
/// Also, R32 should contain only values >= 0x10000 (1<<16).
pub struct RangeTable {
    pub r16: &'static [Range16],
    pub r32: &'static [Range32],
    pub latin_offset: isize, // number of entries in R16 with Hi <= MaxLatin1
}

/// Range16 represents of a range of 16-bit Unicode code points. The range runs from lo to Hi
/// inclusive and has the specified stride.
pub struct Range16 {
    pub lo: u16,
    pub hi: u16,
    pub stride: u16,
}

impl Range16 {
    pub const fn new(lo: u16, hi: u16, stride: u16) -> Self {
        Self { lo, hi, stride }
    }
}

/// Range32 represents of a range of Unicode code points and is used when one or
/// more of the values will not fit in 16 bits. The range runs from lo to Hi
/// inclusive and has the specified stride. lo and Hi must always be >= 1<<16.
pub struct Range32 {
    pub lo: u32,
    pub hi: u32,
    pub stride: u32,
}

impl Range32 {
    pub const fn new(lo: u32, hi: u32, stride: u32) -> Self {
        Self { lo, hi, stride }
    }
}

// // CaseRange represents a range of Unicode code points for simple (one
// // code point to one code point) case conversion.
// // The range runs from lo to Hi inclusive, with a fixed stride of 1. Deltas
// // are the number to add to the code point to reach the code point for a
// // different case for that character. They may be negative. If zero, it
// // means the character is in the corresponding case. There is a special
// // case representing sequences of alternating corresponding Upper and Lower
// // pairs. It appears with a fixed Delta of
// //
// //	{UpperLower, UpperLower, UpperLower}
// //
// // The constant UpperLower has an otherwise impossible delta value.
// type CaseRange struct {
// 	lo    u32
// 	Hi    u32
// 	Delta d
// }

// // SpecialCase represents language-specific case mappings such as Turkish.
// // Methods of SpecialCase customize (by overriding) the standard mappings.
// type SpecialCase []CaseRange

// // BUG(r): There is no mechanism for full case folding, that is, for
// // characters that involve multiple runes in the input or output.

// // Indices into the Delta arrays inside CaseRanges for case mapping.
// const (
// 	UpperCase = iota
// 	LowerCase
// 	TitleCase
// 	MaxCase
// )

// type d [MaxCase]rune // to make the CaseRanges text shorter

// // If the Delta field of a CaseRange is UpperLower, it means
// // this CaseRange represents a sequence of the form (say)
// // Upper Lower Upper Lower.
// const (
// 	UpperLower = MAX_RUNE + 1 // (Cannot be a valid delta.)
// )

/// linearMax is the maximum size table for linear search for non-Latin1 rune.
/// Derived by running 'go test -calibrate'.
const LINEAR_MAX: usize = 18;

// is16 reports whether r is in the sorted slice of 16-bit ranges.
fn is16(ranges: &[Range16], r: u16) -> bool {
    if ranges.len() <= LINEAR_MAX || r as u32 <= MAX_LATIN1 as u32 {
        for range_ in ranges {
            if r < range_.lo {
                return false;
            }
            if r <= range_.hi {
                return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
            }
        }
        return false;
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = ranges.len();
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let range_ = &ranges[m];
        if range_.lo <= r && r <= range_.hi {
            return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
        }
        if r < range_.lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    false
}

/// is32 reports whether r is in the sorted slice of 32-bit ranges.
#[allow(dead_code)]
fn is32(ranges: &[Range32], r: u32) -> bool {
    if ranges.len() <= LINEAR_MAX {
        for range_ in ranges {
            if r < range_.lo {
                return false;
            }
            if r <= range_.hi {
                return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
            }
        }
        return false;
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = ranges.len();
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let range_ = &ranges[m];
        if range_.lo <= r && r <= range_.hi {
            return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
        }
        if r < range_.lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    false
}

/// Is reports whether the rune is in the specified table of ranges.
#[allow(dead_code)]
pub fn is(range_tab: &RangeTable, r: char) -> bool {
    let r16 = range_tab.r16;
    // 	// Compare as u32 to correctly handle negative runes.
    if !r16.is_empty() && r as u32 <= r16[r16.len() - 1].hi as u32 {
        return is16(r16, r as u16);
    }
    let r32 = range_tab.r32;
    if !r32.is_empty() && r as u32 >= r32[0].lo {
        return is32(r32, r as u32);
    }
    false
}

// fn isExcludingLatin(rangeTab *RangeTable, r rune) -> bool {
// 	r16 := rangeTab.R16
// 	// Compare as u32 to correctly handle negative runes.
// 	if off := rangeTab.LatinOffset; r16.len() > off && u32(r) <= u32(r16[r16.len()-1].hi) {
// 		return is16(r16[off:], u16(r))
// 	}
// 	r32 := rangeTab.R32
// 	if r32.len() > 0 && r >= rune(r32[0].lo) {
// 		return is32(r32, u32(r))
// 	}
// 	return false
// }

// // IsUpper reports whether the rune is an upper case letter.
// fn IsUpper(r rune) -> bool {
// 	// See comment in is_graphic.
// 	if u32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pLmask == pLu
// 	}
// 	return isExcludingLatin(Upper, r)
// }

// // IsLower reports whether the rune is a lower case letter.
// fn IsLower(r rune) -> bool {
// 	// See comment in is_graphic.
// 	if u32(r) <= MaxLatin1 {
// 		return properties[uint8(r)]&pLmask == pLl
// 	}
// 	return isExcludingLatin(Lower, r)
// }

// // IsTitle reports whether the rune is a title case letter.
// fn IsTitle(r rune) -> bool {
// 	if r <= MaxLatin1 {
// 		return false
// 	}
// 	return isExcludingLatin(Title, r)
// }

// // to maps the rune using the specified case mapping.
// // It additionally reports whether caseRange contained a mapping for r.
// fn to(_case isize, r rune, caseRange []CaseRange) (mappedRune rune, foundMapping bool) {
// 	if _case < 0 || MaxCase <= _case {
// 		return ReplacementChar, false // as reasonable an error as any
// 	}
// 	// binary search over ranges
// 	lo := 0
// 	hi := len(caseRange)
// 	for lo < hi {
// 		m := lo + (hi-lo)/2
// 		cr := caseRange[m]
// 		if rune(cr.lo) <= r && r <= rune(cr.hi) {
// 			delta := cr.Delta[_case]
// 			if delta > MAX_RUNE {
// 				// In an Upper-Lower sequence, which always starts with
// 				// an UpperCase letter, the real deltas always look like:
// 				//	{0, 1, 0}    UpperCase (Lower is next)
// 				//	{-1, 0, -1}  LowerCase (Upper, Title are previous)
// 				// The characters at even offsets from the beginning of the
// 				// sequence are upper case; the ones at odd offsets are lower.
// 				// The correct mapping can be done by clearing or setting the low
// 				// bit in the sequence offset.
// 				// The constants UpperCase and TitleCase are even while LowerCase
// 				// is odd so we take the low bit from _case.
// 				return rune(cr.lo) + ((r-rune(cr.lo))&^1 | rune(_case&1)), true
// 			}
// 			return r + delta, true
// 		}
// 		if r < rune(cr.lo) {
// 			hi = m
// 		} else {
// 			lo = m + 1
// 		}
// 	}
// 	return r, false
// }

// // To maps the rune to the specified case: UpperCase, LowerCase, or TitleCase.
// fn To(_case isize, r rune) rune {
// 	r, _ = to(_case, r, CaseRanges)
// 	return r
// }

// // ToUpper maps the rune to upper case.
// fn ToUpper(r rune) rune {
// 	if r <= MaxASCII {
// 		if 'a' <= r && r <= 'z' {
// 			r -= 'a' - 'A'
// 		}
// 		return r
// 	}
// 	return To(UpperCase, r)
// }

// // ToLower maps the rune to lower case.
// fn ToLower(r rune) rune {
// 	if r <= MaxASCII {
// 		if 'A' <= r && r <= 'Z' {
// 			r += 'a' - 'A'
// 		}
// 		return r
// 	}
// 	return To(LowerCase, r)
// }

// // ToTitle maps the rune to title case.
// fn ToTitle(r rune) rune {
// 	if r <= MaxASCII {
// 		if 'a' <= r && r <= 'z' { // title case is upper case for ASCII
// 			r -= 'a' - 'A'
// 		}
// 		return r
// 	}
// 	return To(TitleCase, r)
// }

// // ToUpper maps the rune to upper case giving priority to the special mapping.
// fn (special SpecialCase) ToUpper(r rune) rune {
// 	r1, hadMapping := to(UpperCase, r, []CaseRange(special))
// 	if r1 == r && !hadMapping {
// 		r1 = ToUpper(r)
// 	}
// 	return r1
// }

// // ToTitle maps the rune to title case giving priority to the special mapping.
// fn (special SpecialCase) ToTitle(r rune) rune {
// 	r1, hadMapping := to(TitleCase, r, []CaseRange(special))
// 	if r1 == r && !hadMapping {
// 		r1 = ToTitle(r)
// 	}
// 	return r1
// }

// // ToLower maps the rune to lower case giving priority to the special mapping.
// fn (special SpecialCase) ToLower(r rune) rune {
// 	r1, hadMapping := to(LowerCase, r, []CaseRange(special))
// 	if r1 == r && !hadMapping {
// 		r1 = ToLower(r)
// 	}
// 	return r1
// }

// // caseOrbit is defined in tables.go as []foldPair. Right now all the
// // entries fit in u16, so use u16. If that changes, compilation
// // will fail (the constants in the composite literal will not fit in u16)
// // and the types here can change to u32.
// type foldPair struct {
// 	From u16
// 	To   u16
// }

// // SimpleFold iterates over Unicode code points equivalent under
// // the Unicode-defined simple case folding. Among the code points
// // equivalent to rune (including rune itself), SimpleFold returns the
// // smallest rune > r if one exists, or else the smallest rune >= 0.
// // If r is not a valid Unicode code point, SimpleFold(r) returns r.
// //
// // For example:
// //
// //	SimpleFold('A') = 'a'
// //	SimpleFold('a') = 'A'
// //
// //	SimpleFold('K') = 'k'
// //	SimpleFold('k') = '\u212A' (Kelvin symbol, K)
// //	SimpleFold('\u212A') = 'K'
// //
// //	SimpleFold('1') = '1'
// //
// //	SimpleFold(-2) = -2
// fn SimpleFold(r rune) rune {
// 	if r < 0 || r > MAX_RUNE {
// 		return r
// 	}

// 	if isize(r) < len(asciiFold) {
// 		return rune(asciiFold[r])
// 	}

// 	// Consult caseOrbit table for special cases.
// 	lo := 0
// 	hi := len(caseOrbit)
// 	for lo < hi {
// 		m := lo + (hi-lo)/2
// 		if rune(caseOrbit[m].From) < r {
// 			lo = m + 1
// 		} else {
// 			hi = m
// 		}
// 	}
// 	if lo < len(caseOrbit) && rune(caseOrbit[lo].From) == r {
// 		return rune(caseOrbit[lo].To)
// 	}

// 	// No folding specified. This is a one- or two-element
// 	// equivalence class containing rune and ToLower(rune)
// 	// and ToUpper(rune) if they are different from rune.
// 	if l := ToLower(r); l != r {
// 		return l
// 	}
// 	return ToUpper(r)
// }
