// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::deflatefast::DeflateFast;
use super::huffman_bit_writer::HuffmanBitWriter;
use super::token::{literal_token, match_token, Token};
use crate::compat;

pub const NO_COMPRESSION: isize = 0;
pub const BEST_SPEED: isize = 1;
pub const BEST_COMPRESSION: isize = 9;
pub const DEFAULT_COMPRESSION: isize = -1;

// HUFFMAN_ONLY disables Lempel-Ziv match searching and only performs Huffman
// entropy encoding. This mode is useful in compressing data that has
// already been compressed with an LZ style algorithm (e.g. Snappy or LZ4)
// that lacks an entropy encoder. Compression gains are achieved when
// certain bytes in the input stream occur more frequently than others.
//
// Note that HUFFMAN_ONLY produces a compressed output that is
// RFC 1951 compliant. That is, any valid DEFLATE decompressor will
// continue to be able to decompress this output.
pub const HUFFMAN_ONLY: isize = -2;

const LOG_WINDOW_SIZE: usize = 15;
pub(super) const WINDOW_SIZE: usize = 1 << LOG_WINDOW_SIZE;
const WINDOW_MASK: usize = WINDOW_SIZE - 1;

// The LZ77 step produces a sequence of literal tokens and <length, offset>
// pair tokens. The offset is also known as distance. The underlying wire
// format limits the range of lengths and offsets. For example, there are
// 256 legitimate lengths: those in the range [3, 258]. This package's
// compressor uses a higher minimum match length, enabling optimizations
// such as finding matches via 32-bit loads and compares.
pub(super) const BASE_MATCH_LENGTH: usize = 3; // The smallest match length per the RFC section 3.2.5
pub(super) const MIN_MATCH_LENGTH: usize = 4; // The smallest match length that the compressor actually emits
pub(super) const MAX_MATCH_LENGTH: usize = 258; // The largest match length
pub(super) const BASE_MATCH_OFFSET: usize = 1; // The smallest match offset
pub(super) const MAX_MATCH_OFFSET: usize = 1 << 15; // The largest match offset

// The maximum number of tokens we put into a single flate block, just to
// stop things from getting too large.
const MAX_FLATE_BLOCK_TOKENS: usize = 1 << 14;
pub(super) const MAX_STORE_BLOCK_SIZE: usize = 65535;
const HASH_BITS: usize = 17; // After 17 performance degrades
const HASH_SIZE: usize = 1 << HASH_BITS;
const HASH_MASK: u32 = (1 << HASH_BITS) - 1;
const MAX_HASH_OFFSET: usize = 1 << 24;

const SKIP_NEVER: usize = i32::MAX as usize;

struct CompressionLevel {
    level: isize,
    good: usize,
    lazy: usize,
    nice: usize,
    chain: usize,
    fast_skip_hashing: usize,
}

impl CompressionLevel {
    const fn new(
        level: isize,
        good: usize,
        lazy: usize,
        nice: usize,
        chain: usize,
        fast_skip_hashing: usize,
    ) -> Self {
        Self {
            level,
            good,
            lazy,
            nice,
            chain,
            fast_skip_hashing,
        }
    }
}

const LEVELS: &[CompressionLevel] = &[
    CompressionLevel::new(0, 0, 0, 0, 0, 0), // NO_COMPRESSION.
    CompressionLevel::new(1, 0, 0, 0, 0, 0), // BEST_SPEED uses a custom algorithm; see deflatefast.go.
    // For levels 2-3 we don't bother trying with lazy matches.
    CompressionLevel::new(2, 4, 0, 16, 8, 5),
    CompressionLevel::new(3, 4, 0, 32, 32, 6),
    // Levels 4-9 use increasingly more lazy matching
    // and increasingly stringent conditions for "good enough".
    CompressionLevel::new(4, 4, 4, 16, 16, SKIP_NEVER),
    CompressionLevel::new(5, 8, 16, 32, 32, SKIP_NEVER),
    CompressionLevel::new(6, 8, 16, 128, 128, SKIP_NEVER),
    CompressionLevel::new(7, 8, 32, 128, 256, SKIP_NEVER),
    CompressionLevel::new(8, 32, 128, 258, 1024, SKIP_NEVER),
    CompressionLevel::new(9, 32, 258, 258, 4096, SKIP_NEVER),
];

