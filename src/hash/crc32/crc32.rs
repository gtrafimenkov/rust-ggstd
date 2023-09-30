// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::crc32_generic;
use crate::hash;

// The size of a CRC-32 checksum in bytes.
pub const SIZE: usize = 4;

/// Predefined polynomials.
#[allow(dead_code)]
#[repr(usize)]
pub enum PredefinedPolynomials {
    // IEEE is by far and away the most common CRC-32 polynomial.
    // Used by ethernet (IEEE 802.3), v.42, fddi, gzip, zip, png, ...
    IEEE = 0xedb88320,

    // Castagnoli's polynomial, used in iSCSI.
    // Has better error detection characteristics than IEEE.
    // https://dx.doi.org/10.1109/26.231911
    Castagnoli = 0x82f63b78,

    // Koopman's polynomial.
    // Also has better error detection characteristics than IEEE.
    // https://dx.doi.org/10.1109/DSN.2002.1028931
    Koopman = 0xeb31d82e,
}

/// Table is a 256-word table representing the polynomial for efficient processing.
pub type Table = [u32; 256];

// impl Default for Table {
//     fn default() -> Self {
//         [0; 256]
//     }
// }

// // This file makes use of functions implemented in architecture-specific files.
// // The interface that they implement is as follows:
// //
// //    // archAvailableIEEE reports whether an architecture-specific CRC32-IEEE
// //    // algorithm is available.
// //    archAvailableIEEE() bool
// //
// //    // archInitIEEE initializes the architecture-specific CRC3-IEEE algorithm.
// //    // It can only be called if archAvailableIEEE() returns true.
// //    archInitIEEE()
// //
// //    // archUpdateIEEE updates the given CRC32-IEEE. It can only be called if
// //    // archInitIEEE() was previously called.
// //    archUpdateIEEE(crc uint32, p [u8]) uint32
// //
// //    // archAvailableCastagnoli reports whether an architecture-specific
// //    // CRC32-C algorithm is available.
// //    archAvailableCastagnoli() bool
// //
// //    // archInitCastagnoli initializes the architecture-specific CRC32-C
// //    // algorithm. It can only be called if archAvailableCastagnoli() returns
// //    // true.
// //    archInitCastagnoli()
// //
// //    // archUpdateCastagnoli updates the given CRC32-C. It can only be called
// //    // if archInitCastagnoli() was previously called.
// //    archUpdateCastagnoli(crc uint32, p [u8]) uint32

// // castagnoliTable points to a lazily initialized Table for the Castagnoli
// // polynomial. MakeTable will always return this value when asked to make a
// // Castagnoli table so we can compare against it to find when the caller is
// // using this polynomial.
// var castagnoliTable *Table
// var castagnoliTable8 *slicing8Table
// var updateCastagnoli func(crc uint32, p [u8]) uint32
// var castagnoliOnce sync.Once
// var haveCastagnoli atomic.Bool

// func castagnoliInit() {
// 	castagnoliTable = simple_make_table(Castagnoli)

// 	if archAvailableCastagnoli() {
// 		archInitCastagnoli()
// 		updateCastagnoli = archUpdateCastagnoli
// 	} else {
// 		// Initialize the slicing-by-8 table.
// 		castagnoliTable8 = slicingMakeTable(Castagnoli)
// 		updateCastagnoli = func(crc uint32, p [u8]) uint32 {
// 			return slicingUpdate(crc, castagnoliTable8, p)
// 		}
// 	}

// 	haveCastagnoli.Store(true)
// }

