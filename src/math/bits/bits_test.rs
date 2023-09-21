// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2017 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::bits;

// package bits_test

// import (
// 	. "math/bits"
// 	"runtime"
// 	"testing"
// 	"unsafe"
// )

// fn TestUintSize() {
// 	var x uint
// 	if want := unsafe.Sizeof(x) * 8; UintSize != want {
// 		t.Fatalf("UintSize = {}; want {}", UintSize, want)
// 	}
// }

// fn TestLeadingZeros() {
// 	for i := 0; i < 256; i += 1 {
// 		nlz := tab[i].nlz
// 		for k := 0; k < 64-8; k++ {
// 			x := u64(i) << uk as isize
// 			if x <= 1<<8-1 {
// 				got := LeadingZeros8(u8(x))
// 				want := nlz - k + (8 - 8)
// 				if x == 0 {
// 					want = 8
// 				}
// 				if got != want {
// 					t.Fatalf("LeadingZeros8(%#02x) == {}; want {}", x, got, want)
// 				}
// 			}

// 			if x <= 1<<16-1 {
// 				got := LeadingZeros16(u16(x))
// 				want := nlz - k + (16 - 8)
// 				if x == 0 {
// 					want = 16
// 				}
// 				if got != want {
// 					t.Fatalf("LeadingZeros16(%#04x) == {}; want {}", x, got, want)
// 				}
// 			}

// 			if x <= 1<<32-1 {
// 				got := LeadingZeros32(u32(x))
// 				want := nlz - k + (32 - 8)
// 				if x == 0 {
// 					want = 32
// 				}
// 				if got != want {
// 					t.Fatalf("LeadingZeros32(%#08x) == {}; want {}", x, got, want)
// 				}
// 				if UintSize == 32 {
// 					got = LeadingZeros(uint(x))
// 					if got != want {
// 						t.Fatalf("LeadingZeros(%#08x) == {}; want {}", x, got, want)
// 					}
// 				}
// 			}

// 			if x <= 1<<64-1 {
// 				got := LeadingZeros64(u64(x))
// 				want := nlz - k + (64 - 8)
// 				if x == 0 {
// 					want = 64
// 				}
// 				if got != want {
// 					t.Fatalf("LeadingZeros64(%#016x) == {}; want {}", x, got, want)
// 				}
// 				if UintSize == 64 {
// 					got = LeadingZeros(uint(x))
// 					if got != want {
// 						t.Fatalf("LeadingZeros(%#016x) == {}; want {}", x, got, want)
// 					}
// 				}
// 			}
// 		}
// 	}
// }

// // Exported (global) variable serving as input for some
// // of the benchmarks to ensure side-effect free calls
// // are not optimized away.
// var Input u64 = DeBruijn64

// // Exported (global) variable to store function results
// // during benchmarking to ensure side-effect free calls
// // are not optimized away.
// var Output int

// fn BenchmarkLeadingZeros(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += LeadingZeros(uint(Input) >> (uint(i) % UintSize))
// 	}
// 	Output = s
// }

// fn BenchmarkLeadingZeros8(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += LeadingZeros8(u8(Input) >> (uint(i) % 8))
// 	}
// 	Output = s
// }

// fn BenchmarkLeadingZeros16(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += LeadingZeros16(u16(Input) >> (uint(i) % 16))
// 	}
// 	Output = s
// }

// fn BenchmarkLeadingZeros32(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += LeadingZeros32(u32(Input) >> (uint(i) % 32))
// 	}
// 	Output = s
// }

// fn BenchmarkLeadingZeros64(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += LeadingZeros64(u64(Input) >> (uint(i) % 64))
// 	}
// 	Output = s
// }

// fn TestTrailingZeros() {
// 	for i := 0; i < 256; i += 1 {
// 		ntz := tab[i].ntz
// 		for k := 0; k < 64-8; k++ {
// 			x := u64(i) << uk as isize
// 			want := ntz + k
// 			if x <= 1<<8-1 {
// 				got := TrailingZeros8(u8(x))
// 				if x == 0 {
// 					want = 8
// 				}
// 				if got != want {
// 					t.Fatalf("TrailingZeros8(%#02x) == {}; want {}", x, got, want)
// 				}
// 			}

// 			if x <= 1<<16-1 {
// 				got := TrailingZeros16(u16(x))
// 				if x == 0 {
// 					want = 16
// 				}
// 				if got != want {
// 					t.Fatalf("TrailingZeros16(%#04x) == {}; want {}", x, got, want)
// 				}
// 			}

// 			if x <= 1<<32-1 {
// 				got := TrailingZeros32(u32(x))
// 				if x == 0 {
// 					want = 32
// 				}
// 				if got != want {
// 					t.Fatalf("TrailingZeros32(%#08x) == {}; want {}", x, got, want)
// 				}
// 				if UintSize == 32 {
// 					got = TrailingZeros(uint(x))
// 					if got != want {
// 						t.Fatalf("TrailingZeros(%#08x) == {}; want {}", x, got, want)
// 					}
// 				}
// 			}

// 			if x <= 1<<64-1 {
// 				got := TrailingZeros64(u64(x))
// 				if x == 0 {
// 					want = 64
// 				}
// 				if got != want {
// 					t.Fatalf("TrailingZeros64(%#016x) == {}; want {}", x, got, want)
// 				}
// 				if UintSize == 64 {
// 					got = TrailingZeros(uint(x))
// 					if got != want {
// 						t.Fatalf("TrailingZeros(%#016x) == {}; want {}", x, got, want)
// 					}
// 				}
// 			}
// 		}
// 	}
// }

// fn BenchmarkTrailingZeros(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += TrailingZeros(uint(Input) << (uint(i) % UintSize))
// 	}
// 	Output = s
// }

// fn BenchmarkTrailingZeros8(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += TrailingZeros8(u8(Input) << (uint(i) % 8))
// 	}
// 	Output = s
// }

// fn BenchmarkTrailingZeros16(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += TrailingZeros16(u16(Input) << (uint(i) % 16))
// 	}
// 	Output = s
// }

// fn BenchmarkTrailingZeros32(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += TrailingZeros32(u32(Input) << (uint(i) % 32))
// 	}
// 	Output = s
// }

