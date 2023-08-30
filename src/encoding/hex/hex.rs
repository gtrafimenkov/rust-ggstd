// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

const HEXTABLE: &[u8; 16] = b"0123456789abcdef";
const REVERSE_HEX_TABLE: &[u8; 256] = b"\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\xff\xff\xff\xff\xff\xff\
		\xff\x0a\x0b\x0c\x0d\x0e\x0f\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\x0a\x0b\x0c\x0d\x0e\x0f\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\
		\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff";

/// encoded_len returns the length of an encoding of n source bytes.
/// Specifically, it returns n * 2.
pub fn encoded_len(n: usize) -> usize {
    return n * 2;
}

// encode encodes src into encoded_len(src.len())
// bytes of dst. As a convenience, it returns the number
// of bytes written to dst, but this value is always encoded_len(src.len()).
// encode implements hexadecimal encoding.
pub fn encode(dst: &mut [u8], src: &[u8]) -> usize {
    let mut j = 0;
    for v in src {
        dst[j] = HEXTABLE[(v >> 4) as usize];
        dst[j + 1] = HEXTABLE[(v & 0x0f) as usize];
        j += 2;
    }
    return src.len() * 2;
}

#[derive(Debug)]
pub enum Error {
    /// ErrLength reports an attempt to decode an odd-length input
    // using Decode or decode_string.
    // The stream-based Decoder returns ggio::Error::ErrUnexpectedEOF instead of ErrLength.
    ErrLength,
    /// InvalidByteError values describe errors resulting from an invalid byte in a hex string.
    InvalidByteError(u8),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ErrLength => write!(f, "{}", "encoding/hex: odd length hex string"),
            Error::InvalidByteError(v) => write!(f, "encoding/hex: invalid byte: {:02x}", v),
        }
    }
}

/// decoded_len returns the length of a decoding of x source bytes.
/// Specifically, it returns x / 2.
pub fn decoded_len(x: usize) -> usize {
    return x / 2;
}

/// decode decodes src into decoded_len(src.len()) bytes,
/// returning the actual number of bytes written to dst.
///
/// decode expects that src contains only hexadecimal
/// characters and that src has even length.
/// If the input is malformed, decode returns the number
/// of bytes decoded before the error.
pub fn decode(dst: &mut [u8], src: &[u8]) -> (usize, Option<Error>) {
    let mut i = 0;
    let mut j = 1;
    while j < src.len() {
        let p = src[j - 1];
        let q = src[j];

        let a = REVERSE_HEX_TABLE[p as usize];
        let b = REVERSE_HEX_TABLE[q as usize];
        if a > 0x0f {
            return (i, Some(Error::InvalidByteError(p)));
        }
        if b > 0x0f {
            return (i, Some(Error::InvalidByteError(q)));
        }
        dst[i] = (a << 4) | b;
        i += 1;
        j += 2
    }
    if src.len() % 2 == 1 {
        // Check for invalid char before reporting bad length,
        // since the invalid char (if present) is an earlier problem.
        if REVERSE_HEX_TABLE[src[j - 1] as usize] > 0x0f {
            return (i, Some(Error::InvalidByteError(src[j - 1])));
        }
        return (i, Some(Error::ErrLength));
    }
    return (i, None);
}

// encode_to_string returns the hexadecimal encoding of src.
pub fn encode_to_string(src: &[u8]) -> String {
    let mut dst = vec![0; encoded_len(src.len())];
    encode(&mut dst, src);
    return String::from_utf8_lossy(&dst).to_string();
}

/// decode_string returns the bytes represented by the hexadecimal string s.
///
/// decode_string expects that src contains only hexadecimal
/// characters and that src has even length.
/// If the input is malformed, decode_string returns
/// the bytes decoded before the error.
pub fn decode_string(s: &str) -> (Vec<u8>, Option<Error>) {
    let sbytes = s.as_bytes();
    let mut buf = vec![0_u8; decoded_len(sbytes.len())];
    let (n, err) = decode(&mut buf, sbytes);
    buf.truncate(n);
    return (buf, err);
}