const HASHMUL: u32 = 0x1e35a7bd;

/// hash4 returns a hash representation of the first 4 bytes
/// of the supplied slice.
/// The caller must ensure that b.len() >= 4.
pub(super) fn hash4(b: &[u8]) -> u32 {
    ((b[3] as u32) | (b[2] as u32) << 8 | (b[1] as u32) << 16 | (b[0] as u32) << 24)
        .overflowing_mul(HASHMUL)
        .0
        >> (32 - HASH_BITS)
}

/// bulkHash4 will compute hashes using the same
/// algorithm as hash4.
pub(super) fn bulk_hash4(b: &[u8], dst: &mut [u32]) {
    if b.len() < MIN_MATCH_LENGTH {
        return;
    }
    let mut hb = (b[3] as u32) | (b[2] as u32) << 8 | (b[1] as u32) << 16 | (b[0] as u32) << 24;
    dst[0] = hb.overflowing_mul(HASHMUL).0 >> (32 - HASH_BITS);
    let end = b.len() - MIN_MATCH_LENGTH + 1;
    for i in 1..end {
        hb = (hb << 8) | (b[i + 3] as u32);
        dst[i] = hb.overflowing_mul(HASHMUL).0 >> (32 - HASH_BITS);
    }
}

/// match_len returns the number of matching bytes in a and b
/// up to length 'max'. Both slices must be at least 'max'
/// bytes in size.
fn match_len(a: &[u8], b: &[u8], max: usize) -> usize {
    // 	a = a[..max]
    // 	b = b[..len(a)]
    for i in 0..max {
        if b[i] != a[i] {
            return i;
        }
    }
    max
}

pub(super) struct Compressor<'a, Output: std::io::Write> {
    compression_level: &'a CompressionLevel,
    hbw: HuffmanBitWriter<'a, Output>,
    #[allow(clippy::type_complexity)]
    bulk_hasher: Option<fn(&[u8], &mut [u32])>,

    // compression algorithm
    fill: FillFunc, // copy data to window
    step: StepFunc, // process window
    sync: bool,     // requesting flush

    best_speed: Option<DeflateFast>, // Encoder for BestSpeed

    // Input hash chains
    // hashHead[hashValue] contains the largest inputIndex with the specified hash value
    // If hashHead[hashValue] is within the current window, then
    // hashPrev[hashHead[hashValue] & windowMask] contains the previous index
    // with the same hash value.
    chain_head: isize,
    hash_head: Vec<u32>,
    hash_prev: Vec<u32>,
    hash_offset: usize,

    // input window: unprocessed data is window[index:windowEnd]
    index: usize,
    window: Vec<u8>,
    window_end: usize,
    block_start: usize,   // window index where current tokens start
    byte_available: bool, // if true, still need to process window[index-1].

    // queued output tokens
    tokens: Vec<Token>,

    // deflate state
    length: usize,
    offset: usize,
    max_insert_index: isize,
    err: std::io::Result<usize>,
    // hash_match must be able to contain hashes for the maximum match length.
    hash_match: Vec<u32>,

    writer_closed: bool,
}

enum FillFunc {
    Store,
    Deflate,
}

enum StepFunc {
    Deflate,
    Store,
    StoreHuff,
    EncSpeed,
}

#[allow(dead_code)]
impl<'a, Output: std::io::Write> Compressor<'a, Output> {
    fn fill_deflate(&mut self, b: &[u8]) -> usize {
        if self.index >= 2 * WINDOW_SIZE - (MIN_MATCH_LENGTH + MAX_MATCH_LENGTH) {
            // shift the window by WINDOW_SIZE
            compat::copy_within(&mut self.window, WINDOW_SIZE..2 * WINDOW_SIZE, 0);
            self.index -= WINDOW_SIZE;
            self.window_end -= WINDOW_SIZE;
            if self.block_start >= WINDOW_SIZE {
                self.block_start -= WINDOW_SIZE;
            } else {
                self.block_start = usize::MAX;
            }
            self.hash_offset += WINDOW_SIZE;
            if self.hash_offset > MAX_HASH_OFFSET {
                let delta = self.hash_offset - 1;
                self.hash_offset -= delta;
                self.chain_head -= delta as isize;

                let delta = delta as u32;

                for i in 0..self.hash_prev.len() {
                    if self.hash_prev[i] > delta {
                        self.hash_prev[i] -= delta;
                    } else {
                        self.hash_prev[i] = 0;
                    }
                }
                for i in 0..self.hash_head.len() {
                    if self.hash_head[i] > delta {
                        self.hash_head[i] -= delta;
                    } else {
                        self.hash_head[i] = 0;
                    }
                }
            }
        }
        let n = compat::copy(&mut self.window[self.window_end..], b);
        self.window_end += n;
        n
    }

