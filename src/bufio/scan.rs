// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2013 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::bytes;
use crate::compat;

/// Scanner provides a convenient interface for reading data such as
/// a file of newline-delimited lines of text. Successive calls to
/// the Scan method will step through the 'tokens' of a file, skipping
/// the bytes between the tokens. The specification of a token is
/// defined by a split function of type SplitFunc; the default split
/// function breaks the input into lines with line termination stripped. Split
/// functions are defined in this package for scanning a file into
/// lines, bytes, UTF-8-encoded runes, and space-delimited words. The
/// client may instead provide a custom split function.
///
/// Scanning stops unrecoverably at EOF, the first I/O error, or a token too
/// large to fit in the buffer. When a scan stops, the reader may have
/// advanced arbitrarily far past the last token. Programs that need more
/// control over error handling or large tokens, or must run sequential scans
/// on a reader, should use bufio.Reader instead.
pub struct Scanner<'a> {
    r: &'a mut dyn std::io::Read, // The reader provided by the client.

    split: SplitFunc,      // The function to split the tokens.
    max_token_size: usize, // Maximum size of a token; modified by tests.
    token: Vec<u8>,        // Last token returned by split.
    buf: Vec<u8>,          // Buffer used as argument to split.
    start: usize,          // First non-processed byte in buf.
    end: usize,            // End of data in buf.
    err: Option<Error>,    // Sticky error.
    scan_called: bool,     // Scan has been called; buffer is in use.
    done: bool,            // Scan has finished.
    eof_reached: bool,
}

// ggrust TODO
#[derive(Default)]
pub struct ScanResult {
    pub advance: usize,
    pub token: Option<Vec<u8>>, // Vec<u8>,
    pub final_token: bool,
    pub err: Option<std::io::Error>,
}

impl ScanResult {
    pub fn new(advance: usize, token: Option<Vec<u8>>, err: Option<std::io::Error>) -> Self {
        Self {
            advance,
            token,
            err,
            final_token: false,
        }
    }
}

/// SplitFunc is the signature of the split function used to tokenize the
/// input. The arguments are an initial substring of the remaining unprocessed
/// data and a flag, at_eof, that reports whether the Reader has no more data
/// to give. The return values are the number of bytes to advance the input
/// and the next token to return to the user, if any, plus an error, if any.
///
/// Scanning stops if the function returns an error, in which case some of
/// the input may be discarded. If that error is ErrFinalToken, scanning
/// stops with no error.
///
/// Otherwise, the Scanner advances the input. If the token is not nil,
/// the Scanner returns it to the user. If the token is nil, the
/// Scanner reads more data and continues scanning; if there is no more
/// data--if at_eof was true--the Scanner returns. If the data does not
/// yet hold a complete token, for instance if it has no newline while
/// scanning lines, a SplitFunc can return (0, nil, nil) to signal the
/// Scanner to read more data into the slice and try again with a
/// longer slice starting at the same point in the input.
///
/// The function is never called with an empty data slice unless at_eof
/// is true. If at_eof is true, however, data may be non-empty and,
/// as always, holds unprocessed text.
type SplitFunc = fn(data: &[u8], at_eof: bool) -> ScanResult;
// type SplitFunc fn(data [u8], at_eof bool) (advance int, token [u8], err error)

/// Errors returned by Scanner.
#[derive(Debug)]
pub enum Error {
    ErrTooLong,
    // ErrNegativeAdvance,
    ErrAdvanceTooFar,
    // ErrBadReadCount,
    ErrRead(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ErrTooLong => write!(f, "bufio.Scanner: token too long"),
            // Error::ErrNegativeAdvance => {
            //     write!(f, "bufio.Scanner: SplitFunc returns negative advance count")
            // }
            Error::ErrAdvanceTooFar => write!(
                f,
                "bufio.Scanner: SplitFunc returns advance count beyond input"
            ),
            Error::ErrRead(err) => write!(f, "{}", err.to_string()),
            // Error::ErrBadReadCount => write!(f, "bufio.Scanner: Read returned impossible count"),
        }
    }
}

