// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::deflatefast::DeflateFast;
use super::huffman_bit_writer::HuffmanBitWriteFilter;
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
            fast_skip_hashing: fast_skip_hashing,
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

pub(super) struct Compressor<'a> {
    writer: &'a mut dyn std::io::Write,
    compressor: CompressFilter<'a>,
}

fn fill_deflate(d: &mut CompressFilter, b: &[u8]) -> usize {
    if d.index >= 2 * WINDOW_SIZE - (MIN_MATCH_LENGTH + MAX_MATCH_LENGTH) {
        // shift the window by WINDOW_SIZE
        compat::copy_within(&mut d.window, WINDOW_SIZE..2 * WINDOW_SIZE, 0);
        d.index -= WINDOW_SIZE;
        d.window_end -= WINDOW_SIZE;
        if d.block_start >= WINDOW_SIZE {
            d.block_start -= WINDOW_SIZE;
        } else {
            d.block_start = usize::MAX;
        }
        d.hash_offset += WINDOW_SIZE;
        if d.hash_offset > MAX_HASH_OFFSET {
            let delta = d.hash_offset - 1;
            d.hash_offset -= delta;
            d.chain_head -= delta as isize;

            let delta = delta as u32;

            for i in 0..d.hash_prev.len() {
                if d.hash_prev[i] > delta {
                    d.hash_prev[i] = d.hash_prev[i] - delta;
                } else {
                    d.hash_prev[i] = 0;
                }
            }
            for i in 0..d.hash_head.len() {
                if d.hash_head[i] > delta {
                    d.hash_head[i] = d.hash_head[i] - delta;
                } else {
                    d.hash_head[i] = 0;
                }
            }
        }
    }
    let n = compat::copy(&mut d.window[d.window_end..], b);
    d.window_end += n;
    return n;
}

const HASHMUL: u32 = 0x1e35a7bd;

/// hash4 returns a hash representation of the first 4 bytes
/// of the supplied slice.
/// The caller must ensure that b.len() >= 4.
pub(super) fn hash4(b: &[u8]) -> u32 {
    return ((b[3] as u32) | (b[2] as u32) << 8 | (b[1] as u32) << 16 | (b[0] as u32) << 24)
        .overflowing_mul(HASHMUL)
        .0
        >> (32 - HASH_BITS);
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
    return max;
}

/// enc_speed will compress and store the currently added data,
/// if enough has been accumulated or we at the end of the stream.
fn enc_speed(d: &mut CompressFilter, writer: &mut dyn std::io::Write) {
    // We only compress if we have MAX_STORE_BLOCK_SIZE.
    if d.window_end < MAX_STORE_BLOCK_SIZE {
        if !d.sync {
            return;
        }

        // Handle small sizes.
        if d.window_end < 128 {
            if d.window_end == 0 {
                return;
            } else if d.window_end <= 16 {
                d.write_stored_block(writer);
            } else {
                d.hbw
                    .write_block_huff(writer, false, &d.window[..d.window_end]);
            }
            d.window_end = 0;
            d.best_speed.as_mut().unwrap().reset();
            return;
        }
    }
    // Encode the block.
    d.tokens.truncate(0);
    d.best_speed
        .as_mut()
        .unwrap()
        .encode(&mut d.tokens, &d.window[..d.window_end]);

    // If we removed less than 1/16th, Huffman compress the block.
    if d.tokens.len() > d.window_end - (d.window_end >> 4) {
        d.hbw
            .write_block_huff(writer, false, &d.window[..d.window_end]);
    } else {
        d.hbw
            .write_block_dynamic(writer, &d.tokens, false, Some(&d.window[..d.window_end]));
    }
    d.window_end = 0;
}