    fn write_block(&mut self, index: usize) {
        if index > 0 {
            let mut window = None;
            if self.block_start <= index {
                window = Some(&self.window[self.block_start..index]);
            }
            self.block_start = index;
            self.hbw.write_block(&self.tokens, false, window);
        }
    }

    // fillWindow will fill the current window with the supplied
    // dictionary and calculate all hashes.
    // This is much faster than doing a full encode.
    // Should only be used after a reset.
    fn fill_window(&mut self, b: &[u8]) {
        // Do not fill window if we are in store-only mode.
        let cl = self.compression_level;
        if cl.level < 2 {
            return;
        }
        if self.index != 0 || self.window_end != 0 {
            panic!("internal error: fillWindow called with stale data");
        }

        let mut b = b;
        // If we are given too much, cut it.
        if b.len() > WINDOW_SIZE {
            b = &b[b.len() - WINDOW_SIZE..];
        }
        // Add all to window.
        let n = compat::copy(&mut self.window, b);

        // Calculate 256 hashes at the time (more L1 cache hits)
        let loops = (n + 256 - MIN_MATCH_LENGTH) / 256;
        for j in 0..loops {
            let index = j * 256;
            let mut end = index + 256 + MIN_MATCH_LENGTH - 1;
            if end > n {
                end = n;
            }
            let to_check = &self.window[index..end];
            let dst_size = to_check.len() - MIN_MATCH_LENGTH + 1;

            if dst_size == 0 {
                continue;
            }

            // let dst = &self.hash_match[..dstSize];
            (self.bulk_hasher.unwrap())(to_check, &mut self.hash_match[..dst_size]);
            for i in 0..dst_size {
                let di = i + index;
                let hh = &mut self.hash_head[(self.hash_match[i] & HASH_MASK) as usize];
                // Get previous value with the same hash.
                // Our chain should point to the previous value.
                self.hash_prev[di & WINDOW_MASK] = *hh;
                // Set the head of the hash chain to us.
                *hh = (di + self.hash_offset) as u32;
            }
        }
        // Update window information.
        self.window_end = n;
        self.index = n;
    }

    //// Try to find a match starting at index whose length is greater than prevSize.
    //// We only look at chainCount possibilities before giving up.
    fn find_match(
        &mut self,
        pos: usize,
        prev_head: usize,
        prev_length: usize,
        lookahead: usize,
    ) -> (usize, usize, bool) {
        // length, offset isize, ok bool
        let mut length: usize;
        let mut offset: usize = 0;
        let mut ok: bool = false;
        let mut min_match_look = MAX_MATCH_LENGTH;
        if lookahead < min_match_look {
            min_match_look = lookahead;
        }

        let win = &self.window[0..(pos + min_match_look)];

        let cl = self.compression_level;

        // We quit when we get a match that's at least nice long
        let mut nice = win.len() - pos;
        if cl.nice < nice {
            nice = cl.nice;
        }

        // If we've got a match that's good enough, only look in 1/4 the chain.
        let mut tries = cl.chain;
        length = prev_length;
        if length >= cl.good {
            tries >>= 2;
        }

        let mut w_end = win[pos + length];
        let w_pos = &win[pos..];
        let min_index = pos as isize - WINDOW_SIZE as isize;

        // 	for i := prevHead; tries > 0; tries-- {
        let mut i: usize = prev_head;
        while tries > 0 {
            if w_end == win[i + length] {
                let n = match_len(&win[i..], w_pos, min_match_look);

                if n > length && (n > MIN_MATCH_LENGTH || pos - i <= 4096) {
                    length = n;
                    offset = pos - i;
                    ok = true;
                    if n >= nice {
                        // The match is good enough that we don't try to find a better one.
                        break;
                    }
                    w_end = win[pos + n];
                }
            }
            if i as isize == min_index {
                // hashPrev[i & windowMask] has already been overwritten, so stop now.
                break;
            }
            let new_i = self.hash_prev[i & WINDOW_MASK] as isize - self.hash_offset as isize;
            if new_i < min_index || new_i < 0 {
                break;
            }
            i = new_i as usize;
            tries -= 1;
        }
        (length, offset, ok)
    }