/// IEEE_TABLE is the table for the IEEE polynomial.
/// The table was precalculated using crc32_generic::simple_make_table(crc32::PredefinedPolynomials::IEEE as u32) function.
static IEEE_TABLE: Table = [
    0x0, 0x77073096, 0xee0e612c, 0x990951ba, 0x76dc419, 0x706af48f, 0xe963a535, 0x9e6495a3,
    0xedb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988, 0x9b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91,
    0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de, 0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7,
    0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9, 0xfa0f3d63, 0x8d080df5,
    0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
    0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59,
    0x26d930ac, 0x51de003a, 0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
    0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924, 0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d,
    0x76dc4190, 0x1db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x6b6b51f, 0x9fbfe4a5, 0xe8b8d433,
    0x7807c9a2, 0xf00f934, 0x9609a88e, 0xe10e9818, 0x7f6a0dbb, 0x86d3d2d, 0x91646c97, 0xe6635c01,
    0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457,
    0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
    0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2, 0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb,
    0x4369e96a, 0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9,
    0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad,
    0xedb88320, 0x9abfb3b6, 0x3b6e20c, 0x74b1d29a, 0xead54739, 0x9dd277af, 0x4db2615, 0x73dc1683,
    0xe3630b12, 0x94643b84, 0xd6d6a3e, 0x7a6a5aa8, 0xe40ecf0b, 0x9309ff9d, 0xa00ae27, 0x7d079eb1,
    0xf00f9344, 0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb, 0x196c3671, 0x6e6b06e7,
    0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
    0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
    0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef, 0x4669be79,
    0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236, 0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f,
    0xc5ba3bbe, 0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d,
    0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x26d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x5005713,
    0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0xcb61b38, 0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0xbdbdf21,
    0x86d3d2d4, 0xf1d4e242, 0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777,
    0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c, 0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45,
    0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7, 0x4969474d, 0x3e6e77db,
    0xaed16a4a, 0xd9d65adc, 0x40df0b66, 0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605, 0xcdd70693, 0x54de5729, 0x23d967bf,
    0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d,
];

// // ieeeTable8 is the slicing8Table for IEEE
// var ieeeTable8 *slicing8Table
// var updateIEEE func(crc uint32, p [u8]) uint32
// var ieeeOnce sync.Once

// func ieeeInit() {
// 	if archAvailableIEEE() {
// 		archInitIEEE()
// 		updateIEEE = archUpdateIEEE
// 	} else {
// 		// Initialize the slicing-by-8 table.
// 		ieeeTable8 = slicingMakeTable(IEEE)
// 		updateIEEE = func(crc uint32, p [u8]) uint32 {
// 			return slicingUpdate(crc, ieeeTable8, p)
// 		}
// 	}
// }

// // MakeTable returns a Table constructed from the specified polynomial.
// // The contents of this Table must not be modified.
// func MakeTable(poly uint32) *Table {
// 	switch poly {
// 	case IEEE:
// 		ieeeOnce.Do(ieeeInit)
// 		return IEEE_TABLE
// 	case Castagnoli:
// 		castagnoliOnce.Do(castagnoliInit)
// 		return castagnoliTable
// 	default:
// 		return simple_make_table(poly)
// 	}
// }

// digest represents the partial evaluation of a checksum.
pub struct Digest {
    crc: u32,
    tab: &'static Table,
}

// New creates a new hash::Hash32 computing the CRC-32 checksum using the
// polynomial represented by the Table. Its Sum method will lay the
// value out in big-endian byte order. The returned Hash32 also
// implements encoding.BinaryMarshaler and encoding.BinaryUnmarshaler to
// marshal and unmarshal the internal state of the hash.
pub fn new(tab: &'static Table) -> Digest {
    // if tab == IEEE_TABLE {
    // 	ieeeOnce.Do(ieeeInit)
    // }
    Digest { crc: 0, tab }
}

// new_ieee creates a new hash::Hash32 computing the CRC-32 checksum using
// the IEEE polynomial. Its Sum method will lay the value out in
// big-endian byte order. The returned Hash32 also implements
// encoding.BinaryMarshaler and encoding.BinaryUnmarshaler to marshal
// and unmarshal the internal state of the hash.
pub fn new_ieee() -> Digest {
    new(&IEEE_TABLE)
}

impl hash::Hash for Digest {
    fn reset(&mut self) {
        self.crc = 0;
    }