// fn BenchmarkTrailingZeros64(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += TrailingZeros64(u64(Input) << (uint(i) % 64))
// 	}
// 	Output = s
// }

// fn TestOnesCount() {
// 	var x u64
// 	for i := 0; i <= 64; i += 1 {
// 		testOnesCount(t, x, i)
// 		x = x<<1 | 1
// 	}

// 	for i := 64; i >= 0; i-- {
// 		testOnesCount(t, x, i)
// 		x = x << 1
// 	}

// 	for i := 0; i < 256; i += 1 {
// 		for k := 0; k < 64-8; k++ {
// 			testOnesCount(t, u64(i)<<uk as isize, tab[i].pop)
// 		}
// 	}
// }

// fn testOnesCount(, x u64, want int) {
// 	if x <= 1<<8-1 {
// 		got := OnesCount8(u8(x))
// 		if got != want {
// 			t.Fatalf("OnesCount8(%#02x) == {}; want {}", u8(x), got, want)
// 		}
// 	}

// 	if x <= 1<<16-1 {
// 		got := OnesCount16(u16(x))
// 		if got != want {
// 			t.Fatalf("OnesCount16(%#04x) == {}; want {}", u16(x), got, want)
// 		}
// 	}

// 	if x <= 1<<32-1 {
// 		got := OnesCount32(u32(x))
// 		if got != want {
// 			t.Fatalf("OnesCount32(%#08x) == {}; want {}", u32(x), got, want)
// 		}
// 		if UintSize == 32 {
// 			got = OnesCount(uint(x))
// 			if got != want {
// 				t.Fatalf("OnesCount(%#08x) == {}; want {}", u32(x), got, want)
// 			}
// 		}
// 	}

// 	if x <= 1<<64-1 {
// 		got := OnesCount64(u64(x))
// 		if got != want {
// 			t.Fatalf("OnesCount64(%#016x) == {}; want {}", x, got, want)
// 		}
// 		if UintSize == 64 {
// 			got = OnesCount(uint(x))
// 			if got != want {
// 				t.Fatalf("OnesCount(%#016x) == {}; want {}", x, got, want)
// 			}
// 		}
// 	}
// }

// fn BenchmarkOnesCount(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += OnesCount(uint(Input))
// 	}
// 	Output = s
// }

// fn BenchmarkOnesCount8(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += OnesCount8(u8(Input))
// 	}
// 	Output = s
// }

// fn BenchmarkOnesCount16(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += OnesCount16(u16(Input))
// 	}
// 	Output = s
// }

// fn BenchmarkOnesCount32(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += OnesCount32(u32(Input))
// 	}
// 	Output = s
// }

// fn BenchmarkOnesCount64(b *testing.B) {
// 	var s int
// 	for i := 0; i < b.N; i += 1 {
// 		s += OnesCount64(u64(Input))
// 	}
// 	Output = s
// }

const DE_BRUIJ_N64: u32 = 0x077CB531;

#[test]
fn test_rotate_left() {
    let m = DE_BRUIJ_N64 as u64;

    for k in 0..128 {
        // let x8 = m as u8;
        // 		let got8 = RotateLeft8(x8, k as isize);
        // 		let want8 = x8<<(k&0x7) | x8>>(8-k&0x7);
        // 		if got8 != want8 {
        // 			t.Fatalf("RotateLeft8(%#02x, {}) == %#02x; want %#02x", x8, k, got8, want8)
        // 		}
        // 		got8 = RotateLeft8(want8, -k as isize);
        // 		if got8 != x8 {
        // 			t.Fatalf("RotateLeft8(%#02x, -{}) == %#02x; want %#02x", want8, k, got8, x8)
        // 		}

        // 		let x16 = u16(m);
        // 		let got16 = RotateLeft16(x16, k as isize);
        // 		let want16 = x16<<(k&0xf) | x16>>(16-k&0xf);
        // 		if got16 != want16 {
        // 			t.Fatalf("RotateLeft16(%#04x, {}) == %#04x; want %#04x", x16, k, got16, want16)
        // 		}
        // 		got16 = RotateLeft16(want16, -k as isize);
        // 		if got16 != x16 {
        // 			t.Fatalf("RotateLeft16(%#04x, -{}) == %#04x; want %#04x", want16, k, got16, x16)
        // 		}

        let x32 = m as u32;
        let got32 = bits::rotate_left32(x32, k as isize);
        let want32 = x32 << (k & 0x1f) | x32 >> ((32 - k) & 0x1f);
        assert_eq!(want32, got32);
        let got32 = bits::rotate_left32(want32, -k as isize);
        assert_eq!(x32, got32);
        // 		if UintSize == 32 {
        // 			let x = uint(m);
        // 			let got = RotateLeft(x, k as isize);
        // 			let want = x<<(k&0x1f) | x>>(32-k&0x1f);
        // 			if got != want {
        // 				t.Fatalf("RotateLeft(%#08x, {}) == %#08x; want %#08x", x, k, got, want)
        // 			}
        // 			got = RotateLeft(want, -k as isize);
        // 			if got != x {
        // 				t.Fatalf("RotateLeft(%#08x, -{}) == %#08x; want %#08x", want, k, got, x)
        // 			}
        // 		}

        // 		let x64 = u64(m);
        // 		let got64 = RotateLeft64(x64, k as isize);
        // 		let want64 = x64<<(k&0x3f) | x64>>(64-k&0x3f);
        // 		if got64 != want64 {
        // 			t.Fatalf("RotateLeft64(%#016x, {}) == %#016x; want %#016x", x64, k, got64, want64)
        // 		}
        // 		got64 = RotateLeft64(want64, -k as isize);
        // 		if got64 != x64 {
        // 			t.Fatalf("RotateLeft64(%#016x, -{}) == %#016x; want %#016x", want64, k, got64, x64)
        // 		}
        // 		if UintSize == 64 {
        // 			let x = uint(m);
        // 			let got = RotateLeft(x, k as isize);
        // 			let want = x<<(k&0x3f) | x>>(64-k&0x3f);
        // 			if got != want {
        // 				t.Fatalf("RotateLeft(%#016x, {}) == %#016x; want %#016x", x, k, got, want)
        // 			}
        // 			got = RotateLeft(want, -k as isize);
        // 			if got != x {
        // 				t.Fatalf("RotateLeft(%#08x, -{}) == %#08x; want %#08x", want, k, got, x)
        // 			}
        // 		}
    }
}

