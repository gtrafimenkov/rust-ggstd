// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;

/// dictDecoder implements the LZ77 sliding dictionary as used in decompression.
/// LZ77 decompresses data through sequences of two forms of commands:
///
///   - Literal insertions: Runs of one or more symbols are inserted into the data
///     stream as is. This is accomplished through the write_byte method for a
///     single symbol, or combinations of write_slice/write_mark for multiple symbols.
///     Any valid stream must start with a literal insertion if no preset dictionary
///     is used.
///
///   - Backward copies: Runs of one or more symbols are copied from previously
///     emitted data. Backward copies come as the tuple (dist, length) where dist
///     determines how far back in the stream to copy from and length determines how
///     many bytes to copy. Note that it is valid for the length to be greater than
///     the distance. Since LZ77 uses forward copies, that situation is used to
///     perform a form of run-length encoding on repeated runs of symbols.
///     The write_copy and try_write_copy are used to implement this command.
///
/// For performance reasons, this implementation performs little to no sanity
/// checks about the arguments. As such, the invariants documented for each
/// method call must be respected.
pub(super) struct DictDecoder {
    hist: Vec<u8>, // Sliding window history

    // Invariant: 0 <= rd_pos <= wr_pos <= len(hist)
    wr_pos: usize, // Current output position in buffer
    rd_pos: usize, // Have emitted hist[..rd_pos] already
    full: bool,    // Has a full window length been written yet?

    // ggrust: Go implementation in readFlush returned a slice to hist buffer and
    // decoder could save the slice to toRead field and read it later.
    // In rust we can't do that.  We could return the copy of data, but it would be
    // extra work.  We chose to save information about wr_pos and rd_pos at the time
    // of readFlush, name this area "stash" and provide additional functions to read
    // from this stash.
    stash_start_pos: usize,
    stash_end_pos: usize,
}

impl DictDecoder {
    /// init initializes dictDecoder to have a sliding window dictionary of the given
    /// size. If a preset dict is provided, it will initialize the dictionary with
    /// the contents of dict.
    pub fn new(size: usize, dict: &[u8]) -> Self {
        let mut hist = vec![0; size];
        let min_size = size.min(dict.len());
        hist[0..min_size].copy_from_slice(&dict[0..min_size]);
        let mut wr_pos = min_size;
        let mut full = false;
        if wr_pos == size {
            wr_pos = 0;
            full = true;
        }
        Self {
            hist,
            wr_pos,
            rd_pos: wr_pos,
            full,
            stash_start_pos: 0,
            stash_end_pos: 0,
        }
    }

    /// hist_size reports the total amount of historical data in the dictionary.
    pub(super) fn hist_size(&self) -> usize {
        if self.full {
            return self.hist.len();
        }
        self.wr_pos
    }

    /// avail_read reports the number of bytes that can be flushed by read_flush.
    pub fn avail_read(&self) -> usize {
        self.wr_pos - self.rd_pos
    }

    /// avail_write reports the available amount of output buffer space.
    pub fn avail_write(&self) -> usize {
        self.hist.len() - self.wr_pos
    }

    /// write_slice returns a slice of the available buffer to write data to.
    ///
    /// This invariant will be kept: len(s) <= avail_write()
    pub(super) fn write_slice(&mut self, max_len: usize) -> &mut [u8] {
        let len = self.avail_write().min(max_len);
        &mut self.hist[self.wr_pos..self.wr_pos + len]
    }

    /// write_mark advances the writer pointer by cnt.
    ///
    /// This invariant must be kept: 0 <= cnt <= avail_write()
    pub(super) fn write_mark(&mut self, cnt: usize) {
        self.wr_pos += cnt
    }

    /// write_byte writes a single byte to the dictionary.
    ///
    /// This invariant must be kept: 0 < avail_write()
    pub(super) fn write_byte(&mut self, c: u8) {
        self.hist[self.wr_pos] = c;
        self.wr_pos += 1;
    }