fn deflate(d: &mut CompressFilter, writer: &mut dyn std::io::Write) {
    if d.window_end - d.index < MIN_MATCH_LENGTH + MAX_MATCH_LENGTH && !d.sync {
        return;
    }

    d.max_insert_index = d.window_end as isize - (MIN_MATCH_LENGTH as isize - 1);

    'Loop: loop {
        if d.index > d.window_end {
            panic!("index > windowEnd");
        }
        let lookahead = d.window_end - d.index;
        if lookahead < MIN_MATCH_LENGTH + MAX_MATCH_LENGTH {
            if !d.sync {
                break 'Loop;
            }
            if d.index > d.window_end {
                panic!("index > windowEnd");
            }
            if lookahead == 0 {
                // Flush current output block if any.
                if d.byte_available {
                    // There is still one pending token that needs to be flushed
                    d.tokens.push(literal_token(d.window[d.index - 1] as u32));
                    d.byte_available = false;
                }
                if d.tokens.len() > 0 {
                    d.write_block(writer, d.index);
                    if d.error().is_err() {
                        return;
                    }
                    d.tokens.truncate(0);
                }
                break 'Loop;
            }
        }
        if (d.index as isize) < d.max_insert_index {
            // Update the hash
            let hash = hash4(&d.window[d.index..d.index + MIN_MATCH_LENGTH]);
            let hh = &mut d.hash_head[(hash & HASH_MASK) as usize];
            d.chain_head = *hh as isize;
            d.hash_prev[d.index & WINDOW_MASK] = d.chain_head as u32;
            *hh = (d.index + d.hash_offset) as u32;
        }
        let prev_length = d.length;
        let prev_offset = d.offset;
        d.length = MIN_MATCH_LENGTH - 1;
        d.offset = 0;
        let mut min_index = d.index as isize - WINDOW_SIZE as isize;
        if min_index < 0 {
            min_index = 0;
        }

        let cl = d.compression_level;

        if d.chain_head - d.hash_offset as isize >= min_index
            && (cl.fast_skip_hashing != SKIP_NEVER && lookahead > MIN_MATCH_LENGTH - 1
                || cl.fast_skip_hashing == SKIP_NEVER
                    && lookahead > prev_length
                    && prev_length < cl.lazy)
        {
            let (new_length, new_offset, ok) = d.find_match(
                d.index,
                d.chain_head as usize - d.hash_offset,
                MIN_MATCH_LENGTH - 1,
                lookahead,
            );
            if ok {
                d.length = new_length;
                d.offset = new_offset;
            }
        }
        if cl.fast_skip_hashing != SKIP_NEVER && d.length >= MIN_MATCH_LENGTH
            || cl.fast_skip_hashing == SKIP_NEVER
                && prev_length >= MIN_MATCH_LENGTH
                && d.length <= prev_length
        {
            // There was a match at the previous step, and the current match is
            // not better. Output the previous match.
            if cl.fast_skip_hashing != SKIP_NEVER {
                d.tokens.push(match_token(
                    (d.length - BASE_MATCH_LENGTH) as u32,
                    (d.offset - BASE_MATCH_OFFSET) as u32,
                ));
            } else {
                d.tokens.push(match_token(
                    (prev_length - BASE_MATCH_LENGTH) as u32,
                    (prev_offset - BASE_MATCH_OFFSET) as u32,
                ));
            }
            // Insert in the hash table all strings up to the end of the match.
            // index and index-1 are already inserted. If there is not enough
            // lookahead, the last two strings are not inserted into the hash
            // table.
            if d.length <= cl.fast_skip_hashing {
                let new_index = if cl.fast_skip_hashing != SKIP_NEVER {
                    d.index + d.length
                } else {
                    d.index + prev_length - 1
                };
                let mut index = d.index;
                index += 1;
                while index < new_index {
                    if (index as isize) < d.max_insert_index {
                        let hash = hash4(&d.window[index..index + MIN_MATCH_LENGTH]);
                        // Get previous value with the same hash.
                        // Our chain should point to the previous value.
                        let hh = &mut d.hash_head[(hash & HASH_MASK) as usize];
                        d.hash_prev[index & WINDOW_MASK] = *hh;
                        // Set the head of the hash chain to us.
                        *hh = (index + d.hash_offset) as u32;
                    }
                    index += 1;
                }
                d.index = index;

                if cl.fast_skip_hashing == SKIP_NEVER {
                    d.byte_available = false;
                    d.length = MIN_MATCH_LENGTH - 1;
                }
            } else {
                // For matches this long, we don't bother inserting each individual
                // item into the table.
                d.index += d.length;
            }
            if d.tokens.len() == MAX_FLATE_BLOCK_TOKENS {
                // The block includes the current character
                d.write_block(writer, d.index);
                if d.error().is_err() {
                    return;
                }
                d.tokens.truncate(0);
            }
        } else {
            if cl.fast_skip_hashing != SKIP_NEVER || d.byte_available {
                let i = if cl.fast_skip_hashing != SKIP_NEVER {
                    d.index
                } else {
                    d.index - 1
                };
                d.tokens.push(literal_token(d.window[i] as u32));
                if d.tokens.len() == MAX_FLATE_BLOCK_TOKENS {
                    d.write_block(writer, i + 1);
                    if d.error().is_err() {
                        return;
                    }
                    d.tokens.truncate(0);
                }
            }
            d.index += 1;
            if cl.fast_skip_hashing == SKIP_NEVER {
                d.byte_available = true;
            }
        }
    }
}

