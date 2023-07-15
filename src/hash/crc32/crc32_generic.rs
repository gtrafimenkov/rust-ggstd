// Copyright (c) 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! This file contains CRC32 algorithms that are not specific to any architecture
//! and don't use hardware acceleration.
//!
//! The simple (and slow) CRC32 implementation only uses a 256*4 bytes table.
//!
//! The slicing-by-8 algorithm is a faster implementation that uses a bigger
//! table (8*256*4 bytes).

use crate::hash::crc32;

// simple_make_table allocates and constructs a Table for the specified
// polynomial. The table is suitable for use with the simple algorithm
// (simple_update).
#[allow(dead_code)]
pub fn simple_make_table(poly: u32) -> crc32::Table {
    let mut t: crc32::Table = [0; 256];
    simple_populate_table(poly, &mut t);
    t
}

/// simple_populate_table constructs a Table for the specified polynomial, suitable
/// for use with simple_update.
pub fn simple_populate_table(poly: u32, t: &mut crc32::Table) {
    for i in 0..256 {
        let mut crc = i as u32;
        for _j in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ poly
            } else {
                crc >>= 1
            }
        }
        t[i] = crc
    }
}

// simple_update uses the simple algorithm to update the CRC, given a table that
// was previously computed using simple_make_table.
pub fn simple_update(crc: u32, tab: &crc32::Table, p: &[u8]) -> u32 {
    let mut crc = !crc;
    for v in p {
        crc = tab[((crc as u8) ^ v) as usize] ^ (crc >> 8)
    }
    !crc
}

// // Use slicing-by-8 when payload >= this value.
// const slicing8Cutoff = 16

// // slicing8Table is array of 8 Tables, used by the slicing-by-8 algorithm.
// type slicing8Table [8]Table

// // slicingMakeTable constructs a slicing8Table for the specified polynomial. The
// // table is suitable for use with the slicing-by-8 algorithm (slicingUpdate).
// func slicingMakeTable(poly uint32) *slicing8Table {
// 	t := new(slicing8Table)
// 	simple_populate_table(poly, &t[0])
// 	for i := 0; i < 256; i++ {
// 		crc := t[0][i]
// 		for j := 1; j < 8; j++ {
// 			crc = t[0][crc&0xFF] ^ (crc >> 8)
// 			t[j][i] = crc
// 		}
// 	}
// 	return t
// }

// // slicingUpdate uses the slicing-by-8 algorithm to update the CRC, given a
// // table that was previously computed using slicingMakeTable.
// func slicingUpdate(crc uint32, tab *slicing8Table, p []byte) uint32 {
// 	if len(p) >= slicing8Cutoff {
// 		crc = ^crc
// 		for len(p) > 8 {
// 			crc ^= uint32(p[0]) | uint32(p[1])<<8 | uint32(p[2])<<16 | uint32(p[3])<<24
// 			crc = tab[0][p[7]] ^ tab[1][p[6]] ^ tab[2][p[5]] ^ tab[3][p[4]] ^
// 				tab[4][crc>>24] ^ tab[5][(crc>>16)&0xFF] ^
// 				tab[6][(crc>>8)&0xFF] ^ tab[7][crc&0xFF]
// 			p = p[8:]
// 		}
// 		crc = ^crc
// 	}
// 	if len(p) == 0 {
// 		return crc
// 	}
// 	return simple_update(crc, &tab[0], p)
// }
