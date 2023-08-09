// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// SHA256 block step.
// In its own file so that a faster assembly or C version
// can be substituted easily.

use super::sha256::CHUNK;
use crate::math::bits::rotate_left32;

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn block_generic(h: &mut [u32; 8], p: &[u8]) {
    let mut w = [0; 64];
    let (mut h0, mut h1, mut h2, mut h3, mut h4, mut h5, mut h6, mut h7) =
        (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
    let mut p = p;
    while p.len() >= CHUNK {
        // Can interlace the computation of w with the
        // rounds below if needed for speed.
        for i in 0..16 {
            let j = i * 4;
            w[i] = (p[j] as u32) << 24
                | (p[j + 1] as u32) << 16
                | (p[j + 2] as u32) << 8
                | (p[j + 3] as u32);
        }
        for i in 16..64 {
            let v1 = w[i - 2];
            let t1 = (rotate_left32(v1, -17)) ^ (rotate_left32(v1, -19)) ^ (v1 >> 10);
            let v2 = w[i - 15];
            let t2 = (rotate_left32(v2, -7)) ^ (rotate_left32(v2, -18)) ^ (v2 >> 3);
            w[i] = t1
                .wrapping_add(w[i - 7])
                .wrapping_add(t2)
                .wrapping_add(w[i - 16]);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) =
            (h0, h1, h2, h3, h4, h5, h6, h7);

        for i in 0..64 {
            let t1 = h
                .wrapping_add(
                    (rotate_left32(e, -6)) ^ (rotate_left32(e, -11)) ^ (rotate_left32(e, -25)),
                )
                .wrapping_add((e & f) ^ (!e & g))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);

            let t2 = ((rotate_left32(a, -2)) ^ (rotate_left32(a, -13)) ^ (rotate_left32(a, -22)))
                .wrapping_add((a & b) ^ (a & c) ^ (b & c));

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);

        p = &p[CHUNK..];
    }

    (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]) = (h0, h1, h2, h3, h4, h5, h6, h7);
}
