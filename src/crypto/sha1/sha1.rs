// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;
use crate::encoding::binary::{ByteOrder, BIG_ENDIAN};
use crate::hash::{self, Hash};
use crate::math::bits::rotate_left32;

// fn init() {
// 	crypto.RegisterHash(crypto.SHA1, New)
// }

/// The size of a SHA-1 checksum in bytes.
pub const SIZE: usize = 20;

/// The blocksize of SHA-1 in bytes.
pub const BLOCK_SIZE: usize = 64;

const CHUNK: usize = 64;
const INIT0: u32 = 0x67452301;
const INIT1: u32 = 0xEFCDAB89;
const INIT2: u32 = 0x98BADCFE;
const INIT3: u32 = 0x10325476;
const INIT4: u32 = 0xC3D2E1F0;

/// Digest represents the partial evaluation of a checksum.
#[derive(Copy, Clone)]
pub struct Digest {
    h: [u32; 5],
    x: [u8; CHUNK],
    nx: usize,
    len: u64,
}

// const (
// 	magic         = "sha\x01"
// 	marshaledSize = len(magic) + 5*4 + CHUNK + 8
// )

impl Default for Digest {
    fn default() -> Self {
        Self::new()
    }
}

impl Digest {
    /// new returns a new Digest computing the SHA1 checksum.
    // The Hash also
    // implements encoding.BinaryMarshaler and encoding.BinaryUnmarshaler to
    // marshal and unmarshal the internal state of the hash.
    pub fn new() -> Self {
        // 	if boringEnabled {
        // 		return boringNewSHA1()
        // 	}
        let mut d = Self {
            h: [0; 5],
            x: [0; CHUNK],
            nx: 0,
            len: 0,
        };
        d.reset();
        d
    }

    pub fn write(&mut self, p: &[u8]) {
        // 	boringUnreachable()
        let mut p = p;
        self.len += p.len() as u64;
        if self.nx > 0 {
            let n = compat::copy(&mut self.x[self.nx..], p);
            self.nx += n;
            if self.nx == CHUNK {
                block_generic(&mut self.h, &self.x[..]);
                self.nx = 0
            }
            p = &p[n..];
        }
        if p.len() >= CHUNK {
            let n = p.len() & !(CHUNK - 1);
            block_generic(&mut self.h, &p[..n]);
            p = &p[n..];
        }
        if !p.is_empty() {
            self.nx = compat::copy(&mut self.x, p);
        }
    }

    fn checksum(&mut self) -> [u8; SIZE] {
        let len = self.len;
        // Padding. Add a 1 bit and 0 bits until 56 bytes mod 64.
        let mut tmp = [0; 64 + 8]; // padding + length buffer
        tmp[0] = 0x80_u8;
        let t: u64 = if len % 64 < 56 {
            56 - len % 64
        } else {
            64 + 56 - len % 64
        };

        // Length in bits.
        let len = len << 3;
        let padlen = &mut tmp[0..(t + 8) as usize];
        BIG_ENDIAN.put_uint64(&mut padlen[(t as usize)..], len);
        self.write(padlen);

        if self.nx != 0 {
            panic!("self.nx != 0");
        }

        let mut digest = [0; SIZE];
        BIG_ENDIAN.put_uint32(&mut digest[0..], self.h[0]);
        BIG_ENDIAN.put_uint32(&mut digest[4..], self.h[1]);
        BIG_ENDIAN.put_uint32(&mut digest[8..], self.h[2]);
        BIG_ENDIAN.put_uint32(&mut digest[12..], self.h[3]);
        BIG_ENDIAN.put_uint32(&mut digest[16..], self.h[4]);
        digest
    }

    // // ConstantTimeSum computes the same result of Sum() but in constant time
    // pub fn ConstantTimeSum(&mut self, in &[u8]) &[u8] {
    // 	d0 := *d
    // 	hash := d0.constSum()
    // 	return append(in, hash[..]...)
    // }

    // fn constSum(&mut self) [Size]byte {
    // 	var length [8]byte
    // 	l := self.len << 3
    // 	for i := uint(0); i < 8; i++ {
    // 		length[i] = byte(l >> (56 - 8*i))
    // 	}

    // 	nx := byte(self.nx)
    // 	let t = nx - 56                 // if nx < 56 then the MSB of t is one;
    // 	mask1b := byte(int8(t) >> 7) // mask1b is 0xFF iff one block is enough;