    fn write_stored_block(&mut self) {
        self.hbw
            .write_stored_header(self.window[..self.window_end].len(), false);
        if self.hbw.error().is_err() {
            return;
        }
        self.hbw.write_bytes(&self.window[..self.window_end]);
    }

    /// enc_speed will compress and store the currently added data,
    /// if enough has been accumulated or we at the end of the stream.
    fn enc_speed(&mut self) {
        // We only compress if we have MAX_STORE_BLOCK_SIZE.
        if self.window_end < MAX_STORE_BLOCK_SIZE {
            if !self.sync {
                return;
            }

            // Handle small sizes.
            if self.window_end < 128 {
                if self.window_end == 0 {
                    return;
                } else if self.window_end <= 16 {
                    self.write_stored_block();
                } else {
                    self.hbw
                        .write_block_huff(false, &self.window[..self.window_end]);
                }
                self.window_end = 0;
                self.best_speed.as_mut().unwrap().reset();
                return;
            }
        }
        // Encode the block.
        self.tokens.truncate(0);
        self.best_speed
            .as_mut()
            .unwrap()
            .encode(&mut self.tokens, &self.window[..self.window_end]);

        // If we removed less than 1/16th, Huffman compress the block.
        if self.tokens.len() > self.window_end - (self.window_end >> 4) {
            self.hbw
                .write_block_huff(false, &self.window[..self.window_end]);
        } else {
            self.hbw.write_block_dynamic(
                &self.tokens,
                false,
                Some(&self.window[..self.window_end]),
            );
        }
        self.window_end = 0;
    }