    /// write_copy copies a string at a given (dist, length) to the output.
    /// This returns the number of bytes copied and may be less than the requested
    /// length if the available space in the output buffer is too small.
    ///
    /// This invariant must be kept: 0 < dist <= hist_size()
    pub(super) fn write_copy(&mut self, dist: usize, length: usize) -> usize {
        let dst_base = self.wr_pos;
        let mut dst_pos = dst_base;
        let mut src_pos = dst_pos as isize - dist as isize;
        let mut end_pos = dst_pos + length;
        if end_pos > self.hist.len() {
            end_pos = self.hist.len();
        }

        // Copy non-overlapping section after destination position.
        //
        // This section is non-overlapping in that the copy length for this section
        // is always less than or equal to the backwards distance. This can occur
        // if a distance refers to data that wraps-around in the buffer.
        // Thus, a backwards copy is performed here; that is, the exact bytes in
        // the source prior to the copy is placed in the destination.
        if src_pos < 0 {
            src_pos += self.hist.len() as isize;
            let size1 = end_pos - dst_pos;
            let size2 = self.hist.len() - src_pos as usize;
            let size = size1.min(size2);
            let start = src_pos as usize;
            self.hist.copy_within(start..start + size, dst_pos);
            dst_pos += size;
            src_pos = 0;
        }
        let src_pos = src_pos as usize;

        // Copy possibly overlapping section before destination position.
        //
        // This section can overlap if the copy length for this section is larger
        // than the backwards distance. This is allowed by LZ77 so that repeated
        // strings can be succinctly represented using (dist, length) pairs.
        // Thus, a forwards copy is performed here; that is, the bytes copied is
        // possibly dependent on the resulting bytes in the destination as the copy
        // progresses along. This is functionally equivalent to the following:
        //
        //	for i := 0; i < endPos-dstPos; i += 1 {
        //		self.hist[dstPos+i] = self.hist[srcPos+i]
        //	}
        //	dstPos = endPos
        //
        while dst_pos < end_pos {
            let size1 = dst_pos - src_pos;
            let size2 = end_pos - dst_pos;
            let size = size1.min(size2);
            self.hist.copy_within(src_pos..src_pos + size, dst_pos);
            dst_pos += size;
        }

        self.wr_pos = dst_pos;
        dst_pos - dst_base
    }

    /// try_write_copy tries to copy a string at a given (distance, length) to the
    /// output. This specialized version is optimized for short distances.
    ///
    /// This method is designed to be inlined for performance reasons.
    ///
    /// This invariant must be kept: 0 < dist <= hist_size()
    pub(super) fn try_write_copy(&mut self, dist: usize, length: usize) -> usize {
        let mut dst_pos = self.wr_pos;
        let end_pos = dst_pos + length;
        if dst_pos < dist || end_pos > self.hist.len() {
            return 0;
        }
        let dst_base = dst_pos;
        let src_pos = dst_pos - dist;

        // Copy possibly overlapping section before destination position.
        while dst_pos < end_pos {
            let size1 = dst_pos - src_pos;
            let size2 = end_pos - dst_pos;
            let size = size1.min(size2);
            self.hist.copy_within(src_pos..src_pos + size, dst_pos);
            dst_pos += size;
        }

        self.wr_pos = dst_pos;
        dst_pos - dst_base
    }

    // read_flush returns a slice of the historical buffer that is ready to be
    // emitted to the user. The data returned by read_flush must be fully consumed
    // before calling any other dictDecoder methods.
    #[allow(dead_code)]
    pub(super) fn read_flush(&mut self) -> &[u8] {
        let to_read = &self.hist[self.rd_pos..self.wr_pos];
        self.rd_pos = self.wr_pos;
        if self.wr_pos == self.hist.len() {
            self.wr_pos = 0;
            self.rd_pos = 0;
            self.full = true;
        }
        to_read
    }

    // stash_flush stashes data for further reading with stash_read.  The stashed
    // data should be consumed before writing any new data.
    // This is equivalent of Go readFlush method.
    // ggrust TODO A: when everything is tested, convert this to read operations
    pub(super) fn stash_flush(&mut self) {
        self.stash_start_pos = self.rd_pos;
        self.stash_end_pos = self.wr_pos;
        self.rd_pos = self.wr_pos;
        if self.wr_pos == self.hist.len() {
            self.wr_pos = 0;
            self.rd_pos = 0;
            self.full = true;
        }
    }

    /// stash_len returns amount of stashed and not yet read data
    pub(super) fn stash_len(&self) -> usize {
        self.stash_end_pos - self.stash_start_pos
    }

    /// stash_read reads data from the stash to the output buffer b,
    /// returns number of copied bytes.
    pub(super) fn stash_read(&mut self, b: &mut [u8]) -> usize {
        let n = compat::copy(b, &self.hist[self.stash_start_pos..self.stash_end_pos]);
        self.stash_start_pos += n;
        n
    }
}
