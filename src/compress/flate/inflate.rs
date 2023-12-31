// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package flate implements the DEFLATE compressed data format, described in
//! RFC 1951.  The gzip and zlib packages implement access to DEFLATE-based file
//! formats.

use super::deflate::MAX_MATCH_OFFSET;
use super::dict_decoder::DictDecoder;
use super::huffman_bit_writer::END_BLOCK_MARKER;
use crate::compat;
use crate::errors;
use crate::io as ggio;
use crate::math::bits;
use std::sync::OnceLock;

// ggrust TODO: try to get rid of self.err when all tests are passing

const MAX_CODE_LEN: usize = 16; // max length of Huffman code

// The next three numbers come from the RFC section 3.2.7, with the
// additional proviso in section 3.2.5 which implies that distance codes
// 30 and 31 should never occur in compressed data.
pub(super) const MAX_NUM_LIT: usize = 286;
const MAX_NUM_DIST: usize = 30;
const NUM_CODES: usize = 19; // number of codes in Huffman meta-code

// var fixedOnce sync.Once
// var fixedHuffmanDecoder HuffmanDecoder
/// Initialize the fixedHuffmanDecoder only once upon first use.
pub(super) fn get_fixed_huffman_decoder() -> &'static HuffmanDecoder {
    static ENCODER: OnceLock<HuffmanDecoder> = OnceLock::new();
    ENCODER.get_or_init(fixed_huffman_decoder_init)
}

// // Resetter resets a ReadCloser returned by new_reader or new_reader_dict
// // to switch to a new underlying Reader. This permits reusing a ReadCloser
// // instead of allocating a new one.
// type Resetter interface {
// 	// reset discards any buffered data and resets the Resetter as if it was
// 	// newly initialized with the given reader.
// 	reset(r ggio::Reader, dict : &[u8]) error
// }

// The data structure for decoding Huffman tables is based on that of
// zlib. There is a lookup table of a fixed bit width (huffmanChunkBits),
// For codes smaller than the table width, there are multiple entries
// (each combination of trailing bits has the same value). For codes
// larger than the table width, the table contains a link to an overflow
// table. The width of each entry in the link table is the maximum code
// size minus the chunk width.
//
// Note that you can do a lookup in the table even without all bits
// filled. Since the extra bits are zero, and the DEFLATE Huffman codes
// have the property that shorter codes come before longer ones, the
// bit length estimate in the result is a lower bound on the actual
// number of bits.
//
// See the following:
//	https://github.com/madler/zlib/raw/master/doc/algorithm.txt

// chunk & 15 is number of bits
// chunk >> 4 is value, including table link

const HUFFMAN_CHUNK_BITS: u32 = 9;
const HUFFMAN_NUM_CHUNKS: u32 = 1 << HUFFMAN_CHUNK_BITS;
const HUFFMAN_COUNT_MASK: u32 = 15;
const HUFFMAN_VALUE_SHIFT: u32 = 4;

pub(super) struct HuffmanDecoder {
    min: u32,             // the minimum code length
    chunks: Vec<u32>,     // [huffmanNumChunks]u32 // chunks as described above
    links: Vec<Vec<u32>>, // [][]u32               // overflow links
    link_mask: u32,       // mask the width of the link table
}

impl HuffmanDecoder {
    pub(super) fn new() -> Self {
        Self {
            min: 0,
            chunks: vec![0; HUFFMAN_NUM_CHUNKS as usize],
            links: Vec::new(),
            link_mask: 0,
        }
    }