// fn BenchmarkRotateLeft(b *testing.B) {
// 	var s uint
// 	for i := 0; i < b.N; i += 1 {
// 		s += RotateLeft(uint(Input), i)
// 	}
// 	Output = int(s)
// }

// fn BenchmarkRotateLeft8(b *testing.B) {
// 	var s u8
// 	for i := 0; i < b.N; i += 1 {
// 		s += RotateLeft8(u8(Input), i)
// 	}
// 	Output = int(s)
// }

// fn BenchmarkRotateLeft16(b *testing.B) {
// 	var s u16
// 	for i := 0; i < b.N; i += 1 {
// 		s += RotateLeft16(u16(Input), i)
// 	}
// 	Output = int(s)
// }

// fn Benchmarkrotate_left32(b *testing.B) {
// 	var s u32
// 	for i := 0; i < b.N; i += 1 {
// 		s += bits::rotate_left32(u32(Input), i)
// 	}
// 	Output = int(s)
// }

// fn BenchmarkRotateLeft64(b *testing.B) {
// 	var s u64
// 	for i := 0; i < b.N; i += 1 {
// 		s += RotateLeft64(u64(Input), i)
// 	}
// 	Output = int(s)
// }

#[test]
fn test_reverse() {
    // test each bit
    for i in 0..64 {
        test_reverse_internal(1_u64 << i, 1_u64 << (63 - i));
    }

    fn test2(x: u64, r: u64) {
        test_reverse_internal(x, r);
        test_reverse_internal(r, x);
    }

    // test a few patterns
    test2(0, 0);
    test2(0x1, 0x8 << 60);
    test2(0x2, 0x4 << 60);
    test2(0x3, 0xc << 60);
    test2(0x4, 0x2 << 60);
    test2(0x5, 0xa << 60);
    test2(0x6, 0x6 << 60);
    test2(0x7, 0xe << 60);
    test2(0x8, 0x1 << 60);
    test2(0x9, 0x9 << 60);
    test2(0xa, 0x5 << 60);
    test2(0xb, 0xd << 60);
    test2(0xc, 0x3 << 60);
    test2(0xd, 0xb << 60);
    test2(0xe, 0x7 << 60);
    test2(0xf, 0xf << 60);
    test2(0x5686487, 0xe12616a000000000);
    test2(0x0123456789abcdef, 0xf7b3d591e6a2c480);
}

fn test_reverse_internal(x64: u64, want64: u64) {
    let x8 = x64 as u8;
    let got8 = bits::reverse8(x8);
    let want8 = (want64 >> (64 - 8)) as u8;
    assert_eq!(
        want8, got8,
        "reverse8({:02x}) == {:02x}; want {:02x}",
        x8, got8, want8
    );
    // 	}

    let x16 = x64 as u16;
    let got16 = bits::reverse16(x16);
    let want16 = (want64 >> (64 - 16)) as u16;
    assert_eq!(want16, got16);

    // 	let x32 = u32(x64);
    // 	let got32 = Reverse32(x32);
    // 	let want32 = u32(want64 >> (64 - 32));
    // 	if got32 != want32 {
    // 		t.Fatalf("Reverse32(%#08x) == %#08x; want %#08x", x32, got32, want32)
    // 	}
    // 	if UintSize == 32 {
    // 		let x = uint(x32);
    // 		let got = Reverse(x);
    // 		let want = uint(want32);
    // 		if got != want {
    // 			t.Fatalf("Reverse(%#08x) == %#08x; want %#08x", x, got, want)
    // 		}
    // 	}

    // 	let got64 = Reverse64(x64);
    // 	if got64 != want64 {
    // 		t.Fatalf("Reverse64(%#016x) == %#016x; want %#016x", x64, got64, want64)
    // 	}
    // 	if UintSize == 64 {
    // 		let x = uint(x64);
    // 		let got = Reverse(x);
    // 		let want = uint(want64);
    // 		if got != want {
    // 			t.Fatalf("Reverse(%#08x) == %#016x; want %#016x", x, got, want)
    // 		}
    // 	}
}

// fn BenchmarkReverse(b *testing.B) {
// 	var s uint
// 	for i := 0; i < b.N; i += 1 {
// 		s += Reverse(uint(i))
// 	}
// 	Output = int(s)
// }

// fn BenchmarkReverse8(b *testing.B) {
// 	var s u8
// 	for i := 0; i < b.N; i += 1 {
// 		s += Reverse8(u8(i))
// 	}
// 	Output = int(s)
// }

// fn Benchmarkreverse16(b *testing.B) {
// 	var s u16
// 	for i := 0; i < b.N; i += 1 {
// 		s += reverse16(u16(i))
// 	}
// 	Output = int(s)
// }

// fn BenchmarkReverse32(b *testing.B) {
// 	var s u32
// 	for i := 0; i < b.N; i += 1 {
// 		s += Reverse32(u32(i))
// 	}
// 	Output = int(s)
// }

// fn BenchmarkReverse64(b *testing.B) {
// 	var s u64
// 	for i := 0; i < b.N; i += 1 {
// 		s += Reverse64(u64(i))
// 	}
// 	Output = int(s)
// }

// fn TestReverseBytes() {
// 	for _, test := range []struct {
// 		x, r u64
// 	}{
// 		{0, 0},
// 		{0x01, 0x01 << 56},
// 		{0x0123, 0x2301 << 48},
// 		{0x012345, 0x452301 << 40},
// 		{0x01234567, 0x67452301 << 32},
// 		{0x0123456789, 0x8967452301 << 24},
// 		{0x0123456789ab, 0xab8967452301 << 16},
// 		{0x0123456789abcd, 0xcdab8967452301 << 8},
// 		{0x0123456789abcdef, 0xefcdab8967452301 << 0},
// 	} {
// 		testReverseBytes(t, test.x, test.r)
// 		testReverseBytes(t, test.r, test.x)
// 	}
// }

