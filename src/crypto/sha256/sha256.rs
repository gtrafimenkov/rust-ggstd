// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// import (
// 	"crypto"
// 	"crypto/internal/boring"
// 	"encoding/binary"
// 	"errors"
// 	"hash"
// )

use super::sha256block::block_generic;
use crate::encoding::binary::{ByteOrder, BIG_ENDIAN};
use crate::hash::{self, Hash};
use std::io::Write;

// fn init() {
// 	crypto.RegisterHash(crypto.SHA224, New224)
// 	crypto.RegisterHash(crypto.SHA256, New)
// }

/// The size of a SHA256 checksum in bytes.
pub const SIZE: usize = 32;

/// The size of a SHA224 checksum in bytes.
pub const SIZE224: usize = 28;

/// The blocksize of SHA256 and SHA224 in bytes.
pub const BLOCK_SIZE: usize = 64;

pub const CHUNK: usize = 64;
const INIT0: u32 = 0x6A09E667;
const INIT1: u32 = 0xBB67AE85;
const INIT2: u32 = 0x3C6EF372;
const INIT3: u32 = 0xA54FF53A;
const INIT4: u32 = 0x510E527F;
const INIT5: u32 = 0x9B05688C;
const INIT6: u32 = 0x1F83D9AB;
const INIT7: u32 = 0x5BE0CD19;
const INIT0_224: u32 = 0xC1059ED8;
const INIT1_224: u32 = 0x367CD507;
const INIT2_224: u32 = 0x3070DD17;
const INIT3_224: u32 = 0xF70E5939;
const INIT4_224: u32 = 0xFFC00B31;
const INIT5_224: u32 = 0x68581511;
const INIT6_224: u32 = 0x64F98FA7;
const INIT7_224: u32 = 0xBEFA4FA4;

/// digest represents the partial evaluation of a checksum.
#[derive(Copy, Clone)]
struct Digest {
    h: [u32; 8],
    x: [u8; CHUNK],
    nx: usize,
    len: u64,
    is224: bool, // mark if this digest is SHA-224
}

impl Digest {
    fn new() -> Self {
        Self {
            h: [0; 8],
            x: [0; CHUNK],
            nx: 0,
            len: 0,
            is224: false,
        }
    }
}

// const (
// 	magic224      = "sha\x02"
// 	magic256      = "sha\x03"
// 	marshaledSize = len(magic256) + 8*4 + chunk + 8
// )

// fn (d *Digest) MarshalBinary() ([]u8, error) {
// 	b := make([]u8, 0, marshaledSize)
// 	if d.is224 {
// 		b = append(b, magic224...)
// 	} else {
// 		b = append(b, magic256...)
// 	}
// 	b = binary::BigEndian.AppendUint32(b, d.h[0])
// 	b = binary::BigEndian.AppendUint32(b, d.h[1])
// 	b = binary::BigEndian.AppendUint32(b, d.h[2])
// 	b = binary::BigEndian.AppendUint32(b, d.h[3])
// 	b = binary::BigEndian.AppendUint32(b, d.h[4])
// 	b = binary::BigEndian.AppendUint32(b, d.h[5])
// 	b = binary::BigEndian.AppendUint32(b, d.h[6])
// 	b = binary::BigEndian.AppendUint32(b, d.h[7])
// 	b = append(b, d.x[..d.nx]...)
// 	b = b[..b.len()+len(d.x)-d.nx] // already zero
// 	b = binary::BigEndian.AppendUint64(b, d.len)
// 	return b, nil
// }

// fn (d *Digest) UnmarshalBinary(b []u8) error {
// 	if b.len() < len(magic224) || (d.is224 && string(b[..len(magic224)]) != magic224) || (!d.is224 && string(b[..len(magic256)]) != magic256) {
// 		return errors.New("crypto/sha256: invalid hash state identifier")
// 	}
// 	if b.len() != marshaledSize {
// 		return errors.New("crypto/sha256: invalid hash state size")
// 	}
// 	b = b[len(magic224):]
// 	b, d.h[0] = consumeUint32(b)
// 	b, d.h[1] = consumeUint32(b)
// 	b, d.h[2] = consumeUint32(b)
// 	b, d.h[3] = consumeUint32(b)
// 	b, d.h[4] = consumeUint32(b)
// 	b, d.h[5] = consumeUint32(b)
// 	b, d.h[6] = consumeUint32(b)
// 	b, d.h[7] = consumeUint32(b)
// 	b = b[copy(d.x[..], b):]
// 	b, d.len = consumeUint64(b)
// 	d.nx = isize(d.len % chunk)
// 	return nil
// }

// fn consumeUint64(b []u8) ([]u8, u64) {
// 	_ = b[7]
// 	x := u64(b[7]) | u64(b[6])<<8 | u64(b[5])<<16 | u64(b[4])<<24 |
// 		u64(b[3])<<32 | u64(b[2])<<40 | u64(b[1])<<48 | u64(b[0])<<56
// 	return b[8:], x
// }

// fn consumeUint32(b []u8) ([u32; ]u8, ) {
// 	_ = b[3]
// 	xu32;  := (b[3])u32;  | (b[2])<<8u3,2;  | (b[1])<<16u32;  | (b[0])<<24
// 	return b[4:], x
// },