    /// Initialize Huffman decoding tables from array of code lengths.
    /// Following this function, h is guaranteed to be initialized into a complete
    /// tree (i.e., neither over-subscribed nor under-subscribed). The exception is a
    /// degenerate case where the tree has only a single symbol with length 1. Empty
    /// trees are permitted.
    pub(super) fn init(&mut self, lengths: &[u32]) -> bool {
        // Sanity enables additional runtime tests during Huffman
        // table construction. It's intended to be used during
        // development to supplement the currently ad-hoc unit tests.
        const SANITY: bool = false;
        // const SANITY: bool = true;

        if self.min != 0 {
            self.min = 0;
            self.chunks = vec![0; HUFFMAN_NUM_CHUNKS as usize];
            self.links = Vec::new();
            self.link_mask = 0;
        }

        // Count number of codes of each length,
        // compute min and max length.
        let mut count = [0_u32; MAX_CODE_LEN];
        let mut min: u32 = 0;
        let mut max: u32 = 0;
        for n in lengths {
            let n = *n;
            if n == 0 {
                continue;
            }
            if min == 0 || n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            count[n as usize] += 1;
        }

        // Empty tree. The decompressor.huff_sym function will fail later if the tree
        // is used. Technically, an empty tree is only valid for the HDIST tree and
        // not the HCLEN and HLIT tree. However, a stream with an empty HCLEN tree
        // is guaranteed to fail since it will attempt to use the tree to decode the
        // codes for the HLIT and HDIST trees. Similarly, an empty HLIT tree is
        // guaranteed to fail later since the compressed data section must be
        // composed of at least one symbol (the end-of-block marker).
        if max == 0 {
            return true;
        }

        let mut code = 0_u32;
        let mut nextcode = [0_u32; MAX_CODE_LEN];
        for i in min..=max {
            code <<= 1;
            nextcode[i as usize] = code;
            code += count[i as usize];
        }

        // Check that the coding is complete (i.e., that we've
        // assigned all 2-to-the-max possible bit sequences).
        // Exception: To be compatible with zlib, we also need to
        // accept degenerate single-code codings. See also
        // TestDegenerateHuffmanCoding.
        if code != 1 << (max as usize) && !(code == 1 && max == 1) {
            return false;
        }

        self.min = min;
        if max > HUFFMAN_CHUNK_BITS {
            let num_links = 1_u32 << (max - HUFFMAN_CHUNK_BITS);
            self.link_mask = num_links - 1;

            // create link tables
            let link = nextcode[HUFFMAN_CHUNK_BITS as usize + 1] >> 1;
            self.links = vec![Vec::new(); (HUFFMAN_NUM_CHUNKS - link) as usize];
            for j in link..HUFFMAN_NUM_CHUNKS {
                let mut reverse = bits::reverse16(j as u16);
                reverse >>= 16 - HUFFMAN_CHUNK_BITS;
                let off = j - link;
                if SANITY && self.chunks[reverse as usize] != 0 {
                    panic!("impossible: overwriting existing chunk");
                }
                self.chunks[reverse as usize] =
                    (off << HUFFMAN_VALUE_SHIFT) | (HUFFMAN_CHUNK_BITS + 1);
                self.links[off as usize] = vec![0; num_links as usize];
            }
        }

        for (i, n) in lengths.iter().enumerate() {
            let n = *n;
            if n == 0 {
                continue;
            }
            let code = nextcode[n as usize];
            nextcode[n as usize] += 1;
            let chunk = ((i as u32) << HUFFMAN_VALUE_SHIFT) | n;
            let mut reverse = bits::reverse16(code as u16) as u32;
            reverse >>= 16 - n;
            if n <= HUFFMAN_CHUNK_BITS {
                let mut off = reverse;
                while off < self.chunks.len() as u32 {
                    // 			for off := reverse; off < len(self.chunks); off += 1 << usize(n) {
                    // We should never need to overwrite
                    // an existing chunk. Also, 0 is
                    // never a valid chunk, because the
                    // lower 4 "count" bits should be
                    // between 1 and 15.
                    if SANITY && self.chunks[off as usize] != 0 {
                        panic!("impossible: overwriting existing chunk")
                    }
                    self.chunks[off as usize] = chunk;
                    off += 1 << n;
                }
            } else {
                let j = reverse & (HUFFMAN_NUM_CHUNKS - 1);
                if SANITY
                    && (self.chunks[j as usize] & HUFFMAN_COUNT_MASK) != HUFFMAN_CHUNK_BITS + 1
                {
                    // Longer codes should have been
                    // associated with a link table above.
                    panic!("impossible: not an indirect chunk");
                }
                let value = self.chunks[j as usize] >> HUFFMAN_VALUE_SHIFT;
                let linktab = &mut self.links[value as usize];
                reverse >>= HUFFMAN_CHUNK_BITS;
                let mut off = reverse;
                while off < linktab.len() as u32 {
                    if SANITY && linktab[off as usize] != 0 {
                        panic!("impossible: overwriting existing chunk");
                    }
                    linktab[off as usize] = chunk;
                    off += 1 << (n - HUFFMAN_CHUNK_BITS);
                }
            }
        }

        if SANITY {
            // Above we've sanity checked that we never overwrote
            // an existing entry. Here we additionally check that
            // we filled the tables completely.
            for (i, chunk) in self.chunks.iter().enumerate() {
                let chunk = *chunk;
                if chunk == 0 {
                    // As an exception, in the degenerate
                    // single-code case, we allow odd
                    // chunks to be missing.
                    if code == 1 && i % 2 == 1 {
                        continue;
                    }
                    panic!("impossible: missing chunk")
                }
            }
            for linktab in self.links.iter() {
                for chunk in linktab {
                    if *chunk == 0 {
                        panic!("impossible: missing chunk");
                    }
                }
            }
        }

        true
    }
}