// fn testReverseBytes(, x64, want64 u64) {
// 	x16 := u16(x64)
// 	got16 := ReverseBytes16(x16)
// 	want16 := u16(want64 >> (64 - 16))
// 	if got16 != want16 {
// 		t.Fatalf("ReverseBytes16(%#04x) == %#04x; want %#04x", x16, got16, want16)
// 	}

// 	x32 := u32(x64)
// 	got32 := ReverseBytes32(x32)
// 	want32 := u32(want64 >> (64 - 32))
// 	if got32 != want32 {
// 		t.Fatalf("ReverseBytes32(%#08x) == %#08x; want %#08x", x32, got32, want32)
// 	}
// 	if UintSize == 32 {
// 		x := uint(x32)
// 		got := ReverseBytes(x)
// 		want := uint(want32)
// 		if got != want {
// 			t.Fatalf("ReverseBytes(%#08x) == %#08x; want %#08x", x, got, want)
// 		}
// 	}

// 	got64 := ReverseBytes64(x64)
// 	if got64 != want64 {
// 		t.Fatalf("ReverseBytes64(%#016x) == %#016x; want %#016x", x64, got64, want64)
// 	}
// 	if UintSize == 64 {
// 		x := uint(x64)
// 		got := ReverseBytes(x)
// 		want := uint(want64)
// 		if got != want {
// 			t.Fatalf("ReverseBytes(%#016x) == %#016x; want %#016x", x, got, want)
// 		}
// 	}
// }

// fn BenchmarkReverseBytes(b *testing.B) {
// 	var s uint
// 	for i := 0; i < b.N; i += 1 {
// 		s += ReverseBytes(uint(i))
// 	}
// 	Output = int(s)
// }

// fn BenchmarkReverseBytes16(b *testing.B) {
// 	var s u16
// 	for i := 0; i < b.N; i += 1 {
// 		s += ReverseBytes16(u16(i))
// 	}
// 	Output = int(s)
// }

// fn BenchmarkReverseBytes32(b *testing.B) {
// 	var s u32
// 	for i := 0; i < b.N; i += 1 {
// 		s += ReverseBytes32(u32(i))
// 	}
// 	Output = int(s)
// }

// fn BenchmarkReverseBytes64(b *testing.B) {
// 	var s u64
// 	for i := 0; i < b.N; i += 1 {
// 		s += ReverseBytes64(u64(i))
// 	}
// 	Output = int(s)
// }

// fn TestLen() {
// 	for i := 0; i < 256; i += 1 {
// 		len := 8 - tab[i].nlz
// 		for k := 0; k < 64-8; k++ {
// 			x := u64(i) << uk as isize
// 			want := 0
// 			if x != 0 {
// 				want = len + k
// 			}
// 			if x <= 1<<8-1 {
// 				got := Len8(u8(x))
// 				if got != want {
// 					t.Fatalf("Len8(%#02x) == {}; want {}", x, got, want)
// 				}
// 			}

// 			if x <= 1<<16-1 {
// 				got := Len16(u16(x))
// 				if got != want {
// 					t.Fatalf("Len16(%#04x) == {}; want {}", x, got, want)
// 				}
// 			}

// 			if x <= 1<<32-1 {
// 				got := Len32(u32(x))
// 				if got != want {
// 					t.Fatalf("Len32(%#08x) == {}; want {}", x, got, want)
// 				}
// 				if UintSize == 32 {
// 					got := Len(uint(x))
// 					if got != want {
// 						t.Fatalf("Len(%#08x) == {}; want {}", x, got, want)
// 					}
// 				}
// 			}

// 			if x <= 1<<64-1 {
// 				got := Len64(u64(x))
// 				if got != want {
// 					t.Fatalf("Len64(%#016x) == {}; want {}", x, got, want)
// 				}
// 				if UintSize == 64 {
// 					got := Len(uint(x))
// 					if got != want {
// 						t.Fatalf("Len(%#016x) == {}; want {}", x, got, want)
// 					}
// 				}
// 			}
// 		}
// 	}
// }

// const (
// 	_M   = 1<<UintSize - 1
// 	_M32 = 1<<32 - 1
// 	_M64 = 1<<64 - 1
// )

// fn TestAddSubUint() {
// 	test := fn(msg string, f fn(x, y, c uint) (z, cout uint), x, y, c, z, cout uint) {
// 		z1, cout1 := f(x, y, c)
// 		if z1 != z || cout1 != cout {
// 			t.Errorf("%s: got z:cout = %#x:%#x; want %#x:%#x", msg, z1, cout1, z, cout)
// 		}
// 	}
// 	for _, a := range []struct{ x, y, c, z, cout uint }{
// 		{0, 0, 0, 0, 0},
// 		{0, 1, 0, 1, 0},
// 		{0, 0, 1, 1, 0},
// 		{0, 1, 1, 2, 0},
// 		{12345, 67890, 0, 80235, 0},
// 		{12345, 67890, 1, 80236, 0},
// 		{_M, 1, 0, 0, 1},
// 		{_M, 0, 1, 0, 1},
// 		{_M, 1, 1, 1, 1},
// 		{_M, _M, 0, _M - 1, 1},
// 		{_M, _M, 1, _M, 1},
// 	} {
// 		test("Add", Add, a.x, a.y, a.c, a.z, a.cout)
// 		test("Add symmetric", Add, a.y, a.x, a.c, a.z, a.cout)
// 		test("Sub", Sub, a.z, a.x, a.c, a.y, a.cout)
// 		test("Sub symmetric", Sub, a.z, a.y, a.c, a.x, a.cout)
// 		// The above code can't test intrinsic implementation, because the passed function is not called directly.
// 		// The following code uses a closure to test the intrinsic version in case the function is intrinsified.
// 		test("Add intrinsic", fn(x, y, c uint) (uint, uint) { return Add(x, y, c) }, a.x, a.y, a.c, a.z, a.cout)
// 		test("Add intrinsic symmetric", fn(x, y, c uint) (uint, uint) { return Add(x, y, c) }, a.y, a.x, a.c, a.z, a.cout)
// 		test("Sub intrinsic", fn(x, y, c uint) (uint, uint) { return Sub(x, y, c) }, a.z, a.x, a.c, a.y, a.cout)
// 		test("Sub intrinsic symmetric", fn(x, y, c uint) (uint, uint) { return Sub(x, y, c) }, a.z, a.y, a.c, a.x, a.cout)

