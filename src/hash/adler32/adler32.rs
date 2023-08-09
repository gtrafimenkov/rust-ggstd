// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::hash;
use crate::hash::Hash;
use crate::hash::Hash32;

/// MODULO is the largest prime that is less than 65536.
pub(super) const MODULO: u32 = 65521;

/// NMAX is the largest n such that
/// 255 * n * (n+1) / 2 + (n+1) * (mod-1) <= 2^32-1.
/// It is mentioned in RFC 1950 (search for "5552").
const NMAX: usize = 5552;

/// The size of an Adler-32 checksum in bytes.
const SIZE: usize = 4;

/// Digest represents the partial evaluation of a checksum.
/// The low 16 bits are s1, the high 16 bits are s2.
struct Digest {
    hash: u32,
}

/// New returns a new hash.Hash32 computing the Adler-32 checksum. Its
/// Sum method will lay the value out in big-endian byte order.
pub fn new() -> impl hash::Hash32 {
    Digest::new()
}

impl hash::Hash for Digest {
    fn reset(&mut self) {
        self.hash = 1
    }

    // size returns the number of bytes Sum will return.
    fn size(&self) -> usize {
        SIZE
    }

    // block_size returns the hash's underlying block size.
    // The Write method must be able to accept any amount
    // of data, but it may operate more efficiently if all writes
    // are a multiple of the block size.
    fn block_size(&self) -> usize {
        4
    }
}

impl std::io::Write for Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Digest {
    pub fn new() -> Self {
        let mut d = Digest { hash: 0 };
        d.reset();
        d
    }

    // Add p to the running checksum d.
    fn update(&mut self, p: &[u8]) {
        let mut s1 = self.hash & 0xffff;
        let mut s2 = self.hash >> 16;
        let mut p = p;
        while p.len() > 0 {
            let mut q: &[u8] = &[];
            if p.len() > NMAX {
                q = &p[NMAX..];
                p = &p[0..NMAX];
            }
            while p.len() >= 4 {
                s1 += p[0] as u32;
                s2 += s1;
                s1 += p[1] as u32;
                s2 += s1;
                s1 += p[2] as u32;
                s2 += s1;
                s1 += p[3] as u32;
                s2 += s1;
                p = &p[4..];
            }
            for x in p {
                s1 += *x as u32;
                s2 += s1;
            }
            s1 %= MODULO;
            s2 %= MODULO;
            p = q
        }
        self.hash = s2 << 16 | s1;
    }
}

impl Hash32 for Digest {
    fn sum32(&self) -> u32 {
        self.hash
    }

    // fn Sum(&mut self, in []byte) []byte {
    // 	s := uint32(*d)
    // 	return append(in, byte(s>>24), byte(s>>16), byte(s>>8), byte(s))
    // }
}

/// Checksum returns the Adler-32 checksum of data.
pub fn checksum(data: &[u8]) -> u32 {
    let mut d = Digest::new();
    d.update(data);
    d.hash
}
