// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io::Write;

use crate::compat;
use crate::encoding::binary::ByteOrder;
use crate::encoding::binary::BIG_ENDIAN;

/// An Encoding is a radix 64 encoding/decoding scheme, defined by a
/// 64-character alphabet. The most common encoding is the "base64"
/// encoding defined in RFC 4648 and used in MIME (RFC 2045) and PEM
/// (RFC 1421).  RFC 4648 also defines an alternate encoding, which is
/// the standard encoding with - and _ substituted for + and /.
pub struct Encoding {
    encode: [u8; 64],
    decode_map: [u8; 256],
    pad_char: Option<u8>,
    strict: bool,
}

// 	NoPadding           rune = -1  // No padding
const STD_PADDING: u8 = b'='; // Standard padding character

const DECODE_MAP_INITIALIZE: [u8; 256] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf,
];

pub(super) const ENCODE_STD: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub(super) const ENCODE_URL: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// get_std_encoding returns the standard base64 encoding, as defined in
/// RFC 4648.
pub fn get_std_encoding() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new(ENCODE_STD))
}

/// get_std_encoding_strict returns the standard base64 encoding, as defined in
/// RFC 4648, with strict decoding enabled.
pub fn get_std_encoding_strict() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new(ENCODE_STD).strict())
}

/// get_url_encoding returns the standard base64 encoding, as defined in
/// RFC 4648.
pub fn get_url_encoding() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new(ENCODE_URL))
}
// static URLEncoding: Encoding = Encoding::new(encodeURL.as_bytes());

/// get_url_encoding_strict returns the standard base64 encoding, as defined in
/// RFC 4648, with strict decoding enabled.
pub fn get_url_encoding_strict() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new(ENCODE_URL).strict())
}

/// get_raw_std_encoding returns the standard raw, unpadded base64 encoding,
/// as defined in RFC 4648 section 3.2.
/// This is the same as StdEncoding but omits padding characters.
pub fn get_raw_std_encoding() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new(ENCODE_STD).with_padding(None))
}

/// get_raw_std_encoding_strict returns the standard raw, unpadded base64 encoding,
/// as defined in RFC 4648 section 3.2, with strict decoding enabled.
pub fn get_raw_std_encoding_strict() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new_with_options(ENCODE_STD, None, true))
}

/// get_raw_url_encoding returns the unpadded alternate base64 encoding defined in RFC 4648.
/// It is typically used in URLs and file names.
/// This is the same as URLEncoding but omits padding characters.
pub fn get_raw_url_encoding() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new_with_options(ENCODE_URL, None, false))
}

/// get_raw_url_encoding_strict returns the unpadded alternate base64 encoding defined in RFC 4648.
/// It is typically used in URLs and file names, with strict decoding enabled.
pub fn get_raw_url_encoding_strict() -> &'static Encoding {
    static ENC: std::sync::OnceLock<Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| Encoding::new_with_options(ENCODE_URL, None, true))
}

impl Encoding {
    /// new returns a new padded Encoding defined by the given alphabet,
    /// which must be a 64-byte string that does not contain the padding character
    /// or CR / LF ('\r', '\n').
    /// The resulting Encoding uses the default padding character ('='),
    /// which may be changed or disabled via with_padding.
    pub fn new(encoder: &[u8; 64]) -> Self {
        Self::new_with_options(encoder, Some(STD_PADDING), false)
    }

    /// new_with_options returns a new Encoding defined by the given alphabet,
    /// padding and strictness.
    pub fn new_with_options(encoder: &[u8; 64], padding: Option<u8>, strict: bool) -> Self {
        if encoder.len() != 64 {
            panic!("encoding alphabet is not 64-bytes long");
        }
        for v in encoder {
            if *v == b'\n' || *v == b'\r' {
                panic!("encoding alphabet contains newline character");
            }
        }

        let mut e = Encoding {
            encode: *encoder,
            decode_map: DECODE_MAP_INITIALIZE,
            pad_char: padding,
            strict,
        };

        for (i, v) in encoder.iter().enumerate() {
            e.decode_map[*v as usize] = i as u8;
        }
        e
    }