/// MAX_SCAN_TOKEN_SIZE is the maximum size used to buffer a token
/// unless the user provides an explicit buffer with Scanner.Buffer.
/// The actual maximum token size may be smaller as the buffer
/// may need to include, for instance, a newline.
pub const MAX_SCAN_TOKEN_SIZE: usize = 64 * 1024;

/// Size of initial allocation for buffer.
const START_BUF_SIZE: usize = 4096;

// ggstd: use ScanResult.final_token instead of ErrFinalToken
// // ErrFinalToken is a special sentinel error value. It is intended to be
// // returned by a Split function to indicate that the token being delivered
// // with the error is the last token and scanning should stop after this one.
// // After ErrFinalToken is received by Scan, scanning stops with no error.
// // The value is useful to stop processing early or when it is necessary to
// // deliver a final empty token. One could achieve the same behavior
// // with a custom error value but providing one here is tidier.
// // See the emptyFinalToken example for a use of this value.
// var ErrFinalToken = errors.New("final token")

impl<'a> Scanner<'a> {
    /// new returns a new Scanner to read from r.
    /// The split function defaults to scan_lines.
    pub fn new(r: &'a mut dyn std::io::Read) -> Self {
        Self {
            r,
            split: scan_lines,
            max_token_size: MAX_SCAN_TOKEN_SIZE,
            start: 0,
            end: 0,
            err: None,
            scan_called: false,
            done: false,
            buf: Vec::new(),
            token: Vec::with_capacity(0),
            eof_reached: false,
        }
        // fn NewScanner(r io.Reader) *Scanner {
        // 	return &Scanner{
        // 		r:            r,
        // 		split:        scan_lines,
        // 		max_token_size: MAX_SCAN_TOKEN_SIZE,
        // 	}
        // }
    }

    /// err returns the first non-EOF error that was encountered by the Scanner.
    pub fn err(&self) -> &Option<Error> {
        // 	if self.err == io.EOF {
        // 		return nil
        // 	}
        return &self.err;
    }

    /// bytes returns the most recent token generated by a call to Scan.
    // The underlying array may point to data that will be overwritten
    // by a subsequent call to Scan. It does no allocation.
    pub fn bytes(&self) -> &Vec<u8> {
        &self.token
    }

    /// text returns the most recent token generated by a call to Scan
    /// as a newly allocated string holding its bytes.
    pub fn text(&self) -> String {
        compat::string(&self.token)
    }

    /// scan advances the Scanner to the next token, which will then be
    /// available through the bytes or text method. It returns false when the
    /// scan stops, either by reaching the end of the input or an error.
    /// After scan returns false, the Err method will return any error that
    /// occurred during scanning, except that if it was io.EOF, Err
    /// will return nil.
    // scan panics if the split function returns too many empty
    // tokens without advancing the input. This is a common error mode for
    // scanners.
    pub fn scan(&mut self) -> bool {
        if self.done {
            return false;
        }
        self.scan_called = true;
        // Loop until we have a token.
        loop {
            // See if we can get a token with what we already have.
            // If we've run out of data but have an error, give the split function
            // a chance to recover any remaining, possibly empty token.
            if self.end > self.start || self.err.is_some() || self.eof_reached {
                let res = (self.split)(
                    &self.buf[self.start..self.end],
                    self.err.is_some() || self.eof_reached,
                );
                if res.final_token {
                    self.token = res.token.unwrap();
                    self.done = true;
                    return true;
                }
                if let Some(err) = res.err {
                    self.set_err(Error::ErrRead(err));
                    return false;
                }
                if !self.advance(res.advance) {
                    return false;
                }
                let has_token = res.token.is_some();
                self.token = if has_token {
                    res.token.unwrap().to_vec()
                } else {
                    Vec::with_capacity(0)
                };
                if has_token {
                    // if !self.err.is_some() || res.advance > 0 {
                    //     self.empties = 0
                    // } else {
                    //     // Returning tokens not advancing input at EOF.
                    //     self.empties += 1;
                    //     if self.empties > super::bufio::MAX_CONSECUTIVE_EMPTY_READS {
                    //         panic!("bufio.Scan: too many empty tokens without progressing");
                    //     }
                    // }
                    return true;
                }
            }

            if self.eof_reached {
                return false;
            }

            // We cannot generate a token with what we are holding.
            // If we've already hit EOF or an I/O error, we are done.
            if self.err.is_some() {
                // Shut it down.
                self.start = 0;
                self.end = 0;
                return false;
            }
            // Must read more data.
            // First, shift data to beginning of buffer if there's lots of empty space
            // or space is needed.
            if self.start > 0 && (self.end == self.buf.len() || self.start > self.buf.len() / 2) {
                compat::copy_within(&mut self.buf, self.start..self.end, 0);
                self.end -= self.start;
                self.start = 0;
            }
            // Is the buffer full? If so, resize.
            if self.end == self.buf.len() {
                // Guarantee no overflow in the multiplication below.
                if self.buf.len() >= self.max_token_size || self.buf.len() > usize::MAX / 2 {
                    self.set_err(Error::ErrTooLong);
                    return false;
                }
                let mut new_size = self.buf.len() * 2;
                if new_size == 0 {
                    new_size = START_BUF_SIZE;
                }
                if new_size > self.max_token_size {
                    new_size = self.max_token_size;
                }
                self.buf.resize(new_size, 0);
                compat::copy_within(&mut self.buf, self.start..self.end, 0);
                self.end -= self.start;
                self.start = 0;
            }
            let read_range = self.end..self.buf.len();
            match self.r.read(&mut self.buf[read_range]) {
                Ok(n) => {
                    if n == 0 {
                        self.eof_reached = true;
                    } else {
                        self.end += n;
                    }
                }
                Err(err) => {
                    self.set_err(Error::ErrRead(err));
                }
            }
        }
    }