#[derive(PartialEq)]
enum HDDecoder {
    None,
    H2,
}

#[derive(PartialEq)]
enum HLDecoder {
    Fixed,
    H1,
}

/// Reader uncompress data provided by another reader (input source).
/// Reader returns Ok(0) after the final block in the DEFLATE stream has
/// been encountered. Any trailing data after the final block is ignored.
///
/// The input source must be buffered because the implementation uses
/// many 1-byte reads.
pub struct Reader<Input: std::io::BufRead> {
    /// Input source.
    r: Input,
    roffset: u64,
    // Input bits, in top of b.
    b: u32,
    nb: usize,
    // Huffman decoders for literal/length, distance.
    pub(super) h1: HuffmanDecoder,
    h2: HuffmanDecoder,
    // Length arrays used to define Huffman codes.
    bits: Vec<u32>, // [isize; MAX_NUM_LIT + MAX_NUM_DIST],
    codebits: [u32; NUM_CODES],

    // Output history, buffer.
    pub(super) dict: DictDecoder,

    // Temporary buffer (avoids repeated allocation).
    buf: [u8; 4],
    // Next step in the decompression,
    // and decompression state.
    step: StepFunc,
    step_state: StepState,
    final_: bool,
    end_of_stream: bool,
    err: Option<std::io::Error>,
    hl: HLDecoder,
    hd: HDDecoder,
    copy_len: usize,
    copy_dist: usize,
}

enum StepFunc {
    NextBlock,
    HuffmanBlock,
    CopyData,
}

#[derive(PartialEq, Copy, Clone)]
enum StepState {
    StateInit,
    StateDict,
}

pub(super) enum DecoderToUse {
    H1,
    HL,
    H2,
}

// RFC 1951 section 3.2.7.
// Compression with dynamic Huffman codes

const CODE_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

impl<Input: std::io::BufRead> std::io::Read for Reader<Input> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        errors::iores_to_result(crate::io::Reader::read(self, buf))
    }
}