    /// with_padding creates a new encoding identical to self except
    /// with a specified padding character, or NoPadding to disable padding.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet.
    pub fn with_padding(&self, padding: Option<u8>) -> Self {
        if let Some(padding) = padding {
            if padding == b'\r' || padding == b'\n' {
                panic!("invalid padding")
            }

            for v in self.encode {
                if v == padding {
                    panic!("padding contained in alphabet");
                }
            }
        }

        Self {
            encode: self.encode,
            decode_map: self.decode_map,
            pad_char: padding,
            strict: false,
        }
    }

    /// strict creates a new encoding identical to self except with
    /// strict decoding enabled. In this mode, the decoder requires that
    /// trailing padding bits are zero, as described in RFC 4648 section 3.5.
    ///
    /// Note that the input is still malleable, as new line characters
    /// (CR and LF) are still ignored.
    pub fn strict(&self) -> Self {
        Self {
            encode: self.encode,
            decode_map: self.decode_map,
            pad_char: self.pad_char,
            strict: true,
        }
    }

    /// encode encodes src using the encoding enc, writing
    /// encoded_len(src.len()) bytes to dst.
    ///
    /// The encoding pads the output to a multiple of 4 bytes,
    /// so Encode is not appropriate for use on individual blocks
    /// of a large data stream. Use NewEncoder() instead.
    pub fn encode(&self, dst: &mut [u8], src: &[u8]) {
        if src.is_empty() {
            return;
        }
        // 	// enc is a pointer receiver, so the use of self.encode within the hot
        // 	// loop below means a nil check at every operation. Lift that nil check
        // 	// outside of the loop to speed up the encoder.
        // 	_ = self.encode

        let (mut di, mut si) = (0, 0);
        let n = (src.len() / 3) * 3;
        while si < n {
            // Convert 3x 8bit source bytes into 4 bytes
            let val =
                (src[si] as usize) << 16 | (src[si + 1] as usize) << 8 | (src[si + 2] as usize);

            dst[di] = self.encode[(val >> 18) & 0x3F];
            dst[di + 1] = self.encode[(val >> 12) & 0x3F];
            dst[di + 2] = self.encode[(val >> 6) & 0x3F];
            dst[di + 3] = self.encode[val & 0x3F];

            si += 3;
            di += 4;
        }

        let remain = src.len() - si;
        if remain == 0 {
            return;
        }
        // Add the remaining small block
        let mut val = (src[si] as usize) << 16;
        if remain == 2 {
            val |= (src[si + 1] as usize) << 8;
        }

        dst[di] = self.encode[(val >> 18) & 0x3F];
        dst[di + 1] = self.encode[(val >> 12) & 0x3F];

        match remain {
            2 => {
                dst[di + 2] = self.encode[(val >> 6) & 0x3F];
                if let Some(pad_char) = self.pad_char {
                    dst[di + 3] = pad_char;
                }
            }
            1 => {
                if let Some(pad_char) = self.pad_char {
                    dst[di + 2] = pad_char;
                    dst[di + 3] = pad_char;
                }
            }
            _ => panic!("unreacheable code"),
        }
    }

    /// encode_to_string returns the base64 encoding of src.
    pub fn encode_to_string(&self, src: &[u8]) -> String {
        let mut buf = vec![0; self.encoded_len(src.len())];
        self.encode(&mut buf, src);
        String::from_utf8_lossy(&buf).to_string()
    }

    /// encoded_len returns the length in bytes of the base64 encoding
    /// of an input buffer of length n.
    pub fn encoded_len(&self, n: usize) -> usize {
        match self.pad_char {
            None => {
                (n * 8 + 5) / 6 // minimum # chars at 6 bits per char
            }
            Some(_) => {
                (n + 2) / 3 * 4 // minimum # 4-char quanta, 3 bytes each
            }
        }
    }

