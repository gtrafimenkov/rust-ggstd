// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::deflate;
use super::huffman_code::{
    get_fixed_literal_encoding, get_fixed_offset_encoding, new_huffman_encoder,
};
use super::huffman_code::{HCode, HuffmanEncoder};
use super::inflate;
use super::token::{length_code, offset_code, Token, TokenTrait, MATCH_TYPE};
use std::io;
use std::sync::OnceLock;

// The largest offset code.
const OFFSET_CODE_COUNT: usize = 30;

// The special code used to mark the end of a block.
pub(super) const END_BLOCK_MARKER: usize = 256;

// The first length code.
const LENGTH_CODES_START: usize = 257;

// The number of codegen codes.
const CODEGEN_CODE_COUNT: usize = 19;
const BAD_CODE: u8 = 255;

// bufferFlushSize indicates the buffer size
// after which bytes are flushed to the writer.
// Should preferably be a multiple of 6, since
// we accumulate 6 bytes between writes to the buffer.
const BUFFER_FLUSH_SIZE: usize = 240;

// bufferSize is the actual output u8 buffer size.
// It must have additional headroom for a flush
// which can contain up to 8 bytes.
const BUFFER_SIZE: usize = BUFFER_FLUSH_SIZE + 8;

// The number of extra bits needed by length code X - LENGTH_CODES_START.
const LENGTH_EXTRA_BITS: &[u8] = &[
    /* 257 */ 0, 0, 0, //
    /* 260 */ 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, //
    /* 270 */ 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, //
    /* 280 */ 4, 5, 5, 5, 5, 0, //
];

/// The length indicated by length code X - LENGTH_CODES_START.
const LENGTH_BASE: &[u8] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 10, //
    12, 14, 16, 20, 24, 28, 32, 40, 48, 56, //
    64, 80, 96, 112, 128, 160, 192, 224, 255, //
];

/// offset code word extra bits.
const OFFSET_EXTRA_BITS: &[u8] = &[
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, //
    4, 4, 5, 5, 6, 6, 7, 7, 8, 8, //
    9, 9, 10, 10, 11, 11, 12, 12, 13, 13, //
];

const OFFSET_BASE: &[u32] = &[
    0x000000, 0x000001, 0x000002, 0x000003, 0x000004, //
    0x000006, 0x000008, 0x00000c, 0x000010, 0x000018, //
    0x000020, 0x000030, 0x000040, 0x000060, 0x000080, //
    0x0000c0, 0x000100, 0x000180, 0x000200, 0x000300, //
    0x000400, 0x000600, 0x000800, 0x000c00, 0x001000, //
    0x001800, 0x002000, 0x003000, 0x004000, 0x006000, //
];

/// The odd order in which the codegen code sizes are written.
const CODEGEN_ORDER: &[u32] = &[
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

pub(super) struct HuffmanBitWriter<'a> {
    pub(super) writer: &'a mut dyn std::io::Write,
    hbw: HuffmanBitWriteFilter,
}

#[allow(dead_code)]
impl<'a> HuffmanBitWriter<'a> {
    pub(super) fn new(w: &'a mut dyn std::io::Write) -> Self {
        HuffmanBitWriter {
            writer: w,
            hbw: HuffmanBitWriteFilter::new(),
        }
    }

    pub fn reset(&mut self, writer: &'a mut dyn std::io::Write) {
        self.writer = writer;
        self.hbw.reset();
    }

    pub fn flush(&mut self) {
        self.hbw.flush(self.writer);
    }

    pub(super) fn write_bytes(&mut self, bytes: &[u8]) {
        self.hbw.write_bytes(self.writer, bytes);
    }

    pub(super) fn write_stored_header(&mut self, length: usize, is_eof: bool) {
        self.hbw.write_stored_header(self.writer, length, is_eof);
    }

