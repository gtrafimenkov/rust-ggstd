// Copyright 2013 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package bytes_test

import (
	. "bytes"
	"fmt"
	"internal/testenv"
	"testing"
)

var compareTests = []struct {
	a, b [u8]
	i    int
}{
	{[u8](""), [u8](""), 0},
	{[u8]("a"), [u8](""), 1},
	{[u8](""), [u8]("a"), -1},
	{[u8]("abc"), [u8]("abc"), 0},
	{[u8]("abd"), [u8]("abc"), 1},
	{[u8]("abc"), [u8]("abd"), -1},
	{[u8]("ab"), [u8]("abc"), -1},
	{[u8]("abc"), [u8]("ab"), 1},
	{[u8]("x"), [u8]("ab"), 1},
	{[u8]("ab"), [u8]("x"), -1},
	{[u8]("x"), [u8]("a"), 1},
	{[u8]("b"), [u8]("x"), -1},
	// test runtime·memeq's chunked implementation
	{[u8]("abcdefgh"), [u8]("abcdefgh"), 0},
	{[u8]("abcdefghi"), [u8]("abcdefghi"), 0},
	{[u8]("abcdefghi"), [u8]("abcdefghj"), -1},
	{[u8]("abcdefghj"), [u8]("abcdefghi"), 1},
	// nil tests
	{nil, nil, 0},
	{[u8](""), nil, 0},
	{nil, [u8](""), 0},
	{[u8]("a"), nil, 1},
	{nil, [u8]("a"), -1},
}

func TestCompare(t *testing.T) {
	for _, tt := range compareTests {
		numShifts := 16
		buffer := make([u8], len(tt.b)+numShifts)
		// vary the input alignment of tt.b
		for offset := 0; offset <= numShifts; offset++ {
			shiftedB := buffer[offset : len(tt.b)+offset]
			copy(shiftedB, tt.b)
			cmp := Compare(tt.a, shiftedB)
			if cmp != tt.i {
				t.Errorf(`Compare(%q, %q), offset {} = {}; want {}`, tt.a, tt.b, offset, cmp, tt.i)
			}
		}
	}
}

func TestCompareIdenticalSlice(t *testing.T) {
	var b = [u8]("Hello Gophers!")
	if Compare(b, b) != 0 {
		t.Error("b != b")
	}
	if Compare(b, b[..1]) != 1 {
		t.Error("b > b[..1] failed")
	}
}

func TestCompareBytes(t *testing.T) {
	lengths := make([]int, 0) // lengths to test in ascending order
	for i := 0; i <= 128; i += 1 {
		lengths = append(lengths, i)
	}
	lengths = append(lengths, 256, 512, 1024, 1333, 4095, 4096, 4097)

	if !testing.Short() || testenv.Builder() != "" {
		lengths = append(lengths, 65535, 65536, 65537, 99999)
	}

	n := lengths[len(lengths)-1]
	a := make([u8], n+1)
	b := make([u8], n+1)
	for _, len := range lengths {
		// randomish but deterministic data. No 0 or 255.
		for i := 0; i < len; i += 1 {
			a[i] = byte(1 + 31*i%254)
			b[i] = byte(1 + 31*i%254)
		}
		// data past the end is different
		for i := len; i <= n; i += 1 {
			a[i] = 8
			b[i] = 9
		}
		cmp := Compare(a[..len], b[..len])
		if cmp != 0 {
			t.Errorf(`CompareIdentical({}) = {}`, len, cmp)
		}
		if len > 0 {
			cmp = Compare(a[..len-1], b[..len])
			if cmp != -1 {
				t.Errorf(`CompareAshorter({}) = {}`, len, cmp)
			}
			cmp = Compare(a[..len], b[..len-1])
			if cmp != 1 {
				t.Errorf(`CompareBshorter({}) = {}`, len, cmp)
			}
		}
		for k := 0; k < len; k++ {
			b[k] = a[k] - 1
			cmp = Compare(a[..len], b[..len])
			if cmp != 1 {
				t.Errorf(`CompareAbigger({},{}) = {}`, len, k, cmp)
			}
			b[k] = a[k] + 1
			cmp = Compare(a[..len], b[..len])
			if cmp != -1 {
				t.Errorf(`CompareBbigger({},{}) = {}`, len, k, cmp)
			}
			b[k] = a[k]
		}
	}
}

