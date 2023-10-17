// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;
use crate::encoding::binary::{self, ByteOrder};
use crate::hash;
use std::io::Write;

use super::md5block;

// //go:generate go run gen.go -output md5block.go

// fn init() {
// 	crypto.RegisterHash(crypto.MD5, New)
// }

/// The size of an MD5 checksum in bytes.
pub const SIZE: usize = 16;

/// The blocksize of MD5 in bytes.
pub const BLOCK_SIZE: usize = 64;

pub(super) type State = [u32; 4];

const INIT_STATE: State = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];

/// Digest represents the partial evaluation of a MD5 checksum.
#[derive(Copy, Clone)]
pub struct Digest {
    s: State,
    x: [u8; BLOCK_SIZE],
    nx: usize,
    len: u64,
}

impl Default for Digest {
    fn default() -> Self {
        Self::new()
    }
}

impl Digest {
    /// new returns a new Digest for computing the MD5 checksum.
    // The Hash also
    // implements encoding.BinaryMarshaler and encoding.BinaryUnmarshaler to
    // marshal and unmarshal the internal state of the hash.
    pub fn new() -> Self {
        Self {
            s: INIT_STATE,
            x: [0; BLOCK_SIZE],
            nx: 0,
            len: 0,
        }
    }

    fn checksum(&mut self) -> [u8; SIZE] {
        // Append 0x80 to the end of the message and then append zeros
        // until the length is a multiple of 56 bytes. Finally append
        // 8 bytes representing the message length in bits.
        //
        // 1 byte end marker :: 0-63 padding bytes :: 8 byte length
        let mut tmp = [0_u8; 1 + 63 + 8];
        tmp[0] = 0x80;
        let pad = ((55_u64.wrapping_sub(self.len)) % 64) as usize; // calculate number of padding bytes
        binary::LITTLE_ENDIAN.put_uint64(&mut tmp[1 + pad..], self.len << 3); // append length in bits
        _ = self.write(&tmp[..(1 + pad + 8)]);

        // The previous write ensures that a whole number of
        // blocks (i.e. a multiple of 64 bytes) have been hashed.
        if self.nx != 0 {
            panic!("self.nx != 0");
        }

        let mut digest = [0_u8; SIZE];
        binary::LITTLE_ENDIAN.put_uint32(&mut digest[0..], self.s[0]);
        binary::LITTLE_ENDIAN.put_uint32(&mut digest[4..], self.s[1]);
        binary::LITTLE_ENDIAN.put_uint32(&mut digest[8..], self.s[2]);
        binary::LITTLE_ENDIAN.put_uint32(&mut digest[12..], self.s[3]);
        digest
    }
}

// const (
// 	magic         = "md5\x01"
// 	marshaledSize = len(magic) + 4*4 + BlockSize + 8
// )

// fn (d *digest) MarshalBinary() ([]byte, error) {
// 	b := make([]byte, 0, marshaledSize)
// 	b = append(b, magic...)
// 	b = binary.BigEndian.AppendUint32(b, d.s[0])
// 	b = binary.BigEndian.AppendUint32(b, d.s[1])
// 	b = binary.BigEndian.AppendUint32(b, d.s[2])
// 	b = binary.BigEndian.AppendUint32(b, d.s[3])
// 	b = append(b, d.x[..d.nx]...)
// 	b = b[..len(b)+len(d.x)-d.nx] // already zero
// 	b = binary.BigEndian.AppendUint64(b, d.len)
// 	return b, nil
// }

// fn (d *digest) UnmarshalBinary(b []byte) error {
// 	if len(b) < len(magic) || string(b[..len(magic)]) != magic {
// 		return errors.New("crypto/md5: invalid hash state identifier")
// 	}
// 	if len(b) != marshaledSize {
// 		return errors.New("crypto/md5: invalid hash state size")
// 	}
// 	b = b[len(magic)..]
// 	b, d.s[0] = consumeUint32(b)
// 	b, d.s[1] = consumeUint32(b)
// 	b, d.s[2] = consumeUint32(b)
// 	b, d.s[3] = consumeUint32(b)
// 	b = b[copy(d.x[..], b)..]
// 	b, d.len = consumeUint64(b)
// 	d.nx = isize(d.len % BlockSize)
// 	return nil
// }

// fn consumeUint64(b []byte) ([]byte, u64) {
// 	return b[8..], binary.BigEndian.Uint64(b[0:8])
// }

// fn consumeUint32(b []byte) ([]byte, u32) {
// 	return b[4..], binary.BigEndian.Uint32(b[0:4])
// }

impl hash::Hash for Digest {
    fn reset(&mut self) {
        self.s = INIT_STATE;
        self.nx = 0;
        self.len = 0;
    }

    fn size(&self) -> usize {
        SIZE
    }

    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    /// Sum appends the current hash to b and returns the resulting slice.
    /// It does not change the underlying hash state.
    fn sum(&self, b: &[u8]) -> Vec<u8> {
        // Make a copy of d so that caller can keep writing and summing.
        let mut d0 = *self;
        let hash = d0.checksum();
        let mut res = b.to_vec();
        res.extend_from_slice(&hash);
        res
    }
}

impl std::io::Write for Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // fn (d *digest) Write(p []byte) (nn isize, err error) {
        // Note that we currently call block or blockGeneric
        // directly (guarded using haveAsm) because this allows
        // escape analysis to see that p and d don't escape.

        let nn = buf.len();
        let mut buf = buf;
        self.len += nn as u64;
        if self.nx > 0 {
            let n = compat::copy(&mut self.x[self.nx..], buf);
            self.nx += n;
            if self.nx == BLOCK_SIZE {
                // 			if haveAsm {
                // 				block(d, self.x[..])
                // 			} else {
                md5block::block_generic(&mut self.s, &self.x);
                // 			}
                self.nx = 0
            }
            buf = &buf[n..];
        }
        if buf.len() >= BLOCK_SIZE {
            let n = buf.len() & !(BLOCK_SIZE - 1);
            // 		if haveAsm {
            // 			block(d, buf[..n])
            // 		} else {
            md5block::block_generic(&mut self.s, &buf[..n]);
            // 		}
            buf = &buf[n..];
        }
        if !buf.is_empty() {
            self.nx = compat::copy(&mut self.x, buf);
        }
        Ok(nn)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// sum returns the MD5 checksum of the data.
pub fn sum(data: &[u8]) -> [u8; SIZE] {
    let mut d = Digest::new();
    _ = d.write(data);
    d.checksum()
}