fn fill_store(d: &mut CompressFilter, b: &[u8]) -> usize {
    let n = compat::copy(&mut d.window[d.window_end..], b);
    d.window_end += n;
    return n;
}

fn store(d: &mut CompressFilter, writer: &mut dyn std::io::Write) {
    if d.window_end > 0 && (d.window_end == MAX_STORE_BLOCK_SIZE || d.sync) {
        d.write_stored_block(writer);
        d.window_end = 0;
    }
}

/// storeHuff compresses and stores the currently added data
/// when the self.window is full or we are at the end of the stream.
/// Any error that occurred will be in self.err
fn store_huff(d: &mut CompressFilter, writer: &mut dyn std::io::Write) {
    if d.window_end < d.window.len() && !d.sync || d.window_end == 0 {
        return;
    }
    d.hbw
        .write_block_huff(writer, false, &d.window[..d.window_end]);
    d.window_end = 0;
}

#[allow(dead_code)]
impl<'a> Compressor<'a> {
    fn new(w: &'a mut dyn std::io::Write, level: isize) -> Self {
        Self {
            writer: w,
            compressor: CompressFilter::new(level),
        }
    }

    fn reset(&mut self, w: &'a mut dyn std::io::Write) {
        self.writer = w;
        self.compressor.reset();
    }

    fn close(&mut self) -> std::io::Result<()> {
        self.compressor.close(self.writer)
    }
}

/// CompressFilter is like Compressor, but it doesn't hold the underlying writer,
/// which is a writer where compressed data will be written.  Whenever necessary
/// the underlying writer is passed as a method parameter.  It is responsibility
/// of the caller, to always pass the correct writer.
pub(super) struct CompressFilter<'a> {
    compression_level: &'a CompressionLevel,
    hbw: HuffmanBitWriteFilter,
    bulk_hasher: Option<fn(&[u8], &mut [u32])>,

    // compression algorithm
    fill: fn(&mut CompressFilter, &[u8]) -> usize, // copy data to window
    step: fn(&mut CompressFilter, &mut dyn std::io::Write), // process window
    sync: bool,                                    // requesting flush

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

impl<'a> CompressFilter<'a> {
    fn error(&self) -> &std::io::Result<usize> {
        let writer_error = self.hbw.error();
        if writer_error.is_err() {
            return writer_error;
        }
        return &self.err;
    }

