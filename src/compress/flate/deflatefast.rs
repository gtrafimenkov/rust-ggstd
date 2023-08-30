// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! This encoding algorithm, which prioritizes speed over output size, is
//! based on Snappy's LZ77-style encoder: github.com/golang/snappy

use super::deflate::{
    BASE_MATCH_LENGTH, BASE_MATCH_OFFSET, MAX_MATCH_LENGTH, MAX_MATCH_OFFSET, MAX_STORE_BLOCK_SIZE,
};
use super::token::{literal_token, match_token, Token};

const TABLE_BITS: usize = 14; // Bits used in the table.
pub(super) const TABLE_SIZE: u32 = 1 << TABLE_BITS; // Size of the table.
const TABLE_MASK: u32 = TABLE_SIZE - 1; // Mask for table indices. Redundant, but can eliminate bounds checks.
const TABLE_SHIFT: usize = 32 - TABLE_BITS; // Right-shift to get the tableBits most significant bits of a u32.

// Reset the buffer offset when reaching this.
// Offsets are stored between blocks as i32 values.
// Since the offset we are checking against is at the beginning
// of the buffer, we need to subtract the current and input
// buffer to not risk overflowing the i32.
pub(super) const BUFFER_RESET: i32 = i32::MAX - (MAX_STORE_BLOCK_SIZE as i32) * 2;

fn load32(b: &[u8], i: usize) -> u32 {
    // 	b = b[i : i+4 : b.len()] // Help the compiler eliminate bounds checks on the next line.
    let b = &b[i..i + 4];
    return (b[0] as u32) | (b[1] as u32) << 8 | (b[2] as u32) << 16 | (b[3] as u32) << 24;
}

fn load64(b: &[u8], i: usize) -> u64 {
    // 	b = b[i : i+8 : b.len()] // Help the compiler eliminate bounds checks on the next line.
    let b = &b[i..i + 8];
    return (b[0] as u64)
        | (b[1] as u64) << 8
        | (b[2] as u64) << 16
        | (b[3] as u64) << 24
        | (b[4] as u64) << 32
        | (b[5] as u64) << 40
        | (b[6] as u64) << 48
        | (b[7] as u64) << 56;
}

fn hash(u: u32) -> u32 {
    u.overflowing_mul(0x1e35a7bd).0 >> TABLE_SHIFT
    // return (u * 0x1e35a7bd) >> tableShift;
}

// These constants are defined by the Snappy implementation so that its
// assembly implementation can fast-path some 16-bytes-at-a-time copies. They
// aren't necessary in the pure Go implementation, as we don't use those same
// optimizations, but using the same thresholds doesn't really hurt.
const INPUT_MARGIN: usize = 16 - 1;
const MIN_NON_LITERAL_BLOCK_SIZE: usize = 1 + 1 + INPUT_MARGIN;

#[derive(Default, Clone, Copy)]
pub(super) struct TableEntry {
    val: u32, // Value at destination
    offset: i32,
}

/// DeflateFast maintains the table for matches,
/// and the previous byte block for cross block matching.
pub(super) struct DeflateFast {
    pub(super) table: Vec<TableEntry>,
    pub(super) prev: Vec<u8>, // Previous block, zero length if unknown.
    pub(super) cur: i32,      // Current match offset.
}

impl DeflateFast {
    pub(super) fn new() -> Self {
        return Self {
            table: vec![TableEntry::default(); TABLE_SIZE as usize],
            prev: vec![0; MAX_STORE_BLOCK_SIZE],
            cur: MAX_STORE_BLOCK_SIZE as i32,
        };
    }
}