impl<'a, Input: std::io::BufRead> Reader<Input> {
    /// new returns a new Reader that can be used
    /// to read the uncompressed version of r.
    /// The reader returns std::io::Error::EOF after the final block in the DEFLATE stream has
    /// been encountered. Any trailing data after the final block is ignored.
    ///
    // ggrust: not implemented
    // The ReadCloser returned by new_reader also implements Resetter.
    pub fn new(r: Input) -> Reader<Input> {
        Self::new_dict(r, &[])
    }

    /// new_dict is like new but initializes the reader
    /// with a preset dictionary. The returned Reader behaves as if
    /// the uncompressed data stream started with the given dictionary,
    /// which has already been read. new_dict is typically used
    /// to read data compressed by Writer::new_dict.
    ///
    // ggrust: not implemented
    // The ReadCloser returned by new_reader also implements Resetter.
    pub fn new_dict(r: Input, dict: &'a [u8]) -> Reader<Input> {
        Reader {
            r,
            roffset: 0,
            b: 0,
            nb: 0,
            h1: HuffmanDecoder::new(),
            h2: HuffmanDecoder::new(),
            bits: vec![0; MAX_NUM_LIT + MAX_NUM_DIST],
            codebits: [0; NUM_CODES],
            dict: DictDecoder::new(MAX_MATCH_OFFSET, dict),
            buf: [0; 4],
            step: StepFunc::NextBlock,
            step_state: StepState::StateInit,
            final_: false,
            end_of_stream: false,
            err: None,
            hl: HLDecoder::Fixed,
            hd: HDDecoder::None,
            copy_len: 0,
            copy_dist: 0,
        }
    }

    fn next_block(&mut self) {
        while self.nb < 1 + 2 {
            let res = self.more_bits();
            if res.is_err() {
                self.err = Some(res.err().unwrap());
                return;
            }
        }
        self.final_ = self.b & 1 == 1;
        self.b >>= 1;
        let typ = self.b & 3;
        self.b >>= 2;
        self.nb -= 1 + 2;
        match typ {
            0 => self.data_block(),
            1 => {
                // compressed, fixed Huffman tables
                self.hl = HLDecoder::Fixed;
                self.hd = HDDecoder::None;
                self.huffman_block();
            }
            2 => {
                // compressed, dynamic Huffman tables
                let res = self.read_huffman();
                if res.is_ok() {
                    self.hl = HLDecoder::H1;
                    self.hd = HDDecoder::H2;
                    self.huffman_block();
                }
            }
            _ => {
                // 3 is reserved.
                self.err = Some(new_corrupted_input_error(self.roffset));
            }
        }
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        // if self.err.is_some() {
        //     let err = self.err.as_ref().unwrap();
        //     if err.is_eof() {
        //         return Ok(());
        //     }
        //     return Err(err.lossy_copy());
        // }
        Ok(())
    }

    fn read_huffman(&mut self) -> std::io::Result<()> {
        // HLIT[5], HDIST[5], HCLEN[4].
        while self.nb < 5 + 5 + 4 {
            self.more_bits()?
        }
        let nlit = (self.b & 0x1F) as usize + 257;
        if nlit > MAX_NUM_LIT {
            return Err(new_corrupted_input_error(self.roffset));
        }
        self.b >>= 5;
        let ndist = (self.b & 0x1F) as usize + 1;
        if ndist > MAX_NUM_DIST {
            return Err(new_corrupted_input_error(self.roffset));
        }
        self.b >>= 5;
        let nclen = (self.b & 0xF) as usize + 4;
        // NUM_CODES is 19, so nclen is always valid.
        self.b >>= 4;
        self.nb -= 5 + 5 + 4;

        // (HCLEN+4)*3 bits: code lengths in the magic CODE_ORDER order.
        #[allow(clippy::needless_range_loop)]
        for i in 0..nclen {
            while self.nb < 3 {
                self.more_bits()?
            }
            self.codebits[CODE_ORDER[i]] = self.b & 0x7;
            self.b >>= 3;
            self.nb -= 3;
        }
        #[allow(clippy::needless_range_loop)]
        for i in nclen..CODE_ORDER.len() {
            self.codebits[CODE_ORDER[i]] = 0;
        }
        if !self.h1.init(&self.codebits[0..]) {
            return Err(new_corrupted_input_error(self.roffset));
        }

        // HLIT + 257 code lengths, HDIST + 1 code lengths,
        // using the code length Huffman code.
        let mut i = 0;
        let n = nlit + ndist;
        while i < n {
            let x = self.huff_sym(DecoderToUse::H1)?;
            if x < 16 {
                // Actual length.
                self.bits[i] = x as u32;
                i += 1;
                continue;
            }
            // Repeat previous length or zero.
            let mut rep: usize;
            let nb: usize;
            let b: u32;
            match x {
                16 => {
                    rep = 3;
                    nb = 2;
                    if i == 0 {
                        return Err(new_corrupted_input_error(self.roffset));
                    }
                    b = self.bits[i - 1];
                }
                17 => {
                    rep = 3;
                    nb = 3;
                    b = 0;
                }
                18 => {
                    rep = 11;
                    nb = 7;
                    b = 0;
                }
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "unexpected length code",
                    ))
                }
            };
            while self.nb < nb {
                self.more_bits()?
            }
            rep += (self.b & ((1 << nb) - 1) as u32) as usize;
            self.b >>= nb;
            self.nb -= nb;
            if i + rep > n {
                return Err(new_corrupted_input_error(self.roffset));
            }
            for _j in 0..rep {
                self.bits[i] = b;
                i += 1;
            }
        }

        if !self.h1.init(&self.bits[0..nlit]) || !self.h2.init(&self.bits[nlit..nlit + ndist]) {
            return Err(new_corrupted_input_error(self.roffset));
        }

        // As an optimization, we can initialize the min bits to read at a time
        // for the HLIT tree to the length of the EOB marker since we know that
        // every block must terminate with one. This preserves the property that
        // we never read any extra bytes after the end of the DEFLATE stream.
        if self.h1.min < self.bits[END_BLOCK_MARKER] {
            self.h1.min = self.bits[END_BLOCK_MARKER];
        }
        Ok(())
    }

    // Decode a single Huffman block from self.
    // hl and hd are the Huffman states for the lit/length values
    // and the distance values, respectively. If hd == nil, using the
    // fixed distance encoding associated with fixed Huffman blocks.
    fn huffman_block(&mut self) {
        enum StateMachine {
            ReadLiteral,
            CopyHistory,
        }
        let mut next_step = match self.step_state {
            StepState::StateInit => StateMachine::ReadLiteral,
            StepState::StateDict => StateMachine::CopyHistory,
        };
        loop {
            match next_step {
                StateMachine::ReadLiteral => {
                    // Read literal and/or (length, distance) according to RFC section 3.2.3.
                    let v = match self.huff_sym(DecoderToUse::HL) {
                        Ok(v) => v,
                        Err(err) => {
                            self.err = Some(err);
                            return;
                        }
                    };
                    let n: usize; // number of bits extra
                    let length: usize;
                    match v {
                    0..=255 => {
                        // 		case v < 256:
                        self.dict.write_byte(v as u8);
                        if self.dict.avail_write() == 0 {
                            self.dict.stash_flush();
                            self.step = StepFunc::HuffmanBlock;
                            self.step_state = StepState::StateInit;
                            return;
                        }
                        // next_step = StateMachine::ReadLiteral;
                        continue;
                    }
                    256 => {
                        // 		case v == 256:
                        self.finish_block();
                        return;
                    }
                    // otherwise, reference to older data
                    257..=264 => {
                        // 		case v < 265:
                        length = v - (257 - 3);
                        n = 0;
                    }
                    265..=268 => {
                        // 		case v < 269:
                        length = v*2 - (265*2 - 11);
                        n = 1;
                    }
                    269..=272 => {
                        // 		case v < 273:
                        length = v*4 - (269*4 - 19);
                        n = 2;
                    }
                    273..=276 => {
                        // 		case v < 277:
                        length = v*8 - (273*8 - 35);
                        n = 3;
                    }
                    277..=280 => {
                        // 		case v < 281:
                        length = v*16 - (277*16 - 67);
                        n = 4;
                    }
                    281..=284 => {
                        // 		case v < 285:
                        length = v*32 - (281*32 - 131);
                        n = 5;
                    }
                    285..=285 /* MAX_NUM_LIT-1 */ => {
                        // 		case v < MAX_NUM_LIT:
                        length = 258;
                        n = 0;
                    }
                    _ => {
                        self.err = Some(new_corrupted_input_error(self.roffset));
                        return;
                    }
                };
                    let mut length = length;
                    if n > 0 {
                        while self.nb < n {
                            if let Err(err) = self.more_bits() {
                                self.err = Some(err);
                                return;
                            }
                        }
                        length += (self.b & ((1 << n) - 1) as u32) as usize;
                        self.b >>= n;
                        self.nb -= n;
                    }

                    let mut dist: usize;
                    match self.hd {
                        HDDecoder::None => {
                            while self.nb < 5 {
                                if let Err(err) = self.more_bits() {
                                    self.err = Some(err);
                                    return;
                                }
                            }
                            dist = bits::reverse8(((self.b & 0x1F) << 3) as u8) as usize;
                            self.b >>= 5;
                            self.nb -= 5;
                        }
                        HDDecoder::H2 => {
                            let res = self.huff_sym(DecoderToUse::H2);
                            dist = match res {
                                Ok(v) => v,
                                Err(err) => {
                                    self.err = Some(err);
                                    return;
                                }
                            };
                        }
                    }

                    if (0..4).contains(&dist) {
                        // 		case dist < 4:
                        dist += 1;
                    } else if (4..MAX_NUM_DIST).contains(&dist) {
                        // 		case dist < MAX_NUM_DIST:
                        let nb = (dist - 2) >> 1;
                        // have 1 bit in bottom of dist, need nb more.
                        let mut extra = (dist & 1) << nb;
                        while self.nb < nb {
                            if let Err(err) = self.more_bits() {
                                self.err = Some(err);
                                return;
                            }
                        }
                        extra |= (self.b & ((1 << nb) - 1) as u32) as usize;
                        self.b >>= nb;
                        self.nb -= nb;
                        dist = 1_usize.overflowing_shl((nb + 1) as u32).0 + 1 + extra;
                    } else {
                        // 		default:
                        self.err = Some(new_corrupted_input_error(self.roffset));
                        return;
                    }

                    // No check on length; encoding can be prescient.
                    if dist > self.dict.hist_size() {
                        self.err = Some(new_corrupted_input_error(self.roffset));
                        return;
                    }

                    self.copy_len = length;
                    self.copy_dist = dist;
                    next_step = StateMachine::CopyHistory;
                }
                StateMachine::CopyHistory => {
                    // Perform a backwards copy according to RFC section 3.2.3.
                    let mut cnt = self.dict.try_write_copy(self.copy_dist, self.copy_len);
                    if cnt == 0 {
                        cnt = self.dict.write_copy(self.copy_dist, self.copy_len);
                    }
                    self.copy_len -= cnt;

                    if self.dict.avail_write() == 0 || self.copy_len > 0 {
                        self.dict.stash_flush();
                        self.step = StepFunc::HuffmanBlock; // We need to continue this work
                        self.step_state = StepState::StateDict;
                        return;
                    }
                    next_step = StateMachine::ReadLiteral;
                }
            }
        }
    }

    /// Copy a single uncompressed data block from input to output.
    fn data_block(&mut self) {
        // Uncompressed.
        // Discard current half-u8.
        self.nb = 0;
        self.b = 0;

        // Length then ones-complement of length.
        let (n, err) = ggio::read_full(&mut self.r, &mut self.buf[0..4]);
        self.roffset += n as u64;
        if let Some(err) = err {
            self.err = Some(err);
            return;
        }
        let n = (self.buf[0]) as usize | ((self.buf[1] as usize) << 8);
        let nn = (self.buf[2]) as usize | ((self.buf[3] as usize) << 8);
        if (nn as u16) != (!n as u16) {
            self.err = Some(new_corrupted_input_error(self.roffset));
            return;
        }

        if n == 0 {
            self.dict.stash_flush();
            self.finish_block();
            return;
        }

        self.copy_len = n;
        self.copy_data();
    }

    /// copy_data copies self.copy_len bytes from the underlying reader into self.hist.
    /// It pauses for reads when self.hist is full.
    fn copy_data(&mut self) {
        let buf = self.dict.write_slice(self.copy_len);
        let (n, err) = ggio::read_full(&mut self.r, buf);
        self.roffset += n as u64;
        self.copy_len -= n;
        self.dict.write_mark(n);
        if let Some(err) = err {
            self.err = Some(err);
            return;
        }

        if self.dict.avail_write() == 0 || self.copy_len > 0 {
            self.dict.stash_flush();
            self.step = StepFunc::CopyData;
            return;
        }
        self.finish_block();
    }

    fn finish_block(&mut self) {
        if self.final_ {
            if self.dict.avail_read() > 0 {
                self.dict.stash_flush();
            }
            self.end_of_stream = true;
        }
        self.step = StepFunc::NextBlock;
    }

    fn more_bits(&mut self) -> std::io::Result<()> {
        let c = compat::readers::read_byte(&mut self.r)?;
        self.roffset += 1;
        self.b |= (c as u32) << self.nb;
        self.nb += 8;
        Ok(())
    }

    /// Read the next Huffman-encoded symbol from f according to h.
    pub(super) fn huff_sym(&mut self, decoder: DecoderToUse) -> Result<usize, std::io::Error> {
        let h = match decoder {
            DecoderToUse::H1 => &self.h1,
            DecoderToUse::HL => match self.hl {
                HLDecoder::Fixed => get_fixed_huffman_decoder(),
                HLDecoder::H1 => &self.h1,
            },
            DecoderToUse::H2 => &self.h2,
        };
        // Since a HuffmanDecoder can be empty or be composed of a degenerate tree
        // with single element, huff_sym must error on these two edge cases. In both
        // cases, the chunks slice will be 0 for the invalid sequence, leading it
        // satisfy the n == 0 check below.
        let mut n = h.min as usize;
        // Optimization. Compiler isn't smart enough to keep self.b,self.nb in registers,
        // but is smart enough to keep local variables in registers, so use nb and b,
        // inline call to more_bits and reassign b,nb back to f on return.
        let (mut nb, mut b) = (self.nb, self.b);
        loop {
            while nb < n {
                match compat::readers::read_byte(&mut self.r) {
                    Err(err) => {
                        self.b = b;
                        self.nb = nb;
                        return Err(err);
                    }
                    Ok(c) => {
                        self.roffset += 1;
                        b |= (c as u32) << (nb & 31);
                        nb = nb.wrapping_add(8);
                    }
                }
            }
            let mut chunk = h.chunks[(b & (HUFFMAN_NUM_CHUNKS - 1)) as usize];
            n = (chunk & HUFFMAN_COUNT_MASK) as usize;
            if n > HUFFMAN_CHUNK_BITS as usize {
                chunk = h.links[(chunk >> HUFFMAN_VALUE_SHIFT) as usize]
                    [((b >> HUFFMAN_CHUNK_BITS) & h.link_mask) as usize];
                n = (chunk & HUFFMAN_COUNT_MASK) as usize;
            }
            if n <= nb {
                if n == 0 {
                    self.b = b;
                    self.nb = nb;
                    self.err = Some(new_corrupted_input_error(self.roffset));
                    return Err(errors::copy_stdio_error(self.err.as_ref().unwrap()));
                }
                self.b = b >> (n & 31);
                self.nb = nb - n;
                return Ok((chunk >> HUFFMAN_VALUE_SHIFT) as usize);
            }
        }
    }

    // Reset discards any buffered data and resets the instance if it was
    // newly initialized with the given reader.
    pub fn reset(&mut self, r: Input, dict: &[u8]) {
        self.r = r;
        self.reset_state(dict);
    }

    // reset_state discards any buffered data and resets the instance if it was
    // newly initialized with the reader it already holds.
    pub fn reset_state(&mut self, dict: &[u8]) {
        // keep following fields
        // self.bits = vec![0; MAX_NUM_LIT + MAX_NUM_DIST];
        // self.codebits = [0; NUM_CODES];

        // reset everything else
        self.dict = DictDecoder::new(MAX_MATCH_OFFSET, dict);

        self.roffset = 0;
        self.b = 0;
        self.nb = 0;
        self.h1 = HuffmanDecoder::new();
        self.h2 = HuffmanDecoder::new();
        self.buf = [0; 4];
        self.step = StepFunc::NextBlock;
        self.step_state = StepState::StateInit;
        self.final_ = false;
        self.end_of_stream = false;
        self.err = None;
        self.hl = HLDecoder::Fixed;
        self.hd = HDDecoder::None;
        self.copy_len = 0;
        self.copy_dist = 0;
    }

    /// input_reader returns mutable reference to the underlying reader.
    pub fn input_reader(&mut self) -> &mut Input {
        &mut self.r
    }
}