// // Dump returns a string that contains a hex dump of the given data. The format
// // of the hex dump matches the output of `hexdump -C` on the command line.
// fn Dump(data [u8]) string {
// 	if len(data) == 0 {
// 		return ""
// 	}

// 	var buf strings.Builder
// 	// Dumper will write 79 bytes per complete 16 byte chunk, and at least
// 	// 64 bytes for whatever remains. Round the allocation up, since only a
// 	// maximum of 15 bytes will be wasted.
// 	buf.Grow((1 + ((len(data) - 1) / 16)) * 79)

// 	dumper := Dumper(&buf)
// 	dumper.Write(data)
// 	dumper.Close()
// 	return buf.String()
// }

// // bufferSize is the number of hexadecimal characters to buffer in encoder and decoder.
// const bufferSize = 1024

// type encoder struct {
// 	w   ggio::Writer
// 	err error
// 	out [bufferSize]byte // output buffer
// }

// // NewEncoder returns an ggio::Writer that writes lowercase hexadecimal characters to w.
// fn NewEncoder(w: &mut dyn ggio::Writer) ggio::Writer {
// 	return &encoder{w: w}
// }

// fn (e *encoder) Write(p [u8]) (n usize, err error) {
// 	for len(p) > 0 && e.err == nil {
// 		chunkSize := bufferSize / 2
// 		if len(p) < chunkSize {
// 			chunkSize = len(p)
// 		}

// 		var written usize
// 		encoded := encode(e.out[..], p[..chunkSize])
// 		written, e.err = e.w.write(e.out[..encoded])
// 		n += written / 2
// 		p = p[chunkSize:]
// 	}
// 	return n, e.err
// }

// type decoder struct {
// 	r   io.Reader
// 	err error
// 	in  [u8]           // input buffer (encoded form)
// 	arr [bufferSize]byte // backing array for in
// }

// // NewDecoder returns an io.Reader that decodes hexadecimal characters from r.
// // NewDecoder expects that r contain only an even number of hexadecimal characters.
// fn NewDecoder(r io.Reader) io.Reader {
// 	return &decoder{r: r}
// }

// fn (d *decoder) Read(p [u8]) (n usize, err error) {
// 	// Fill internal buffer with sufficient bytes to decode
// 	if len(d.in) < 2 && d.err == nil {
// 		var numCopy, numRead usize
// 		numCopy = copy(d.arr[..], d.in) // Copies either 0 or 1 bytes
// 		numRead, d.err = d.r.Read(d.arr[numCopy:])
// 		d.in = d.arr[..numCopy+numRead]
// 		if d.err == io.EOF && len(d.in)%2 != 0 {

// 			if a := REVERSE_HEX_TABLE[d.in[len(d.in)-1]]; a > 0x0f {
// 				d.err = InvalidByteError(d.in[len(d.in)-1])
// 			} else {
// 				d.err = ggio::Error::ErrUnexpectedEOF
// 			}
// 		}
// 	}

// 	// Decode internal buffer into output buffer
// 	if numAvail := len(d.in) / 2; len(p) > numAvail {
// 		p = p[..numAvail]
// 	}
// 	numDec, err := Decode(p, d.in[..len(p)*2])
// 	d.in = d.in[2*numDec:]
// 	if err != nil {
// 		d.in, d.err = nil, err // Decode error; discard input remainder
// 	}

// 	if len(d.in) < 2 {
// 		return numDec, d.err // Only expose errors when buffer fully consumed
// 	}
// 	return numDec, nil
// }

// // Dumper returns a WriteCloser that writes a hex dump of all written data to
// // w. The format of the dump matches the output of `hexdump -C` on the command
// // line.
// fn Dumper(w: &mut dyn ggio::Writer) io.WriteCloser {
// 	return &dumper{w: w}
// }

