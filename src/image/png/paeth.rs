// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

const INT_SIZE: usize = std::mem::size_of::<isize>() * 8;

pub(super) fn abs(x: isize) -> isize {
    // m := -1 if x < 0. m := 0 otherwise.
    let m = x >> (INT_SIZE - 1);

    // In two's complement representation, the negative number
    // of any number (except the smallest one) can be computed
    // by flipping all the bits and add 1. This is faster than
    // code with a branch.
    // See Hacker's Delight, section 2-4.
    return (x ^ m) - m;
}

/// paeth implements the Paeth filter function, as per the PNG specification.
pub(super) fn paeth(a: u8, b: u8, c: u8) -> u8 {
    // This is an optimized version of the sample code in the PNG spec.
    // For example, the sample code starts with:
    //	p := isize(a) + isize(b) - isize(c)
    //	pa := abs(p - isize(a))
    // but the optimized form uses fewer arithmetic operations:
    //	pa := isize(b) - isize(c)
    //	pa = abs(pa)
    let mut pc = c as isize;
    let mut pa = (b as isize).wrapping_sub(pc);
    let mut pb = (a as isize).wrapping_sub(pc);
    pc = abs(pa + pb);
    pa = abs(pa);
    pb = abs(pb);
    if pa <= pb && pa <= pc {
        return a;
    } else if pb <= pc {
        return b;
    }
    return c;
}

/// filter_paeth applies the Paeth filter to the cdat slice.
/// cdat is the current row's data, pdat is the previous row's data.
pub(super) fn filter_paeth(cdat: &mut [u8], pdat: &[u8], bytes_per_pixel: usize) {
    for i in 0..bytes_per_pixel {
        let mut a = 0;
        let mut c = 0;
        let mut j = i;
        while j < cdat.len() {
            let b = pdat[j] as isize;
            let mut pa = b - c;
            let mut pb = a - c;
            let pc = abs(pa + pb);
            pa = abs(pa);
            pb = abs(pb);
            if pa <= pb && pa <= pc {
                // No-op.
            } else if pb <= pc {
                a = b
            } else {
                a = c
            }
            a += cdat[j] as isize;
            a &= 0xff;
            cdat[j] = a as u8;
            c = b;
            j += bytes_per_pixel;
        }
    }
}
