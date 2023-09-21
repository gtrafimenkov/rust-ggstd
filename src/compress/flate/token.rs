// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// 2 bits:   type   0 = literal  1=EOF  2=Match   3=Unused
// 8 bits:   xlength = length - MIN_MATCH_LENGTH
// 22 bits   xoffset = offset - MIN_OFFSET_SIZE, or literal
const LENGTH_SHIFT: u32 = 22;
const OFFSET_MASK: u32 = (1 << LENGTH_SHIFT) - 1;
// const TYPE_MASK: u32 = 3 << 30;
const LITERAL_TYPE: u32 = 0 << 30;
pub(super) const MATCH_TYPE: u32 = 1 << 30;

// The length code for length X (MIN_MATCH_LENGTH <= X <= MAX_MATCH_LENGTH)
// is lengthCodes[length - MIN_MATCH_LENGTH]
const LENGTH_CODES: &[u32] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 8, //
    9, 9, 10, 10, 11, 11, 12, 12, 12, 12, //
    13, 13, 13, 13, 14, 14, 14, 14, 15, 15, //
    15, 15, 16, 16, 16, 16, 16, 16, 16, 16, //
    17, 17, 17, 17, 17, 17, 17, 17, 18, 18, //
    18, 18, 18, 18, 18, 18, 19, 19, 19, 19, //
    19, 19, 19, 19, 20, 20, 20, 20, 20, 20, //
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, //
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, //
    21, 21, 21, 21, 21, 21, 22, 22, 22, 22, //
    22, 22, 22, 22, 22, 22, 22, 22, 22, 22, //
    22, 22, 23, 23, 23, 23, 23, 23, 23, 23, //
    23, 23, 23, 23, 23, 23, 23, 23, 24, 24, //
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, //
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, //
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, //
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, //
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, //
    25, 25, 25, 25, 25, 25, 25, 25, 25, 25, //
    25, 25, 26, 26, 26, 26, 26, 26, 26, 26, //
    26, 26, 26, 26, 26, 26, 26, 26, 26, 26, //
    26, 26, 26, 26, 26, 26, 26, 26, 26, 26, //
    26, 26, 26, 26, 27, 27, 27, 27, 27, 27, //
    27, 27, 27, 27, 27, 27, 27, 27, 27, 27, //
    27, 27, 27, 27, 27, 27, 27, 27, 27, 27, //
    27, 27, 27, 27, 27, 28, //
];

const OFFSET_CODES: &[u32] = &[
    0, 1, 2, 3, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, //
    8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, //
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, //
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, //
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, //
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, //
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, //
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, //
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, //
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, //
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, //
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, //
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, //
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, //
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, //
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, //
];

pub(super) type Token = u32;

/// Convert a literal into a literal token.
pub(super) fn literal_token(literal: u32) -> Token {
    (LITERAL_TYPE + literal) as Token
}

/// Convert a < xlength, xoffset > pair into a match token.
pub(super) fn match_token(xlength: u32, xoffset: u32) -> Token {
    (MATCH_TYPE + (xlength << LENGTH_SHIFT) + xoffset) as Token
}

pub(super) trait TokenTrait {
    fn literal(&self) -> u32;
    fn offset(&self) -> u32;
    fn length(&self) -> u32;
}

impl TokenTrait for Token {
    /// Returns the literal of a literal token.
    fn literal(&self) -> u32 {
        self - LITERAL_TYPE
    }

    /// Returns the extra offset of a match token.
    fn offset(&self) -> u32 {
        self & OFFSET_MASK
    }

    fn length(&self) -> u32 {
        (self - MATCH_TYPE) >> LENGTH_SHIFT
    }
}

pub(super) fn length_code(len: u32) -> usize {
    LENGTH_CODES[len as usize] as usize
}

// Returns the offset code corresponding to a specific offset.
pub(super) fn offset_code(off: usize) -> usize {
    if off < OFFSET_CODES.len() {
        return OFFSET_CODES[off] as usize;
    }
    if off >> 7 < OFFSET_CODES.len() {
        return (OFFSET_CODES[off >> 7] + 14) as usize;
    }
    (OFFSET_CODES[off >> 14] + 28) as usize
}
