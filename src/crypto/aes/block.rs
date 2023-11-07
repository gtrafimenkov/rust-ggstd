// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::consts::{POWX, SBOX0, SBOX1, TD0, TD1, TD2, TD3, TE0, TE1, TE2, TE3};
use crate::encoding::binary::{self, ByteOrder};

// This Go implementation is derived in part from the reference
// ANSI C implementation, which carries the following notice:
//
//	rijndael-alg-fst.c
//
//	@version 3.0 (December 2000)
//
//	Optimised ANSI C code for the Rijndael cipher (now AES)
//
//	@author Vincent Rijmen <vincent.rijmen@esat.kuleuven.ac.be>
//	@author Antoon Bosselaers <antoon.bosselaers@esat.kuleuven.ac.be>
//	@author Paulo Barreto <paulo.barreto@terra.com.br>
//
//	This code is hereby placed in the public domain.
//
//	THIS SOFTWARE IS PROVIDED BY THE AUTHORS ''AS IS'' AND ANY EXPRESS
//	OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
//	WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
//	ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE
//	LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
//	CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
//	SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
//	BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//	WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE
//	OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
//	EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// See FIPS 197 for specification, and see Daemen and Rijmen's Rijndael submission
// for implementation details.
//	https://csrc.nist.gov/csrc/media/publications/fips/197/final/documents/fips-197.pdf
//	https://csrc.nist.gov/archive/aes/rijndael/Rijndael-ammended.pdf