    // 	separator := byte(0x80) // gets reset to 0x00 once used
    // 	for i := byte(0); i < CHUNK; i++ {
    // 		mask := byte(int8(i-nx) >> 7) // 0x00 after the end of data

    // 		// if we reached the end of the data, replace with 0x80 or 0x00
    // 		self.x[i] = (^mask & separator) | (mask & self.x[i])

    // 		// zero the separator once used
    // 		separator &= mask

    // 		if i >= 56 {
    // 			// we might have to write the length here if all fit in one block
    // 			self.x[i] |= mask1b & length[i-56]
    // 		}
    // 	}

    // 	// compress, and only keep the digest if all fit in one block
    // 	block(d, self.x[..])

    // 	var digest [Size]byte
    // 	for i, s := range self.h {
    // 		digest[i*4] = mask1b & byte(s>>24)
    // 		digest[i*4+1] = mask1b & byte(s>>16)
    // 		digest[i*4+2] = mask1b & byte(s>>8)
    // 		digest[i*4+3] = mask1b & byte(s)
    // 	}

    // 	for i := byte(0); i < CHUNK; i++ {
    // 		// second block, it's always past the end of data, might start with 0x80
    // 		if i < 56 {
    // 			self.x[i] = separator
    // 			separator = 0
    // 		} else {
    // 			self.x[i] = length[i-56]
    // 		}
    // 	}

    // 	// compress, and only keep the digest if we actually needed the second block
    // 	block(d, self.x[..])

    // 	for i, s := range self.h {
    // 		digest[i*4] |= ^mask1b & byte(s>>24)
    // 		digest[i*4+1] |= ^mask1b & byte(s>>16)
    // 		digest[i*4+2] |= ^mask1b & byte(s>>8)
    // 		digest[i*4+3] |= ^mask1b & byte(s)
    // 	}

    // 	return digest
    // }

    // fn MarshalBinary(&mut self) (&[u8], error) {
    // 	b := make(&[u8], 0, marshaledSize)
    // 	b = append(b, magic...)
    // 	b = binary.BigEndian.AppendUint32(b, self.h[0])
    // 	b = binary.BigEndian.AppendUint32(b, self.h[1])
    // 	b = binary.BigEndian.AppendUint32(b, self.h[2])
    // 	b = binary.BigEndian.AppendUint32(b, self.h[3])
    // 	b = binary.BigEndian.AppendUint32(b, self.h[4])
    // 	b = append(b, self.x[:self.nx]...)
    // 	b = b[:len(b)+len(self.x)-self.nx] // already zero
    // 	b = binary.BigEndian.AppendUint64(b, self.len)
    // 	return b, nil
    // }

    // fn UnmarshalBinary(&mut self, b &[u8]) error {
    // 	if len(b) < len(magic) || string(b[:len(magic)]) != magic {
    // 		return errors.New("crypto/sha1: invalid hash state identifier")
    // 	}
    // 	if len(b) != marshaledSize {
    // 		return errors.New("crypto/sha1: invalid hash state size")
    // 	}
    // 	b = b[len(magic)..]
    // 	b, self.h[0] = consumeUint32(b)
    // 	b, self.h[1] = consumeUint32(b)
    // 	b, self.h[2] = consumeUint32(b)
    // 	b, self.h[3] = consumeUint32(b)
    // 	b, self.h[4] = consumeUint32(b)
    // 	b = b[compat::copy(self.&x[..], b)..]
    // 	b, self.len = consumeUint64(b)
    // 	self.nx = isize(self.len % CHUNK)
    // 	return nil
    // }

    // fn consumeUint64(b &[u8]) (&[u8], u64) {
    // 	_ = b[7]
    // 	x := u64(b[7]) | u64(b[6])<<8 | u64(b[5])<<16 | u64(b[4])<<24 |
    // 		u64(b[3])<<32 | u64(b[2])<<40 | u64(b[1])<<48 | u64(b[0])<<56
    // 	return b[8..], x
    // }

    // fn consumeUint32(b &[u8]) (&[u8], u32) {
    // 	_ = b[3]
    // 	x := u32(b[3]) | u32(b[2])<<8 | u32(b[1])<<16 | u32(b[0])<<24
    // 	return b[4..], x
    // }
}