// 	}
// }

// fn TestAddSubUint32() {
// 	test := fn(msg string, f fn(x, y, c u32) (z, cout u32), x, y, c, z, cout u32) {
// 		z1, cout1 := f(x, y, c)
// 		if z1 != z || cout1 != cout {
// 			t.Errorf("%s: got z:cout = %#x:%#x; want %#x:%#x", msg, z1, cout1, z, cout)
// 		}
// 	}
// 	for _, a := range []struct{ x, y, c, z, cout u32 }{
// 		{0, 0, 0, 0, 0},
// 		{0, 1, 0, 1, 0},
// 		{0, 0, 1, 1, 0},
// 		{0, 1, 1, 2, 0},
// 		{12345, 67890, 0, 80235, 0},
// 		{12345, 67890, 1, 80236, 0},
// 		{_M32, 1, 0, 0, 1},
// 		{_M32, 0, 1, 0, 1},
// 		{_M32, 1, 1, 1, 1},
// 		{_M32, _M32, 0, _M32 - 1, 1},
// 		{_M32, _M32, 1, _M32, 1},
// 	} {
// 		test("Add32", Add32, a.x, a.y, a.c, a.z, a.cout)
// 		test("Add32 symmetric", Add32, a.y, a.x, a.c, a.z, a.cout)
// 		test("Sub32", Sub32, a.z, a.x, a.c, a.y, a.cout)
// 		test("Sub32 symmetric", Sub32, a.z, a.y, a.c, a.x, a.cout)
// 	}
// }

// fn TestAddSubUint64() {
// 	test := fn(msg string, f fn(x, y, c u64) (z, cout u64), x, y, c, z, cout u64) {
// 		z1, cout1 := f(x, y, c)
// 		if z1 != z || cout1 != cout {
// 			t.Errorf("%s: got z:cout = %#x:%#x; want %#x:%#x", msg, z1, cout1, z, cout)
// 		}
// 	}
// 	for _, a := range []struct{ x, y, c, z, cout u64 }{
// 		{0, 0, 0, 0, 0},
// 		{0, 1, 0, 1, 0},
// 		{0, 0, 1, 1, 0},
// 		{0, 1, 1, 2, 0},
// 		{12345, 67890, 0, 80235, 0},
// 		{12345, 67890, 1, 80236, 0},
// 		{_M64, 1, 0, 0, 1},
// 		{_M64, 0, 1, 0, 1},
// 		{_M64, 1, 1, 1, 1},
// 		{_M64, _M64, 0, _M64 - 1, 1},
// 		{_M64, _M64, 1, _M64, 1},
// 	} {
// 		test("Add64", Add64, a.x, a.y, a.c, a.z, a.cout)
// 		test("Add64 symmetric", Add64, a.y, a.x, a.c, a.z, a.cout)
// 		test("Sub64", Sub64, a.z, a.x, a.c, a.y, a.cout)
// 		test("Sub64 symmetric", Sub64, a.z, a.y, a.c, a.x, a.cout)
// 		// The above code can't test intrinsic implementation, because the passed function is not called directly.
// 		// The following code uses a closure to test the intrinsic version in case the function is intrinsified.
// 		test("Add64 intrinsic", fn(x, y, c u64) (u64, u64) { return Add64(x, y, c) }, a.x, a.y, a.c, a.z, a.cout)
// 		test("Add64 intrinsic symmetric", fn(x, y, c u64) (u64, u64) { return Add64(x, y, c) }, a.y, a.x, a.c, a.z, a.cout)
// 		test("Sub64 intrinsic", fn(x, y, c u64) (u64, u64) { return Sub64(x, y, c) }, a.z, a.x, a.c, a.y, a.cout)
// 		test("Sub64 intrinsic symmetric", fn(x, y, c u64) (u64, u64) { return Sub64(x, y, c) }, a.z, a.y, a.c, a.x, a.cout)
// 	}
// }

// fn TestAdd64OverflowPanic() {
// 	// Test that 64-bit overflow panics fire correctly.
// 	// These are designed to improve coverage of compiler intrinsics.
// 	tests := []fn(u64, u64) u64{
// 		fn(a, b u64) u64 {
// 			x, c := Add64(a, b, 0)
// 			if c > 0 {
// 				panic("overflow")
// 			}
// 			return x
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Add64(a, b, 0)
// 			if c != 0 {
// 				panic("overflow")
// 			}
// 			return x
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Add64(a, b, 0)
// 			if c == 1 {
// 				panic("overflow")
// 			}
// 			return x
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Add64(a, b, 0)
// 			if c != 1 {
// 				return x
// 			}
// 			panic("overflow")
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Add64(a, b, 0)
// 			if c == 0 {
// 				return x
// 			}
// 			panic("overflow")
// 		},
// 	}
// 	for _, test := range tests {
// 		shouldPanic := fn(f fn()) {
// 			defer fn() {
// 				if err := recover(); err == nil {
// 					t.Fatalf("expected panic")
// 				}
// 			}()
// 			f()
// 		}

// 		// overflow
// 		shouldPanic(fn() { test(_M64, 1) })
// 		shouldPanic(fn() { test(1, _M64) })
// 		shouldPanic(fn() { test(_M64, _M64) })

// 		// no overflow
// 		test(_M64, 0)
// 		test(0, 0)
// 		test(1, 1)
// 	}
// }

// fn TestSub64OverflowPanic() {
// 	// Test that 64-bit overflow panics fire correctly.
// 	// These are designed to improve coverage of compiler intrinsics.
// 	tests := []fn(u64, u64) u64{
// 		fn(a, b u64) u64 {
// 			x, c := Sub64(a, b, 0)
// 			if c > 0 {
// 				panic("overflow")
// 			}
// 			return x
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Sub64(a, b, 0)
// 			if c != 0 {
// 				panic("overflow")
// 			}
// 			return x
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Sub64(a, b, 0)
// 			if c == 1 {
// 				panic("overflow")
// 			}
// 			return x
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Sub64(a, b, 0)
// 			if c != 1 {
// 				return x
// 			}
// 			panic("overflow")
// 		},
// 		fn(a, b u64) u64 {
// 			x, c := Sub64(a, b, 0)
// 			if c == 0 {
// 				return x
// 			}
// 			panic("overflow")
// 		},
// 	}
// 	for _, test := range tests {
// 		shouldPanic := fn(f fn()) {
// 			defer fn() {
// 				if err := recover(); err == nil {
// 					t.Fatalf("expected panic")
// 				}
// 			}()
// 			f()
// 		}

