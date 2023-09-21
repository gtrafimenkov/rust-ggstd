// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::inflate::MAX_NUM_LIT;
use crate::math::bits;
use std::sync::OnceLock;

/// hcode is a huffman code with a bit code and bit length.
#[derive(Default, Clone, Copy)]
pub(super) struct HCode {
    pub code: u16,
    pub len: u16,
}

pub(super) struct HuffmanEncoder {
    pub codes: Vec<HCode>,
    freqcache: Option<Vec<LiteralNode>>,
    // bit_count[i] indicates the number of literals that should be encoded in i bits
    bit_count: [u32; 17],
}

#[derive(Default, Clone, Copy)]
struct LiteralNode {
    literal: u16,
    freq: i32,
}

/// A levelInfo describes the state of the constructed tree for a given depth.
#[derive(Default, Clone, Copy)]
struct LevelInfo {
    // Our level.  for better printing
    level: u32,

    // The frequency of the last node at this level
    last_freq: i32,

    // The frequency of the next character to add to this level
    next_char_freq: i32,

    // The frequency of the next pair (from level below) to add to this level.
    // Only valid if the "needed" value of the next lower level is 0.
    next_pair_freq: i32,

    // The number of chains remaining to generate for this level before moving
    // up to the next level
    needed: u32,
}

impl HCode {
    /// set sets the code and length of an hcode.
    fn set(&mut self, code: u16, length: u16) {
        self.len = length;
        self.code = code;
    }
}

fn max_node() -> LiteralNode {
    LiteralNode {
        literal: u16::MAX,
        freq: i32::MAX,
    }
}

pub(super) fn new_huffman_encoder(size: usize) -> HuffmanEncoder {
    HuffmanEncoder {
        codes: vec![HCode::default(); size],
        freqcache: None,
        bit_count: [0; 17],
    }
}

/// Generates a HuffmanCode corresponding to the fixed literal table.
fn generate_fixed_literal_encoding() -> HuffmanEncoder {
    let mut h = new_huffman_encoder(MAX_NUM_LIT);
    for ch in 0..MAX_NUM_LIT as u16 {
        let bits;
        let size;
        if ch < 144 {
            // size 8, 000110000  .. 10111111
            bits = ch + 48;
            size = 8;
        } else if (144..256).contains(&ch) {
            // size 9, 110010000 .. 111111111
            bits = ch + 400 - 144;
            size = 9;
        } else if (256..280).contains(&ch) {
            // size 7, 0000000 .. 0010111
            bits = ch - 256;
            size = 7;
        } else {
            // size 8, 11000000 .. 11000111
            bits = ch + 192 - 280;
            size = 8;
        }
        h.codes[ch as usize] = HCode {
            code: reverse_bits(bits, size as u8),
            len: size,
        }
    }
    h
}

fn generate_fixed_offset_encoding() -> HuffmanEncoder {
    let mut h = new_huffman_encoder(30);
    for ch in 0..h.codes.len() {
        h.codes[ch] = HCode {
            code: reverse_bits(ch as u16, 5),
            len: 5,
        }
    }
    h
}

// var fixedLiteralEncoding *huffmanEncoder = generate_fixed_literal_encoding()
// var fixedOffsetEncoding *huffmanEncoder = generate_fixed_offset_encoding()
pub(super) fn get_fixed_literal_encoding() -> &'static HuffmanEncoder {
    static ENCODER: OnceLock<HuffmanEncoder> = OnceLock::new();
    ENCODER.get_or_init(generate_fixed_literal_encoding)
}
pub(super) fn get_fixed_offset_encoding() -> &'static HuffmanEncoder {
    static ENCODER: OnceLock<HuffmanEncoder> = OnceLock::new();
    ENCODER.get_or_init(generate_fixed_offset_encoding)
}

const MAX_BITS_LIMIT: usize = 16;

impl HuffmanEncoder {
    pub fn bit_length(&self, freq: &[i32]) -> usize {
        let mut total = 0;
        for (i, f) in freq.iter().enumerate() {
            if *f != 0 {
                total += *f as usize * self.codes[i].len as usize;
            }
        }
        total
    }