    /// decoded_len returns the maximum length in bytes of the decoded data
    /// corresponding to n bytes of base64-encoded data.
    pub fn decoded_len(&self, n: usize) -> usize {
        match self.pad_char {
            None => {
                // Unpadded data may end with partial block of 2-3 characters.
                n * 6 / 8
            }
            Some(_) => {
                // Padded base64 should always be a multiple of 4 characters in length.
                n / 4 * 3
            }
        }
    }
}

pub struct Encoder<'a> {
    err: Option<std::io::Error>,
    enc: &'a Encoding,
    w: &'a mut dyn std::io::Write,
    buf: [u8; 3],    // buffered data waiting to be encoded
    nbuf: usize,     // number of bytes in buf
    out: [u8; 1024], // output buffer
}

impl<'a> Encoder<'a> {
    /// new returns a new base64 stream encoder. Data written to
    /// the returned writer will be encoded using enc and then written to w.
    /// Base64 encodings operate in 4-byte blocks; when finished
    /// writing, the caller must close the returned encoder to flush any
    /// partially written blocks.
    pub fn new(enc: &'a Encoding, w: &'a mut dyn std::io::Write) -> Self {
        Encoder {
            enc,
            w,
            err: None,
            buf: [0; 3],
            nbuf: 0,
            out: [0; 1024],
        }
    }
}

impl std::io::Write for Encoder<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(err) = &self.err {
            return Err(compat::copy_stdio_error(err));
        }
        let mut n = 0;
        let mut buf = buf;
        // Leading fringe.
        if self.nbuf > 0 {
            let mut i = 0;
            while i < buf.len() && self.nbuf < 3 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }
            n += i;
            buf = &buf[i..];
            if self.nbuf < 3 {
                return Ok(n);
            }
            self.enc.encode(&mut self.out, &self.buf);
            if let Err(err) = self.w.write(&self.out[..4]) {
                let err_copy = compat::copy_stdio_error(&err);
                self.err = Some(err);
                return Err(err_copy);
            };
            self.nbuf = 0;
        }

        // Large interior chunks.
        while buf.len() >= 3 {
            let mut nn = self.out.len() / 4 * 3;
            if nn > buf.len() {
                nn = buf.len();
                nn -= nn % 3;
            }
            self.enc.encode(&mut self.out, &buf[..nn]);
            if let Err(err) = self.w.write_all(&self.out[0..nn / 3 * 4]) {
                let copy = compat::copy_stdio_error(&err);
                self.err = Some(err);
                return Err(copy);
            }
            n += nn;
            buf = &buf[nn..];
        }

        // Trailing fringe.
        compat::copy(&mut self.buf, buf);
        self.nbuf = buf.len();
        n += buf.len();
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // If there's anything left in the buffer, flush it out
        if self.err.is_none() && self.nbuf > 0 {
            self.enc.encode(&mut self.out, &self.buf[..self.nbuf]);
            if let Err(err) = self.w.write(&self.out[..self.enc.encoded_len(self.nbuf)]) {
                self.err = Some(err);
            }
            self.nbuf = 0;
        }
        match &self.err {
            Some(err) => Err(compat::copy_stdio_error(err)),
            None => Ok(()),
        }
    }
}

impl Encoder<'_> {
    /// close flushes any pending output from the encoder.
    /// It is an error to call Write after calling close.
    pub fn close(&mut self) -> std::io::Result<()> {
        self.flush()
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    CorruptInputError(usize),
}

impl Error {
    fn to_stdio_err(&self) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, self.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CorruptInputError(v) => write!(f, "illegal base64 data at input byte {}", v),
        }
    }
}