// type dumper struct {
// 	w          ggio::Writer
// 	rightChars [18]byte
// 	buf        [14]byte
// 	used       int  // number of bytes in the current line
// 	n          uint // number of bytes, total
// 	closed     bool
// }

// fn toChar(b byte) byte {
// 	if b < 32 || b > 126 {
// 		return '.'
// 	}
// 	return b
// }

// fn (h *dumper) Write(data [u8]) (n int, err error) {
// 	if h.closed {
// 		return 0, errors.New("encoding/hex: dumper closed")
// 	}

// 	// Output lines look like:
// 	// 00000010  2e 2f 30 31 32 33 34 35  36 37 38 39 3a 3b 3c 3d  |./0123456789:;<=|
// 	// ^ offset                          ^ extra space              ^ ASCII of line.
// 	for i := range data {
// 		if h.used == 0 {
// 			// At the beginning of a line we print the current
// 			// offset in hex.
// 			h.buf[0] = byte(h.n >> 24)
// 			h.buf[1] = byte(h.n >> 16)
// 			h.buf[2] = byte(h.n >> 8)
// 			h.buf[3] = byte(h.n)
// 			encode(h.buf[4:], h.buf[..4])
// 			h.buf[12] = ' '
// 			h.buf[13] = ' '
// 			_, err = h.w.write(h.buf[4:])
// 			if err != nil {
// 				return
// 			}
// 		}
// 		encode(h.buf[..], data[i:i+1])
// 		h.buf[2] = ' '
// 		l := 3
// 		if h.used == 7 {
// 			// There's an additional space after the 8th byte.
// 			h.buf[3] = ' '
// 			l = 4
// 		} else if h.used == 15 {
// 			// At the end of the line there's an extra space and
// 			// the bar for the right column.
// 			h.buf[3] = ' '
// 			h.buf[4] = '|'
// 			l = 5
// 		}
// 		_, err = h.w.write(h.buf[..l])
// 		if err != nil {
// 			return
// 		}
// 		n++
// 		h.rightChars[h.used] = toChar(data[i])
// 		h.used++
// 		h.n++
// 		if h.used == 16 {
// 			h.rightChars[16] = '|'
// 			h.rightChars[17] = '\n'
// 			_, err = h.w.write(h.rightChars[..])
// 			if err != nil {
// 				return
// 			}
// 			h.used = 0
// 		}
// 	}
// 	return
// }

// fn (h *dumper) Close() (err error) {
// 	// See the comments in Write() for the details of this format.
// 	if h.closed {
// 		return
// 	}
// 	h.closed = true
// 	if h.used == 0 {
// 		return
// 	}
// 	h.buf[0] = ' '
// 	h.buf[1] = ' '
// 	h.buf[2] = ' '
// 	h.buf[3] = ' '
// 	h.buf[4] = '|'
// 	nBytes := h.used
// 	for h.used < 16 {
// 		l := 3
// 		if h.used == 7 {
// 			l = 4
// 		} else if h.used == 15 {
// 			l = 5
// 		}
// 		_, err = h.w.write(h.buf[..l])
// 		if err != nil {
// 			return
// 		}
// 		h.used++
// 	}
// 	h.rightChars[nBytes] = '|'
// 	h.rightChars[nBytes+1] = '\n'
// 	_, err = h.w.write(h.rightChars[..nBytes+2])
// 	return
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        // assert_eq!()
        assert_eq!(
            "0001020304050607",
            encode_to_string(&[0, 1, 2, 3, 4, 5, 6, 7])
        );
        assert_eq!(
            "08090a0b0c0d0e0f",
            encode_to_string(&[8, 9, 10, 11, 12, 13, 14, 15])
        );
        assert_eq!(
            "f0f1f2f3f4f5f6f7",
            encode_to_string(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7])
        );
        assert_eq!(
            "f8f9fafbfcfdfeff",
            encode_to_string(&[0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
        );
        assert_eq!("67", encode_to_string(&[b'g']));
        assert_eq!("e3a1", encode_to_string(&[0xe3, 0xa1]));
    }
}
