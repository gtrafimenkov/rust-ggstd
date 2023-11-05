// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! CTR AES test vectors.
//!
//! See U.S. National Institute of Standards and Technology (NIST)
//! Special Publication 800-38A, ``Recommendation for Block Cipher
//! Modes of Operation,'' 2001 Edition, pp. 55-58.

use super::common_test::{COMMON_INPUT, COMMON_KEY128, COMMON_KEY192, COMMON_KEY256};
use crate::crypto::aes;
use crate::crypto::cipher::{self, Stream};

const COMMON_COUNTER: &[u8] = &[
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
];

struct TestCase {
    name: &'static str,
    key: &'static [u8],
    iv: &'static [u8],
    input: &'static [u8],
    out: &'static [u8],
}

impl TestCase {
    const fn new(
        name: &'static str,
        key: &'static [u8],
        iv: &'static [u8],
        input: &'static [u8],
        out: &'static [u8],
    ) -> Self {
        Self {
            name,
            key,
            iv,
            input,
            out,
        }
    }
}

const CTR_AES_TESTS: &[TestCase] = &[
    // NIST SP 800-38A pp 55-58
    TestCase::new(
        "CTR-AES128",
        COMMON_KEY128,
        COMMON_COUNTER,
        COMMON_INPUT,
        &[
            0x87, 0x4d, 0x61, 0x91, 0xb6, 0x20, 0xe3, 0x26, 0x1b, 0xef, 0x68, 0x64, 0x99, 0x0d,
            0xb6, 0xce, 0x98, 0x06, 0xf6, 0x6b, 0x79, 0x70, 0xfd, 0xff, 0x86, 0x17, 0x18, 0x7b,
            0xb9, 0xff, 0xfd, 0xff, 0x5a, 0xe4, 0xdf, 0x3e, 0xdb, 0xd5, 0xd3, 0x5e, 0x5b, 0x4f,
            0x09, 0x02, 0x0d, 0xb0, 0x3e, 0xab, 0x1e, 0x03, 0x1d, 0xda, 0x2f, 0xbe, 0x03, 0xd1,
            0x79, 0x21, 0x70, 0xa0, 0xf3, 0x00, 0x9c, 0xee,
        ],
    ),
    TestCase::new(
        "CTR-AES192",
        COMMON_KEY192,
        COMMON_COUNTER,
        COMMON_INPUT,
        &[
            0x1a, 0xbc, 0x93, 0x24, 0x17, 0x52, 0x1c, 0xa2, 0x4f, 0x2b, 0x04, 0x59, 0xfe, 0x7e,
            0x6e, 0x0b, 0x09, 0x03, 0x39, 0xec, 0x0a, 0xa6, 0xfa, 0xef, 0xd5, 0xcc, 0xc2, 0xc6,
            0xf4, 0xce, 0x8e, 0x94, 0x1e, 0x36, 0xb2, 0x6b, 0xd1, 0xeb, 0xc6, 0x70, 0xd1, 0xbd,
            0x1d, 0x66, 0x56, 0x20, 0xab, 0xf7, 0x4f, 0x78, 0xa7, 0xf6, 0xd2, 0x98, 0x09, 0x58,
            0x5a, 0x97, 0xda, 0xec, 0x58, 0xc6, 0xb0, 0x50,
        ],
    ),
    TestCase::new(
        "CTR-AES256",
        COMMON_KEY256,
        COMMON_COUNTER,
        COMMON_INPUT,
        &[
            0x60, 0x1e, 0xc3, 0x13, 0x77, 0x57, 0x89, 0xa5, 0xb7, 0xa7, 0xf5, 0x04, 0xbb, 0xf3,
            0xd2, 0x28, 0xf4, 0x43, 0xe3, 0xca, 0x4d, 0x62, 0xb5, 0x9a, 0xca, 0x84, 0xe9, 0x90,
            0xca, 0xca, 0xf5, 0xc5, 0x2b, 0x09, 0x30, 0xda, 0xa2, 0x3d, 0xe9, 0x4c, 0xe8, 0x70,
            0x17, 0xba, 0x2d, 0x84, 0x98, 0x8d, 0xdf, 0xc9, 0xc5, 0x8d, 0xb6, 0x7a, 0xad, 0xa6,
            0x13, 0xc2, 0xdd, 0x08, 0x45, 0x79, 0x41, 0xa6,
        ],
    ),
];

#[test]
fn test_ctr_aes() {
    for tt in CTR_AES_TESTS {
        let test = tt.name;

        // testing encryption with input data size multiple of the block size and not multiple
        let c = aes::Cipher::new(tt.key).unwrap();
        for j in (0..=5).step_by(5) {
            let input = &tt.input[0..tt.input.len() - j];
            let mut ctr = cipher::CTR::new(&c, tt.iv);
            let mut encrypted = vec![0; input.len()];
            ctr.xor_key_stream(&mut encrypted, input);
            let out = &tt.out[0..input.len()];
            assert_eq!(
                out,
                &encrypted,
                "{}/{}: CTR\ninpt {:?}\nhave {:?}\nwant {:?}",
                test,
                input.len(),
                input,
                encrypted,
                out
            );
        }

        // testing encryption inplace
        {
            let c = aes::Cipher::new(tt.key).unwrap();
            let mut ctr = cipher::CTR::new(&c, tt.iv);
            let mut buffer = tt.input.to_vec();
            ctr.xor_key_stream_inplace(&mut buffer);
            assert_eq!(&buffer, tt.out);
        }

        // testing decryption with input data size multiple of the block size and not multiple
        for j in (0..=7).step_by(7) {
            let input = &tt.out[0..tt.out.len() - j];
            let mut ctr = cipher::CTR::new(&c, tt.iv);
            let mut plain = vec![0; input.len()];
            ctr.xor_key_stream(&mut plain, input);
            let out = &tt.input[0..input.len()];
            assert_eq!(
                out,
                plain,
                "{}/{}: CTRReader\nhave {:?}\nwant {:?}",
                test,
                out.len(),
                plain,
                out
            );
        }

        // testing decryption inplace
        {
            let c = aes::Cipher::new(tt.key).unwrap();
            let mut ctr = cipher::CTR::new(&c, tt.iv);
            let mut buffer = tt.out.to_vec();
            ctr.xor_key_stream_inplace(&mut buffer);
            assert_eq!(&buffer, tt.input);
        }
    }
}