/// new returns a new hash.Hash computing the SHA256 checksum. The Hash
/// also implements encoding.BinaryMarshaler and
/// encoding.BinaryUnmarshaler to marshal and unmarshal the internal
/// state of the hash.
pub fn new() -> impl hash::Hash {
    // if boring.Enabled {
    // 	return boring.NewSHA256()
    // }
    let mut d = Digest::new();
    d.reset();
    return d;
}

/// New224 returns a new hash.Hash computing the SHA224 checksum.
// fn New224() hash.Hash {
// 	if boring.Enabled {
// 		return boring.NewSHA224()
// 	}
// 	d := new(digest)
// 	d.is224 = true
// 	d.Reset()
// 	return d
// }

impl hash::Hash for Digest {
    fn reset(&mut self) {
        if !self.is224 {
            self.h[0] = INIT0;
            self.h[1] = INIT1;
            self.h[2] = INIT2;
            self.h[3] = INIT3;
            self.h[4] = INIT4;
            self.h[5] = INIT5;
            self.h[6] = INIT6;
            self.h[7] = INIT7;
        } else {
            self.h[0] = INIT0_224;
            self.h[1] = INIT1_224;
            self.h[2] = INIT2_224;
            self.h[3] = INIT3_224;
            self.h[4] = INIT4_224;
            self.h[5] = INIT5_224;
            self.h[6] = INIT6_224;
            self.h[7] = INIT7_224;
        }
        self.nx = 0;
        self.len = 0;
    }

    fn size(&self) -> usize {
        if !self.is224 {
            SIZE
        } else {
            SIZE224
        }
    }

    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    fn sum(&self, b: &[u8]) -> Vec<u8> {
        // Make a copy of d so that caller can keep writing and summing.
        let mut d0 = self.clone();
        let hash = d0.checksum();
        let mut res = b.to_vec();
        if d0.is224 {
            res.extend_from_slice(&hash[..SIZE224]);
        } else {
            res.extend_from_slice(&hash);
        }
        res
    }
}

impl Digest {
    /// Fills the internal buffer from the slice, returns number of bytes copied.
    fn fill_buf(&mut self, p: &[u8]) -> usize {
        let free_buf_space = CHUNK - self.nx;
        let bytes_to_copy = free_buf_space.min(p.len());
        if bytes_to_copy > 0 {
            self.x[self.nx..(self.nx + bytes_to_copy)].copy_from_slice(&p[..bytes_to_copy]);
        }
        bytes_to_copy
    }
}

impl std::io::Write for Digest {
    fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        // boring.Unreachable()
        let mut p = p;
        let nn = p.len();
        self.len += nn as u64;
        if self.nx > 0 {
            let n = self.fill_buf(p);
            self.nx += n;
            if self.nx == CHUNK {
                block_generic(&mut self.h, &self.x);
                self.nx = 0;
            }
            p = &p[n..];
        }
        if p.len() >= CHUNK {
            let n = p.len() & !(CHUNK - 1);
            block_generic(&mut self.h, &p[..n]);
            p = &p[n..];
        }
        if p.len() > 0 {
            self.nx = self.fill_buf(p);
        }
        Ok(nn)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Digest {
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
        self.write(padlen).expect("write error is not expected");

        if self.nx != 0 {
            panic!("self.nx != 0");
        }

        let mut digest = [0; SIZE];

        BIG_ENDIAN.put_uint32(&mut digest[0..], self.h[0]);
        BIG_ENDIAN.put_uint32(&mut digest[4..], self.h[1]);
        BIG_ENDIAN.put_uint32(&mut digest[8..], self.h[2]);
        BIG_ENDIAN.put_uint32(&mut digest[12..], self.h[3]);
        BIG_ENDIAN.put_uint32(&mut digest[16..], self.h[4]);
        BIG_ENDIAN.put_uint32(&mut digest[20..], self.h[5]);
        BIG_ENDIAN.put_uint32(&mut digest[24..], self.h[6]);
        if !self.is224 {
            BIG_ENDIAN.put_uint32(&mut digest[28..], self.h[7]);
        }

        digest
    }
}

/// sum256 returns the SHA256 checksum of the data.
pub fn sum256(data: &[u8]) -> [u8; SIZE] {
    // if boring.Enabled {
    // 	return boring.SHA256(data)
    // }
    let mut d = Digest::new();
    d.reset();
    d.write(data).expect("write error is not expected");
    d.checksum()
}

// Sum224 returns the SHA224 checksum of the data.
// fn Sum224(data []u8) [Size224]u8 {
// 	if boring.Enabled {
// 		return boring.SHA224(data)
// 	}
// 	var d digest
// 	d.is224 = true
// 	d.Reset()
// 	d.Write(data)
// 	sum := d.checksum()
// 	ap := (*[Size224]u8)(sum[..])
// 	return *ap
// }

#[cfg(test)]
mod tests {

    use super::*;
    use crate::encoding::hex;

    #[test]
    fn test_sha256() {
        assert_eq!(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            hex::encode_to_string(&sum256("".as_bytes()))
        );
        assert_eq!(
            "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb",
            hex::encode_to_string(&sum256("a".as_bytes()))
        );
        assert_eq!(
            "fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603",
            hex::encode_to_string(&sum256("ab".as_bytes()))
        );
        assert_eq!(
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            hex::encode_to_string(&sum256("abc".as_bytes()))
        );
        assert_eq!(
            "a948904f2f0f479b8f8197694b30184b0d2ed1c1cd2a1ec0fb85d299a192a447",
            hex::encode_to_string(&sum256("hello world\n".as_bytes()))
        );
    }
}