impl DeflateFast {
    /// encode encodes a block given in src and appends tokens
    /// to dst.
    pub(super) fn encode(&mut self, dst: &mut Vec<Token>, src: &[u8]) {
        // Ensure that self.cur doesn't wrap.
        if self.cur >= BUFFER_RESET {
            self.shift_offsets();
        }

        // This check isn't in the Snappy implementation, but there, the caller
        // instead of the callee handles this case.
        if src.len() < MIN_NON_LITERAL_BLOCK_SIZE {
            self.cur += MAX_STORE_BLOCK_SIZE as i32;
            self.prev.truncate(0);
            emit_literal(dst, src);
            return;
        }

        // sLimit is when to stop looking for offset/length copies. The inputMargin
        // lets us use a fast path for emitLiteral in the main loop, while we are
        // looking for copies.
        let s_limit = src.len() - INPUT_MARGIN;

        // nextEmit is where in src the next emitLiteral should start from.
        let mut next_emit = 0 as usize;
        let mut s = 0 as usize;
        let mut cv = load32(src, s);
        let mut next_hash = hash(cv);

        'outer: loop {
            // Copied from the C++ snappy implementation:
            //
            // Heuristic match skipping: If 32 bytes are scanned with no matches
            // found, start looking only at every other byte. If 32 more bytes are
            // scanned (or skipped), look at every third byte, etc.. When a match
            // is found, immediately go back to looking at every byte. This is a
            // small loss (~5% performance, ~0.1% density) for compressible data
            // due to more bookkeeping, but for non-compressible data (such as
            // JPEG) it's a huge win since the compressor quickly "realizes" the
            // data is incompressible and doesn't bother looking for matches
            // everywhere.
            //
            // The "skip" variable keeps track of how many bytes there are since
            // the last match; dividing it by 32 (ie. right-shifting by five) gives
            // the number of bytes to move ahead for each iteration.
            let mut skip = 32 as usize;

            let mut next_s = s;
            let mut candidate: TableEntry;
            loop {
                s = next_s;
                let bytes_between_hash_lookups = skip >> 5;
                next_s = s + bytes_between_hash_lookups;
                skip += bytes_between_hash_lookups;
                if next_s > s_limit {
                    break 'outer;
                }
                candidate = self.table[(next_hash & TABLE_MASK) as usize];
                let now = load32(src, next_s);
                self.table[(next_hash & TABLE_MASK) as usize] = TableEntry {
                    offset: s as i32 + self.cur,
                    val: cv,
                };
                next_hash = hash(now);

                let offset = s as i32 - (candidate.offset - self.cur);
                if offset > MAX_MATCH_OFFSET as i32 || cv != candidate.val {
                    // Out of range or not matched.
                    cv = now;
                    continue;
                }
                break;
            }

            // A 4-byte match has been found. We'll later see if more than 4 bytes
            // match. But, prior to the match, src[nextEmit:s] are unmatched. Emit
            // them as literal bytes.
            emit_literal(dst, &src[next_emit..s as usize]);

            // Call emitCopy, and then see if another emitCopy could be our next
            // move. Repeat until we find no match for the input immediately after
            // what was consumed by the last emitCopy call.
            //
            // If we exit this loop normally then we need to call emitLiteral next,
            // though we don't yet know how big the literal will be. We handle that
            // by proceeding to the next iteration of the main loop. We also can
            // exit this loop via goto if we get close to exhausting the input.
            loop {
                // Invariant: we have a 4-byte match at s, and no need to emit any
                // literal bytes prior to s.

                // Extend the 4-byte match as long as possible.
                s += 4;
                let t = candidate.offset - self.cur + 4;
                let l = self.match_len(s, t, src);

                // matchToken is flate's equivalent of Snappy's emitCopy. (length,offset)
                dst.push(match_token(
                    (l + 4 - BASE_MATCH_LENGTH) as u32,
                    (s as i32 - t - BASE_MATCH_OFFSET as i32) as u32,
                ));
                s += l;
                next_emit = s;
                if s >= s_limit {
                    break 'outer;
                }

                // We could immediately start working at s now, but to improve
                // compression we first update the hash table at s-1 and at s. If
                // another emitCopy is not our next move, also calculate nextHash
                // at s+1. At least on GOARCH=amd64, these three hash calculations
                // are faster as one load64 call (with some shifts) instead of
                // three load32 calls.
                let mut x = load64(src, s - 1);
                let prev_hash = hash(x as u32);
                self.table[(prev_hash & TABLE_MASK) as usize] = TableEntry {
                    offset: self.cur + s as i32 - 1,
                    val: x as u32,
                };
                x >>= 8;
                let curr_hash = hash(x as u32);
                candidate = self.table[(curr_hash & TABLE_MASK) as usize];
                self.table[(curr_hash & TABLE_MASK) as usize] = TableEntry {
                    offset: self.cur + s as i32,
                    val: x as u32,
                };

                let offset = (s as i32) - (candidate.offset - self.cur);
                if offset > MAX_MATCH_OFFSET as i32 || (x as u32) != candidate.val {
                    cv = (x >> 8) as u32;
                    next_hash = hash(cv);
                    s += 1;
                    break;
                }
            }
        }