// 		// overflow
// 		shouldPanic(fn() { test(0, 1) })
// 		shouldPanic(fn() { test(1, _M64) })
// 		shouldPanic(fn() { test(_M64-1, _M64) })

// 		// no overflow
// 		test(_M64, 0)
// 		test(0, 0)
// 		test(1, 1)
// 	}
// }

// fn TestMulDiv() {
// 	testMul := fn(msg string, f fn(x, y uint) (hi, lo uint), x, y, hi, lo uint) {
// 		hi1, lo1 := f(x, y)
// 		if hi1 != hi || lo1 != lo {
// 			t.Errorf("%s: got hi:lo = %#x:%#x; want %#x:%#x", msg, hi1, lo1, hi, lo)
// 		}
// 	}
// 	testDiv := fn(msg string, f fn(hi, lo, y uint) (q, r uint), hi, lo, y, q, r uint) {
// 		q1, r1 := f(hi, lo, y)
// 		if q1 != q || r1 != r {
// 			t.Errorf("%s: got q:r = %#x:%#x; want %#x:%#x", msg, q1, r1, q, r)
// 		}
// 	}
// 	for _, a := range []struct {
// 		x, y      uint
// 		hi, lo, r uint
// 	}{
// 		{1 << (UintSize - 1), 2, 1, 0, 1},
// 		{_M, _M, _M - 1, 1, 42},
// 	} {
// 		testMul("Mul", Mul, a.x, a.y, a.hi, a.lo)
// 		testMul("Mul symmetric", Mul, a.y, a.x, a.hi, a.lo)
// 		testDiv("Div", Div, a.hi, a.lo+a.r, a.y, a.x, a.r)
// 		testDiv("Div symmetric", Div, a.hi, a.lo+a.r, a.x, a.y, a.r)
// 		// The above code can't test intrinsic implementation, because the passed function is not called directly.
// 		// The following code uses a closure to test the intrinsic version in case the function is intrinsified.
// 		testMul("Mul intrinsic", fn(x, y uint) (uint, uint) { return Mul(x, y) }, a.x, a.y, a.hi, a.lo)
// 		testMul("Mul intrinsic symmetric", fn(x, y uint) (uint, uint) { return Mul(x, y) }, a.y, a.x, a.hi, a.lo)
// 		testDiv("Div intrinsic", fn(hi, lo, y uint) (uint, uint) { return Div(hi, lo, y) }, a.hi, a.lo+a.r, a.y, a.x, a.r)
// 		testDiv("Div intrinsic symmetric", fn(hi, lo, y uint) (uint, uint) { return Div(hi, lo, y) }, a.hi, a.lo+a.r, a.x, a.y, a.r)
// 	}
// }

// fn TestMulDiv32() {
// 	testMul := fn(msg string, f fn(x, y u32) (hi, lo u32), x, y, hi, lo u32) {
// 		hi1, lo1 := f(x, y)
// 		if hi1 != hi || lo1 != lo {
// 			t.Errorf("%s: got hi:lo = %#x:%#x; want %#x:%#x", msg, hi1, lo1, hi, lo)
// 		}
// 	}
// 	testDiv := fn(msg string, f fn(hi, lo, y u32) (q, r u32), hi, lo, y, q, r u32) {
// 		q1, r1 := f(hi, lo, y)
// 		if q1 != q || r1 != r {
// 			t.Errorf("%s: got q:r = %#x:%#x; want %#x:%#x", msg, q1, r1, q, r)
// 		}
// 	}
// 	for _, a := range []struct {
// 		x, y      u32
// 		hi, lo, r u32
// 	}{
// 		{1 << 31, 2, 1, 0, 1},
// 		{0xc47dfa8c, 50911, 0x98a4, 0x998587f4, 13},
// 		{_M32, _M32, _M32 - 1, 1, 42},
// 	} {
// 		testMul("Mul32", Mul32, a.x, a.y, a.hi, a.lo)
// 		testMul("Mul32 symmetric", Mul32, a.y, a.x, a.hi, a.lo)
// 		testDiv("Div32", Div32, a.hi, a.lo+a.r, a.y, a.x, a.r)
// 		testDiv("Div32 symmetric", Div32, a.hi, a.lo+a.r, a.x, a.y, a.r)
// 	}
// }

// fn TestMulDiv64() {
// 	testMul := fn(msg string, f fn(x, y u64) (hi, lo u64), x, y, hi, lo u64) {
// 		hi1, lo1 := f(x, y)
// 		if hi1 != hi || lo1 != lo {
// 			t.Errorf("%s: got hi:lo = %#x:%#x; want %#x:%#x", msg, hi1, lo1, hi, lo)
// 		}
// 	}
// 	testDiv := fn(msg string, f fn(hi, lo, y u64) (q, r u64), hi, lo, y, q, r u64) {
// 		q1, r1 := f(hi, lo, y)
// 		if q1 != q || r1 != r {
// 			t.Errorf("%s: got q:r = %#x:%#x; want %#x:%#x", msg, q1, r1, q, r)
// 		}
// 	}
// 	for _, a := range []struct {
// 		x, y      u64
// 		hi, lo, r u64
// 	}{
// 		{1 << 63, 2, 1, 0, 1},
// 		{0x3626229738a3b9, 0xd8988a9f1cc4a61, 0x2dd0712657fe8, 0x9dd6a3364c358319, 13},
// 		{_M64, _M64, _M64 - 1, 1, 42},
// 	} {
// 		testMul("Mul64", Mul64, a.x, a.y, a.hi, a.lo)
// 		testMul("Mul64 symmetric", Mul64, a.y, a.x, a.hi, a.lo)
// 		testDiv("Div64", Div64, a.hi, a.lo+a.r, a.y, a.x, a.r)
// 		testDiv("Div64 symmetric", Div64, a.hi, a.lo+a.r, a.x, a.y, a.r)
// 		// The above code can't test intrinsic implementation, because the passed function is not called directly.
// 		// The following code uses a closure to test the intrinsic version in case the function is intrinsified.
// 		testMul("Mul64 intrinsic", fn(x, y u64) (u64, u64) { return Mul64(x, y) }, a.x, a.y, a.hi, a.lo)
// 		testMul("Mul64 intrinsic symmetric", fn(x, y u64) (u64, u64) { return Mul64(x, y) }, a.y, a.x, a.hi, a.lo)
// 		testDiv("Div64 intrinsic", fn(hi, lo, y u64) (u64, u64) { return Div64(hi, lo, y) }, a.hi, a.lo+a.r, a.y, a.x, a.r)
// 		testDiv("Div64 intrinsic symmetric", fn(hi, lo, y u64) (u64, u64) { return Div64(hi, lo, y) }, a.hi, a.lo+a.r, a.x, a.y, a.r)
// 	}
// }