    fn write_block(&mut self, w: &mut dyn std::io::Write, index: usize) {
        if index > 0 {
            let mut window = None;
            if self.block_start <= index {
                window = Some(&self.window[self.block_start..index]);
            }
            self.block_start = index;
            self.hbw.write_block(w, &self.tokens, false, window);
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

            if dst_size <= 0 {
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
        return (length, offset, ok);
    }

    fn write_stored_block(&mut self, w: &mut dyn std::io::Write) {
        self.hbw
            .write_stored_header(w, self.window[..self.window_end].len(), false);
        if self.hbw.error().is_err() {
            return;
        }
        self.hbw.write_bytes(w, &self.window[..self.window_end]);
    }

    // result of the operation will be stored in self.err, use self.error() to get it
    fn write(&mut self, writer: &mut dyn std::io::Write, b: &[u8]) -> std::io::Result<usize> {
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        let mut b = b;
        let n = b.len();
        while b.len() > 0 {
            (self.step)(self, writer);
            b = &b[(self.fill)(self, b)..];
            if let Err(e) = self.error() {
                return Err(compat::copy_stdio_error(e));
            }
        }
        return Ok(n);
    }

    fn sync_flush(&mut self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.sync = true;
        (self.step)(self, w);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.write_stored_header(w, 0, false);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.flush(w);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.sync = false;
        Ok(())
    }

    fn new(level: isize) -> Self {
        let compression_level: &'a CompressionLevel;
        let mut bulk_hasher: Option<fn(&[u8], &mut [u32])> = None;
        let fill: fn(&mut CompressFilter, &[u8]) -> usize;
        let step: fn(&mut CompressFilter, &mut dyn std::io::Write);
        let mut best_speed: Option<DeflateFast> = None;
        let mut chain_head: isize = 0;
        let mut hash_offset: usize = 0;
        let window;
        let mut tokens: Vec<Token> = Vec::new();
        let mut length: usize = 0;

        if level == NO_COMPRESSION {
            compression_level = &LEVELS[NO_COMPRESSION as usize];
            window = vec![0; MAX_STORE_BLOCK_SIZE];
            fill = fill_store;
            step = store;
        } else if level == HUFFMAN_ONLY {
            compression_level = &LEVELS[NO_COMPRESSION as usize];
            window = vec![0; MAX_STORE_BLOCK_SIZE];
            fill = fill_store;
            step = store_huff;
        } else if level == BEST_SPEED {
            compression_level = &LEVELS[level as usize];
            window = vec![0; MAX_STORE_BLOCK_SIZE];
            fill = fill_store;
            step = enc_speed;
            best_speed = Some(DeflateFast::new());
            tokens = vec![Token::default(); MAX_STORE_BLOCK_SIZE];
        } else if level == DEFAULT_COMPRESSION || (2 <= level && level <= 9) {
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

            fill = fill_deflate;
            step = deflate;
        } else {
            panic!(
                "flate: invalid compression level {}: want value in range [-2, 9]",
                level
            );
        }
        return Self {
            compression_level,
            hbw: HuffmanBitWriteFilter::new(),
            bulk_hasher,
            fill: fill,
            step: step,
            sync: false,
            best_speed,
            chain_head,
            hash_head: vec![0; HASH_SIZE],
            hash_prev: vec![0; WINDOW_SIZE],
            hash_offset,
            index: 0,
            window: window,
            window_end: 0,
            block_start: 0,
            byte_available: false,
            tokens: tokens,
            length: length,
            offset: 0,
            max_insert_index: 0,
            err: Ok(0),
            hash_match: vec![0; MAX_MATCH_LENGTH - 1],
            writer_closed: false,
        };
    }

    fn reset(&mut self) {
        self.hbw.reset();
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

    fn close(&mut self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        if self.writer_closed {
            return Ok(());
        }
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.sync = true;
        (self.step)(self, w);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.write_stored_header(w, 0, true);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.hbw.flush(w);
        if let Err(e) = self.error() {
            return Err(compat::copy_stdio_error(e));
        }
        self.writer_closed = true;
        // set error so that the next write operation will fail
        self.err = Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "flate: closed writer",
        ));
        return Ok(());
    }
}

// var errWriterClosed = errors.New("flate: closed writer")

// A Writer takes data written to it and writes the compressed
// form of that data to an underlying writer.
pub struct Writer<'a> {
    writer: &'a mut dyn std::io::Write,
    tw: WriteFilter<'a>,
}