impl hash::Hash for Digest {
    fn reset(&mut self) {
        self.h[0] = INIT0;
        self.h[1] = INIT1;
        self.h[2] = INIT2;
        self.h[3] = INIT3;
        self.h[4] = INIT4;
        self.nx = 0;
        self.len = 0;
    }

    fn size(&self) -> usize {
        SIZE
    }

    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

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
        Digest::write(self, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// sum returns the SHA-1 checksum of the data.
pub fn sum(data: &[u8]) -> [u8; SIZE] {
    // 	if boringEnabled {
    // 		return boringSHA1(data)
    // 	}
    let mut d = Digest::new();
    d.write(data);
    d.checksum()
}

const _K0: u32 = 0x5A827999;
const _K1: u32 = 0x6ED9EBA1;
const _K2: u32 = 0x8F1BBCDC;
const _K3: u32 = 0xCA62C1D6;

/// block_generic is a portable, pure Go version of the SHA-1 block step.
// It's used by sha1block_generic.go and tests.
fn block_generic(h: &mut [u32; 5], p: &[u8]) {
    let mut p = p;
    let mut w = [0_u32; 16];

    let (mut h0, mut h1, mut h2, mut h3, mut h4) = (h[0], h[1], h[2], h[3], h[4]);
    while p.len() >= CHUNK {
        // Can interlace the computation of w with the
        // rounds below if needed for speed.
        #[allow(clippy::needless_range_loop)]
        for i in 0..16 {
            let j = i * 4;
            w[i] = (p[j] as u32) << 24
                | (p[j + 1] as u32) << 16
                | (p[j + 2] as u32) << 8
                | (p[j + 3] as u32)
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

        // Each of the four 20-iteration rounds
        // differs only in the computation of f and
        // the choice of K (_K0, _K1, etc).
        for i in 0_usize..16 {
            let f = (b & c) | ((!b) & d);
            let t = rotate_left32(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(w[i & 0xf])
                .wrapping_add(_K0);
            (a, b, c, d, e) = (t, a, rotate_left32(b, 30), c, d);
        }
        for i in 16_usize..20 {
            let tmp = w[(i - 3) & 0xf] ^ w[(i - 8) & 0xf] ^ w[(i - 14) & 0xf] ^ w[(i) & 0xf];
            w[i & 0xf] = (tmp << 1) | (tmp >> (32 - 1));

            let f = (b & c) | ((!b) & d);
            let t = rotate_left32(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(w[i & 0xf])
                .wrapping_add(_K0);
            (a, b, c, d, e) = (t, a, rotate_left32(b, 30), c, d);
        }
        for i in 20_usize..40 {
            let tmp = w[(i - 3) & 0xf] ^ w[(i - 8) & 0xf] ^ w[(i - 14) & 0xf] ^ w[(i) & 0xf];
            w[i & 0xf] = (tmp << 1) | (tmp >> (32 - 1));
            let f = b ^ c ^ d;
            let t = rotate_left32(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(w[i & 0xf])
                .wrapping_add(_K1);
            (a, b, c, d, e) = (t, a, rotate_left32(b, 30), c, d);
        }
        for i in 40_usize..60 {
            let tmp = w[(i - 3) & 0xf] ^ w[(i - 8) & 0xf] ^ w[(i - 14) & 0xf] ^ w[(i) & 0xf];
            w[i & 0xf] = (tmp << 1) | (tmp >> (32 - 1));
            let f = ((b | c) & d) | (b & c);
            let t = rotate_left32(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(w[i & 0xf])
                .wrapping_add(_K2);
            (a, b, c, d, e) = (t, a, rotate_left32(b, 30), c, d);
        }
        for i in 60_usize..80 {
            let tmp = w[(i - 3) & 0xf] ^ w[(i - 8) & 0xf] ^ w[(i - 14) & 0xf] ^ w[(i) & 0xf];
            w[i & 0xf] = (tmp << 1) | (tmp >> (32 - 1));
            let f = b ^ c ^ d;
            let t = rotate_left32(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(w[i & 0xf])
                .wrapping_add(_K3);
            (a, b, c, d, e) = (t, a, rotate_left32(b, 30), c, d);
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);

        p = &p[CHUNK..];
    }

    (h[0], h[1], h[2], h[3], h[4]) = (h0, h1, h2, h3, h4);
}