// const (
// 	divZeroError  = "runtime error: integer divide by zero"
// 	overflowError = "runtime error: integer overflow"
// )

// fn TestDivPanicOverflow() {
// 	// Expect a panic
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Error("Div should have panicked when y<=hi")
// 		} else if e, ok := err.(runtime.Error); !ok || e.Error() != overflowError {
// 			t.Errorf("Div expected panic: %q, got: %q ", overflowError, e.Error())
// 		}
// 	}()
// 	q, r := Div(1, 0, 1)
// 	t.Errorf("undefined q, r = {}, {} calculated when Div should have panicked", q, r)
// }

// fn TestDiv32PanicOverflow() {
// 	// Expect a panic
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Error("Div32 should have panicked when y<=hi")
// 		} else if e, ok := err.(runtime.Error); !ok || e.Error() != overflowError {
// 			t.Errorf("Div32 expected panic: %q, got: %q ", overflowError, e.Error())
// 		}
// 	}()
// 	q, r := Div32(1, 0, 1)
// 	t.Errorf("undefined q, r = {}, {} calculated when Div32 should have panicked", q, r)
// }

// fn TestDiv64PanicOverflow() {
// 	// Expect a panic
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Error("Div64 should have panicked when y<=hi")
// 		} else if e, ok := err.(runtime.Error); !ok || e.Error() != overflowError {
// 			t.Errorf("Div64 expected panic: %q, got: %q ", overflowError, e.Error())
// 		}
// 	}()
// 	q, r := Div64(1, 0, 1)
// 	t.Errorf("undefined q, r = {}, {} calculated when Div64 should have panicked", q, r)
// }

// fn TestDivPanicZero() {
// 	// Expect a panic
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Error("Div should have panicked when y==0")
// 		} else if e, ok := err.(runtime.Error); !ok || e.Error() != divZeroError {
// 			t.Errorf("Div expected panic: %q, got: %q ", divZeroError, e.Error())
// 		}
// 	}()
// 	q, r := Div(1, 1, 0)
// 	t.Errorf("undefined q, r = {}, {} calculated when Div should have panicked", q, r)
// }

// fn TestDiv32PanicZero() {
// 	// Expect a panic
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Error("Div32 should have panicked when y==0")
// 		} else if e, ok := err.(runtime.Error); !ok || e.Error() != divZeroError {
// 			t.Errorf("Div32 expected panic: %q, got: %q ", divZeroError, e.Error())
// 		}
// 	}()
// 	q, r := Div32(1, 1, 0)
// 	t.Errorf("undefined q, r = {}, {} calculated when Div32 should have panicked", q, r)
// }

// fn TestDiv64PanicZero() {
// 	// Expect a panic
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Error("Div64 should have panicked when y==0")
// 		} else if e, ok := err.(runtime.Error); !ok || e.Error() != divZeroError {
// 			t.Errorf("Div64 expected panic: %q, got: %q ", divZeroError, e.Error())
// 		}
// 	}()
// 	q, r := Div64(1, 1, 0)
// 	t.Errorf("undefined q, r = {}, {} calculated when Div64 should have panicked", q, r)
// }

// fn TestRem32() {
// 	// Sanity check: for non-oveflowing dividends, the result is the
// 	// same as the rem returned by Div32
// 	hi, lo, y := u32(510510), u32(9699690), u32(510510+1) // ensure hi < y
// 	for i := 0; i < 1000; i += 1 {
// 		r := Rem32(hi, lo, y)
// 		_, r2 := Div32(hi, lo, y)
// 		if r != r2 {
// 			t.Errorf("Rem32({}, {}, {}) returned {}, but Div32 returned rem {}", hi, lo, y, r, r2)
// 		}
// 		y += 13
// 	}
// }

// fn TestRem32Overflow() {
// 	// To trigger a quotient overflow, we need y <= hi
// 	hi, lo, y := u32(510510), u32(9699690), u32(7)
// 	for i := 0; i < 1000; i += 1 {
// 		r := Rem32(hi, lo, y)
// 		_, r2 := Div64(0, u64(hi)<<32|u64(lo), u64(y))
// 		if r != u32(r2) {
// 			t.Errorf("Rem32({}, {}, {}) returned {}, but Div64 returned rem {}", hi, lo, y, r, r2)
// 		}
// 		y += 13
// 	}
// }

// fn TestRem64() {
// 	// Sanity check: for non-oveflowing dividends, the result is the
// 	// same as the rem returned by Div64
// 	hi, lo, y := u64(510510), u64(9699690), u64(510510+1) // ensure hi < y
// 	for i := 0; i < 1000; i += 1 {
// 		r := Rem64(hi, lo, y)
// 		_, r2 := Div64(hi, lo, y)
// 		if r != r2 {
// 			t.Errorf("Rem64({}, {}, {}) returned {}, but Div64 returned rem {}", hi, lo, y, r, r2)
// 		}
// 		y += 13
// 	}
// }