impl Encoding {
    /// decodeQuantum decodes up to 4 base64 bytes. The received parameters are
    /// the destination buffer dst, the source buffer src and an index in the
    /// source buffer si.
    /// It returns the number of bytes read from src, the number of bytes written
    /// to dst, and an error, if any.
    fn decode_quantum(
        &self,
        dst: &mut [u8],
        src: &[u8],
        si: usize,
    ) -> (usize, usize, Option<Error>) {
        // Decode quantum using the base64 alphabet
        let mut dbuf = [0_u8; 4];
        let mut dlen = 4;

        let mut err = None;

        let mut si = si;

        // 	// Lift the nil check outside of the loop.
        // 	_ = self.decode_map

        let mut j = 0;
        while j < dbuf.len() {
            if src.len() == si {
                if j == 0 {
                    return (si, 0, None);
                } else if j == 1 || self.pad_char.is_some() {
                    return (si, 0, Some(Error::CorruptInputError(si - j)));
                }
                dlen = j;
                break;
            }
            let input = src[si];
            si += 1;

            let output = self.decode_map[input as usize];
            if output != 0xff {
                dbuf[j] = output;
                j += 1;
                continue;
            }

            if input == b'\n' || input == b'\r' {
                continue;
            }

            if self.pad_char.is_none() || input != self.pad_char.unwrap() {
                // 		if rune(input) != self.pad_char {
                // 			return si, 0, CorruptInputError(si - 1)
                return (si, 0, Some(Error::CorruptInputError(si - 1)));
            }

            // We've reached the end and there's padding
            if let 0 | 1 = j {
                // incorrect padding
                return (si, 0, Some(Error::CorruptInputError(si - 1)));
            }
            if j == 2 {
                // "==" is expected, the first "=" is already consumed.
                // skip over newlines
                while si < src.len() && (src[si] == b'\n' || src[si] == b'\r') {
                    si += 1;
                }
                if si == src.len() {
                    // not enough padding
                    return (si, 0, Some(Error::CorruptInputError(src.len())));
                }
                if self.pad_char.is_none() || src[si] != self.pad_char.unwrap() {
                    // incorrect padding
                    return (si, 0, Some(Error::CorruptInputError(si - 1)));
                }
                si += 1;
            };

            // skip over newlines
            while si < src.len() && (src[si] == b'\n' || src[si] == b'\r') {
                si += 1;
            }
            if si < src.len() {
                // trailing garbage
                err = Some(Error::CorruptInputError(si));
            }
            dlen = j;
            break;
        }

        // Convert 4x 6bit source bytes into 3 bytes
        let val = (dbuf[0] as usize) << 18
            | (dbuf[1] as usize) << 12
            | (dbuf[2] as usize) << 6
            | (dbuf[3] as usize);
        (dbuf[2], dbuf[1], dbuf[0]) = (val as u8, (val >> 8) as u8, (val >> 16) as u8);
        if dlen == 4 {
            dst[2] = dbuf[2];
            dbuf[2] = 0;
        }
        if dlen >= 3 {
            dst[1] = dbuf[1];
            if self.strict && dbuf[2] != 0 {
                return (si, 0, Some(Error::CorruptInputError(si - 1)));
            }
            dbuf[1] = 0;
        }
        if dlen >= 2 {
            dst[0] = dbuf[0];
            if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                return (si, 0, Some(Error::CorruptInputError(si - 2)));
            }
        }