    fn deflate(&mut self) {
        if self.window_end - self.index < MIN_MATCH_LENGTH + MAX_MATCH_LENGTH && !self.sync {
            return;
        }

        self.max_insert_index = self.window_end as isize - (MIN_MATCH_LENGTH as isize - 1);

        'Loop: loop {
            if self.index > self.window_end {
                panic!("index > windowEnd");
            }
            let lookahead = self.window_end - self.index;
            if lookahead < MIN_MATCH_LENGTH + MAX_MATCH_LENGTH {
                if !self.sync {
                    break 'Loop;
                }
                if self.index > self.window_end {
                    panic!("index > windowEnd");
                }
                if lookahead == 0 {
                    // Flush current output block if any.
                    if self.byte_available {
                        // There is still one pending token that needs to be flushed
                        self.tokens
                            .push(literal_token(self.window[self.index - 1] as u32));
                        self.byte_available = false;
                    }
                    if !self.tokens.is_empty() {
                        self.write_block(self.index);
                        if self.error().is_err() {
                            return;
                        }
                        self.tokens.truncate(0);
                    }
                    break 'Loop;
                }
            }
            if (self.index as isize) < self.max_insert_index {
                // Update the hash
                let hash = hash4(&self.window[self.index..self.index + MIN_MATCH_LENGTH]);
                let hh = &mut self.hash_head[(hash & HASH_MASK) as usize];
                self.chain_head = *hh as isize;
                self.hash_prev[self.index & WINDOW_MASK] = self.chain_head as u32;
                *hh = (self.index + self.hash_offset) as u32;
            }
            let prev_length = self.length;
            let prev_offset = self.offset;
            self.length = MIN_MATCH_LENGTH - 1;
            self.offset = 0;
            let mut min_index = self.index as isize - WINDOW_SIZE as isize;
            if min_index < 0 {
                min_index = 0;
            }

            let cl = self.compression_level;

            if self.chain_head - self.hash_offset as isize >= min_index
                && (cl.fast_skip_hashing != SKIP_NEVER && lookahead > MIN_MATCH_LENGTH - 1
                    || cl.fast_skip_hashing == SKIP_NEVER
                        && lookahead > prev_length
                        && prev_length < cl.lazy)
            {
                let (new_length, new_offset, ok) = self.find_match(
                    self.index,
                    self.chain_head as usize - self.hash_offset,
                    MIN_MATCH_LENGTH - 1,
                    lookahead,
                );
                if ok {
                    self.length = new_length;
                    self.offset = new_offset;
                }
            }
            if cl.fast_skip_hashing != SKIP_NEVER && self.length >= MIN_MATCH_LENGTH
                || cl.fast_skip_hashing == SKIP_NEVER
                    && prev_length >= MIN_MATCH_LENGTH
                    && self.length <= prev_length
            {
                // There was a match at the previous step, and the current match is
                // not better. Output the previous match.
                if cl.fast_skip_hashing != SKIP_NEVER {
                    self.tokens.push(match_token(
                        (self.length - BASE_MATCH_LENGTH) as u32,
                        (self.offset - BASE_MATCH_OFFSET) as u32,
                    ));
                } else {
                    self.tokens.push(match_token(
                        (prev_length - BASE_MATCH_LENGTH) as u32,
                        (prev_offset - BASE_MATCH_OFFSET) as u32,
                    ));
                }
                // Insert in the hash table all strings up to the end of the match.
                // index and index-1 are already inserted. If there is not enough
                // lookahead, the last two strings are not inserted into the hash
                // table.
                if self.length <= cl.fast_skip_hashing {
                    let new_index = if cl.fast_skip_hashing != SKIP_NEVER {
                        self.index + self.length
                    } else {
                        self.index + prev_length - 1
                    };
                    let mut index = self.index;
                    index += 1;
                    while index < new_index {
                        if (index as isize) < self.max_insert_index {
                            let hash = hash4(&self.window[index..index + MIN_MATCH_LENGTH]);
                            // Get previous value with the same hash.
                            // Our chain should point to the previous value.
                            let hh = &mut self.hash_head[(hash & HASH_MASK) as usize];
                            self.hash_prev[index & WINDOW_MASK] = *hh;
                            // Set the head of the hash chain to us.
                            *hh = (index + self.hash_offset) as u32;
                        }
                        index += 1;
                    }
                    self.index = index;

                    if cl.fast_skip_hashing == SKIP_NEVER {
                        self.byte_available = false;
                        self.length = MIN_MATCH_LENGTH - 1;
                    }
                } else {
                    // For matches this long, we don't bother inserting each individual
                    // item into the table.
                    self.index += self.length;
                }
                if self.tokens.len() == MAX_FLATE_BLOCK_TOKENS {
                    // The block includes the current character
                    self.write_block(self.index);
                    if self.error().is_err() {
                        return;
                    }
                    self.tokens.truncate(0);
                }
            } else {
                if cl.fast_skip_hashing != SKIP_NEVER || self.byte_available {
                    let i = if cl.fast_skip_hashing != SKIP_NEVER {
                        self.index
                    } else {
                        self.index - 1
                    };
                    self.tokens.push(literal_token(self.window[i] as u32));
                    if self.tokens.len() == MAX_FLATE_BLOCK_TOKENS {
                        self.write_block(i + 1);
                        if self.error().is_err() {
                            return;
                        }
                        self.tokens.truncate(0);
                    }
                }
                self.index += 1;
                if cl.fast_skip_hashing == SKIP_NEVER {
                    self.byte_available = true;
                }
            }
        }
    }

    fn fill_store(&mut self, b: &[u8]) -> usize {
        let n = compat::copy(&mut self.window[self.window_end..], b);
        self.window_end += n;
        n
    }

    fn store(&mut self) {
        if self.window_end > 0 && (self.window_end == MAX_STORE_BLOCK_SIZE || self.sync) {
            self.write_stored_block();
            self.window_end = 0;
        }
    }

    /// storeHuff compresses and stores the currently added data
    /// when the self.window is full or we are at the end of the stream.
    /// Any error that occurred will be in self.err
    fn store_huff(&mut self) {
        if self.window_end < self.window.len() && !self.sync || self.window_end == 0 {
            return;
        }
        self.hbw
            .write_block_huff(false, &self.window[..self.window_end]);
        self.window_end = 0;
    }

    fn error(&self) -> &std::io::Result<usize> {
        let writer_error = self.hbw.error();
        if writer_error.is_err() {
            return writer_error;
        }
        &self.err
    }

    // result of the operation will be stored in self.err, use self.error() to get it
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        let mut b = b;
        let n = b.len();
        while !b.is_empty() {
            self.do_step();
            let x = match self.fill {
                FillFunc::Store => self.fill_store(b),
                FillFunc::Deflate => self.fill_deflate(b),
            };
            b = &b[x..];
            if let Err(e) = self.error() {
                return Err(compat::copy_stdio_error(e));
            }
        }
        Ok(n)
    }

    fn do_step(&mut self) {
        match self.step {
            StepFunc::Deflate => self.deflate(),
            StepFunc::Store => self.store(),
            StepFunc::StoreHuff => self.store_huff(),
            StepFunc::EncSpeed => self.enc_speed(),
        }
    }

    fn sync_flush(&mut self) -> std::io::Result<()> {
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.sync = true;
        self.do_step();
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.write_stored_header(0, false);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.flush();
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.sync = false;
        Ok(())
    }

    fn new(w: &'a mut Output, level: isize) -> Self {
        let compression_level: &'a CompressionLevel;
        #[allow(clippy::type_complexity)]
        let mut bulk_hasher: Option<fn(&[u8], &mut [u32])> = None;
        let fill: FillFunc;
        let step: StepFunc;
        let mut best_speed: Option<DeflateFast> = None;
        let mut chain_head: isize = 0;
        let mut hash_offset: usize = 0;
        let window;
        let mut tokens: Vec<Token> = Vec::new();
        let mut length: usize = 0;

        if level == NO_COMPRESSION {
            compression_level = &LEVELS[NO_COMPRESSION as usize];
            window = vec![0; MAX_STORE_BLOCK_SIZE];
            fill = FillFunc::Store;
            step = StepFunc::Store;
        } else if level == HUFFMAN_ONLY {
            compression_level = &LEVELS[NO_COMPRESSION as usize];
            window = vec![0; MAX_STORE_BLOCK_SIZE];
            fill = FillFunc::Store;
            step = StepFunc::StoreHuff;
        } else if level == BEST_SPEED {
            compression_level = &LEVELS[level as usize];
            window = vec![0; MAX_STORE_BLOCK_SIZE];
            fill = FillFunc::Store;
            step = StepFunc::EncSpeed;
            best_speed = Some(DeflateFast::new());
            tokens = vec![Token::default(); MAX_STORE_BLOCK_SIZE];
        } else if level == DEFAULT_COMPRESSION || (2..=9).contains(&level) {
            let level = if level == DEFAULT_COMPRESSION {
                6
            } else {
                level
            };
            compression_level = &LEVELS[level as usize];

            // initDeflate();
            window = vec![0; 2 * WINDOW_SIZE];
            hash_offset = 1;
            tokens = Vec::with_capacity(MAX_FLATE_BLOCK_TOKENS + 1);
            length = MIN_MATCH_LENGTH - 1;
            chain_head = -1;
            bulk_hasher = Some(bulk_hash4);

            fill = FillFunc::Deflate;
            step = StepFunc::Deflate;
        } else {
            panic!(
                "flate: invalid compression level {}: want value in range [-2, 9]",
                level
            );
        }
        Self {
            compression_level,
            hbw: HuffmanBitWriter::new(w),
            bulk_hasher,
            fill,
            step,
            sync: false,
            best_speed,
            chain_head,
            hash_head: vec![0; HASH_SIZE],
            hash_prev: vec![0; WINDOW_SIZE],
            hash_offset,
            index: 0,
            window,
            window_end: 0,
            block_start: 0,
            byte_available: false,
            tokens,
            length,
            offset: 0,
            max_insert_index: 0,
            err: Ok(0),
            hash_match: vec![0; MAX_MATCH_LENGTH - 1],
            writer_closed: false,
        }
    }

    fn reset(&mut self, w: &'a mut Output) {
        self.hbw.reset(w);
        self.sync = false;
        self.err = Ok(0);
        if self.best_speed.is_some() {
            self.best_speed.as_mut().unwrap().reset();
        }
        self.chain_head = -1;
        self.hash_head.fill(0);
        self.hash_prev.fill(0);
        self.hash_offset = 1;
        self.index = 0;
        self.window_end = 0;
        self.block_start = 0;
        self.byte_available = false;
        self.tokens.truncate(0);
        self.length = MIN_MATCH_LENGTH - 1;
        self.offset = 0;
        self.max_insert_index = 0;
        self.err = Ok(0);
        self.hash_match = vec![0; MAX_MATCH_LENGTH - 1];
        self.writer_closed = false;
    }

    fn close(&mut self) -> std::io::Result<()> {
        if self.writer_closed {
            return Ok(());
        }
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.sync = true;
        self.do_step();
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.write_stored_header(0, true);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.flush();
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.writer_closed = true;
        // set error so that the next write operation will fail
        self.err = Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "flate: closed writer",
        ));
        Ok(())
    }

    /// Returns a mutable reference to the output writer.
    pub fn output(&mut self) -> &mut Output {
        self.hbw.output()
    }
}