    /// write_block will write a block of tokens with the smallest encoding.
    /// The original input can be supplied, and if the huffman encoded data
    /// is larger than the original bytes, the data will be written as a
    /// stored block.
    /// If the input is nil, the tokens will always be Huffman encoded.
    pub(super) fn write_block(&mut self, tokens: &[Token], eof: bool, input: Option<&[u8]>) {
        self.hbw.write_block(self.writer, tokens, eof, input);
    }

    /// write_block_dynamic encodes a block using a dynamic Huffman table.
    /// This should be used if the symbols used have a disproportionate
    /// histogram distribution.
    /// If input is supplied and the compression savings are below 1/16th of the
    /// input size the block is stored.
    pub(super) fn write_block_dynamic(
        &mut self,
        tokens: &[Token],
        eof: bool,
        input: Option<&[u8]>,
    ) {
        self.hbw
            .write_block_dynamic(self.writer, tokens, eof, input);
    }

    /// write_block_huff encodes a block of bytes as either
    /// Huffman encoded literals or uncompressed bytes if the
    /// results only gains very little from compression.
    pub fn write_block_huff(&mut self, eof: bool, input: &[u8]) {
        self.hbw.write_block_huff(self.writer, eof, input);
    }

    pub(super) fn error(&self) -> &std::io::Result<usize> {
        self.hbw.error()
    }
}

// huffOffset is a static offset encoder used for huffman only encoding.
// It can be reused since we will not be encoding offset values.
// var huffOffset *HuffmanEncoder
fn get_huff_offset() -> &'static HuffmanEncoder {
    static ENCODER: OnceLock<HuffmanEncoder> = OnceLock::new();
    ENCODER.get_or_init(generate_huff_offset)
}

fn generate_huff_offset() -> HuffmanEncoder {
    let mut offset_freq: [i32; OFFSET_CODE_COUNT] = [0; OFFSET_CODE_COUNT];
    offset_freq[0] = 1;
    let mut huff_offset = new_huffman_encoder(OFFSET_CODE_COUNT);
    huff_offset.generate(&offset_freq, 15);
    huff_offset
}