        (si, dlen - 1, err)
    }

    /// decode_string returns the bytes represented by the base64 string s.
    pub fn decode_string(&self, s: &str) -> (Vec<u8>, Option<Error>) {
        let bytes = s.as_bytes();
        let mut dbuf = vec![0; self.decoded_len(bytes.len())];
        let (n, err) = self.decode(&mut dbuf, bytes);
        dbuf.truncate(n);
        (dbuf, err)
    }

    /// decode_to_vec decodes src to `Vec<u8>`.
    /// New line characters (\r and \n) are ignored.
    pub fn decode_to_vec(&self, src: &[u8]) -> Result<Vec<u8>, Error> {
        let len = self.decoded_len(src.len());
        let mut dst = vec![0; len];
        let (n, err) = self.decode(&mut dst, src);
        dst.truncate(n);
        match err {
            Some(err) => Err(err),
            None => Ok(dst),
        }
    }

    /// decode decodes src using the encoding enc. It writes at most
    /// decoded_len(src.len()) bytes to dst and returns the number of bytes
    /// written. If src contains invalid base64 data, it will return the
    /// number of bytes successfully written and CorruptInputError.
    /// New line characters (\r and \n) are ignored.
    pub fn decode(&self, dst: &mut [u8], src: &[u8]) -> (usize, Option<Error>) {
        if src.is_empty() {
            return (0, None);
        }

        let mut n = 0;
        let mut err = None;

        // 	// Lift the nil check outside of the loop. enc.decode_map is directly
        // 	// used later in this function, to let the compiler know that the
        // 	// receiver can't be nil.
        // 	_ = enc.decode_map

        let mut si = 0;
        if std::mem::size_of::<isize>() > 8 {
            while src.len() - si >= 8 && dst.len() - n >= 8 {
                let src2 = &src[si..si + 8];
                let (dn, ok) = assemble64(
                    self.decode_map[src2[0] as usize],
                    self.decode_map[src2[1] as usize],
                    self.decode_map[src2[2] as usize],
                    self.decode_map[src2[3] as usize],
                    self.decode_map[src2[4] as usize],
                    self.decode_map[src2[5] as usize],
                    self.decode_map[src2[6] as usize],
                    self.decode_map[src2[7] as usize],
                );
                if ok {
                    BIG_ENDIAN.put_uint64(&mut dst[n..], dn);
                    n += 6;
                    si += 8;
                } else {
                    let ninc: usize;
                    (si, ninc, err) = self.decode_quantum(&mut dst[n..], src, si);
                    n += ninc;
                    if err.is_some() {
                        return (n, err);
                    }
                }
            }
        }

        while src.len() - si >= 4 && dst.len() - n >= 4 {
            let src2 = &src[si..si + 4];
            let (dn, ok) = assemble32(
                self.decode_map[src2[0] as usize],
                self.decode_map[src2[1] as usize],
                self.decode_map[src2[2] as usize],
                self.decode_map[src2[3] as usize],
            );
            if ok {
                BIG_ENDIAN.put_uint32(&mut dst[n..], dn);
                n += 3;
                si += 4;
            } else {
                let ninc: usize;
                (si, ninc, err) = self.decode_quantum(&mut dst[n..], src, si);
                n += ninc;
                if err.is_some() {
                    return (n, err);
                }
            }
        }

        while si < src.len() {
            let ninc: usize;
            (si, ninc, err) = self.decode_quantum(&mut dst[n..], src, si);
            n += ninc;
            if err.is_some() {
                return (n, err);
            }
        }
        (n, err)
    }
}

pub struct Decoder<'a> {
    err: Option<Error>,
    read_err: Option<std::io::Error>, // error from r.Read
    enc: &'a Encoding,
    r: NewlineFilteringReader<'a>,
    buf: [u8; 1024], // leftover input
    nbuf: usize,
    out: std::ops::Range<usize>, // leftover decoded output
    outbuf: [u8; 1024 / 4 * 3],  // decoded output
}

impl<'a> Decoder<'a> {
    /// new constructs a new base64 stream decoder.
    pub fn new(enc: &'a Encoding, r: &'a mut dyn std::io::Read) -> Self {
        Self {
            err: None,
            read_err: None,
            enc,
            r: NewlineFilteringReader { wrapped: r },
            buf: [0; 1024],
            nbuf: 0,
            out: (0..0),
            outbuf: [0; 1024 / 4 * 3],
        }
    }
}