    /// bit_counts computes the number of literals assigned to each bit size in the Huffman encoding.
    /// It is only called when list.length >= 3.
    /// The cases of 0, 1, and 2 literals are handled by special case code.
    ///
    /// list is an array of the literals with non-zero frequencies
    /// and their associated frequencies. The array is in order of increasing
    /// frequency and has as its last element a special element with frequency
    /// MaxInt32.
    ///
    /// max_bits is the maximum number of bits that should be used to encode any literal.
    /// It must be less than 16.
    ///
    /// Results of the calculation will be put to self.bitCount array.
    /// self.bitCount[i] indicates the number of literals that should be encoded in i bits.
    ///
    /// bit_counts returns number of valid elements in self.bitCount array.
    fn bit_counts(&mut self, list_size: usize, max_bits: usize) -> usize {
        if max_bits >= MAX_BITS_LIMIT {
            panic!("flate: max_bits too large");
        }
        let n = list_size;
        let list = &mut self.freqcache.as_mut().unwrap()[..n + 1];

        list[n] = max_node();

        let mut max_bits = max_bits;
        // The tree can't have greater depth than n - 1, no matter what. This
        // saves a little bit of work in some small cases
        if max_bits > n - 1 {
            max_bits = n - 1;
        }

        // Create information about each of the levels.
        // A bogus "Level 0" whose sole purpose is so that
        // level1.prev.needed==0.  This makes level1.nextPairFreq
        // be a legitimate value that never gets chosen.
        let mut levels = vec![LevelInfo::default(); MAX_BITS_LIMIT];

        // leafCounts[i] counts the number of literals at the left
        // of ancestors of the rightmost node at level i.
        // leafCounts[i][j] is the number of literals at the left
        // of the level j ancestor.
        let mut leaf_counts = vec![vec![0; MAX_BITS_LIMIT]; MAX_BITS_LIMIT];

        for level in 1..=max_bits {
            // For every level, the first two items are the first two characters.
            // We initialize the levels as if we had already figured this out.
            levels[level] = LevelInfo {
                level: level as u32,
                last_freq: list[1].freq,
                next_char_freq: list[2].freq,
                next_pair_freq: list[0].freq + list[1].freq,
                needed: 0,
            };
            leaf_counts[level][level] = 2;
            if level == 1 {
                levels[level].next_pair_freq = i32::MAX;
            }
        }

        // We need a total of 2*n - 2 items at top level and have already generated 2.
        levels[max_bits].needed = 2 * n as u32 - 4;

        let mut level = max_bits;
        loop {
            // let l = &mut levels[level];
            if levels[level].next_pair_freq == i32::MAX && levels[level].next_char_freq == i32::MAX
            {
                // We've run out of both leafs and pairs.
                // End all calculations for this level.
                // To make sure we never come back to this level or any lower level,
                // set nextPairFreq impossibly large.
                levels[level].needed = 0;
                levels[level + 1].next_pair_freq = i32::MAX;
                level += 1;
                continue;
            }

            let prev_freq = levels[level].last_freq;
            if levels[level].next_char_freq < levels[level].next_pair_freq {
                // The next item on this row is a leaf node.
                let n = leaf_counts[level][level] + 1;
                levels[level].last_freq = levels[level].next_char_freq;
                // Lower leafCounts are the same of the previous node.
                leaf_counts[level][level] = n;
                levels[level].next_char_freq = list[n].freq;
            } else {
                // The next item on this row is a pair from the previous row.
                // nextPairFreq isn't valid until we generate two
                // more values in the level below
                levels[level].last_freq = levels[level].next_pair_freq;
                // Take leaf counts from the lower level, except counts[level] remains the same.
                // copy(leafCounts[level][..level], leafCounts[level - 1][..level]);
                for i in 0..level {
                    leaf_counts[level][i] = leaf_counts[level - 1][i];
                }
                let index = levels[level].level as usize - 1;
                levels[index].needed = 2;
            }

            levels[level].needed -= 1;
            if levels[level].needed == 0 {
                // We've done everything we need to do for this level.
                // Continue calculating one level up. Fill in nextPairFreq
                // of that level with the sum of the two nodes we've just calculated on
                // this level.
                if levels[level].level == max_bits as u32 {
                    // All done!
                    break;
                }
                let index = levels[level].level as usize + 1;
                levels[index].next_pair_freq = prev_freq + levels[level].last_freq;
                level += 1;
            } else {
                // If we stole from below, move down temporarily to replenish it.
                while levels[level - 1].needed > 0 {
                    level -= 1;
                }
            }
        }

        // Somethings is wrong if at the end, the top level is null or hasn't used
        // all of the leaves.
        if leaf_counts[max_bits][max_bits] != n {
            panic!("leafCounts[max_bits][max_bits] != n");
        }

        let mut bits = 1;
        let counts = &leaf_counts[max_bits];
        for level in (1..=max_bits).rev() {
            // 	for level := max_bits; level > 0; level-- {
            // chain.leafCount gives the number of literals requiring at least "bits"
            // bits to encode.
            self.bit_count[bits] = (counts[level] - counts[level - 1]) as u32;
            bits += 1;
        }
        // return &self.bitCount[..max_bits + 1];
        max_bits
    }