// var errWriterClosed = errors.New("flate: closed writer")

// A Writer takes data written to it and writes the compressed
// form of that data to the output writer.
pub struct Writer<'a, Output: std::io::Write> {
    d: Compressor<'a, Output>,
    dict: Option<Vec<u8>>,
}

impl<'a, Output: std::io::Write> Writer<'a, Output> {
    /// new returns a new Writer compressing data at the given level.
    /// Following zlib, levels range from 1 (BEST_SPEED) to 9 (BEST_COMPRESSION);
    /// higher levels typically run slower but compress more. Level 0
    /// (NO_COMPRESSION) does not attempt any compression; it only adds the
    /// necessary DEFLATE framing.
    /// Level -1 (DEFAULT_COMPRESSION) uses the default compression level.
    /// Level -2 (HUFFMAN_ONLY) will use Huffman compression only, giving
    /// a very fast compression for all types of input, but sacrificing considerable
    /// compression efficiency.
    ///
    /// If level is in the range [-2, 9] then the error returned will be nil.
    /// Otherwise the error returned will be non-nil.
    pub fn new(w: &'a mut Output, level: isize) -> std::io::Result<Self> {
        if level == NO_COMPRESSION
            || level == HUFFMAN_ONLY
            || level == BEST_SPEED
            || level == DEFAULT_COMPRESSION
            || (2..=9).contains(&level)
        {
            // correct level
        } else {
            let msg = format!(
                "flate: invalid compression level {}: want value in range [-2, 9]",
                level
            );
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg));
        }

        return Ok(Self {
            d: Compressor::new(w, level),
            dict: None,
        });
    }

    /// new_dict is like new but initializes the new
    /// Writer with a preset dictionary. The returned Writer behaves
    /// as if the dictionary had been written to it without producing
    /// any compressed output. The compressed data written to w
    /// can only be decompressed by a Reader initialized with the
    /// same dictionary.
    pub fn new_dict(w: &'a mut Output, level: isize, dict: &[u8]) -> std::io::Result<Self> {
        let mut zw = Self::new(w, level)?;
        zw.d.fill_window(dict);
        zw.dict = Some(dict.to_vec()); // duplicate dictionary for Reset method.
        Ok(zw)
    }

    /// close flushes and closes the writer.
    pub fn close(&mut self) -> std::io::Result<()> {
        self.d.close()
    }

    /// reset discards the writer's state and makes it equivalent to
    /// tOutput: std::io::Writehe result of new_writer or Writer::new_dict called with dst
    /// and w's level and dictionary.
    pub fn reset(&mut self, dst: &'a mut Output) {
        if self.dict.is_some() {
            // w was created with Writer::new_dict
            self.d.reset(dst);
            self.d.fill_window(self.dict.as_ref().unwrap());
        } else {
            // w was created with new
            self.d.reset(dst)
        }
    }

    /// Returns a mutable reference to the output writer.
    pub fn output(&mut self) -> &mut Output {
        self.d.output()
    }
}

impl<Output: std::io::Write> std::io::Write for Writer<'_, Output> {
    /// write writes data to w, which will eventually write the
    /// compressed form of data to its underlying writer.
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.d.write(data)
    }

    /// flush flushes any pending data to the underlying writer.
    /// It is useful mainly in compressed network protocols, to ensure that
    /// a remote reader has enough data to reconstruct a packet.
    /// Flush does not return until the data has been written.
    /// Calling Flush when there is no pending data still causes the Writer
    /// to emit a sync marker of at least 4 bytes.
    /// If the underlying writer returns an error, Flush returns that error.
    ///
    /// In the terminology of the zlib library, Flush is equivalent to Z_SYNC_FLUSH.
    fn flush(&mut self) -> std::io::Result<()> {
        self.d.sync_flush()
    }
}