// fn TestRem64Overflow() {
// 	Rem64Tests := []struct {
// 		hi, lo, y u64
// 		rem       u64
// 	}{
// 		// Testcases computed using Python 3, as:
// 		//   >>> hi = 42; lo = 1119; y = 42
// 		//   >>> ((hi<<64)+lo) % y
// 		{42, 1119, 42, 27},
// 		{42, 1119, 38, 9},
// 		{42, 1119, 26, 23},
// 		{469, 0, 467, 271},
// 		{469, 0, 113, 58},
// 		{111111, 111111, 1171, 803},
// 		{3968194946088682615, 3192705705065114702, 1000037, 56067},
// 	}

// 	for _, rt := range Rem64Tests {
// 		if rt.hi < rt.y {
// 			t.Fatalf("Rem64({}, {}, {}) is not a test with quo overflow", rt.hi, rt.lo, rt.y)
// 		}
// 		rem := Rem64(rt.hi, rt.lo, rt.y)
// 		if rem != rt.rem {
// 			t.Errorf("Rem64({}, {}, {}) returned {}, wanted {}",
// 				rt.hi, rt.lo, rt.y, rem, rt.rem)
// 		}
// 	}
// }

// fn BenchmarkAdd(b *testing.B) {
// 	var z, c uint
// 	for i := 0; i < b.N; i += 1 {
// 		z, c = Add(uint(Input), uint(i), c)
// 	}
// 	Output = int(z + c)
// }

// fn BenchmarkAdd32(b *testing.B) {
// 	var z, c u32
// 	for i := 0; i < b.N; i += 1 {
// 		z, c = Add32(u32(Input), u32(i), c)
// 	}
// 	Output = int(z + c)
// }

// fn BenchmarkAdd64(b *testing.B) {
// 	var z, c u64
// 	for i := 0; i < b.N; i += 1 {
// 		z, c = Add64(u64(Input), u64(i), c)
// 	}
// 	Output = int(z + c)
// }

// fn BenchmarkAdd64multiple(b *testing.B) {
// 	var z0 = u64(Input)
// 	var z1 = u64(Input)
// 	var z2 = u64(Input)
// 	var z3 = u64(Input)
// 	for i := 0; i < b.N; i += 1 {
// 		var c u64
// 		z0, c = Add64(z0, u64(i), c)
// 		z1, c = Add64(z1, u64(i), c)
// 		z2, c = Add64(z2, u64(i), c)
// 		z3, _ = Add64(z3, u64(i), c)
// 	}
// 	Output = int(z0 + z1 + z2 + z3)
// }

// fn BenchmarkSub(b *testing.B) {
// 	var z, c uint
// 	for i := 0; i < b.N; i += 1 {
// 		z, c = Sub(uint(Input), uint(i), c)
// 	}
// 	Output = int(z + c)
// }

// fn BenchmarkSub32(b *testing.B) {
// 	var z, c u32
// 	for i := 0; i < b.N; i += 1 {
// 		z, c = Sub32(u32(Input), u32(i), c)
// 	}
// 	Output = int(z + c)
// }

// fn BenchmarkSub64(b *testing.B) {
// 	var z, c u64
// 	for i := 0; i < b.N; i += 1 {
// 		z, c = Sub64(u64(Input), u64(i), c)
// 	}
// 	Output = int(z + c)
// }

// fn BenchmarkSub64multiple(b *testing.B) {
// 	var z0 = u64(Input)
// 	var z1 = u64(Input)
// 	var z2 = u64(Input)
// 	var z3 = u64(Input)
// 	for i := 0; i < b.N; i += 1 {
// 		var c u64
// 		z0, c = Sub64(z0, u64(i), c)
// 		z1, c = Sub64(z1, u64(i), c)
// 		z2, c = Sub64(z2, u64(i), c)
// 		z3, _ = Sub64(z3, u64(i), c)
// 	}
// 	Output = int(z0 + z1 + z2 + z3)
// }

// fn BenchmarkMul(b *testing.B) {
// 	var hi, lo uint
// 	for i := 0; i < b.N; i += 1 {
// 		hi, lo = Mul(uint(Input), uint(i))
// 	}
// 	Output = int(hi + lo)
// }

// fn BenchmarkMul32(b *testing.B) {
// 	var hi, lo u32
// 	for i := 0; i < b.N; i += 1 {
// 		hi, lo = Mul32(u32(Input), u32(i))
// 	}
// 	Output = int(hi + lo)
// }

// fn BenchmarkMul64(b *testing.B) {
// 	var hi, lo u64
// 	for i := 0; i < b.N; i += 1 {
// 		hi, lo = Mul64(u64(Input), u64(i))
// 	}
// 	Output = int(hi + lo)
// }

// fn BenchmarkDiv(b *testing.B) {
// 	var q, r uint
// 	for i := 0; i < b.N; i += 1 {
// 		q, r = Div(1, uint(i), uint(Input))
// 	}
// 	Output = int(q + r)
// }

// fn BenchmarkDiv32(b *testing.B) {
// 	var q, r u32
// 	for i := 0; i < b.N; i += 1 {
// 		q, r = Div32(1, u32(i), u32(Input))
// 	}
// 	Output = int(q + r)
// }

// fn BenchmarkDiv64(b *testing.B) {
// 	var q, r u64
// 	for i := 0; i < b.N; i += 1 {
// 		q, r = Div64(1, u64(i), u64(Input))
// 	}
// 	Output = int(q + r)
// }

// // ----------------------------------------------------------------------------
// // Testing support

// type entry = struct {
// 	nlz, ntz, pop int
// }

// // tab contains results for all u8 values
// var tab [256]entry

// fn init() {
// 	tab[0] = entry{8, 8, 0}
// 	for i := 1; i < len(tab); i += 1 {
// 		// nlz
// 		x := i // x != 0
// 		n := 0
// 		for x&0x80 == 0 {
// 			n++
// 			x <<= 1
// 		}
// 		tab[i].nlz = n

// 		// ntz
// 		x = i // x != 0
// 		n = 0
// 		for x&1 == 0 {
// 			n++
// 			x >>= 1
// 		}
// 		tab[i].ntz = n

// 		// pop
// 		x = i // x != 0
// 		n = 0
// 		for x != 0 {
// 			n += int(x & 1)
// 			x >>= 1
// 		}
// 		tab[i].pop = n
// 	}
// }