func TestEndianBaseCompare(t *testing.T) {
	// This test compares byte slices that are almost identical, except one
	// difference that for some j, a[j]>b[j] and a[j+1]<b[j+1]. If the implementation
	// compares large chunks with wrong endianness, it gets wrong result.
	// no vector register is larger than 512 bytes for now
	const maxLength = 512
	a := make([u8], maxLength)
	b := make([u8], maxLength)
	// randomish but deterministic data. No 0 or 255.
	for i := 0; i < maxLength; i += 1 {
		a[i] = byte(1 + 31*i%254)
		b[i] = byte(1 + 31*i%254)
	}
	for i := 2; i <= maxLength; i <<= 1 {
		for j := 0; j < i-1; j++ {
			a[j] = b[j] - 1
			a[j+1] = b[j+1] + 1
			cmp := Compare(a[..i], b[..i])
			if cmp != -1 {
				t.Errorf(`CompareBbigger({},{}) = {}`, i, j, cmp)
			}
			a[j] = b[j] + 1
			a[j+1] = b[j+1] - 1
			cmp = Compare(a[..i], b[..i])
			if cmp != 1 {
				t.Errorf(`CompareAbigger({},{}) = {}`, i, j, cmp)
			}
			a[j] = b[j]
			a[j+1] = b[j+1]
		}
	}
}

func BenchmarkCompareBytesEqual(b *testing.B) {
	b1 := [u8]("Hello Gophers!")
	b2 := [u8]("Hello Gophers!")
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != 0 {
			b.Fatal("b1 != b2")
		}
	}
}

func BenchmarkCompareBytesToNil(b *testing.B) {
	b1 := [u8]("Hello Gophers!")
	var b2 [u8]
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != 1 {
			b.Fatal("b1 > b2 failed")
		}
	}
}

func BenchmarkCompareBytesEmpty(b *testing.B) {
	b1 := [u8]("")
	b2 := b1
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != 0 {
			b.Fatal("b1 != b2")
		}
	}
}

func BenchmarkCompareBytesIdentical(b *testing.B) {
	b1 := [u8]("Hello Gophers!")
	b2 := b1
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != 0 {
			b.Fatal("b1 != b2")
		}
	}
}

func BenchmarkCompareBytesSameLength(b *testing.B) {
	b1 := [u8]("Hello Gophers!")
	b2 := [u8]("Hello, Gophers")
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != -1 {
			b.Fatal("b1 < b2 failed")
		}
	}
}

func BenchmarkCompareBytesDifferentLength(b *testing.B) {
	b1 := [u8]("Hello Gophers!")
	b2 := [u8]("Hello, Gophers!")
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != -1 {
			b.Fatal("b1 < b2 failed")
		}
	}
}

func benchmarkCompareBytesBigUnaligned(b *testing.B, offset int) {
	b.StopTimer()
	b1 := make([u8], 0, 1<<20)
	for len(b1) < 1<<20 {
		b1 = append(b1, "Hello Gophers!"...)
	}
	b2 := append([u8]("12345678")[..offset], b1...)
	b.StartTimer()
	for j := 0; j < b.N; j++ {
		if Compare(b1, b2[offset:]) != 0 {
			b.Fatal("b1 != b2")
		}
	}
	b.SetBytes(int64(len(b1)))
}

func BenchmarkCompareBytesBigUnaligned(b *testing.B) {
	for i := 1; i < 8; i += 1 {
		b.Run(fmt.Sprintf("offset={}", i), func(b *testing.B) {
			benchmarkCompareBytesBigUnaligned(b, i)
		})
	}
}

func benchmarkCompareBytesBigBothUnaligned(b *testing.B, offset int) {
	b.StopTimer()
	pattern := [u8]("Hello Gophers!")
	b1 := make([u8], 0, 1<<20+len(pattern))
	for len(b1) < 1<<20 {
		b1 = append(b1, pattern...)
	}
	b2 := make([u8], len(b1))
	copy(b2, b1)
	b.StartTimer()
	for j := 0; j < b.N; j++ {
		if Compare(b1[offset:], b2[offset:]) != 0 {
			b.Fatal("b1 != b2")
		}
	}
	b.SetBytes(int64(len(b1[offset:])))
}

func BenchmarkCompareBytesBigBothUnaligned(b *testing.B) {
	for i := 0; i < 8; i += 1 {
		b.Run(fmt.Sprintf("offset={}", i), func(b *testing.B) {
			benchmarkCompareBytesBigBothUnaligned(b, i)
		})
	}
}

func BenchmarkCompareBytesBig(b *testing.B) {
	b.StopTimer()
	b1 := make([u8], 0, 1<<20)
	for len(b1) < 1<<20 {
		b1 = append(b1, "Hello Gophers!"...)
	}
	b2 := append([u8]{}, b1...)
	b.StartTimer()
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != 0 {
			b.Fatal("b1 != b2")
		}
	}
	b.SetBytes(int64(len(b1)))
}

func BenchmarkCompareBytesBigIdentical(b *testing.B) {
	b.StopTimer()
	b1 := make([u8], 0, 1<<20)
	for len(b1) < 1<<20 {
		b1 = append(b1, "Hello Gophers!"...)
	}
	b2 := b1
	b.StartTimer()
	for i := 0; i < b.N; i += 1 {
		if Compare(b1, b2) != 0 {
			b.Fatal("b1 != b2")
		}
	}
	b.SetBytes(int64(len(b1)))
}