    /// Look at the leaves and assign them a bit count and an encoding as specified
    /// in RFC 1951 3.2.2
    fn assign_encoding_and_size(&mut self, max_bits: usize, list_size: usize) {
        let bit_count = &self.bit_count[..max_bits + 1];
        let mut list = &mut self.freqcache.as_mut().unwrap()[..list_size];
        let mut code = 0_u16;
        for (n, bits) in bit_count.iter().enumerate() {
            code <<= 1;
            if n == 0 || *bits == 0 {
                continue;
            }
            // The literals list[list_size-bits] .. list[list_size-bits]
            // are encoded using "bits" bits, and get the values
            // code, code + 1, ....  The code values are
            // assigned in literal order (not frequency order).
            let size = list.len() - (*bits as usize);
            let chunk = &mut list[size..];

            chunk.sort_by_key(|node| node.literal);

            for node in chunk {
                self.codes[node.literal as usize] = HCode {
                    code: reverse_bits(code, n as u8),
                    len: n as u16,
                };
                code += 1;
            }
            list = &mut list[0..size];
        }
    }

    /// Update this Huffman Code object to be the minimum code for the specified frequency count.
    ///
    /// freq is an array of frequencies, in which freq[i] gives the frequency of literal i.
    /// max_bits  The maximum number of bits to use for any literal.
    pub fn generate(&mut self, freq: &[i32], max_bits: usize) {
        if self.freqcache.is_none() {
            // Allocate a reusable buffer with the longest possible frequency table.
            // Possible lengths are codegenCodeCount, offsetCodeCount and maxNumLit.
            // The largest of these is maxNumLit, so we allocate for that case.
            // self.freqcache = make([]literalNode, maxNumLit+1)
            self.freqcache = Some(vec![LiteralNode::default(); MAX_NUM_LIT + 1]);
        }
        let freqcache = self.freqcache.as_mut().unwrap();
        // Number of non-zero literals
        let mut list_size = 0;
        // Set list to be the set of all non-zero literals and their frequencies
        for (i, f) in freq.iter().enumerate() {
            if *f != 0 {
                freqcache[list_size] = LiteralNode {
                    literal: i as u16,
                    freq: *f,
                };
                list_size += 1;
            } else {
                self.codes[i].len = 0;
            }
        }

        let list = &mut freqcache[..list_size];
        if list_size <= 2 {
            // Handle the small cases here, because they are awkward for the general case code. With
            // two or fewer literals, everything has bit length 1.
            for (i, node) in list.iter().enumerate() {
                // "list" is in order of increasing literal value.
                self.codes[node.literal as usize].set(i as u16, 1);
            }
            return;
        }

        list.sort_by_key(|node| node.freq);

        // Get the number of literals for each bit count
        let max_bits = self.bit_counts(list_size, max_bits);
        // And do the assignment
        self.assign_encoding_and_size(max_bits, list_size)
    }
}

// type byLiteral []literalNode

// fn (s *byLiteral) sort(a []literalNode) {
// 	*s = byLiteral(a)
// 	sort.Sort(s)
// }

// fn (s byLiteral) Len() int { return len(s) }

// fn (s byLiteral) Less(i, j int) bool {
// 	return s[i].literal < s[j].literal
// }

// fn (s byLiteral) Swap(i, j int) { s[i], s[j] = s[j], s[i] }

// type byFreq []literalNode

// fn (s *byFreq) sort(a []literalNode) {
// 	*s = byFreq(a)
// 	sort.Sort(s)
// }

// fn (s byFreq) Len() int { return len(s) }

// fn (s byFreq) Less(i, j int) bool {
// 	if s[i].freq == s[j].freq {
// 		return s[i].literal < s[j].literal
// 	}
// 	return s[i].freq < s[j].freq
// }

// fn (s byFreq) Swap(i, j int) { s[i], s[j] = s[j], s[i] }

pub(super) fn reverse_bits(number: u16, bit_length: u8) -> u16 {
    bits::reverse16(number << (16 - bit_length))
}