    fn size(&self) -> usize {
        SIZE
    }

    fn block_size(&self) -> usize {
        1
    }

    fn sum(&self, b: &[u8]) -> Vec<u8> {
        let s = hash::Hash32::sum32(self);
        let mut res = b.to_vec();
        res.extend_from_slice(&[(s >> 24) as u8, (s >> 16) as u8, (s >> 8) as u8, (s) as u8]);
        res
    }
}

impl hash::Hash32 for Digest {
    fn sum32(&self) -> u32 {
        // func (d *digest) Sum32() uint32 { return d.crc }
        self.crc
    }
}

// const (
// 	magic         = "crc\x01"
// 	marshaledSize = len(magic) + 4 + 4
// )

// func (d *digest) MarshalBinary() ([u8], error) {
// 	b := make([u8], 0, marshaledSize)
// 	b = append(b, magic...)
// 	b = appendUint32(b, tableSum(d.tab))
// 	b = appendUint32(b, d.crc)
// 	return b, nil
// }

// func (d *digest) UnmarshalBinary(b [u8]) error {
// 	if b.len() < len(magic) || string(b[..len(magic)]) != magic {
// 		return errors.New("hash/crc32: invalid hash state identifier")
// 	}
// 	if b.len() != marshaledSize {
// 		return errors.New("hash/crc32: invalid hash state size")
// 	}
// 	if tableSum(d.tab) != readUint32(b[4:]) {
// 		return errors.New("hash/crc32: tables do not match")
// 	}
// 	d.crc = readUint32(b[8:])
// 	return nil
// }

// func appendUint32(b [u8], x uint32) [u8] {
// 	a := [4]byte{
// 		byte(x >> 24),
// 		byte(x >> 16),
// 		byte(x >> 8),
// 		byte(x),
// 	}
// 	return append(b, a[..]...)
// }

// func readUint32(b [u8]) uint32 {
// 	_ = b[3]
// 	return uint32(b[3]) | uint32(b[2])<<8 | uint32(b[1])<<16 | uint32(b[0])<<24
// }

fn update_internal(crc: u32, tab: &Table, p: &[u8]) -> u32 {
    // 	switch {
    // 	case haveCastagnoli.Load() && tab == castagnoliTable:
    // 		return updateCastagnoli(crc, p)
    // 	case tab == IEEE_TABLE:
    // 		if checkInitIEEE {
    // 			ieeeOnce.Do(ieeeInit)
    // 		}
    // 		return updateIEEE(crc, p)
    // 	default:
    // 		return simple_update(crc, tab, p)
    // 	}
    crc32_generic::simple_update(crc, tab, p)
}

// update returns the result of adding the bytes in p to the crc.
pub fn update(crc: u32, tab: &Table, p: &[u8]) -> u32 {
    // // Unfortunately, because IEEE_TABLE is exported, IEEE may be used without a
    // // call to MakeTable. We have to make sure it gets initialized in that case.
    // return update(crc, tab, p, true);

    update_internal(crc, tab, p)
}

impl std::io::Write for Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.crc = update(self.crc, self.tab, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// checksum returns the CRC-32 checksum of data
// using the polynomial represented by the Table.
pub fn checksum(data: &[u8], tab: &Table) -> u32 {
    update(0, tab, data)
}

// checksum_ieee returns the CRC-32 checksum of data
// using the IEEE polynomial.
pub fn checksum_ieee(data: &[u8]) -> u32 {
    // ieeeOnce.Do(ieeeInit)
    // return updateIEEE(0, data)
    checksum(data, &IEEE_TABLE)
}

// // tableSum returns the IEEE checksum of table t.
// func tableSum(t *Table) uint32 {
// 	var a [1024]byte
// 	b := a[..0]
// 	if t != nil {
// 		for _, x := range t {
// 			b = appendUint32(b, x)
// 		}
// 	}
// 	return checksum_ieee(b)
// }