/// histogram accumulates a histogram of b in h.
///
/// len(h) must be >= 256, and h's elements must be all zeroes.
fn histogram(b: &[u8], h: &mut [i32]) {
    for t in b {
        h[*t as usize] += 1;
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Same as HuffmanBitWriter, but doesn't keep Writer in the internal state.
pub(super) struct HuffmanBitWriteFilter {
    // Data waiting to be written is bytes[0:nbytes]
    // and then the low nbits of bits.  Data is always written
    // sequentially into the bytes array.
    bits: u64,
    nbits: usize,
    bytes: Vec<u8>,
    codegen_freq: Vec<i32>,
    nbytes: usize,
    literal_freq: Vec<i32>,
    offset_freq: Vec<i32>,
    codegen: Vec<u8>,
    literal_encoding: Box<HuffmanEncoder>,
    offset_encoding: Box<HuffmanEncoder>,
    codegen_encoding: Box<HuffmanEncoder>,
    err: std::io::Result<usize>,
}

impl HuffmanBitWriteFilter {
    pub(super) fn new() -> Self {
        HuffmanBitWriteFilter {
            // writer: w,
            bits: 0,
            nbits: 0,
            nbytes: 0,
            codegen_freq: vec![0; CODEGEN_CODE_COUNT],
            bytes: vec![0; BUFFER_SIZE],
            err: Ok(0),
            literal_freq: vec![0; inflate::MAX_NUM_LIT],
            offset_freq: vec![0; OFFSET_CODE_COUNT],
            codegen: vec![0; inflate::MAX_NUM_LIT + OFFSET_CODE_COUNT + 1],
            literal_encoding: Box::new(new_huffman_encoder(inflate::MAX_NUM_LIT)),
            codegen_encoding: Box::new(new_huffman_encoder(CODEGEN_CODE_COUNT)),
            offset_encoding: Box::new(new_huffman_encoder(OFFSET_CODE_COUNT)),
        }
    }

    pub fn reset(&mut self) {
        self.bits = 0;
        self.nbits = 0;
        self.nbytes = 0;
        self.err = Ok(0);
    }

    pub fn flush(&mut self, writer: &mut dyn std::io::Write) {
        if self.err.is_err() {
            self.nbits = 0;
            return;
        }
        let mut n = self.nbytes;
        while self.nbits != 0 {
            self.bytes[n] = self.bits as u8;
            self.bits >>= 8;
            if self.nbits > 8 {
                // Avoid underflow
                self.nbits -= 8;
            } else {
                self.nbits = 0;
            }
            n += 1;
        }
        self.bits = 0;
        self.write_bytes_from_internal_buffer(writer, n);
        self.nbytes = 0;
    }

    fn write(&mut self, writer: &mut dyn std::io::Write, b: &[u8]) {
        if self.err.is_err() {
            return;
        }
        self.err = writer.write(b);
    }

    /// write data from an internal buffer
    fn write_bytes_from_internal_buffer(&mut self, writer: &mut dyn std::io::Write, n: usize) {
        if self.err.is_err() {
            return;
        }
        self.err = writer.write(&self.bytes[0..n]);
    }

    fn write_bits(&mut self, writer: &mut dyn std::io::Write, b: u32, nb: usize) {
        if self.err.is_err() {
            return;
        }
        self.bits |= (b as u64) << self.nbits;
        self.nbits += nb;
        if self.nbits >= 48 {
            let bits = self.bits;
            self.bits >>= 48;
            self.nbits -= 48;
            let mut n = self.nbytes;
            {
                let bytes = &mut self.bytes[n..n + 6];
                bytes[0] = (bits) as u8;
                bytes[1] = (bits >> 8) as u8;
                bytes[2] = (bits >> 16) as u8;
                bytes[3] = (bits >> 24) as u8;
                bytes[4] = (bits >> 32) as u8;
                bytes[5] = (bits >> 40) as u8;
            }
            n += 6;
            if n >= BUFFER_FLUSH_SIZE {
                self.write_bytes_from_internal_buffer(writer, n);
                n = 0;
            }
            self.nbytes = n
        }
    }

    pub(super) fn write_bytes(&mut self, writer: &mut dyn std::io::Write, bytes: &[u8]) {
        if self.err.is_err() {
            return;
        }
        let mut n = self.nbytes;
        if self.nbits & 7 != 0 {
            self.err = Err(io::Error::new(
                io::ErrorKind::Other,
                "writeBytes with unfinished bits",
            ));
            return;
        }
        while self.nbits != 0 {
            self.bytes[n] = self.bits as u8;
            self.bits >>= 8;
            self.nbits -= 8;
            n += 1;
        }
        if n != 0 {
            self.write_bytes_from_internal_buffer(writer, n);
        }
        self.nbytes = 0;
        self.write(writer, bytes);
    }

    /// RFC 1951 3.2.7 specifies a special run-length encoding for specifying
    /// the literal and offset lengths arrays (which are concatenated into a single
    /// array).  This method generates that run-length encoding.
    ///
    /// The result is written into the codegen array, and the frequencies
    /// of each code is written into the codegen_freq array.
    /// Codes 0-15 are single u8 codes. Codes 16-18 are followed by additional
    /// information. Code badCode is an end marker
    ///
    /// num_literals      The number of literals in literal_encoding
    /// num_offsets       The number of offsets in offset_encoding
    /// offenc           The offset encoder to use.  If None, use internal.
    fn generate_codegen(
        &mut self,
        num_literals: usize,
        num_offsets: usize,
        off_enc: Option<&HuffmanEncoder>,
    ) {
        self.codegen_freq.fill(0);
        // Note that we are using codegen both as a temporary variable for holding
        // a copy of the frequencies, and as the place where we put the result.
        // This is fine because the output is always shorter than the input used
        // so far.

        // Copy the concatenated code sizes to codegen. Put a marker at the end.
        // let cgnl = &mut self.codegen[..num_literals];
        for i in 0..num_literals {
            self.codegen[i] = self.literal_encoding.codes[i].len as u8;
        }

        for i in 0..num_offsets {
            self.codegen[num_literals + i] = match off_enc {
                None => &self.offset_encoding,
                Some(enc) => enc,
            }
            .codes[i]
                .len as u8;
        }
        self.codegen[num_literals + num_offsets] = BAD_CODE;

        let mut size = self.codegen[0];
        let mut count = 1;
        let mut out_index = 0;
        let mut in_index = 1;
        while size != BAD_CODE {
            // INVARIANT: We have seen "count" copies of size that have not yet
            // had output generated for them.
            let next_size = self.codegen[in_index];
            if next_size == size {
                count += 1;
                in_index += 1;
                continue;
            }
            // We need to generate codegen indicating "count" of size.
            if size != 0 {
                self.codegen[out_index] = size;
                out_index += 1;
                self.codegen_freq[size as usize] += 1;
                count -= 1;
                while count >= 3 {
                    let mut n = 6;
                    if n > count {
                        n = count;
                    }
                    self.codegen[out_index] = 16;
                    out_index += 1;
                    self.codegen[out_index] = (n - 3) as u8;
                    out_index += 1;
                    self.codegen_freq[16] += 1;
                    count -= n;
                }
            } else {
                while count >= 11 {
                    let mut n = 138;
                    if n > count {
                        n = count;
                    }
                    self.codegen[out_index] = 18;
                    out_index += 1;
                    self.codegen[out_index] = (n - 11) as u8;
                    out_index += 1;
                    self.codegen_freq[18] += 1;
                    count -= n;
                }
                if count >= 3 {
                    // count >= 3 && count <= 10
                    self.codegen[out_index] = 17;
                    out_index += 1;
                    self.codegen[out_index] = (count - 3) as u8;
                    out_index += 1;
                    self.codegen_freq[17] += 1;
                    count = 0;
                }
            }
            count -= 1;
            while count >= 0 {
                self.codegen[out_index] = size;
                out_index += 1;
                self.codegen_freq[size as usize] += 1;
                count -= 1;
            }
            // Set up invariant for next time through the loop.
            size = next_size;
            count = 1;
            in_index += 1;
        }
        // Marker indicating the end of the codegen.
        self.codegen[out_index] = BAD_CODE;
    }

    /// dynamic_size returns the size of dynamically encoded data in bits.
    fn dynamic_size(
        &self,
        lit_enc: &HuffmanEncoder,
        off_enc: &HuffmanEncoder,
        extra_bits: usize,
    ) -> (usize, usize) {
        let mut num_codegens = self.codegen_freq.len();
        while num_codegens > 4 && self.codegen_freq[CODEGEN_ORDER[num_codegens - 1] as usize] == 0 {
            num_codegens -= 1;
        }
        let header = 3
            + 5
            + 5
            + 4
            + (3 * num_codegens)
            + self.codegen_encoding.bit_length(&self.codegen_freq)
            + (self.codegen_freq[16] as usize) * 2
            + (self.codegen_freq[17] as usize) * 3
            + (self.codegen_freq[18] as usize) * 7;
        let size = header
            + lit_enc.bit_length(&self.literal_freq)
            + off_enc.bit_length(&self.offset_freq)
            + extra_bits;

        (size, num_codegens)
    }

    /// fixed_size returns the size of dynamically encoded data in bits.
    fn fixed_size(&mut self, extra_bits: usize) -> usize {
        3 + get_fixed_literal_encoding().bit_length(&self.literal_freq)
            + get_fixed_offset_encoding().bit_length(&self.offset_freq)
            + extra_bits
    }

    /// stored_size calculates the stored size, including header.
    /// The function returns the size in bits and whether the block
    /// fits inside a single block.
    fn stored_size(&mut self, input: Option<&[u8]>) -> (usize, bool) {
        if let Some(input) = input {
            if input.len() <= deflate::MAX_STORE_BLOCK_SIZE {
                return ((input.len() + 5) * 8, true);
            }
        }
        (0, false)
    }

    fn write_code(&mut self, writer: &mut dyn std::io::Write, c: HCode) {
        if self.err.is_err() {
            return;
        }
        self.bits |= (c.code as u64) << self.nbits;
        self.nbits += c.len as usize;
        if self.nbits >= 48 {
            let bits = self.bits;
            self.bits >>= 48;
            self.nbits -= 48;
            let mut n = self.nbytes;
            {
                let bytes = &mut self.bytes[n..n + 6];
                bytes[0] = (bits) as u8;
                bytes[1] = (bits >> 8) as u8;
                bytes[2] = (bits >> 16) as u8;
                bytes[3] = (bits >> 24) as u8;
                bytes[4] = (bits >> 32) as u8;
                bytes[5] = (bits >> 40) as u8;
            }
            n += 6;
            if n >= BUFFER_FLUSH_SIZE {
                self.write_bytes_from_internal_buffer(writer, n);
                n = 0;
            }
            self.nbytes = n;
        }
    }

    /// Write the header of a dynamic Huffman block to the output stream.
    ///
    /// num_literals  The number of literals specified in codegen
    /// num_offsets   The number of offsets specified in codegen
    /// num_codegens  The number of codegens used in codegen
    fn write_dynamic_header(
        &mut self,
        writer: &mut dyn std::io::Write,
        num_literals: usize,
        num_offsets: usize,
        num_codegens: usize,
        is_eof: bool,
    ) {
        if self.err.is_err() {
            return;
        }
        let first_bits: u32 = if is_eof { 5 } else { 4 };
        self.write_bits(writer, first_bits, 3);
        self.write_bits(writer, (num_literals - 257) as u32, 5);
        self.write_bits(writer, (num_offsets - 1) as u32, 5);
        self.write_bits(writer, (num_codegens - 4) as u32, 4);

        #[allow(clippy::needless_range_loop)]
        for i in 0..num_codegens {
            let value = self.codegen_encoding.codes[CODEGEN_ORDER[i] as usize].len;
            self.write_bits(writer, value as u32, 3);
        }

        let mut i = 0;
        loop {
            let code_word = self.codegen[i];
            i += 1;
            if code_word == BAD_CODE {
                break;
            }
            self.write_code(writer, self.codegen_encoding.codes[code_word as usize]);

            match code_word {
                16 => {
                    self.write_bits(writer, self.codegen[i] as u32, 2);
                    i += 1;
                }
                17 => {
                    self.write_bits(writer, self.codegen[i] as u32, 3);
                    i += 1;
                }
                18 => {
                    self.write_bits(writer, self.codegen[i] as u32, 7);
                    i += 1;
                }
                _ => {}
            }
        }
    }

    pub(super) fn write_stored_header(
        &mut self,
        writer: &mut dyn std::io::Write,
        length: usize,
        is_eof: bool,
    ) {
        if self.err.is_err() {
            return;
        }
        let flag: u32 = if is_eof { 1 } else { 0 };
        self.write_bits(writer, flag, 3);
        self.flush(writer);
        self.write_bits(writer, length as u32, 16);
        self.write_bits(writer, (!(length as u16)) as u32, 16);
    }

    fn write_fixed_header(&mut self, writer: &mut dyn std::io::Write, is_eof: bool) {
        if self.err.is_err() {
            return;
        }
        // Indicate that we are a fixed Huffman block
        let value = if is_eof { 3 } else { 2 };
        self.write_bits(writer, value, 3)
    }

    /// write_block will write a block of tokens with the smallest encoding.
    /// The original input can be supplied, and if the huffman encoded data
    /// is larger than the original bytes, the data will be written as a
    /// stored block.
    /// If the input is nil, the tokens will always be Huffman encoded.
    pub(super) fn write_block(
        &mut self,
        writer: &mut dyn std::io::Write,
        tokens: &[Token],
        eof: bool,
        input: Option<&[u8]>,
    ) {
        if self.err.is_err() {
            return;
        }

        let mut tokens = tokens.to_vec();
        tokens.push(END_BLOCK_MARKER as u32);
        let (num_literals, num_offsets) = self.index_tokens(&tokens);

        let mut extra_bits = 0;
        let (stored_size, storable) = self.stored_size(input);
        if storable {
            // We only bother calculating the costs of the extra bits required by
            // the length of offset fields (which will be the same for both fixed
            // and dynamic encoding), if we need to compare those two encodings
            // against stored encoding.
            for length_code in LENGTH_CODES_START + 8..num_literals {
                // First eight length codes have extra size = 0.
                extra_bits += (self.literal_freq[length_code]) as usize
                    * (LENGTH_EXTRA_BITS[length_code - LENGTH_CODES_START]) as usize;
            }
            #[allow(clippy::needless_range_loop)]
            for offset_code in 4..num_offsets {
                // First four offset codes have extra size = 0.
                extra_bits += (self.offset_freq[offset_code]) as usize
                    * (OFFSET_EXTRA_BITS[offset_code]) as usize;
            }
        }

        // Figure out smallest code.
        // Fixed Huffman baseline.
        let literal_encoding = get_fixed_literal_encoding();
        let offset_encoding = get_fixed_offset_encoding();
        let mut size = self.fixed_size(extra_bits);

        // Dynamic Huffman?
        // let num_codegens = 0;

        // Generate codegen and codegenFrequencies, which indicates how to encode
        // the literal_encoding and the offset_encoding.
        self.generate_codegen(num_literals, num_offsets, None);
        self.codegen_encoding.generate(&self.codegen_freq, 7);
        let (dynamic_size, num_codegens) =
            self.dynamic_size(&self.literal_encoding, &self.offset_encoding, extra_bits);

        let mut use_dynamic = false;
        if dynamic_size < size {
            use_dynamic = true;
            size = dynamic_size;
        }

        // Stored bytes?
        if storable && stored_size < size {
            self.write_stored_header(writer, input.unwrap().len(), eof);
            self.write_bytes(writer, input.unwrap());
            return;
        }

        // Huffman.
        if !use_dynamic {
            self.write_fixed_header(writer, eof);
            self.write_tokens(
                writer,
                &tokens,
                &literal_encoding.codes,
                &offset_encoding.codes,
            );
        } else {
            self.write_dynamic_header(writer, num_literals, num_offsets, num_codegens, eof);
            self.write_tokens_using_internal_codes(writer, &tokens);
        }
    }

    /// write_block_dynamic encodes a block using a dynamic Huffman table.
    /// This should be used if the symbols used have a disproportionate
    /// histogram distribution.
    /// If input is supplied and the compression savings are below 1/16th of the
    /// input size the block is stored.
    pub(super) fn write_block_dynamic(
        &mut self,
        writer: &mut dyn std::io::Write,
        tokens: &[Token],
        eof: bool,
        input: Option<&[u8]>,
    ) {
        if self.err.is_err() {
            return;
        }

        let mut tokens = tokens.to_vec();
        tokens.push(END_BLOCK_MARKER as u32);
        let (num_literals, num_offsets) = self.index_tokens(&tokens);

        // Generate codegen and codegenFrequencies, which indicates how to encode
        // the literal_encoding and the offset_encoding.
        self.generate_codegen(num_literals, num_offsets, None);
        self.codegen_encoding.generate(&self.codegen_freq, 7);
        let (size, num_codegens) =
            self.dynamic_size(&self.literal_encoding, &self.offset_encoding, 0);

        // Store bytes, if we don't get a reasonable improvement.
        let (ssize, storable) = self.stored_size(input);
        if storable && ssize < (size + (size >> 4)) {
            let input = input.unwrap();
            self.write_stored_header(writer, input.len(), eof);
            self.write_bytes(writer, input);
            return;
        }

        // Write Huffman table.
        self.write_dynamic_header(writer, num_literals, num_offsets, num_codegens, eof);

        // Write the tokens.
        self.write_tokens_using_internal_codes(writer, &tokens);
    }

    /// indexTokens indexes a slice of tokens, and updates
    /// literal_freq and offset_freq, and generates literal_encoding
    /// and offset_encoding.
    /// The number of literal and offset tokens is returned.
    fn index_tokens(&mut self, tokens: &[Token]) -> (usize, usize) {
        for i in 0..self.literal_freq.len() {
            self.literal_freq[i] = 0;
        }
        for i in 0..self.offset_freq.len() {
            self.offset_freq[i] = 0;
        }

        for t in tokens {
            if *t < MATCH_TYPE {
                self.literal_freq[t.literal() as usize] += 1;
                continue;
            }
            let length = t.length();
            let offset = t.offset();
            self.literal_freq[LENGTH_CODES_START + length_code(length)] += 1;
            self.offset_freq[offset_code(offset as usize)] += 1;
        }

        // get the number of literals
        let mut num_literals = self.literal_freq.len();
        while self.literal_freq[num_literals - 1] == 0 {
            num_literals -= 1;
        }
        // get the number of offsets
        let mut num_offsets = self.offset_freq.len();
        while num_offsets > 0 && self.offset_freq[num_offsets - 1] == 0 {
            num_offsets -= 1;
        }
        if num_offsets == 0 {
            // We haven't found a single match. If we want to go with the dynamic encoding,
            // we should count at least one offset to be sure that the offset huffman tree could be encoded.
            self.offset_freq[0] = 1;
            num_offsets = 1;
        }
        self.literal_encoding.generate(&self.literal_freq, 15);
        self.offset_encoding.generate(&self.offset_freq, 15);
        (num_literals, num_offsets)
    }

    /// write_tokens writes a slice of tokens to the output.
    /// codes for literal and offset encoding must be supplied.
    fn write_tokens(
        &mut self,
        writer: &mut dyn std::io::Write,
        tokens: &[Token],
        le_codes: &[HCode],
        oe_codes: &[HCode],
    ) {
        if self.err.is_err() {
            return;
        }
        for t in tokens {
            if *t < MATCH_TYPE {
                self.write_code(writer, le_codes[t.literal() as usize]);
                continue;
            }
            // Write the length
            let length = t.length();
            let length_code = length_code(length);
            self.write_code(writer, le_codes[length_code + LENGTH_CODES_START]);
            let extra_length_bits = LENGTH_EXTRA_BITS[length_code] as usize;
            if extra_length_bits > 0 {
                let extra_length = length - LENGTH_BASE[length_code] as u32;
                self.write_bits(writer, extra_length, extra_length_bits);
            }
            // Write the offset
            let offset = t.offset() as usize;
            let offset_code = offset_code(offset);
            self.write_code(writer, oe_codes[offset_code]);
            let extra_offset_bits = OFFSET_EXTRA_BITS[offset_code] as usize;
            if extra_offset_bits > 0 {
                let extra_offset = offset as u32 - OFFSET_BASE[offset_code];
                self.write_bits(writer, extra_offset, extra_offset_bits);
            }
        }
    }

    fn write_tokens_using_internal_codes(
        &mut self,
        writer: &mut dyn std::io::Write,
        tokens: &[Token],
    ) {
        if self.err.is_err() {
            return;
        }
        for t in tokens {
            if *t < MATCH_TYPE {
                self.write_code(writer, self.literal_encoding.codes[t.literal() as usize]);
                continue;
            }
            // Write the length
            let length = t.length();
            let length_code = length_code(length);
            self.write_code(
                writer,
                self.literal_encoding.codes[length_code + LENGTH_CODES_START],
            );
            let extra_length_bits = LENGTH_EXTRA_BITS[length_code] as usize;
            if extra_length_bits > 0 {
                let extra_length = length - LENGTH_BASE[length_code] as u32;
                self.write_bits(writer, extra_length, extra_length_bits);
            }
            // Write the offset
            let offset = t.offset() as usize;
            let offset_code = offset_code(offset);
            self.write_code(writer, self.offset_encoding.codes[offset_code]);
            let extra_offset_bits = OFFSET_EXTRA_BITS[offset_code] as usize;
            if extra_offset_bits > 0 {
                let extra_offset = offset as u32 - OFFSET_BASE[offset_code];
                self.write_bits(writer, extra_offset, extra_offset_bits);
            }
        }
    }

    /// write_block_huff encodes a block of bytes as either
    /// Huffman encoded literals or uncompressed bytes if the
    /// results only gains very little from compression.
    pub fn write_block_huff(&mut self, writer: &mut dyn std::io::Write, eof: bool, input: &[u8]) {
        if self.err.is_err() {
            return;
        }

        // Clear histogram
        self.literal_freq.fill(0);

        // Add everything as literals
        histogram(input, &mut self.literal_freq);

        self.literal_freq[END_BLOCK_MARKER] = 1;

        let num_literals = END_BLOCK_MARKER + 1;
        self.offset_freq[0] = 1;
        let num_offsets = 1;

        self.literal_encoding.generate(&self.literal_freq, 15);

        // Figure out smallest code.
        // Always use dynamic Huffman or Store

        // Generate codegen and codegenFrequencies, which indicates how to encode
        // the literal_encoding and the offset_encoding.
        self.generate_codegen(num_literals, num_offsets, Some(get_huff_offset()));
        self.codegen_encoding.generate(&self.codegen_freq, 7);
        let (size, num_codegens) = self.dynamic_size(&self.literal_encoding, get_huff_offset(), 0);

        // Store bytes, if we don't get a reasonable improvement.
        let (ssize, storable) = self.stored_size(Some(input));
        if storable && ssize < (size + (size >> 4)) {
            self.write_stored_header(writer, input.len(), eof);
            self.write_bytes(writer, input);
            return;
        }

        // Huffman.
        self.write_dynamic_header(writer, num_literals, num_offsets, num_codegens, eof);
        let mut n = self.nbytes;
        for t in input {
            // Bitwriting inlined, ~30% speedup
            let c = self.literal_encoding.codes[*t as usize];
            self.bits |= (c.code as u64) << self.nbits;
            self.nbits += c.len as usize;
            if self.nbits < 48 {
                continue;
            }
            // Store 6 bytes
            let bits = self.bits;
            self.bits >>= 48;
            self.nbits -= 48;
            {
                let bytes = &mut self.bytes[n..n + 6];
                bytes[0] = (bits) as u8;
                bytes[1] = (bits >> 8) as u8;
                bytes[2] = (bits >> 16) as u8;
                bytes[3] = (bits >> 24) as u8;
                bytes[4] = (bits >> 32) as u8;
                bytes[5] = (bits >> 40) as u8;
            }
            n += 6;
            if n < BUFFER_FLUSH_SIZE {
                continue;
            }
            self.write_bytes_from_internal_buffer(writer, n);
            if self.err.is_err() {
                return; // Return early in the event of write failures
            }
            n = 0;
        }
        self.nbytes = n;
        self.write_code(writer, self.literal_encoding.codes[END_BLOCK_MARKER]);
    }

    pub(super) fn error(&self) -> &std::io::Result<usize> {
        &self.err
    }
}