impl<Input: std::io::BufRead> crate::io::Reader for Reader<Input> {
    fn read(&mut self, p: &mut [u8]) -> ggio::IoRes {
        // first returning available data, then an error if it exists
        loop {
            if self.dict.stash_len() > 0 {
                let n = self.dict.stash_read(p);
                if n > 0 {
                    return (n, None);
                }
            }
            if self.err.is_some() {
                return (0, errors::copy_stdio_option_error(&self.err));
            }
            if self.end_of_stream {
                return ggio::EOF;
            }
            match self.step {
                StepFunc::NextBlock => self.next_block(),
                StepFunc::HuffmanBlock => self.huffman_block(),
                StepFunc::CopyData => self.copy_data(),
            }
            if self.err.is_some() && self.dict.stash_len() == 0 {
                self.dict.stash_flush(); // Flush what's left in case of error
            }
        }
    }
}

fn fixed_huffman_decoder_init() -> HuffmanDecoder {
    let mut h = HuffmanDecoder::new();
    // These come from the RFC section 3.2.6.
    let mut bits = [0; 288];
    #[allow(clippy::needless_range_loop)]
    for i in 0..144 {
        bits[i] = 8;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 144..256 {
        bits[i] = 9;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 256..280 {
        bits[i] = 7;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 280..288 {
        bits[i] = 8;
    }
    h.init(&bits);
    h
}

fn new_corrupted_input_error(offset: u64) -> std::io::Error {
    let msg = format!("flate: corrupt input before offset {}", offset);
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg)
}
