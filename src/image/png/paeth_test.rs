// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::paeth;
use crate::compat;
use crate::time;

fn slow_abs(x: isize) -> isize {
    if x < 0 {
        return -x;
    }
    x
}

/// slow_paeth is a slow but simple implementation of the Paeth function.
/// It is a straight port of the sample code in the PNG spec, section 9.4.
fn slow_paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = a as isize + b as isize - c as isize;
    let pa = slow_abs(p - a as isize);
    let pb = slow_abs(p - b as isize);
    let pc = slow_abs(p - c as isize);
    if pa <= pb && pa <= pc {
        return a;
    } else if pb <= pc {
        return b;
    }
    c
}

/// slow_filter_paeth is a slow but simple implementation of fn filter_paeth.
fn slow_filter_paeth(cdat: &mut [u8], pdat: &[u8], bytes_per_pixel: usize) {
    for i in 0..bytes_per_pixel {
        cdat[i] = cdat[i].wrapping_add(paeth::paeth(0, pdat[i], 0));
    }
    for i in bytes_per_pixel..cdat.len() {
        cdat[i] = cdat[i].wrapping_add(paeth::paeth(
            cdat[i - bytes_per_pixel],
            pdat[i],
            pdat[i - bytes_per_pixel],
        ));
    }
}

#[test]
fn test_paeth() {
    let mut a = 0;
    while a < 256 {
        let mut b = 0;
        while b < 256 {
            let mut c = 0;
            while c < 256 {
                let got = paeth::paeth(a as u8, b as u8, c as u8);
                let want = slow_paeth(a as u8, b as u8, c as u8);
                assert_eq!(
                    got, want,
                    "a, b, c = {}, {}, {}: got {}, want {}",
                    a, b, c, got, want
                );
                c += 15
            }
            b += 15
        }
        a += 15
    }
}

// fn BenchmarkPaeth(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		paeth(u8(i>>16), u8(i>>8), u8(i))
// 	}
// }

#[test]
fn test_paeth_decode() {
    let mut pdat0 = [0_u8; 32];
    let mut pdat1 = [0_u8; 32];
    let mut pdat2 = [0_u8; 32];
    let mut cdat0 = [0_u8; 32];
    let mut cdat1 = [0_u8; 32];
    let mut cdat2 = [0_u8; 32];
    // 	r := rand.New(rand.NewSource(1))
    for bytes_per_pixel in 1..=8 {
        for _i in 0..100 {
            for j in 0..pdat0.len() {
                // 				pdat0[j] = r.Uint32() as u8;
                // 				cdat0[j] = r.Uint32() as u8;
                pdat0[j] = (time::now().nanosecond() & 0xff) as u8;
                cdat0[j] = ((time::now().nanosecond() % 11) & 0xff) as u8;
            }
            compat::copy(&mut pdat1, &pdat0);
            compat::copy(&mut pdat1, &pdat0);
            compat::copy(&mut pdat2, &pdat0);
            compat::copy(&mut cdat1, &cdat0);
            compat::copy(&mut cdat2, &cdat0);
            {
                let cdat: &mut [u8] = &mut cdat1;
                let pdat: &[u8] = &pdat1;
                for i in 0..bytes_per_pixel {
                    let mut a = 0;
                    let mut c = 0;
                    let mut j = i;
                    while j < cdat.len() {
                        let b = pdat[j] as isize;
                        let mut pa = b - c;
                        let mut pb = a - c;
                        let pc = paeth::abs(pa + pb);
                        pa = paeth::abs(pa);
                        pb = paeth::abs(pb);
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
            };
            slow_filter_paeth(&mut cdat2, &pdat2, bytes_per_pixel);
            assert_eq!(
                cdat1, cdat2,
                "bytes_per_pixel: {}\npdat0: {:?}\ncdat0: {:?}\ngot:   {:?}\nwant:  {:?}",
                bytes_per_pixel, pdat0, cdat0, cdat1, cdat2
            );
        }
    }
}