        // emitRemainder:
        if next_emit < src.len() {
            emit_literal(dst, &src[next_emit..])
        }
        self.cur += src.len() as i32;
        self.prev.resize(src.len(), 0);
        self.prev.copy_from_slice(src);
    }
}

fn emit_literal(dst: &mut Vec<Token>, lit: &[u8]) {
    for v in lit {
        dst.push(literal_token(*v as u32));
    }
}

impl DeflateFast {
    /// matchLen returns the match length between src[s..] and src[t..].
    /// t can be negative to indicate the match is starting in self.prev.
    /// We assume that src[s-4:s] and src[t-4:t] already match.
    pub(super) fn match_len(&mut self, s: usize, t: i32, src: &[u8]) -> usize {
        let mut s1 = s + MAX_MATCH_LENGTH - 4;
        if s1 > src.len() {
            s1 = src.len();
        }

        // If we are inside the current block
        if t >= 0 {
            let t = t as usize;
            let b = &src[t..];
            let a = &src[s..s1];
            let b = &b[..a.len()];
            // Extend the match to be as long as possible.
            for i in 0..a.len() {
                if a[i] != b[i] {
                    return i;
                }
            }
            return a.len();
        }

        // We found a match in the previous block.
        let tp = self.prev.len() as i32 + t;
        if tp < 0 {
            return 0;
        }

        // Extend the match to be as long as possible.
        let tp = tp as usize;
        let mut a = &src[s..s1];
        let mut b = &self.prev[tp..];
        if b.len() > a.len() {
            b = &b[..a.len()];
        }
        a = &a[..b.len()];
        for i in 0..b.len() {
            if a[i] != b[i] {
                return i;
            }
        }

        // If we reached our limit, we matched everything we are
        // allowed to in the previous block and we return.
        let n = b.len();
        if (s + n) == s1 {
            return n;
        }

        // Continue looking for more matches in the current block.
        a = &src[s + n..s1];
        b = &src[..a.len()];
        for i in 0..a.len() {
            if a[i] != b[i] {
                return i + n;
            }
        }
        return a.len() + n;
    }

    /// reset resets the encoding history.
    /// This ensures that no matches are made to the previous block.
    pub(super) fn reset(&mut self) {
        self.prev.truncate(0);
        // Bump the offset, so all matches will fail distance check.
        // Nothing should be >= self.cur in the table.
        self.cur += MAX_MATCH_OFFSET as i32;

        // Protect against self.cur wraparound.
        if self.cur >= BUFFER_RESET {
            self.shift_offsets();
        }
    }

    /// shiftOffsets will shift down all match offset.
    /// This is only called in rare situations to prevent integer overflow.
    ///
    /// See https://golang.org/issue/18636 and https://github.com/golang/go/issues/34121.
    pub(super) fn shift_offsets(&mut self) {
        if self.prev.len() == 0 {
            // We have no history; just clear the table.
            for i in 0..self.table.len() {
                self.table[i] = TableEntry::default();
            }
            self.cur = MAX_MATCH_OFFSET as i32 + 1;
            return;
        }

        // Shift down everything in the table that isn't already too far away.
        for i in 0..self.table.len() {
            let mut v = self.table[i].offset - self.cur + MAX_MATCH_OFFSET as i32 + 1;
            if v < 0 {
                // We want to reset self.cur to maxMatchOffset + 1, so we need to shift
                // all table entries down by (self.cur - (maxMatchOffset + 1)).
                // Because we ignore matches > maxMatchOffset, we can cap
                // any negative offsets at 0.
                v = 0;
            }
            self.table[i].offset = v;
        }
        self.cur = MAX_MATCH_OFFSET as i32 + 1;
    }
}