impl std::io::Read for Decoder<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Use leftover decoded output from last read.
        if !self.out.is_empty() {
            let n = compat::copy(buf, &self.outbuf[self.out.start..self.out.end]);
            self.out.start += n;
            return Ok(n);
        }

        if let Some(err) = &self.err {
            return Err(err.to_stdio_err());
        }

        if let Some(err) = &self.read_err {
            return Err(crate::errors::copy_stdio_error(err));
        }

        // This code assumes that self.r strips supported whitespace ('\r' and '\n').

        let mut n;

        // Refill buffer.
        while self.nbuf < 4 && self.read_err.is_none() {
            let mut nn = buf.len() / 3 * 4;
            if nn < 4 {
                nn = 4;
            }
            if nn > self.buf.len() {
                nn = self.buf.len()
            }
            match self.r.read(&mut self.buf[self.nbuf..nn]) {
                Ok(nn) => {
                    if nn > 0 {
                        self.nbuf += nn
                    } else {
                        break;
                    }
                }
                Err(err) => self.read_err = Some(err),
            }
        }

        if self.nbuf < 4 {
            if self.enc.pad_char.is_none() && self.nbuf > 0 {
                // Decode final fragment, without padding.
                let nw: usize;
                (nw, self.err) = self.enc.decode(&mut self.outbuf, &self.buf[..self.nbuf]);
                self.nbuf = 0;
                self.out = 0..nw;
                n = compat::copy(buf, &self.outbuf[self.out.start..self.out.end]);
                self.out.start += n;
                if n > 0 || buf.is_empty() && !self.out.is_empty() {
                    return Ok(n);
                }
                if let Some(err) = &self.err {
                    return Err(err.to_stdio_err());
                }
            }
            if let Some(err) = &self.read_err {
                return Err(crate::errors::copy_stdio_error(err));
            }
        }

        // Decode chunk into buf, or self.out and then buf if buf is too small.
        let nr = self.nbuf / 4 * 4;
        let mut nw = self.nbuf / 4 * 3;
        if nw > buf.len() {
            (nw, self.err) = self.enc.decode(&mut self.outbuf, &self.buf[..nr]);
            self.out = 0..nw;
            n = compat::copy(buf, &self.outbuf[self.out.start..self.out.end]);
            self.out.start += n;
        } else {
            (n, self.err) = self.enc.decode(buf, &self.buf[..nr]);
        }
        self.nbuf -= nr;
        compat::copy_within(&mut self.buf, nr..nr + self.nbuf, 0);
        Ok(n)
    }
}

/// assemble32 assembles 4 base64 digits into 3 bytes.
/// Each digit comes from the decode map, and will be 0xff
/// if it came from an invalid character.
fn assemble32(n1: u8, n2: u8, n3: u8, n4: u8) -> (u32, bool) {
    // Check that all the digits are valid. If any of them was 0xff, their
    // bitwise OR will be 0xff.
    if n1 | n2 | n3 | n4 == 0xff {
        return (0, false);
    }
    (
        (n1 as u32) << 26 | (n2 as u32) << 20 | (n3 as u32) << 14 | (n4 as u32) << 8,
        true,
    )
}

/// assemble64 assembles 8 base64 digits into 6 bytes.
/// Each digit comes from the decode map, and will be 0xff
/// if it came from an invalid character.
#[allow(clippy::too_many_arguments)]
fn assemble64(n1: u8, n2: u8, n3: u8, n4: u8, n5: u8, n6: u8, n7: u8, n8: u8) -> (u64, bool) {
    // Check that all the digits are valid. If any of them was 0xff, their
    // bitwise OR will be 0xff.
    if n1 | n2 | n3 | n4 | n5 | n6 | n7 | n8 == 0xff {
        return (0, false);
    }
    (
        (n1 as u64) << 58
            | (n2 as u64) << 52
            | (n3 as u64) << 46
            | (n4 as u64) << 40
            | (n5 as u64) << 34
            | (n6 as u64) << 28
            | (n7 as u64) << 22
            | (n8 as u64) << 16,
        true,
    )
}

struct NewlineFilteringReader<'a> {
    wrapped: &'a mut dyn std::io::Read,
}

impl std::io::Read for NewlineFilteringReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut n = self.wrapped.read(buf)?;
        while n > 0 {
            let mut offset = 0;
            for i in 0..n {
                let b = buf[i];
                if b != b'\r' && b != b'\n' {
                    if i != offset {
                        buf[offset] = b;
                    }
                    offset += 1;
                }
            }
            if offset > 0 {
                return Ok(offset);
            }
            // Previous buffer entirely whitespace, read again
            n = self.wrapped.read(buf)?;
        }
        Ok(0)
    }
}