    /// advance consumes n bytes of the buffer. It reports whether the advance was legal.
    fn advance(&mut self, n: usize) -> bool {
        // 	if n < 0 {
        // 		self.set_err(ErrNegativeAdvance)
        // 		return false
        // 	}
        if n > self.end - self.start {
            self.set_err(Error::ErrAdvanceTooFar);
            return false;
        }
        self.start += n;
        return true;
    }

    /// set_err records the first error encountered.
    fn set_err(&mut self, err: Error) {
        if self.err.is_none() {
            self.err = Some(err);
        }
    }

    // // Buffer sets the initial buffer to use when scanning and the maximum
    // // size of buffer that may be allocated during scanning. The maximum
    // // token size is the larger of max and cap(buf). If max <= cap(buf),
    // // Scan will use this buffer only and do no allocation.
    // //
    // // By default, Scan uses an internal buffer and sets the
    // // maximum token size to MAX_SCAN_TOKEN_SIZE.
    // //
    // // Buffer panics if it is called after scanning has started.
    // fn Buffer(&selfbuf [u8], max int) {
    // 	if self.scan_called {
    // 		panic("Buffer called after Scan")
    // 	}
    // 	self.buf = buf[0..cap(buf)]
    // 	self.max_token_size = max
    // }

    /// split sets the split function for the Scanner.
    /// The default split function is scan_lines.
    ///
    /// split panics if it is called after scanning has started.
    pub fn split(&mut self, split: SplitFunc) {
        if self.scan_called {
            panic!("Split called after Scan");
        }
        self.split = split;
    }
}

// Split functions

/// scan_bytes is a split function for a Scanner that returns each byte as a token.
pub fn scan_bytes(data: &[u8], at_eof: bool) -> ScanResult {
    if at_eof && data.len() == 0 {
        return ScanResult::default();
    }
    ScanResult {
        advance: 1,
        token: Some(data[0..1].to_vec()),
        err: None,
        final_token: false,
    }
}

// var errorRune = [u8](string(utf8.RUNE_ERROR))

// // ScanRunes is a split function for a Scanner that returns each
// // UTF-8-encoded rune as a token. The sequence of runes returned is
// // equivalent to that from a range loop over the input as a string, which
// // means that erroneous UTF-8 encodings translate to U+FFFD = "\xef\xbf\xbd".
// // Because of the Scan interface, this makes it impossible for the client to
// // distinguish correctly encoded replacement runes from encoding errors.
// fn ScanRunes(data: &[u8], at_eof: bool) -> ScanResult {
// 	if at_eof && data.len() == 0 {
// return ScanResult::default();
// 	}

// 	// Fast path 1: ASCII.
// 	if data[0] < utf8::RUNE_SELF {
// 		return 1, data[0..1], nil
// 	}