impl<'a> Writer<'a> {
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
    pub fn new(w: &'a mut dyn std::io::Write, level: isize) -> std::io::Result<Self> {
        return Ok(Self {
            writer: w,
            tw: WriteFilter::new(level)?,
        });
    }

    /// new_dict is like new but initializes the new
    /// Writer with a preset dictionary. The returned Writer behaves
    /// as if the dictionary had been written to it without producing
    /// any compressed output. The compressed data written to w
    /// can only be decompressed by a Reader initialized with the
    /// same dictionary.
    pub fn new_dict(
        w: &'a mut dyn std::io::Write,
        level: isize,
        dict: &[u8],
    ) -> std::io::Result<Self> {
        return Ok(Self {
            writer: w,
            tw: WriteFilter::new_dict(level, dict)?,
        });
    }

    /// close flushes and closes the writer.
    pub fn close(&mut self) -> std::io::Result<()> {
        return self.tw.close(self.writer);
    }

    /// reset discards the writer's state and makes it equivalent to
    /// the result of new_writer or Writer::new_dict called with dst
    /// and w's level and dictionary.
    pub fn reset(&mut self, dst: &'a mut dyn std::io::Write) {
        self.writer = dst;
        self.tw.reset();
    }
}

impl std::io::Write for Writer<'_> {
    /// write writes data to w, which will eventually write the
    /// compressed form of data to its underlying writer.
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.tw.write(self.writer, data)
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
        self.tw.flush(self.writer)
    }
}

/// WriteFilter is like a Writer, but doesn't keep reference
/// to the underlying writer.
pub struct WriteFilter<'a> {
    d: CompressFilter<'a>,
    dict: Option<Vec<u8>>,
}

impl<'a> WriteFilter<'a> {
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
    pub fn new(level: isize) -> std::io::Result<Self> {
        if level == NO_COMPRESSION
            || level == HUFFMAN_ONLY
            || level == BEST_SPEED
            || level == DEFAULT_COMPRESSION
            || (2 <= level && level <= 9)
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
            d: CompressFilter::new(level),
            dict: None,
        });
    }

    /// new_dict is like new but initializes the new
    /// Writer with a preset dictionary. The returned Writer behaves
    /// as if the dictionary had been written to it without producing
    /// any compressed output. The compressed data written to w
    /// can only be decompressed by a Reader initialized with the
    /// same dictionary.
    pub fn new_dict(level: isize, dict: &[u8]) -> std::io::Result<Self> {
        let mut zw = Self::new(level)?;
        zw.d.fill_window(dict);
        zw.dict = Some(dict.to_vec()); // duplicate dictionary for Reset method.
        Ok(zw)
    }

    /// close flushes and closes the writer.
    pub fn close(&mut self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        return self.d.close(writer);
    }

    /// reset discards the writer's state and makes it equivalent to
    /// the result of new_writer or Writer::new_dict called with dst
    /// and w's level and dictionary.
    pub fn reset(&mut self) {
        if self.dict.is_some() {
            // w was created with Writer::new_dict
            self.d.reset();
            self.d.fill_window(&self.dict.as_ref().unwrap());
        } else {
            // w was created with new
            self.d.reset()
        }
    }

    /// write writes data to w, which will eventually write the
    /// compressed form of data to its underlying writer.
    pub fn write(
        &mut self,
        writer: &mut dyn std::io::Write,
        data: &[u8],
    ) -> std::io::Result<usize> {
        self.d.write(writer, data)
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
    pub fn flush(&mut self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.d.sync_flush(writer)
    }
}