/// encrypt one block from src into dst, using the expanded key xk.
pub fn encrypt_block_go(xk: &[u32], dst: &mut [u8], src: &[u8]) {
    assert!(src.len() >= 16); // bounds check

    let mut s0 = binary::BIG_ENDIAN.uint32(&src[0..4]);
    let mut s1 = binary::BIG_ENDIAN.uint32(&src[4..8]);
    let mut s2 = binary::BIG_ENDIAN.uint32(&src[8..12]);
    let mut s3 = binary::BIG_ENDIAN.uint32(&src[12..16]);

    // First round just XORs input with key.
    s0 ^= xk[0];
    s1 ^= xk[1];
    s2 ^= xk[2];
    s3 ^= xk[3];

    // Middle rounds shuffle using tables.
    // Number of rounds is set by length of expanded key.
    let nr = xk.len() / 4 - 2; // - 2: one above, one more below
    let mut k = 4;
    let mut t0: u32;
    let mut t1: u32;
    let mut t2: u32;
    let mut t3: u32;
    let mut r = 0;
    loop {
        t0 = xk[k]
            ^ TE0[((s0 >> 24) as u8) as usize]
            ^ TE1[((s1 >> 16) as u8) as usize]
            ^ TE2[((s2 >> 8) as u8) as usize]
            ^ TE3[((s3) as u8) as usize];
        t1 = xk[k + 1]
            ^ TE0[((s1 >> 24) as u8) as usize]
            ^ TE1[((s2 >> 16) as u8) as usize]
            ^ TE2[((s3 >> 8) as u8) as usize]
            ^ TE3[((s0) as u8) as usize];
        t2 = xk[k + 2]
            ^ TE0[((s2 >> 24) as u8) as usize]
            ^ TE1[((s3 >> 16) as u8) as usize]
            ^ TE2[((s0 >> 8) as u8) as usize]
            ^ TE3[((s1) as u8) as usize];
        t3 = xk[k + 3]
            ^ TE0[((s3 >> 24) as u8) as usize]
            ^ TE1[((s0 >> 16) as u8) as usize]
            ^ TE2[((s1 >> 8) as u8) as usize]
            ^ TE3[((s2) as u8) as usize];
        k += 4;
        (s0, s1, s2, s3) = (t0, t1, t2, t3);
        r += 1;
        if r == nr {
            break;
        }
    }

    // Last round uses s-box directly and XORs to produce output.
    s0 = (((SBOX0[(t0 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t1 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t2 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t3 & 0xff) as usize] as u32);
    s1 = (((SBOX0[(t1 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t2 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t3 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t0 & 0xff) as usize] as u32);
    s2 = (((SBOX0[(t2 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t3 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t0 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t1 & 0xff) as usize] as u32);
    s3 = (((SBOX0[(t3 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t0 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t1 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t2 & 0xff) as usize] as u32);

    s0 ^= xk[k];
    s1 ^= xk[k + 1];
    s2 ^= xk[k + 2];
    s3 ^= xk[k + 3];

    assert!(dst.len() >= 16); // bounds check
    binary::BIG_ENDIAN.put_uint32(&mut dst[0..4], s0);
    binary::BIG_ENDIAN.put_uint32(&mut dst[4..8], s1);
    binary::BIG_ENDIAN.put_uint32(&mut dst[8..12], s2);
    binary::BIG_ENDIAN.put_uint32(&mut dst[12..16], s3);
}

/// encrypt one block in the buffer
pub fn encrypt_block_inplace_go(xk: &[u32], buffer: &mut [u8]) {
    assert!(buffer.len() >= 16); // bounds check

    let mut s0 = binary::BIG_ENDIAN.uint32(&buffer[0..4]);
    let mut s1 = binary::BIG_ENDIAN.uint32(&buffer[4..8]);
    let mut s2 = binary::BIG_ENDIAN.uint32(&buffer[8..12]);
    let mut s3 = binary::BIG_ENDIAN.uint32(&buffer[12..16]);

    // First round just XORs input with key.
    s0 ^= xk[0];
    s1 ^= xk[1];
    s2 ^= xk[2];
    s3 ^= xk[3];

    // Middle rounds shuffle using tables.
    // Number of rounds is set by length of expanded key.
    let nr = xk.len() / 4 - 2; // - 2: one above, one more below
    let mut k = 4;
    let mut t0: u32;
    let mut t1: u32;
    let mut t2: u32;
    let mut t3: u32;
    let mut r = 0;
    loop {
        t0 = xk[k]
            ^ TE0[((s0 >> 24) as u8) as usize]
            ^ TE1[((s1 >> 16) as u8) as usize]
            ^ TE2[((s2 >> 8) as u8) as usize]
            ^ TE3[((s3) as u8) as usize];
        t1 = xk[k + 1]
            ^ TE0[((s1 >> 24) as u8) as usize]
            ^ TE1[((s2 >> 16) as u8) as usize]
            ^ TE2[((s3 >> 8) as u8) as usize]
            ^ TE3[((s0) as u8) as usize];
        t2 = xk[k + 2]
            ^ TE0[((s2 >> 24) as u8) as usize]
            ^ TE1[((s3 >> 16) as u8) as usize]
            ^ TE2[((s0 >> 8) as u8) as usize]
            ^ TE3[((s1) as u8) as usize];
        t3 = xk[k + 3]
            ^ TE0[((s3 >> 24) as u8) as usize]
            ^ TE1[((s0 >> 16) as u8) as usize]
            ^ TE2[((s1 >> 8) as u8) as usize]
            ^ TE3[((s2) as u8) as usize];
        k += 4;
        (s0, s1, s2, s3) = (t0, t1, t2, t3);
        r += 1;
        if r == nr {
            break;
        }
    }

    // Last round uses s-box directly and XORs to produce output.
    s0 = (((SBOX0[(t0 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t1 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t2 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t3 & 0xff) as usize] as u32);
    s1 = (((SBOX0[(t1 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t2 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t3 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t0 & 0xff) as usize] as u32);
    s2 = (((SBOX0[(t2 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t3 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t0 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t1 & 0xff) as usize] as u32);
    s3 = (((SBOX0[(t3 >> 24) as usize]) as u32) << 24)
        | ((SBOX0[((t0 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX0[((t1 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX0[(t2 & 0xff) as usize] as u32);

    s0 ^= xk[k];
    s1 ^= xk[k + 1];
    s2 ^= xk[k + 2];
    s3 ^= xk[k + 3];

    assert!(buffer.len() >= 16); // bounds check
    binary::BIG_ENDIAN.put_uint32(&mut buffer[0..4], s0);
    binary::BIG_ENDIAN.put_uint32(&mut buffer[4..8], s1);
    binary::BIG_ENDIAN.put_uint32(&mut buffer[8..12], s2);
    binary::BIG_ENDIAN.put_uint32(&mut buffer[12..16], s3);
}

/// decrypt one block from src into dst, using the expanded key xk.
pub fn decrypt_block_go(xk: &[u32], dst: &mut [u8], src: &[u8]) {
    assert!(src.len() >= 16); // bounds check
    let mut s0 = binary::BIG_ENDIAN.uint32(&src[0..4]);
    let mut s1 = binary::BIG_ENDIAN.uint32(&src[4..8]);
    let mut s2 = binary::BIG_ENDIAN.uint32(&src[8..12]);
    let mut s3 = binary::BIG_ENDIAN.uint32(&src[12..16]);

    // First round just XORs input with key.
    s0 ^= xk[0];
    s1 ^= xk[1];
    s2 ^= xk[2];
    s3 ^= xk[3];

    // Middle rounds shuffle using tables.
    // Number of rounds is set by length of expanded key.
    let nr = xk.len() / 4 - 2; // - 2: one above, one more below
    let mut k = 4;
    let mut t0: u32;
    let mut t1: u32;
    let mut t2: u32;
    let mut t3: u32;
    let mut r = 0;
    loop {
        t0 = xk[k]
            ^ TD0[((s0 >> 24) as u8) as usize]
            ^ TD1[((s3 >> 16) as u8) as usize]
            ^ TD2[((s2 >> 8) as u8) as usize]
            ^ TD3[((s1) as u8) as usize];
        t1 = xk[k + 1]
            ^ TD0[((s1 >> 24) as u8) as usize]
            ^ TD1[((s0 >> 16) as u8) as usize]
            ^ TD2[((s3 >> 8) as u8) as usize]
            ^ TD3[((s2) as u8) as usize];
        t2 = xk[k + 2]
            ^ TD0[((s2 >> 24) as u8) as usize]
            ^ TD1[((s1 >> 16) as u8) as usize]
            ^ TD2[((s0 >> 8) as u8) as usize]
            ^ TD3[((s3) as u8) as usize];
        t3 = xk[k + 3]
            ^ TD0[((s3 >> 24) as u8) as usize]
            ^ TD1[((s2 >> 16) as u8) as usize]
            ^ TD2[((s1 >> 8) as u8) as usize]
            ^ TD3[((s0) as u8) as usize];
        k += 4;
        (s0, s1, s2, s3) = (t0, t1, t2, t3);
        r += 1;
        if r == nr {
            break;
        }
    }

    // Last round uses s-box directly and XORs to produce output.
    s0 = (((SBOX1[(t0 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t3 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t2 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t1 & 0xff) as usize] as u32);
    s1 = (((SBOX1[(t1 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t0 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t3 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t2 & 0xff) as usize] as u32);
    s2 = (((SBOX1[(t2 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t1 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t0 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t3 & 0xff) as usize] as u32);
    s3 = (((SBOX1[(t3 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t2 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t1 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t0 & 0xff) as usize] as u32);

    s0 ^= xk[k];
    s1 ^= xk[k + 1];
    s2 ^= xk[k + 2];
    s3 ^= xk[k + 3];

    assert!(dst.len() >= 16); // bounds check
    binary::BIG_ENDIAN.put_uint32(&mut dst[0..4], s0);
    binary::BIG_ENDIAN.put_uint32(&mut dst[4..8], s1);
    binary::BIG_ENDIAN.put_uint32(&mut dst[8..12], s2);
    binary::BIG_ENDIAN.put_uint32(&mut dst[12..16], s3);
}

/// decrypt one block in the buffer, using the expanded key xk.
pub fn decrypt_block_inplace_go(xk: &[u32], buffer: &mut [u8]) {
    assert!(buffer.len() >= 16); // bounds check
    let mut s0 = binary::BIG_ENDIAN.uint32(&buffer[0..4]);
    let mut s1 = binary::BIG_ENDIAN.uint32(&buffer[4..8]);
    let mut s2 = binary::BIG_ENDIAN.uint32(&buffer[8..12]);
    let mut s3 = binary::BIG_ENDIAN.uint32(&buffer[12..16]);

    // First round just XORs input with key.
    s0 ^= xk[0];
    s1 ^= xk[1];
    s2 ^= xk[2];
    s3 ^= xk[3];

    // Middle rounds shuffle using tables.
    // Number of rounds is set by length of expanded key.
    let nr = xk.len() / 4 - 2; // - 2: one above, one more below
    let mut k = 4;
    let mut t0: u32;
    let mut t1: u32;
    let mut t2: u32;
    let mut t3: u32;
    let mut r = 0;
    loop {
        t0 = xk[k]
            ^ TD0[((s0 >> 24) as u8) as usize]
            ^ TD1[((s3 >> 16) as u8) as usize]
            ^ TD2[((s2 >> 8) as u8) as usize]
            ^ TD3[((s1) as u8) as usize];
        t1 = xk[k + 1]
            ^ TD0[((s1 >> 24) as u8) as usize]
            ^ TD1[((s0 >> 16) as u8) as usize]
            ^ TD2[((s3 >> 8) as u8) as usize]
            ^ TD3[((s2) as u8) as usize];
        t2 = xk[k + 2]
            ^ TD0[((s2 >> 24) as u8) as usize]
            ^ TD1[((s1 >> 16) as u8) as usize]
            ^ TD2[((s0 >> 8) as u8) as usize]
            ^ TD3[((s3) as u8) as usize];
        t3 = xk[k + 3]
            ^ TD0[((s3 >> 24) as u8) as usize]
            ^ TD1[((s2 >> 16) as u8) as usize]
            ^ TD2[((s1 >> 8) as u8) as usize]
            ^ TD3[((s0) as u8) as usize];
        k += 4;
        (s0, s1, s2, s3) = (t0, t1, t2, t3);
        r += 1;
        if r == nr {
            break;
        }
    }

    // Last round uses s-box directly and XORs to produce output.
    s0 = (((SBOX1[(t0 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t3 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t2 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t1 & 0xff) as usize] as u32);
    s1 = (((SBOX1[(t1 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t0 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t3 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t2 & 0xff) as usize] as u32);
    s2 = (((SBOX1[(t2 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t1 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t0 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t3 & 0xff) as usize] as u32);
    s3 = (((SBOX1[(t3 >> 24) as usize]) as u32) << 24)
        | ((SBOX1[((t2 >> 16) & 0xff) as usize] as u32) << 16)
        | ((SBOX1[((t1 >> 8) & 0xff) as usize] as u32) << 8)
        | (SBOX1[(t0 & 0xff) as usize] as u32);

    s0 ^= xk[k];
    s1 ^= xk[k + 1];
    s2 ^= xk[k + 2];
    s3 ^= xk[k + 3];

    assert!(buffer.len() >= 16); // bounds check
    binary::BIG_ENDIAN.put_uint32(&mut buffer[0..4], s0);
    binary::BIG_ENDIAN.put_uint32(&mut buffer[4..8], s1);
    binary::BIG_ENDIAN.put_uint32(&mut buffer[8..12], s2);
    binary::BIG_ENDIAN.put_uint32(&mut buffer[12..16], s3);
}

/// Apply sbox0 to each byte in w.
fn subw(w: u32) -> u32 {
    ((SBOX0[(w >> 24) as usize]) as u32) << 24
        | (SBOX0[((w >> 16) & 0xff) as usize] as u32) << 16
        | (SBOX0[((w >> 8) & 0xff) as usize] as u32) << 8
        | ((SBOX0[(w & 0xff) as usize]) as u32)
}

/// Rotate
fn rotw(w: u32) -> u32 {
    (w << 8) | (w >> 24)
}

/// Key expansion algorithm. See FIPS-197, Figure 11.
/// Their rcon[i] is our powx[i-1] << 24.
pub fn expand_key_go(key: &[u8], enc: &mut [u32], dec: &mut [u32]) {
    // Encryption key setup.
    // 	var i int
    let nk = key.len() / 4;
    for i in 0..nk {
        enc[i] = binary::BIG_ENDIAN.uint32(&key[4 * i..]);
    }
    for i in nk..enc.len() {
        let mut t = enc[i - 1];
        if i % nk == 0 {
            t = subw(rotw(t)) ^ ((POWX[i / nk - 1] as u32) << 24);
        } else if nk > 6 && i % nk == 4 {
            t = subw(t);
        }
        enc[i] = enc[i - nk] ^ t;
    }

    // Derive decryption key from encryption key.
    // Reverse the 4-word round key sets from enc to produce dec.
    // All sets but the first and last get the MixColumn transform applied.
    // 	if dec == nil {
    // 		return
    // 	}
    let n = enc.len();
    for i in (0..n).step_by(4) {
        let ei = n - i - 4;
        for j in 0..4 {
            let mut x = enc[ei + j];
            if i > 0 && i + 4 < n {
                x = TD0[SBOX0[(x >> 24) as usize] as usize]
                    ^ TD1[SBOX0[((x >> 16) & 0xff) as usize] as usize]
                    ^ TD2[SBOX0[((x >> 8) & 0xff) as usize] as usize]
                    ^ TD3[SBOX0[(x & 0xff) as usize] as usize];
            }
            dec[i + j] = x;
        }
    }
}