// 	// Fast path 2: Correct UTF-8 decode without error.
// 	_, width := utf8.decode_rune(data)
// 	if width > 1 {
// 		// It's a valid encoding. Width cannot be one for a correctly encoded
// 		// non-ASCII rune.
// 		return width, data[0..width], nil
// 	}

// 	// We know it's an error: we have width==1 and implicitly r==utf8.RUNE_ERROR.
// 	// Is the error because there wasn't a full rune to be decoded?
// 	// full_rune distinguishes correctly between erroneous and incomplete encodings.
// 	if !at_eof && !utf8.full_rune(data) {
// 		// Incomplete; get more bytes.
// return ScanResult::default();
// 	}

// 	// We have a real UTF-8 encoding error. Return a properly encoded error rune
// 	// but advance only one byte. This matches the behavior of a range loop over
// 	// an incorrectly encoded string.
// 	return 1, errorRune, nil
// }

/// drop_cr drops a terminal \r from the data.
fn drop_cr(data: &[u8]) -> &[u8] {
    if data.len() > 0 && data[data.len() - 1] == b'\r' {
        return &data[0..data.len() - 1];
    }
    return data;
}

/// scan_lines is a split function for a Scanner that returns each line of
/// text, stripped of any trailing end-of-line marker. The returned line may
/// be empty. The end-of-line marker is one optional carriage return followed
/// by one mandatory newline. In regular expression notation, it is `\r?\n`.
/// The last non-empty line of input will be returned even if it has no
/// newline.
pub fn scan_lines(data: &[u8], at_eof: bool) -> ScanResult {
    if at_eof && data.len() == 0 {
        return ScanResult::default();
    }
    // 	if
    let i = bytes::index_byte(data, b'\n');
    if i >= 0 {
        let i = i as usize;
        // We have a full newline-terminated line.
        return ScanResult {
            advance: i + 1,
            token: Some(drop_cr(&data[0..i as usize]).to_vec()),
            err: None,
            final_token: false,
        };
    }
    // If we're at EOF, we have a final, non-terminated line. Return it.
    if at_eof {
        return ScanResult {
            advance: data.len(),
            token: Some(drop_cr(data).to_vec()),
            err: None,
            final_token: false,
        };
    }
    // Request more data.
    return ScanResult::default();
}

// // isSpace reports whether the character is a Unicode white space character.
// // We avoid dependency on the unicode package, but check validity of the implementation
// // in the tests.
// fn isSpace(r rune) -> bool {
// 	if r <= '\u00FF' {
// 		// Obvious ASCII ones: \t through \r plus space. Plus two Latin-1 oddballs.
// 		switch r {
// 		case ' ', '\t', '\n', '\v', '\f', '\r':
// 			return true
// 		case '\u0085', '\u00A0':
// 			return true
// 		}
// 		return false
// 	}
// 	// High-valued ones.
// 	if '\u2000' <= r && r <= '\u200a' {
// 		return true
// 	}
// 	switch r {
// 	case '\u1680', '\u2028', '\u2029', '\u202f', '\u205f', '\u3000':
// 		return true
// 	}
// 	return false
// }

// // ScanWords is a split function for a Scanner that returns each
// // space-separated word of text, with surrounding spaces deleted. It will
// // never return an empty string. The definition of space is set by
// // unicode.IsSpace.
// fn ScanWords(data: &[u8], at_eof: bool) -> ScanResult {
// 	// Skip leading spaces.
// 	start := 0
// 	for width := 0; start < data.len(); start += width {
// 		var r rune
// 		r, width = utf8.decode_rune(data[start:])
// 		if !isSpace(r) {
// 			break
// 		}
// 	}
// 	// Scan until space, marking end of word.
// 	for width, i := 0, start; i < data.len(); i += width {
// 		var r rune
// 		r, width = utf8.decode_rune(data[i:])
// 		if isSpace(r) {
// 			return i + width, data[start:i], nil
// 		}
// 	}
// 	// If we're at EOF, we have a final, non-empty, non-terminated word. Return it.
// 	if at_eof && data.len() > start {
// 		return data.len(), data[start:], nil
// 	}
// 	// Request more data.
// 	return start, nil, nil
// }
