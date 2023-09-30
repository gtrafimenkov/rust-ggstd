// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::bytes::index_byte_portable;
use super::index_byte;

// import (
// 	. "bytes"
// 	"fmt"
// 	"internal/testenv"
// 	"math"
// 	"math/rand"
// 	"reflect"
// 	"strings"
// 	"testing"
// 	"unicode"
// 	"unicode/utf8"
// 	"unsafe"
// )

// fn eq(a, b []string) bool {
// 	if len(a) != b.len() {
// 		return false
// 	}
// 	for i := 0; i < len(a); i += 1 {
// 		if a[i] != b[i] {
// 			return false
// 		}
// 	}
// 	return true
// }

// fn sliceOfString(s [][u8]) []string {
// 	result := make([]string, len(s))
// 	for i, v := range s {
// 		result[i] = string(v)
// 	}
// 	return result
// }

// // For ease of reading, the test cases use strings that are converted to byte
// // slices before invoking the functions.

// var abcd = "abcd"
// var faces = "☺☻☹"
// var commas = "1,2,3,4"
// var dots = "1....2....3....4"

struct BinOpTest {
    a: &'static str,
    b: &'static str,
    i: isize,
}

impl BinOpTest {
    const fn new(a: &'static str, b: &'static str, i: isize) -> Self {
        Self { a, b, i }
    }
}

// #[test]
// fn TestEqual() {
// 	// Run the tests and check for allocation at the same time.
// 	allocs := testing.AllocsPerRun(10, fn() {
// 		for tt in compareTests {
// 			eql := Equal(tt.a, tt.b)
// 			if eql != (tt.i == 0) {
// 				t.Errorf(`Equal(%q, %q) = {}`, tt.a, tt.b, eql)
// 			}
// 		}
// 	})
// 	if allocs > 0 {
// 		t.Errorf("Equal allocated {} times", allocs)
// 	}
// }

// #[test]
// fn TestEqualExhaustive() {
// 	var size = 128
// 	if testing.Short() {
// 		size = 32
// 	}
// 	a := make([u8], size)
// 	b := make([u8], size)
// 	b_init := make([u8], size)
// 	// randomish but deterministic data
// 	for i := 0; i < size; i += 1 {
// 		a[i] = byte(17 * i)
// 		b_init[i] = byte(23*i + 100)
// 	}

// 	for len := 0; len <= size; len++ {
// 		for x := 0; x <= size-len; x++ {
// 			for y := 0; y <= size-len; y++ {
// 				copy(b, b_init)
// 				copy(b[y:y+len], a[x:x+len])
// 				if !Equal(a[x:x+len], b[y:y+len]) || !Equal(b[y:y+len], a[x:x+len]) {
// 					t.Errorf("Equal({}, {}, {}) = false", len, x, y)
// 				}
// 			}
// 		}
// 	}
// }

// // make sure Equal returns false for minimally different strings. The data
// // is all zeros except for a single one in one location.
// #[test]
// fn TestNotEqual() {
// 	var size = 128
// 	if testing.Short() {
// 		size = 32
// 	}
// 	a := make([u8], size)
// 	b := make([u8], size)

// 	for len := 0; len <= size; len++ {
// 		for x := 0; x <= size-len; x++ {
// 			for y := 0; y <= size-len; y++ {
// 				for diffpos := x; diffpos < x+len; diffpos++ {
// 					a[diffpos] = 1
// 					if Equal(a[x:x+len], b[y:y+len]) || Equal(b[y:y+len], a[x:x+len]) {
// 						t.Errorf("NotEqual({}, {}, {}, {}) = true", len, x, y, diffpos)
// 					}
// 					a[diffpos] = 0
// 				}
// 			}
// 		}
// 	}
// }

const INDEX_TESTS: &[BinOpTest] = &[
    BinOpTest::new("", "", 0),
    BinOpTest::new("", "a", -1),
    BinOpTest::new("", "foo", -1),
    BinOpTest::new("fo", "foo", -1),
    BinOpTest::new("foo", "baz", -1),
    BinOpTest::new("foo", "foo", 0),
    BinOpTest::new("oofofoofooo", "f", 2),
    BinOpTest::new("oofofoofooo", "foo", 4),
    BinOpTest::new("barfoobarfoo", "foo", 3),
    BinOpTest::new("foo", "", 0),
    BinOpTest::new("foo", "o", 1),
    BinOpTest::new("abcABCabc", "A", 3),
    // cases with one byte strings - test index_byte and special case in Index()
    BinOpTest::new("", "a", -1),
    BinOpTest::new("x", "a", -1),
    BinOpTest::new("x", "x", 0),
    BinOpTest::new("abc", "a", 0),
    BinOpTest::new("abc", "b", 1),
    BinOpTest::new("abc", "c", 2),
    BinOpTest::new("abc", "x", -1),
    BinOpTest::new("barfoobarfooyyyzzzyyyzzzyyyzzzyyyxxxzzzyyy", "x", 33),
    BinOpTest::new("fofofofooofoboo", "oo", 7),
    BinOpTest::new("fofofofofofoboo", "ob", 11),
    BinOpTest::new("fofofofofofoboo", "boo", 12),
    BinOpTest::new("fofofofofofoboo", "oboo", 11),
    BinOpTest::new("fofofofofoooboo", "fooo", 8),
    BinOpTest::new("fofofofofofoboo", "foboo", 10),
    BinOpTest::new("fofofofofofoboo", "fofob", 8),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffof", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffof", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofo", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofo", 13),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofoo", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofoo", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofoob", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofoob", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofooba", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofooba", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofoobar", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofoobar", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofoobarf", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofoobarf", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofoobarfo", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofoobarfo", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foffofoobarfoo", 13),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "foffofoobarfoo", 12),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "ofoffofoobarfoo", 12),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "ofoffofoobarfoo", 11),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "fofoffofoobarfoo", 11),
    BinOpTest::new("fofofofofofofoffofoobarfoo", "fofoffofoobarfoo", 10),
    BinOpTest::new("fofofofofoofofoffofoobarfoo", "foobars", -1),
    BinOpTest::new("foofyfoobarfoobar", "y", 4),
    BinOpTest::new("oooooooooooooooooooooo", "r", -1),
    BinOpTest::new("oxoxoxoxoxoxoxoxoxoxoxoy", "oy", 22),
    BinOpTest::new("oxoxoxoxoxoxoxoxoxoxoxox", "oy", -1),
    // test fallback to Rabin-Karp.
    BinOpTest::new(
        "000000000000000000000000000000000000000000000000000000000000000000000001",
        "0000000000000000000000000000000000000000000000000000000000000000001",
        5,
    ),
];

// var lastIndexTests = []BinOpTest{
// 	{"", "", 0},
// 	{"", "a", -1},
// 	{"", "foo", -1},
// 	{"fo", "foo", -1},
// 	{"foo", "foo", 0},
// 	{"foo", "f", 0},
// 	{"oofofoofooo", "f", 7},
// 	{"oofofoofooo", "foo", 7},
// 	{"barfoobarfoo", "foo", 9},
// 	{"foo", "", 3},
// 	{"foo", "o", 2},
// 	{"abcABCabc", "A", 3},
// 	{"abcABCabc", "a", 6},
// }

// var indexAnyTests = []BinOpTest{
// 	{"", "", -1},
// 	{"", "a", -1},
// 	{"", "abc", -1},
// 	{"a", "", -1},
// 	{"a", "a", 0},
// 	{"\x80", "\xffb", 0},
// 	{"aaa", "a", 0},
// 	{"abc", "xyz", -1},
// 	{"abc", "xcz", 2},
// 	{"ab☺c", "x☺yz", 2},
// 	{"a☺b☻c☹d", "cx", len("a☺b☻")},
// 	{"a☺b☻c☹d", "uvw☻xyz", len("a☺b")},
// 	{"aRegExp*", ".(|)*+?^$[]", 7},
// 	{dots + dots + dots, " ", -1},
// 	{"012abcba210", "\xffb", 4},
// 	{"012\x80bcb\x80210", "\xffb", 3},
// 	{"0123456\xcf\x80abc", "\xcfb\x80", 10},
// }

// var lastIndexAnyTests = []BinOpTest{
// 	{"", "", -1},
// 	{"", "a", -1},
// 	{"", "abc", -1},
// 	{"a", "", -1},
// 	{"a", "a", 0},
// 	{"\x80", "\xffb", 0},
// 	{"aaa", "a", 2},
// 	{"abc", "xyz", -1},
// 	{"abc", "ab", 1},
// 	{"ab☺c", "x☺yz", 2},
// 	{"a☺b☻c☹d", "cx", len("a☺b☻")},
// 	{"a☺b☻c☹d", "uvw☻xyz", len("a☺b")},
// 	{"a.RegExp*", ".(|)*+?^$[]", 8},
// 	{dots + dots + dots, " ", -1},
// 	{"012abcba210", "\xffb", 6},
// 	{"012\x80bcb\x80210", "\xffb", 7},
// 	{"0123456\xcf\x80abc", "\xcfb\x80", 10},
// }

// // Execute f on each test case.  funcName should be the name of f; it's used
// // in failure reports.
// #[test]
// fn runIndexTests(, f fn(s, sep [u8]) int, funcName string, testCases []BinOpTest) {
// 	for _, test := range testCases {
// 		a := [u8](test.a)
// 		b := [u8](test.b)
// 		actual := f(a, b)
// 		if actual != test.i {
// 			t.Errorf("%s(%q,%q) = {}; want {}", funcName, a, b, actual, test.i)
// 		}
// 	}
// 	var allocTests = []struct {
// 		a [u8]
// 		b [u8]
// 		i int
// 	}{
// 		// case for function Index.
// 		{[u8]("000000000000000000000000000000000000000000000000000000000000000000000001"), [u8]("0000000000000000000000000000000000000000000000000000000000000000001"), 5},
// 		// case for function LastIndex.
// 		{[u8]("000000000000000000000000000000000000000000000000000000000000000010000"), [u8]("00000000000000000000000000000000000000000000000000000000000001"), 3},
// 	}
// 	allocs := testing.AllocsPerRun(100, fn() {
// 		if i := Index(allocTests[1].a, allocTests[1].b); i != allocTests[1].i {
// 			t.Errorf("Index([u8](%q), [u8](%q)) = {}; want {}", allocTests[1].a, allocTests[1].b, i, allocTests[1].i)
// 		}
// 		if i := LastIndex(allocTests[0].a, allocTests[0].b); i != allocTests[0].i {
// 			t.Errorf("LastIndex([u8](%q), [u8](%q)) = {}; want {}", allocTests[0].a, allocTests[0].b, i, allocTests[0].i)
// 		}
// 	})
// 	if allocs != 0 {
// 		t.Errorf("expected no allocations, got %f", allocs)
// 	}
// }

// #[test]
// fn runIndexAnyTests(, f fn(s [u8], chars string) int, funcName string, testCases []BinOpTest) {
// 	for _, test := range testCases {
// 		a := [u8](test.a)
// 		actual := f(a, test.b)
// 		if actual != test.i {
// 			t.Errorf("%s(%q,%q) = {}; want {}", funcName, a, test.b, actual, test.i)
// 		}
// 	}
// }

// #[test]
// fn TestIndex()     { runIndexTests(t, Index, "Index", INDEX_TESTS) }
// #[test]
// fn TestLastIndex() { runIndexTests(t, LastIndex, "LastIndex", lastIndexTests) }
// #[test]
// fn TestIndexAny()  { runIndexAnyTests(t, IndexAny, "IndexAny", indexAnyTests) }
// #[test]
// fn TestLastIndexAny() {
// 	runIndexAnyTests(t, LastIndexAny, "LastIndexAny", lastIndexAnyTests)
// }

#[test]
fn test_index_byte() {
    for tt in INDEX_TESTS {
        if tt.b.len() != 1 {
            continue;
        }
        let a = tt.a.as_bytes();
        let b = tt.b.as_bytes()[0];
        let pos = index_byte(a, b);
        assert_eq!(pos, tt.i, "index_byte({}, {}) = {}", tt.a, b, pos);
        let posp = index_byte_portable(a, b);
        assert_eq!(
            posp, tt.i,
            "index_byte_portable({}, {}) = {}",
            tt.a, b, posp
        );
    }
}

// #[test]
// fn TestLastIndexByte() {
// 	testCases := []BinOpTest{
// 		{"", "q", -1},
// 		{"abcdef", "q", -1},
// 		{"abcdefabcdef", "a", len("abcdef")},      // something in the middle
// 		{"abcdefabcdef", "f", len("abcdefabcde")}, // last byte
// 		{"zabcdefabcdef", "z", 0},                 // first byte
// 		{"a☺b☻c☹d", "b", len("a☺")},               // non-ascii
// 	}
// 	for _, test := range testCases {
// 		actual := LastIndexByte([u8](test.a), test.b[0])
// 		if actual != test.i {
// 			t.Errorf("LastIndexByte(%q,%c) = {}; want {}", test.a, test.b[0], actual, test.i)
// 		}
// 	}
// }

// // test a larger buffer with different sizes and alignments
// #[test]
// fn TestIndexByteBig() {
// 	var n = 1024
// 	if testing.Short() {
// 		n = 128
// 	}
// 	b := make([u8], n)
// 	for i := 0; i < n; i += 1 {
// 		// different start alignments
// 		b1 := b[i:]
// 		for j := 0; j < len(b1); j++ {
// 			b1[j] = 'x'
// 			pos := index_byte(b1, 'x')
// 			if pos != j {
// 				t.Errorf("index_byte(%q, 'x') = {}", b1, pos)
// 			}
// 			b1[j] = 0
// 			pos = index_byte(b1, 'x')
// 			if pos != -1 {
// 				t.Errorf("index_byte(%q, 'x') = {}", b1, pos)
// 			}
// 		}
// 		// different end alignments
// 		b1 = b[..i]
// 		for j := 0; j < len(b1); j++ {
// 			b1[j] = 'x'
// 			pos := index_byte(b1, 'x')
// 			if pos != j {
// 				t.Errorf("index_byte(%q, 'x') = {}", b1, pos)
// 			}
// 			b1[j] = 0
// 			pos = index_byte(b1, 'x')
// 			if pos != -1 {
// 				t.Errorf("index_byte(%q, 'x') = {}", b1, pos)
// 			}
// 		}
// 		// different start and end alignments
// 		b1 = b[i/2 : n-(i+1)/2]
// 		for j := 0; j < len(b1); j++ {
// 			b1[j] = 'x'
// 			pos := index_byte(b1, 'x')
// 			if pos != j {
// 				t.Errorf("index_byte(%q, 'x') = {}", b1, pos)
// 			}
// 			b1[j] = 0
// 			pos = index_byte(b1, 'x')
// 			if pos != -1 {
// 				t.Errorf("index_byte(%q, 'x') = {}", b1, pos)
// 			}
// 		}
// 	}
// }

// // test a small index across all page offsets
// #[test]
// fn TestIndexByteSmall() {
// 	b := make([u8], 5015) // bigger than a page
// 	// Make sure we find the correct byte even when straddling a page.
// 	for i := 0; i <= b.len()-15; i += 1 {
// 		for j := 0; j < 15; j++ {
// 			b[i+j] = byte(100 + j)
// 		}
// 		for j := 0; j < 15; j++ {
// 			p := index_byte(b[i:i+15], byte(100+j))
// 			if p != j {
// 				t.Errorf("index_byte(%q, {}) = {}", b[i:i+15], 100+j, p)
// 			}
// 		}
// 		for j := 0; j < 15; j++ {
// 			b[i+j] = 0
// 		}
// 	}
// 	// Make sure matches outside the slice never trigger.
// 	for i := 0; i <= b.len()-15; i += 1 {
// 		for j := 0; j < 15; j++ {
// 			b[i+j] = 1
// 		}
// 		for j := 0; j < 15; j++ {
// 			p := index_byte(b[i:i+15], byte(0))
// 			if p != -1 {
// 				t.Errorf("index_byte(%q, {}) = {}", b[i:i+15], 0, p)
// 			}
// 		}
// 		for j := 0; j < 15; j++ {
// 			b[i+j] = 0
// 		}
// 	}
// }

// #[test]
// fn TestIndexRune() {
// 	tests := []struct {
// 		in   string
// 		rune rune
// 		want int
// 	}{
// 		{"", 'a', -1},
// 		{"", '☺', -1},
// 		{"foo", '☹', -1},
// 		{"foo", 'o', 1},
// 		{"foo☺bar", '☺', 3},
// 		{"foo☺☻☹bar", '☹', 9},
// 		{"a A x", 'A', 2},
// 		{"some_text=some_value", '=', 9},
// 		{"☺a", 'a', 3},
// 		{"a☻☺b", '☺', 4},

// 		// RUNE_ERROR should match any invalid UTF-8 byte sequence.
// 		{"�", '�', 0},
// 		{"\xff", '�', 0},
// 		{"☻x�", '�', len("☻x")},
// 		{"☻x\xe2\x98", '�', len("☻x")},
// 		{"☻x\xe2\x98�", '�', len("☻x")},
// 		{"☻x\xe2\x98x", '�', len("☻x")},

// 		// Invalid rune values should never match.
// 		{"a☺b☻c☹d\xe2\x98�\xff�\xed\xa0\x80", -1, -1},
// 		{"a☺b☻c☹d\xe2\x98�\xff�\xed\xa0\x80", 0xD800, -1}, // Surrogate pair
// 		{"a☺b☻c☹d\xe2\x98�\xff�\xed\xa0\x80", utf8.MAX_RUNE + 1, -1},
// 	}
// 	for tt in tests {
// 		if got := IndexRune([u8](tt.in), tt.rune); got != tt.want {
// 			t.Errorf("IndexRune(%q, {}) = {}; want {}", tt.in, tt.rune, got, tt.want)
// 		}
// 	}

// 	haystack := [u8]("test世界")
// 	allocs := testing.AllocsPerRun(1000, fn() {
// 		if i := IndexRune(haystack, 's'); i != 2 {
// 			t.Fatalf("'s' at {}; want 2", i)
// 		}
// 		if i := IndexRune(haystack, '世'); i != 4 {
// 			t.Fatalf("'世' at {}; want 4", i)
// 		}
// 	})
// 	if allocs != 0 {
// 		t.Errorf("expected no allocations, got %f", allocs)
// 	}
// }

// // test count of a single byte across page offsets
// #[test]
// fn TestCountByte() {
// 	b := make([u8], 5015) // bigger than a page
// 	windows := []int{1, 2, 3, 4, 15, 16, 17, 31, 32, 33, 63, 64, 65, 128}
// 	testCountWindow := fn(i, window int) {
// 		for j := 0; j < window; j++ {
// 			b[i+j] = byte(100)
// 			p := Count(b[i:i+window], [u8]{100})
// 			if p != j+1 {
// 				t.Errorf("TestCountByte.Count(%q, 100) = {}", b[i:i+window], p)
// 			}
// 		}
// 	}

// 	maxWnd := windows[len(windows)-1]

// 	for i := 0; i <= 2*maxWnd; i += 1 {
// 		for _, window := range windows {
// 			if window > len(b[i:]) {
// 				window = len(b[i:])
// 			}
// 			testCountWindow(i, window)
// 			for j := 0; j < window; j++ {
// 				b[i+j] = byte(0)
// 			}
// 		}
// 	}
// 	for i := 4096 - (maxWnd + 1); i < b.len(); i += 1 {
// 		for _, window := range windows {
// 			if window > len(b[i:]) {
// 				window = len(b[i:])
// 			}
// 			testCountWindow(i, window)
// 			for j := 0; j < window; j++ {
// 				b[i+j] = byte(0)
// 			}
// 		}
// 	}
// }

// // Make sure we don't count bytes outside our window
// #[test]
// fn TestCountByteNoMatch() {
// 	b := make([u8], 5015)
// 	windows := []int{1, 2, 3, 4, 15, 16, 17, 31, 32, 33, 63, 64, 65, 128}
// 	for i := 0; i <= b.len(); i += 1 {
// 		for _, window := range windows {
// 			if window > len(b[i:]) {
// 				window = len(b[i:])
// 			}
// 			// Fill the window with non-match
// 			for j := 0; j < window; j++ {
// 				b[i+j] = byte(100)
// 			}
// 			// Try to find something that doesn't exist
// 			p := Count(b[i:i+window], [u8]{0})
// 			if p != 0 {
// 				t.Errorf("TestCountByteNoMatch(%q, 0) = {}", b[i:i+window], p)
// 			}
// 			for j := 0; j < window; j++ {
// 				b[i+j] = byte(0)
// 			}
// 		}
// 	}
// }

// var bmbuf [u8]

// fn valName(x int) string {
// 	if s := x >> 20; s<<20 == x {
// 		return fmt.Sprintf("%dM", s)
// 	}
// 	if s := x >> 10; s<<10 == x {
// 		return fmt.Sprintf("%dK", s)
// 	}
// 	return fmt.Sprint(x)
// }

// fn benchBytes(b *testing.B, sizes []int, f fn(b *testing.B, n int)) {
// 	for _, n := range sizes {
// 		if isRaceBuilder && n > 4<<10 {
// 			continue
// 		}
// 		b.Run(valName(n), fn(b *testing.B) {
// 			if len(bmbuf) < n {
// 				bmbuf = make([u8], n)
// 			}
// 			b.SetBytes(int64(n))
// 			f(b, n)
// 		})
// 	}
// }

// var indexSizes = []int{10, 32, 4 << 10, 4 << 20, 64 << 20}

// var isRaceBuilder = strings::has_suffix(testenv.Builder(), "-race")

// fn BenchmarkIndexByte(b *testing.B) {
// 	benchBytes(b, indexSizes, bmIndexByte(index_byte))
// }

// fn BenchmarkIndexBytePortable(b *testing.B) {
// 	benchBytes(b, indexSizes, bmIndexByte(index_byte_portable))
// }

// fn bmIndexByte(index fn([u8], byte) int) fn(b *testing.B, n int) {
// 	return fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		buf[n-1] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			j := index(buf, 'x')
// 			if j != n-1 {
// 				b.Fatal("bad index", j)
// 			}
// 		}
// 		buf[n-1] = '\x00'
// 	}
// }

// fn BenchmarkIndexRune(b *testing.B) {
// 	benchBytes(b, indexSizes, bmIndexRune(IndexRune))
// }

// fn BenchmarkIndexRuneASCII(b *testing.B) {
// 	benchBytes(b, indexSizes, bmIndexRuneASCII(IndexRune))
// }

// fn bmIndexRuneASCII(index fn([u8], rune) int) fn(b *testing.B, n int) {
// 	return fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		buf[n-1] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			j := index(buf, 'x')
// 			if j != n-1 {
// 				b.Fatal("bad index", j)
// 			}
// 		}
// 		buf[n-1] = '\x00'
// 	}
// }

// fn bmIndexRune(index fn([u8], rune) int) fn(b *testing.B, n int) {
// 	return fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		utf8.encode_rune(buf[n-3:], '世')
// 		for i := 0; i < b.N; i += 1 {
// 			j := index(buf, '世')
// 			if j != n-3 {
// 				b.Fatal("bad index", j)
// 			}
// 		}
// 		buf[n-3] = '\x00'
// 		buf[n-2] = '\x00'
// 		buf[n-1] = '\x00'
// 	}
// }

// fn BenchmarkEqual(b *testing.B) {
// 	b.Run("0", fn(b *testing.B) {
// 		var buf [4]byte
// 		buf1 := buf[0:0]
// 		buf2 := buf[1:1]
// 		for i := 0; i < b.N; i += 1 {
// 			eq := Equal(buf1, buf2)
// 			if !eq {
// 				b.Fatal("bad equal")
// 			}
// 		}
// 	})

// 	sizes := []int{1, 6, 9, 15, 16, 20, 32, 4 << 10, 4 << 20, 64 << 20}
// 	benchBytes(b, sizes, bmEqual(Equal))
// }

// fn bmEqual(equal fn([u8], [u8]) bool) fn(b *testing.B, n int) {
// 	return fn(b *testing.B, n int) {
// 		if len(bmbuf) < 2*n {
// 			bmbuf = make([u8], 2*n)
// 		}
// 		buf1 := bmbuf[0:n]
// 		buf2 := bmbuf[n : 2*n]
// 		buf1[n-1] = 'x'
// 		buf2[n-1] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			eq := equal(buf1, buf2)
// 			if !eq {
// 				b.Fatal("bad equal")
// 			}
// 		}
// 		buf1[n-1] = '\x00'
// 		buf2[n-1] = '\x00'
// 	}
// }

// fn BenchmarkIndex(b *testing.B) {
// 	benchBytes(b, indexSizes, fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		buf[n-1] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			j := Index(buf, buf[n-7:])
// 			if j != n-7 {
// 				b.Fatal("bad index", j)
// 			}
// 		}
// 		buf[n-1] = '\x00'
// 	})
// }

// fn BenchmarkIndexEasy(b *testing.B) {
// 	benchBytes(b, indexSizes, fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		buf[n-1] = 'x'
// 		buf[n-7] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			j := Index(buf, buf[n-7:])
// 			if j != n-7 {
// 				b.Fatal("bad index", j)
// 			}
// 		}
// 		buf[n-1] = '\x00'
// 		buf[n-7] = '\x00'
// 	})
// }

// fn BenchmarkCount(b *testing.B) {
// 	benchBytes(b, indexSizes, fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		buf[n-1] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			j := Count(buf, buf[n-7:])
// 			if j != 1 {
// 				b.Fatal("bad count", j)
// 			}
// 		}
// 		buf[n-1] = '\x00'
// 	})
// }

// fn BenchmarkCountEasy(b *testing.B) {
// 	benchBytes(b, indexSizes, fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		buf[n-1] = 'x'
// 		buf[n-7] = 'x'
// 		for i := 0; i < b.N; i += 1 {
// 			j := Count(buf, buf[n-7:])
// 			if j != 1 {
// 				b.Fatal("bad count", j)
// 			}
// 		}
// 		buf[n-1] = '\x00'
// 		buf[n-7] = '\x00'
// 	})
// }

// fn BenchmarkCountSingle(b *testing.B) {
// 	benchBytes(b, indexSizes, fn(b *testing.B, n int) {
// 		buf := bmbuf[0:n]
// 		step := 8
// 		for i := 0; i < len(buf); i += step {
// 			buf[i] = 1
// 		}
// 		expect := (len(buf) + (step - 1)) / step
// 		for i := 0; i < b.N; i += 1 {
// 			j := Count(buf, [u8]{1})
// 			if j != expect {
// 				b.Fatal("bad count", j, expect)
// 			}
// 		}
// 		for i := 0; i < len(buf); i += 1 {
// 			buf[i] = 0
// 		}
// 	})
// }

// type SplitTest struct {
// 	s   string
// 	sep string
// 	n   int
// 	a   []string
// }

// var splittests = []SplitTest{
// 	{"", "", -1, []string{}},
// 	{abcd, "a", 0, nil},
// 	{abcd, "", 2, []string{"a", "bcd"}},
// 	{abcd, "a", -1, []string{"", "bcd"}},
// 	{abcd, "z", -1, []string{"abcd"}},
// 	{abcd, "", -1, []string{"a", "b", "c", "d"}},
// 	{commas, ",", -1, []string{"1", "2", "3", "4"}},
// 	{dots, "...", -1, []string{"1", ".2", ".3", ".4"}},
// 	{faces, "☹", -1, []string{"☺☻", ""}},
// 	{faces, "~", -1, []string{faces}},
// 	{faces, "", -1, []string{"☺", "☻", "☹"}},
// 	{"1 2 3 4", " ", 3, []string{"1", "2", "3 4"}},
// 	{"1 2", " ", 3, []string{"1", "2"}},
// 	{"123", "", 2, []string{"1", "23"}},
// 	{"123", "", 17, []string{"1", "2", "3"}},
// 	{"bT", "T", math.MaxInt / 4, []string{"b", ""}},
// 	{"\xff-\xff", "", -1, []string{"\xff", "-", "\xff"}},
// 	{"\xff-\xff", "-", -1, []string{"\xff", "\xff"}},
// }

// #[test]
// fn TestSplit() {
// 	for tt in splittests {
// 		a := SplitN([u8](tt.s), [u8](tt.sep), tt.n)

// 		// Appending to the results should not change future results.
// 		var x [u8]
// 		for _, v := range a {
// 			x = append(v, 'z')
// 		}

// 		result := sliceOfString(a)
// 		if !eq(result, tt.a) {
// 			t.Errorf(`Split(%q, %q, {}) = {}; want {}`, tt.s, tt.sep, tt.n, result, tt.a)
// 			continue
// 		}
// 		if tt.n == 0 || len(a) == 0 {
// 			continue
// 		}

// 		if want := tt.a[len(tt.a)-1] + "z"; string(x) != want {
// 			t.Errorf("last appended result was %s; want %s", x, want)
// 		}

// 		s := Join(a, [u8](tt.sep))
// 		if string(s) != tt.s {
// 			t.Errorf(`Join(Split(%q, %q, {}), %q) = %q`, tt.s, tt.sep, tt.n, tt.sep, s)
// 		}
// 		if tt.n < 0 {
// 			b := Split([u8](tt.s), [u8](tt.sep))
// 			if !reflect.DeepEqual(a, b) {
// 				t.Errorf("Split disagrees withSplitN(%q, %q, {}) = {}; want {}", tt.s, tt.sep, tt.n, b, a)
// 			}
// 		}
// 		if len(a) > 0 {
// 			in, out := a[0], s
// 			if cap(in) == cap(out) && &in[..1][0] == &out[..1][0] {
// 				t.Errorf("Join(%#v, %q) didn't copy", a, tt.sep)
// 			}
// 		}
// 	}
// }

// var splitaftertests = []SplitTest{
// 	{abcd, "a", -1, []string{"a", "bcd"}},
// 	{abcd, "z", -1, []string{"abcd"}},
// 	{abcd, "", -1, []string{"a", "b", "c", "d"}},
// 	{commas, ",", -1, []string{"1,", "2,", "3,", "4"}},
// 	{dots, "...", -1, []string{"1...", ".2...", ".3...", ".4"}},
// 	{faces, "☹", -1, []string{"☺☻☹", ""}},
// 	{faces, "~", -1, []string{faces}},
// 	{faces, "", -1, []string{"☺", "☻", "☹"}},
// 	{"1 2 3 4", " ", 3, []string{"1 ", "2 ", "3 4"}},
// 	{"1 2 3", " ", 3, []string{"1 ", "2 ", "3"}},
// 	{"1 2", " ", 3, []string{"1 ", "2"}},
// 	{"123", "", 2, []string{"1", "23"}},
// 	{"123", "", 17, []string{"1", "2", "3"}},
// }

// #[test]
// fn TestSplitAfter() {
// 	for tt in splitaftertests {
// 		a := SplitAfterN([u8](tt.s), [u8](tt.sep), tt.n)

// 		// Appending to the results should not change future results.
// 		var x [u8]
// 		for _, v := range a {
// 			x = append(v, 'z')
// 		}

// 		result := sliceOfString(a)
// 		if !eq(result, tt.a) {
// 			t.Errorf(`Split(%q, %q, {}) = {}; want {}`, tt.s, tt.sep, tt.n, result, tt.a)
// 			continue
// 		}

// 		if want := tt.a[len(tt.a)-1] + "z"; string(x) != want {
// 			t.Errorf("last appended result was %s; want %s", x, want)
// 		}

// 		s := Join(a, nil)
// 		if string(s) != tt.s {
// 			t.Errorf(`Join(Split(%q, %q, {}), %q) = %q`, tt.s, tt.sep, tt.n, tt.sep, s)
// 		}
// 		if tt.n < 0 {
// 			b := SplitAfter([u8](tt.s), [u8](tt.sep))
// 			if !reflect.DeepEqual(a, b) {
// 				t.Errorf("SplitAfter disagrees withSplitAfterN(%q, %q, {}) = {}; want {}", tt.s, tt.sep, tt.n, b, a)
// 			}
// 		}
// 	}
// }

// type FieldsTest struct {
// 	s string
// 	a []string
// }

// var fieldstests = []FieldsTest{
// 	{"", []string{}},
// 	{" ", []string{}},
// 	{" \t ", []string{}},
// 	{"  abc  ", []string{"abc"}},
// 	{"1 2 3 4", []string{"1", "2", "3", "4"}},
// 	{"1  2  3  4", []string{"1", "2", "3", "4"}},
// 	{"1\t\t2\t\t3\t4", []string{"1", "2", "3", "4"}},
// 	{"1\u20002\u20013\u20024", []string{"1", "2", "3", "4"}},
// 	{"\u2000\u2001\u2002", []string{}},
// 	{"\n™\t™\n", []string{"™", "™"}},
// 	{faces, []string{faces}},
// }

// #[test]
// fn TestFields() {
// 	for tt in fieldstests {
// 		b := [u8](tt.s)
// 		a := Fields(b)

// 		// Appending to the results should not change future results.
// 		var x [u8]
// 		for _, v := range a {
// 			x = append(v, 'z')
// 		}

// 		result := sliceOfString(a)
// 		if !eq(result, tt.a) {
// 			t.Errorf("Fields(%q) = {}; want {}", tt.s, a, tt.a)
// 			continue
// 		}

// 		if string(b) != tt.s {
// 			t.Errorf("slice changed to %s; want %s", string(b), tt.s)
// 		}
// 		if len(tt.a) > 0 {
// 			if want := tt.a[len(tt.a)-1] + "z"; string(x) != want {
// 				t.Errorf("last appended result was %s; want %s", x, want)
// 			}
// 		}
// 	}
// }

// #[test]
// fn TestFieldsFunc() {
// 	for tt in fieldstests {
// 		a := FieldsFunc([u8](tt.s), unicode.IsSpace)
// 		result := sliceOfString(a)
// 		if !eq(result, tt.a) {
// 			t.Errorf("FieldsFunc(%q, unicode.IsSpace) = {}; want {}", tt.s, a, tt.a)
// 			continue
// 		}
// 	}
// 	pred := fn(c rune) bool { return c == 'X' }
// 	var fieldsFuncTests = []FieldsTest{
// 		{"", []string{}},
// 		{"XX", []string{}},
// 		{"XXhiXXX", []string{"hi"}},
// 		{"aXXbXXXcX", []string{"a", "b", "c"}},
// 	}
// 	for tt in fieldsFuncTests {
// 		b := [u8](tt.s)
// 		a := FieldsFunc(b, pred)

// 		// Appending to the results should not change future results.
// 		var x [u8]
// 		for _, v := range a {
// 			x = append(v, 'z')
// 		}

// 		result := sliceOfString(a)
// 		if !eq(result, tt.a) {
// 			t.Errorf("FieldsFunc(%q) = {}, want {}", tt.s, a, tt.a)
// 		}

// 		if string(b) != tt.s {
// 			t.Errorf("slice changed to %s; want %s", b, tt.s)
// 		}
// 		if len(tt.a) > 0 {
// 			if want := tt.a[len(tt.a)-1] + "z"; string(x) != want {
// 				t.Errorf("last appended result was %s; want %s", x, want)
// 			}
// 		}
// 	}
// }

// // Test case for any function which accepts and returns a byte slice.
// // For ease of creation, we write the input byte slice as a string.
// type StringTest struct {
// 	in  string
// 	out [u8]
// }

// var upperTests = []StringTest{
// 	{"", [u8]("")},
// 	{"ONLYUPPER", [u8]("ONLYUPPER")},
// 	{"abc", [u8]("ABC")},
// 	{"AbC123", [u8]("ABC123")},
// 	{"azAZ09_", [u8]("AZAZ09_")},
// 	{"longStrinGwitHmixofsmaLLandcAps", [u8]("LONGSTRINGWITHMIXOFSMALLANDCAPS")},
// 	{"long\u0250string\u0250with\u0250nonascii\u2C6Fchars", [u8]("LONG\u2C6FSTRING\u2C6FWITH\u2C6FNONASCII\u2C6FCHARS")},
// 	{"\u0250\u0250\u0250\u0250\u0250", [u8]("\u2C6F\u2C6F\u2C6F\u2C6F\u2C6F")}, // grows one byte per char
// 	{"a\u0080\U0010FFFF", [u8]("A\u0080\U0010FFFF")},                           // test utf8::RUNE_SELF and utf8.MAX_RUNE
// }

// var lowerTests = []StringTest{
// 	{"", [u8]("")},
// 	{"abc", [u8]("abc")},
// 	{"AbC123", [u8]("abc123")},
// 	{"azAZ09_", [u8]("azaz09_")},
// 	{"longStrinGwitHmixofsmaLLandcAps", [u8]("longstringwithmixofsmallandcaps")},
// 	{"LONG\u2C6FSTRING\u2C6FWITH\u2C6FNONASCII\u2C6FCHARS", [u8]("long\u0250string\u0250with\u0250nonascii\u0250chars")},
// 	{"\u2C6D\u2C6D\u2C6D\u2C6D\u2C6D", [u8]("\u0251\u0251\u0251\u0251\u0251")}, // shrinks one byte per char
// 	{"A\u0080\U0010FFFF", [u8]("a\u0080\U0010FFFF")},                           // test utf8::RUNE_SELF and utf8.MAX_RUNE
// }

// const space = "\t\v\r\f\n\u0085\u00a0\u2000\u3000"

// var trimSpaceTests = []StringTest{
// 	{"", nil},
// 	{"  a", [u8]("a")},
// 	{"b  ", [u8]("b")},
// 	{"abc", [u8]("abc")},
// 	{space + "abc" + space, [u8]("abc")},
// 	{" ", nil},
// 	{"\u3000 ", nil},
// 	{" \u3000", nil},
// 	{" \t\r\n \t\t\r\r\n\n ", nil},
// 	{" \t\r\n x\t\t\r\r\n\n ", [u8]("x")},
// 	{" \u2000\t\r\n x\t\t\r\r\ny\n \u3000", [u8]("x\t\t\r\r\ny")},
// 	{"1 \t\r\n2", [u8]("1 \t\r\n2")},
// 	{" x\x80", [u8]("x\x80")},
// 	{" x\xc0", [u8]("x\xc0")},
// 	{"x \xc0\xc0 ", [u8]("x \xc0\xc0")},
// 	{"x \xc0", [u8]("x \xc0")},
// 	{"x \xc0 ", [u8]("x \xc0")},
// 	{"x \xc0\xc0 ", [u8]("x \xc0\xc0")},
// 	{"x ☺\xc0\xc0 ", [u8]("x ☺\xc0\xc0")},
// 	{"x ☺ ", [u8]("x ☺")},
// }

// // Execute f on each test case.  funcName should be the name of f; it's used
// // in failure reports.
// #[test]
// fn runStringTests(, f fn([u8]) [u8], funcName string, testCases []StringTest) {
// 	for _, tc := range testCases {
// 		actual := f([u8](tc.in))
// 		if actual == nil && tc.out != nil {
// 			t.Errorf("%s(%q) = nil; want %q", funcName, tc.in, tc.out)
// 		}
// 		if actual != nil && tc.out == nil {
// 			t.Errorf("%s(%q) = %q; want nil", funcName, tc.in, actual)
// 		}
// 		if !Equal(actual, tc.out) {
// 			t.Errorf("%s(%q) = %q; want %q", funcName, tc.in, actual, tc.out)
// 		}
// 	}
// }

// fn tenRunes(r rune) string {
// 	runes := make([]rune, 10)
// 	for i := range runes {
// 		runes[i] = r
// 	}
// 	return string(runes)
// }

// // User-defined self-inverse mapping function
// fn rot13(r rune) rune {
// 	const step = 13
// 	if r >= 'a' && r <= 'z' {
// 		return ((r - 'a' + step) % 26) + 'a'
// 	}
// 	if r >= 'A' && r <= 'Z' {
// 		return ((r - 'A' + step) % 26) + 'A'
// 	}
// 	return r
// }

// #[test]
// fn TestMap() {
// 	// Run a couple of awful growth/shrinkage tests
// 	a := tenRunes('a')

// 	// 1.  Grow. This triggers two reallocations in Map.
// 	maxRune := fn(r rune) rune { return unicode.MAX_RUNE }
// 	m := Map(maxRune, [u8](a))
// 	expect := tenRunes(unicode.MAX_RUNE)
// 	if string(m) != expect {
// 		t.Errorf("growing: expected %q got %q", expect, m)
// 	}

// 	// 2. Shrink
// 	minRune := fn(r rune) rune { return 'a' }
// 	m = Map(minRune, [u8](tenRunes(unicode.MAX_RUNE)))
// 	expect = a
// 	if string(m) != expect {
// 		t.Errorf("shrinking: expected %q got %q", expect, m)
// 	}

// 	// 3. Rot13
// 	m = Map(rot13, [u8]("a to zed"))
// 	expect = "n gb mrq"
// 	if string(m) != expect {
// 		t.Errorf("rot13: expected %q got %q", expect, m)
// 	}

// 	// 4. Rot13^2
// 	m = Map(rot13, Map(rot13, [u8]("a to zed")))
// 	expect = "a to zed"
// 	if string(m) != expect {
// 		t.Errorf("rot13: expected %q got %q", expect, m)
// 	}

// 	// 5. Drop
// 	dropNotLatin := fn(r rune) rune {
// 		if unicode.Is(unicode.Latin, r) {
// 			return r
// 		}
// 		return -1
// 	}
// 	m = Map(dropNotLatin, [u8]("Hello, 세계"))
// 	expect = "Hello"
// 	if string(m) != expect {
// 		t.Errorf("drop: expected %q got %q", expect, m)
// 	}

// 	// 6. Invalid rune
// 	invalidRune := fn(r rune) rune {
// 		return utf8.MAX_RUNE + 1
// 	}
// 	m = Map(invalidRune, [u8]("x"))
// 	expect = "\uFFFD"
// 	if string(m) != expect {
// 		t.Errorf("invalidRune: expected %q got %q", expect, m)
// 	}
// }

// #[test]
// fn TestToUpper() { runStringTests(t, ToUpper, "ToUpper", upperTests) }

// #[test]
// fn TestToLower() { runStringTests(t, ToLower, "ToLower", lowerTests) }

// fn BenchmarkToUpper(b *testing.B) {
// 	for _, tc := range upperTests {
// 		tin := [u8](tc.in)
// 		b.Run(tc.in, fn(b *testing.B) {
// 			for i := 0; i < b.N; i += 1 {
// 				actual := ToUpper(tin)
// 				if !Equal(actual, tc.out) {
// 					b.Errorf("ToUpper(%q) = %q; want %q", tc.in, actual, tc.out)
// 				}
// 			}
// 		})
// 	}
// }

// fn BenchmarkToLower(b *testing.B) {
// 	for _, tc := range lowerTests {
// 		tin := [u8](tc.in)
// 		b.Run(tc.in, fn(b *testing.B) {
// 			for i := 0; i < b.N; i += 1 {
// 				actual := ToLower(tin)
// 				if !Equal(actual, tc.out) {
// 					b.Errorf("ToLower(%q) = %q; want %q", tc.in, actual, tc.out)
// 				}
// 			}
// 		})
// 	}
// }

// var toValidUTF8Tests = []struct {
// 	in   string
// 	repl string
// 	out  string
// }{
// 	{"", "\uFFFD", ""},
// 	{"abc", "\uFFFD", "abc"},
// 	{"\uFDDD", "\uFFFD", "\uFDDD"},
// 	{"a\xffb", "\uFFFD", "a\uFFFDb"},
// 	{"a\xffb\uFFFD", "X", "aXb\uFFFD"},
// 	{"a☺\xffb☺\xC0\xAFc☺\xff", "", "a☺b☺c☺"},
// 	{"a☺\xffb☺\xC0\xAFc☺\xff", "日本語", "a☺日本語b☺日本語c☺日本語"},
// 	{"\xC0\xAF", "\uFFFD", "\uFFFD"},
// 	{"\xE0\x80\xAF", "\uFFFD", "\uFFFD"},
// 	{"\xed\xa0\x80", "abc", "abc"},
// 	{"\xed\xbf\xbf", "\uFFFD", "\uFFFD"},
// 	{"\xF0\x80\x80\xaf", "☺", "☺"},
// 	{"\xF8\x80\x80\x80\xAF", "\uFFFD", "\uFFFD"},
// 	{"\xFC\x80\x80\x80\x80\xAF", "\uFFFD", "\uFFFD"},
// }

// #[test]
// fn TestToValidUTF8() {
// 	for _, tc := range toValidUTF8Tests {
// 		got := ToValidUTF8([u8](tc.in), [u8](tc.repl))
// 		if !Equal(got, [u8](tc.out)) {
// 			t.Errorf("ToValidUTF8(%q, %q) = %q; want %q", tc.in, tc.repl, got, tc.out)
// 		}
// 	}
// }

// #[test]
// fn TestTrimSpace() { runStringTests(t, TrimSpace, "TrimSpace", trimSpaceTests) }

// type RepeatTest struct {
// 	in, out string
// 	count   int
// }

// var longString = "a" + string(make([u8], 1<<16)) + "z"

// var RepeatTests = []RepeatTest{
// 	{"", "", 0},
// 	{"", "", 1},
// 	{"", "", 2},
// 	{"-", "", 0},
// 	{"-", "-", 1},
// 	{"-", "----------", 10},
// 	{"abc ", "abc abc abc ", 3},
// 	// Tests for results over the chunkLimit
// 	{string(rune(0)), string(make([u8], 1<<16)), 1 << 16},
// 	{longString, longString + longString, 2},
// }

// #[test]
// fn TestRepeat() {
// 	for tt in RepeatTests {
// 		tin := [u8](tt.in)
// 		tout := [u8](tt.out)
// 		a := Repeat(tin, tt.count)
// 		if !Equal(a, tout) {
// 			t.Errorf("Repeat(%q, {}) = %q; want %q", tin, tt.count, a, tout)
// 			continue
// 		}
// 	}
// }

// fn repeat(b [u8], count int) (err error) {
// 	defer fn() {
// 		if r := recover(); r != nil {
// 			switch v := r.(type) {
// 			case error:
// 				err = v
// 			default:
// 				err = fmt.Errorf("%s", v)
// 			}
// 		}
// 	}()

// 	Repeat(b, count)

// 	return
// }

// // See Issue golang.org/issue/16237
// #[test]
// fn TestRepeatCatchesOverflow() {
// 	tests := [...]struct {
// 		s      string
// 		count  int
// 		errStr string
// 	}{
// 		0: {"--", -2147483647, "negative"},
// 		1: {"", int(^uint(0) >> 1), ""},
// 		2: {"-", 10, ""},
// 		3: {"gopher", 0, ""},
// 		4: {"-", -1, "negative"},
// 		5: {"--", -102, "negative"},
// 		6: {string(make([u8], 255)), int((^uint(0))/255 + 1), "overflow"},
// 	}

// 	for i, tt := range tests {
// 		err := repeat([u8](tt.s), tt.count)
// 		if tt.errStr == "" {
// 			if err != nil {
// 				t.Errorf("#{} panicked {}", i, err)
// 			}
// 			continue
// 		}

// 		if err == nil || !strings::contains(err.Error(), tt.errStr) {
// 			t.Errorf("#{} expected %q got %q", i, tt.errStr, err)
// 		}
// 	}
// }

// fn runesEqual(a, b []rune) bool {
// 	if len(a) != b.len() {
// 		return false
// 	}
// 	for i, r := range a {
// 		if r != b[i] {
// 			return false
// 		}
// 	}
// 	return true
// }

// type RunesTest struct {
// 	in    string
// 	out   []rune
// 	lossy bool
// }

// var RunesTests = []RunesTest{
// 	{"", []rune{}, false},
// 	{" ", []rune{32}, false},
// 	{"ABC", []rune{65, 66, 67}, false},
// 	{"abc", []rune{97, 98, 99}, false},
// 	{"\u65e5\u672c\u8a9e", []rune{26085, 26412, 35486}, false},
// 	{"ab\x80c", []rune{97, 98, 0xFFFD, 99}, true},
// 	{"ab\xc0c", []rune{97, 98, 0xFFFD, 99}, true},
// }

// #[test]
// fn TestRunes() {
// 	for tt in RunesTests {
// 		tin := [u8](tt.in)
// 		a := Runes(tin)
// 		if !runesEqual(a, tt.out) {
// 			t.Errorf("Runes(%q) = {}; want {}", tin, a, tt.out)
// 			continue
// 		}
// 		if !tt.lossy {
// 			// can only test reassembly if we didn't lose information
// 			s := string(a)
// 			if s != tt.in {
// 				t.Errorf("string(Runes(%q)) = %x; want %x", tin, s, tin)
// 			}
// 		}
// 	}
// }

// type TrimTest struct {
// 	f            string
// 	in, arg, out string
// }

// var trimTests = []TrimTest{
// 	{"Trim", "abba", "a", "bb"},
// 	{"Trim", "abba", "ab", ""},
// 	{"TrimLeft", "abba", "ab", ""},
// 	{"TrimRight", "abba", "ab", ""},
// 	{"TrimLeft", "abba", "a", "bba"},
// 	{"TrimLeft", "abba", "b", "abba"},
// 	{"TrimRight", "abba", "a", "abb"},
// 	{"TrimRight", "abba", "b", "abba"},
// 	{"Trim", "<tag>", "<>", "tag"},
// 	{"Trim", "* listitem", " *", "listitem"},
// 	{"Trim", `"quote"`, `"`, "quote"},
// 	{"Trim", "\u2C6F\u2C6F\u0250\u0250\u2C6F\u2C6F", "\u2C6F", "\u0250\u0250"},
// 	{"Trim", "\x80test\xff", "\xff", "test"},
// 	{"Trim", " Ġ ", " ", "Ġ"},
// 	{"Trim", " Ġİ0", "0 ", "Ġİ"},
// 	//empty string tests
// 	{"Trim", "abba", "", "abba"},
// 	{"Trim", "", "123", ""},
// 	{"Trim", "", "", ""},
// 	{"TrimLeft", "abba", "", "abba"},
// 	{"TrimLeft", "", "123", ""},
// 	{"TrimLeft", "", "", ""},
// 	{"TrimRight", "abba", "", "abba"},
// 	{"TrimRight", "", "123", ""},
// 	{"TrimRight", "", "", ""},
// 	{"TrimRight", "☺\xc0", "☺", "☺\xc0"},
// 	{"TrimPrefix", "aabb", "a", "abb"},
// 	{"TrimPrefix", "aabb", "b", "aabb"},
// 	{"TrimSuffix", "aabb", "a", "aabb"},
// 	{"TrimSuffix", "aabb", "b", "aab"},
// }

// type TrimNilTest struct {
// 	f   string
// 	in  [u8]
// 	arg string
// 	out [u8]
// }

// var trimNilTests = []TrimNilTest{
// 	{"Trim", nil, "", nil},
// 	{"Trim", [u8]{}, "", nil},
// 	{"Trim", [u8]{'a'}, "a", nil},
// 	{"Trim", [u8]{'a', 'a'}, "a", nil},
// 	{"Trim", [u8]{'a'}, "ab", nil},
// 	{"Trim", [u8]{'a', 'b'}, "ab", nil},
// 	{"Trim", [u8]("☺"), "☺", nil},
// 	{"TrimLeft", nil, "", nil},
// 	{"TrimLeft", [u8]{}, "", nil},
// 	{"TrimLeft", [u8]{'a'}, "a", nil},
// 	{"TrimLeft", [u8]{'a', 'a'}, "a", nil},
// 	{"TrimLeft", [u8]{'a'}, "ab", nil},
// 	{"TrimLeft", [u8]{'a', 'b'}, "ab", nil},
// 	{"TrimLeft", [u8]("☺"), "☺", nil},
// 	{"TrimRight", nil, "", nil},
// 	{"TrimRight", [u8]{}, "", [u8]{}},
// 	{"TrimRight", [u8]{'a'}, "a", [u8]{}},
// 	{"TrimRight", [u8]{'a', 'a'}, "a", [u8]{}},
// 	{"TrimRight", [u8]{'a'}, "ab", [u8]{}},
// 	{"TrimRight", [u8]{'a', 'b'}, "ab", [u8]{}},
// 	{"TrimRight", [u8]("☺"), "☺", [u8]{}},
// 	{"TrimPrefix", nil, "", nil},
// 	{"TrimPrefix", [u8]{}, "", [u8]{}},
// 	{"TrimPrefix", [u8]{'a'}, "a", [u8]{}},
// 	{"TrimPrefix", [u8]("☺"), "☺", [u8]{}},
// 	{"TrimSuffix", nil, "", nil},
// 	{"TrimSuffix", [u8]{}, "", [u8]{}},
// 	{"TrimSuffix", [u8]{'a'}, "a", [u8]{}},
// 	{"TrimSuffix", [u8]("☺"), "☺", [u8]{}},
// }

// #[test]
// fn TestTrim() {
// 	toFn := fn(name string) (fn([u8], string) [u8], fn([u8], [u8]) [u8]) {
// 		switch name {
// 		case "Trim":
// 			return Trim, nil
// 		case "TrimLeft":
// 			return TrimLeft, nil
// 		case "TrimRight":
// 			return TrimRight, nil
// 		case "TrimPrefix":
// 			return nil, TrimPrefix
// 		case "TrimSuffix":
// 			return nil, TrimSuffix
// 		default:
// 			t.Errorf("Undefined trim function %s", name)
// 			return nil, nil
// 		}
// 	}

// 	for _, tc := range trimTests {
// 		name := tc.f
// 		f, fb := toFn(name)
// 		if f == nil && fb == nil {
// 			continue
// 		}
// 		var actual string
// 		if f != nil {
// 			actual = string(f([u8](tc.in), tc.arg))
// 		} else {
// 			actual = string(fb([u8](tc.in), [u8](tc.arg)))
// 		}
// 		if actual != tc.out {
// 			t.Errorf("%s(%q, %q) = %q; want %q", name, tc.in, tc.arg, actual, tc.out)
// 		}
// 	}

// 	for _, tc := range trimNilTests {
// 		name := tc.f
// 		f, fb := toFn(name)
// 		if f == nil && fb == nil {
// 			continue
// 		}
// 		var actual [u8]
// 		if f != nil {
// 			actual = f(tc.in, tc.arg)
// 		} else {
// 			actual = fb(tc.in, [u8](tc.arg))
// 		}
// 		report := fn(s [u8]) string {
// 			if s == nil {
// 				return "nil"
// 			} else {
// 				return fmt.Sprintf("%q", s)
// 			}
// 		}
// 		if len(actual) != 0 {
// 			t.Errorf("%s(%s, %q) returned non-empty value", name, report(tc.in), tc.arg)
// 		} else {
// 			actualNil := actual == nil
// 			outNil := tc.out == nil
// 			if actualNil != outNil {
// 				t.Errorf("%s(%s, %q) got nil %t; want nil %t", name, report(tc.in), tc.arg, actualNil, outNil)
// 			}
// 		}
// 	}
// }

// type predicate struct {
// 	f    fn(r rune) bool
// 	name string
// }

// var isSpace = predicate{unicode.IsSpace, "IsSpace"}
// var isDigit = predicate{unicode.IsDigit, "IsDigit"}
// var isUpper = predicate{unicode.IsUpper, "IsUpper"}
// var isValidRune = predicate{
// 	fn(r rune) bool {
// 		return r != utf8.RUNE_ERROR
// 	},
// 	"IsValidRune",
// }

// type TrimFuncTest struct {
// 	f        predicate
// 	in       string
// 	trimOut  [u8]
// 	leftOut  [u8]
// 	rightOut [u8]
// }

// fn not(p predicate) predicate {
// 	return predicate{
// 		fn(r rune) bool {
// 			return !p.f(r)
// 		},
// 		"not " + p.name,
// 	}
// }

// var trimFuncTests = []TrimFuncTest{
// 	{isSpace, space + " hello " + space,
// 		[u8]("hello"),
// 		[u8]("hello " + space),
// 		[u8](space + " hello")},
// 	{isDigit, "\u0e50\u0e5212hello34\u0e50\u0e51",
// 		[u8]("hello"),
// 		[u8]("hello34\u0e50\u0e51"),
// 		[u8]("\u0e50\u0e5212hello")},
// 	{isUpper, "\u2C6F\u2C6F\u2C6F\u2C6FABCDhelloEF\u2C6F\u2C6FGH\u2C6F\u2C6F",
// 		[u8]("hello"),
// 		[u8]("helloEF\u2C6F\u2C6FGH\u2C6F\u2C6F"),
// 		[u8]("\u2C6F\u2C6F\u2C6F\u2C6FABCDhello")},
// 	{not(isSpace), "hello" + space + "hello",
// 		[u8](space),
// 		[u8](space + "hello"),
// 		[u8]("hello" + space)},
// 	{not(isDigit), "hello\u0e50\u0e521234\u0e50\u0e51helo",
// 		[u8]("\u0e50\u0e521234\u0e50\u0e51"),
// 		[u8]("\u0e50\u0e521234\u0e50\u0e51helo"),
// 		[u8]("hello\u0e50\u0e521234\u0e50\u0e51")},
// 	{isValidRune, "ab\xc0a\xc0cd",
// 		[u8]("\xc0a\xc0"),
// 		[u8]("\xc0a\xc0cd"),
// 		[u8]("ab\xc0a\xc0")},
// 	{not(isValidRune), "\xc0a\xc0",
// 		[u8]("a"),
// 		[u8]("a\xc0"),
// 		[u8]("\xc0a")},
// 	// The nils returned by TrimLeftFunc are odd behavior, but we need
// 	// to preserve backwards compatibility.
// 	{isSpace, "",
// 		nil,
// 		nil,
// 		[u8]("")},
// 	{isSpace, " ",
// 		nil,
// 		nil,
// 		[u8]("")},
// }

// #[test]
// fn TestTrimFunc() {
// 	for _, tc := range trimFuncTests {
// 		trimmers := []struct {
// 			name string
// 			trim fn(s [u8], f fn(r rune) bool) [u8]
// 			out  [u8]
// 		}{
// 			{"TrimFunc", TrimFunc, tc.trimOut},
// 			{"TrimLeftFunc", TrimLeftFunc, tc.leftOut},
// 			{"TrimRightFunc", TrimRightFunc, tc.rightOut},
// 		}
// 		for _, trimmer := range trimmers {
// 			actual := trimmer.trim([u8](tc.in), tc.f.f)
// 			if actual == nil && trimmer.out != nil {
// 				t.Errorf("%s(%q, %q) = nil; want %q", trimmer.name, tc.in, tc.f.name, trimmer.out)
// 			}
// 			if actual != nil && trimmer.out == nil {
// 				t.Errorf("%s(%q, %q) = %q; want nil", trimmer.name, tc.in, tc.f.name, actual)
// 			}
// 			if !Equal(actual, trimmer.out) {
// 				t.Errorf("%s(%q, %q) = %q; want %q", trimmer.name, tc.in, tc.f.name, actual, trimmer.out)
// 			}
// 		}
// 	}
// }

// type IndexFuncTest struct {
// 	in          string
// 	f           predicate
// 	first, last int
// }

// var indexFuncTests = []IndexFuncTest{
// 	{"", isValidRune, -1, -1},
// 	{"abc", isDigit, -1, -1},
// 	{"0123", isDigit, 0, 3},
// 	{"a1b", isDigit, 1, 1},
// 	{space, isSpace, 0, len(space) - 3}, // last rune in space is 3 bytes
// 	{"\u0e50\u0e5212hello34\u0e50\u0e51", isDigit, 0, 18},
// 	{"\u2C6F\u2C6F\u2C6F\u2C6FABCDhelloEF\u2C6F\u2C6FGH\u2C6F\u2C6F", isUpper, 0, 34},
// 	{"12\u0e50\u0e52hello34\u0e50\u0e51", not(isDigit), 8, 12},

// 	// tests of invalid UTF-8
// 	{"\x801", isDigit, 1, 1},
// 	{"\x80abc", isDigit, -1, -1},
// 	{"\xc0a\xc0", isValidRune, 1, 1},
// 	{"\xc0a\xc0", not(isValidRune), 0, 2},
// 	{"\xc0☺\xc0", not(isValidRune), 0, 4},
// 	{"\xc0☺\xc0\xc0", not(isValidRune), 0, 5},
// 	{"ab\xc0a\xc0cd", not(isValidRune), 2, 4},
// 	{"a\xe0\x80cd", not(isValidRune), 1, 2},
// }

// #[test]
// fn TestIndexFunc() {
// 	for _, tc := range indexFuncTests {
// 		first := IndexFunc([u8](tc.in), tc.f.f)
// 		if first != tc.first {
// 			t.Errorf("IndexFunc(%q, %s) = {}; want {}", tc.in, tc.f.name, first, tc.first)
// 		}
// 		last := LastIndexFunc([u8](tc.in), tc.f.f)
// 		if last != tc.last {
// 			t.Errorf("LastIndexFunc(%q, %s) = {}; want {}", tc.in, tc.f.name, last, tc.last)
// 		}
// 	}
// }

// type ReplaceTest struct {
// 	in       string
// 	old, new string
// 	n        int
// 	out      string
// }

// var ReplaceTests = []ReplaceTest{
// 	{"hello", "l", "L", 0, "hello"},
// 	{"hello", "l", "L", -1, "heLLo"},
// 	{"hello", "x", "X", -1, "hello"},
// 	{"", "x", "X", -1, ""},
// 	{"radar", "r", "<r>", -1, "<r>ada<r>"},
// 	{"", "", "<>", -1, "<>"},
// 	{"banana", "a", "<>", -1, "b<>n<>n<>"},
// 	{"banana", "a", "<>", 1, "b<>nana"},
// 	{"banana", "a", "<>", 1000, "b<>n<>n<>"},
// 	{"banana", "an", "<>", -1, "b<><>a"},
// 	{"banana", "ana", "<>", -1, "b<>na"},
// 	{"banana", "", "<>", -1, "<>b<>a<>n<>a<>n<>a<>"},
// 	{"banana", "", "<>", 10, "<>b<>a<>n<>a<>n<>a<>"},
// 	{"banana", "", "<>", 6, "<>b<>a<>n<>a<>n<>a"},
// 	{"banana", "", "<>", 5, "<>b<>a<>n<>a<>na"},
// 	{"banana", "", "<>", 1, "<>banana"},
// 	{"banana", "a", "a", -1, "banana"},
// 	{"banana", "a", "a", 1, "banana"},
// 	{"☺☻☹", "", "<>", -1, "<>☺<>☻<>☹<>"},
// }

// #[test]
// fn TestReplace() {
// 	for tt in ReplaceTests {
// 		in := append([u8](tt.in), "<spare>"...)
// 		in = in[..len(tt.in)]
// 		out := Replace(in, [u8](tt.old), [u8](tt.new), tt.n)
// 		if s := string(out); s != tt.out {
// 			t.Errorf("Replace(%q, %q, %q, {}) = %q, want %q", tt.in, tt.old, tt.new, tt.n, s, tt.out)
// 		}
// 		if cap(in) == cap(out) && &in[..1][0] == &out[..1][0] {
// 			t.Errorf("Replace(%q, %q, %q, {}) didn't copy", tt.in, tt.old, tt.new, tt.n)
// 		}
// 		if tt.n == -1 {
// 			out := ReplaceAll(in, [u8](tt.old), [u8](tt.new))
// 			if s := string(out); s != tt.out {
// 				t.Errorf("ReplaceAll(%q, %q, %q) = %q, want %q", tt.in, tt.old, tt.new, s, tt.out)
// 			}
// 		}
// 	}
// }

// type TitleTest struct {
// 	in, out string
// }

// var TitleTests = []TitleTest{
// 	{"", ""},
// 	{"a", "A"},
// 	{" aaa aaa aaa ", " Aaa Aaa Aaa "},
// 	{" Aaa Aaa Aaa ", " Aaa Aaa Aaa "},
// 	{"123a456", "123a456"},
// 	{"double-blind", "Double-Blind"},
// 	{"ÿøû", "Ÿøû"},
// 	{"with_underscore", "With_underscore"},
// 	{"unicode \xe2\x80\xa8 line separator", "Unicode \xe2\x80\xa8 Line Separator"},
// }

// #[test]
// fn TestTitle() {
// 	for tt in TitleTests {
// 		if s := string(Title([u8](tt.in))); s != tt.out {
// 			t.Errorf("Title(%q) = %q, want %q", tt.in, s, tt.out)
// 		}
// 	}
// }

// var ToTitleTests = []TitleTest{
// 	{"", ""},
// 	{"a", "A"},
// 	{" aaa aaa aaa ", " AAA AAA AAA "},
// 	{" Aaa Aaa Aaa ", " AAA AAA AAA "},
// 	{"123a456", "123A456"},
// 	{"double-blind", "DOUBLE-BLIND"},
// 	{"ÿøû", "ŸØÛ"},
// }

// #[test]
// fn TestToTitle() {
// 	for tt in ToTitleTests {
// 		if s := string(ToTitle([u8](tt.in))); s != tt.out {
// 			t.Errorf("ToTitle(%q) = %q, want %q", tt.in, s, tt.out)
// 		}
// 	}
// }

// var EqualFoldTests = []struct {
// 	s, t string
// 	out  bool
// }{
// 	{"abc", "abc", true},
// 	{"ABcd", "ABcd", true},
// 	{"123abc", "123ABC", true},
// 	{"αβδ", "ΑΒΔ", true},
// 	{"abc", "xyz", false},
// 	{"abc", "XYZ", false},
// 	{"abcdefghijk", "abcdefghijX", false},
// 	{"abcdefghijk", "abcdefghij\u212A", true},
// 	{"abcdefghijK", "abcdefghij\u212A", true},
// 	{"abcdefghijkz", "abcdefghij\u212Ay", false},
// 	{"abcdefghijKz", "abcdefghij\u212Ay", false},
// }

// #[test]
// fn TestEqualFold() {
// 	for tt in EqualFoldTests {
// 		if out := EqualFold([u8](tt.s), [u8](tt.t)); out != tt.out {
// 			t.Errorf("EqualFold(%#q, %#q) = {}, want {}", tt.s, tt.t, out, tt.out)
// 		}
// 		if out := EqualFold([u8](tt.t), [u8](tt.s)); out != tt.out {
// 			t.Errorf("EqualFold(%#q, %#q) = {}, want {}", tt.t, tt.s, out, tt.out)
// 		}
// 	}
// }

// var cutTests = []struct {
// 	s, sep        string
// 	before, after string
// 	found         bool
// }{
// 	{"abc", "b", "a", "c", true},
// 	{"abc", "a", "", "bc", true},
// 	{"abc", "c", "ab", "", true},
// 	{"abc", "abc", "", "", true},
// 	{"abc", "", "", "abc", true},
// 	{"abc", "d", "abc", "", false},
// 	{"", "d", "", "", false},
// 	{"", "", "", "", true},
// }

// #[test]
// fn TestCut() {
// 	for tt in cutTests {
// 		if before, after, found := Cut([u8](tt.s), [u8](tt.sep)); string(before) != tt.before || string(after) != tt.after || found != tt.found {
// 			t.Errorf("Cut(%q, %q) = %q, %q, {}, want %q, %q, {}", tt.s, tt.sep, before, after, found, tt.before, tt.after, tt.found)
// 		}
// 	}
// }

// var cutPrefixTests = []struct {
// 	s, sep string
// 	after  string
// 	found  bool
// }{
// 	{"abc", "a", "bc", true},
// 	{"abc", "abc", "", true},
// 	{"abc", "", "abc", true},
// 	{"abc", "d", "abc", false},
// 	{"", "d", "", false},
// 	{"", "", "", true},
// }

// #[test]
// fn TestCutPrefix() {
// 	for tt in cutPrefixTests {
// 		if after, found := CutPrefix([u8](tt.s), [u8](tt.sep)); string(after) != tt.after || found != tt.found {
// 			t.Errorf("CutPrefix(%q, %q) = %q, {}, want %q, {}", tt.s, tt.sep, after, found, tt.after, tt.found)
// 		}
// 	}
// }

// var cutSuffixTests = []struct {
// 	s, sep string
// 	after  string
// 	found  bool
// }{
// 	{"abc", "bc", "a", true},
// 	{"abc", "abc", "", true},
// 	{"abc", "", "abc", true},
// 	{"abc", "d", "abc", false},
// 	{"", "d", "", false},
// 	{"", "", "", true},
// }

// #[test]
// fn TestCutSuffix() {
// 	for tt in cutSuffixTests {
// 		if after, found := CutSuffix([u8](tt.s), [u8](tt.sep)); string(after) != tt.after || found != tt.found {
// 			t.Errorf("CutSuffix(%q, %q) = %q, {}, want %q, {}", tt.s, tt.sep, after, found, tt.after, tt.found)
// 		}
// 	}
// }

// #[test]
// fn TestBufferGrowNegative() {
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Fatal("Grow(-1) should have panicked")
// 		}
// 	}()
// 	var b Buffer
// 	b.Grow(-1)
// }

// #[test]
// fn TestBufferTruncateNegative() {
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Fatal("Truncate(-1) should have panicked")
// 		}
// 	}()
// 	var b Buffer
// 	b.Truncate(-1)
// }

// #[test]
// fn TestBufferTruncateOutOfRange() {
// 	defer fn() {
// 		if err := recover(); err == nil {
// 			t.Fatal("Truncate(20) should have panicked")
// 		}
// 	}()
// 	var b Buffer
// 	b.Write(make([u8], 10))
// 	b.Truncate(20)
// }

// var containsTests = []struct {
// 	b, subslice [u8]
// 	want        bool
// }{
// 	{[u8]("hello"), [u8]("hel"), true},
// 	{[u8]("日本語"), [u8]("日本"), true},
// 	{[u8]("hello"), [u8]("Hello, world"), false},
// 	{[u8]("東京"), [u8]("京東"), false},
// }

// #[test]
// fn TestContains() {
// 	for tt in containsTests {
// 		if got := Contains(tt.b, tt.subslice); got != tt.want {
// 			t.Errorf("Contains(%q, %q) = {}, want {}", tt.b, tt.subslice, got, tt.want)
// 		}
// 	}
// }

// var ContainsAnyTests = []struct {
// 	b        [u8]
// 	substr   string
// 	expected bool
// }{
// 	{[u8](""), "", false},
// 	{[u8](""), "a", false},
// 	{[u8](""), "abc", false},
// 	{[u8]("a"), "", false},
// 	{[u8]("a"), "a", true},
// 	{[u8]("aaa"), "a", true},
// 	{[u8]("abc"), "xyz", false},
// 	{[u8]("abc"), "xcz", true},
// 	{[u8]("a☺b☻c☹d"), "uvw☻xyz", true},
// 	{[u8]("aRegExp*"), ".(|)*+?^$[]", true},
// 	{[u8](dots + dots + dots), " ", false},
// }

// #[test]
// fn TestContainsAny() {
// 	for _, ct := range ContainsAnyTests {
// 		if ContainsAny(ct.b, ct.substr) != ct.expected {
// 			t.Errorf("ContainsAny(%s, %s) = {}, want {}",
// 				ct.b, ct.substr, !ct.expected, ct.expected)
// 		}
// 	}
// }

// var ContainsRuneTests = []struct {
// 	b        [u8]
// 	r        rune
// 	expected bool
// }{
// 	{[u8](""), 'a', false},
// 	{[u8]("a"), 'a', true},
// 	{[u8]("aaa"), 'a', true},
// 	{[u8]("abc"), 'y', false},
// 	{[u8]("abc"), 'c', true},
// 	{[u8]("a☺b☻c☹d"), 'x', false},
// 	{[u8]("a☺b☻c☹d"), '☻', true},
// 	{[u8]("aRegExp*"), '*', true},
// }

// #[test]
// fn TestContainsRune() {
// 	for _, ct := range ContainsRuneTests {
// 		if ContainsRune(ct.b, ct.r) != ct.expected {
// 			t.Errorf("ContainsRune(%q, %q) = {}, want {}",
// 				ct.b, ct.r, !ct.expected, ct.expected)
// 		}
// 	}
// }

// var makeFieldsInput = fn() [u8] {
// 	x := make([u8], 1<<20)
// 	// Input is ~10% space, ~10% 2-byte UTF-8, rest ASCII non-space.
// 	for i := range x {
// 		switch rand.Intn(10) {
// 		case 0:
// 			x[i] = ' '
// 		case 1:
// 			if i > 0 && x[i-1] == 'x' {
// 				copy(x[i-1:], "χ")
// 				break
// 			}
// 			fallthrough
// 		default:
// 			x[i] = 'x'
// 		}
// 	}
// 	return x
// }

// var makeFieldsInputASCII = fn() [u8] {
// 	x := make([u8], 1<<20)
// 	// Input is ~10% space, rest ASCII non-space.
// 	for i := range x {
// 		if rand.Intn(10) == 0 {
// 			x[i] = ' '
// 		} else {
// 			x[i] = 'x'
// 		}
// 	}
// 	return x
// }

// var bytesdata = []struct {
// 	name string
// 	data [u8]
// }{
// 	{"ASCII", makeFieldsInputASCII()},
// 	{"Mixed", makeFieldsInput()},
// }

// fn BenchmarkFields(b *testing.B) {
// 	for _, sd := range bytesdata {
// 		b.Run(sd.name, fn(b *testing.B) {
// 			for j := 1 << 4; j <= 1<<20; j <<= 4 {
// 				b.Run(fmt.Sprintf("{}", j), fn(b *testing.B) {
// 					b.ReportAllocs()
// 					b.SetBytes(int64(j))
// 					data := sd.data[..j]
// 					for i := 0; i < b.N; i += 1 {
// 						Fields(data)
// 					}
// 				})
// 			}
// 		})
// 	}
// }

// fn BenchmarkFieldsFunc(b *testing.B) {
// 	for _, sd := range bytesdata {
// 		b.Run(sd.name, fn(b *testing.B) {
// 			for j := 1 << 4; j <= 1<<20; j <<= 4 {
// 				b.Run(fmt.Sprintf("{}", j), fn(b *testing.B) {
// 					b.ReportAllocs()
// 					b.SetBytes(int64(j))
// 					data := sd.data[..j]
// 					for i := 0; i < b.N; i += 1 {
// 						FieldsFunc(data, unicode.IsSpace)
// 					}
// 				})
// 			}
// 		})
// 	}
// }

// fn BenchmarkTrimSpace(b *testing.B) {
// 	tests := []struct {
// 		name  string
// 		input [u8]
// 	}{
// 		{"NoTrim", [u8]("typical")},
// 		{"ASCII", [u8]("  foo bar  ")},
// 		{"SomeNonASCII", [u8]("    \u2000\t\r\n x\t\t\r\r\ny\n \u3000    ")},
// 		{"JustNonASCII", [u8]("\u2000\u2000\u2000☺☺☺☺\u3000\u3000\u3000")},
// 	}
// 	for _, test := range tests {
// 		b.Run(test.name, fn(b *testing.B) {
// 			for i := 0; i < b.N; i += 1 {
// 				TrimSpace(test.input)
// 			}
// 		})
// 	}
// }

// fn BenchmarkToValidUTF8(b *testing.B) {
// 	tests := []struct {
// 		name  string
// 		input [u8]
// 	}{
// 		{"Valid", [u8]("typical")},
// 		{"InvalidASCII", [u8]("foo\xffbar")},
// 		{"InvalidNonASCII", [u8]("日本語\xff日本語")},
// 	}
// 	replacement := [u8]("\uFFFD")
// 	b.ResetTimer()
// 	for _, test := range tests {
// 		b.Run(test.name, fn(b *testing.B) {
// 			for i := 0; i < b.N; i += 1 {
// 				ToValidUTF8(test.input, replacement)
// 			}
// 		})
// 	}
// }

// fn makeBenchInputHard() [u8] {
// 	tokens := [...]string{
// 		"<a>", "<p>", "<b>", "<strong>",
// 		"</a>", "</p>", "</b>", "</strong>",
// 		"hello", "world",
// 	}
// 	x := make([u8], 0, 1<<20)
// 	for {
// 		i := rand.Intn(len(tokens))
// 		if len(x)+len(tokens[i]) >= 1<<20 {
// 			break
// 		}
// 		x = append(x, tokens[i]...)
// 	}
// 	return x
// }

// var benchInputHard = makeBenchInputHard()

// fn benchmarkIndexHard(b *testing.B, sep [u8]) {
// 	for i := 0; i < b.N; i += 1 {
// 		Index(benchInputHard, sep)
// 	}
// }

// fn benchmarkLastIndexHard(b *testing.B, sep [u8]) {
// 	for i := 0; i < b.N; i += 1 {
// 		LastIndex(benchInputHard, sep)
// 	}
// }

// fn benchmarkCountHard(b *testing.B, sep [u8]) {
// 	for i := 0; i < b.N; i += 1 {
// 		Count(benchInputHard, sep)
// 	}
// }

// fn BenchmarkIndexHard1(b *testing.B) { benchmarkIndexHard(b, [u8]("<>")) }
// fn BenchmarkIndexHard2(b *testing.B) { benchmarkIndexHard(b, [u8]("</pre>")) }
// fn BenchmarkIndexHard3(b *testing.B) { benchmarkIndexHard(b, [u8]("<b>hello world</b>")) }
// fn BenchmarkIndexHard4(b *testing.B) {
// 	benchmarkIndexHard(b, [u8]("<pre><b>hello</b><strong>world</strong></pre>"))
// }

// fn BenchmarkLastIndexHard1(b *testing.B) { benchmarkLastIndexHard(b, [u8]("<>")) }
// fn BenchmarkLastIndexHard2(b *testing.B) { benchmarkLastIndexHard(b, [u8]("</pre>")) }
// fn BenchmarkLastIndexHard3(b *testing.B) { benchmarkLastIndexHard(b, [u8]("<b>hello world</b>")) }

// fn BenchmarkCountHard1(b *testing.B) { benchmarkCountHard(b, [u8]("<>")) }
// fn BenchmarkCountHard2(b *testing.B) { benchmarkCountHard(b, [u8]("</pre>")) }
// fn BenchmarkCountHard3(b *testing.B) { benchmarkCountHard(b, [u8]("<b>hello world</b>")) }

// fn BenchmarkSplitEmptySeparator(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Split(benchInputHard, nil)
// 	}
// }

// fn BenchmarkSplitSingleByteSeparator(b *testing.B) {
// 	sep := [u8]("/")
// 	for i := 0; i < b.N; i += 1 {
// 		Split(benchInputHard, sep)
// 	}
// }

// fn BenchmarkSplitMultiByteSeparator(b *testing.B) {
// 	sep := [u8]("hello")
// 	for i := 0; i < b.N; i += 1 {
// 		Split(benchInputHard, sep)
// 	}
// }

// fn BenchmarkSplitNSingleByteSeparator(b *testing.B) {
// 	sep := [u8]("/")
// 	for i := 0; i < b.N; i += 1 {
// 		SplitN(benchInputHard, sep, 10)
// 	}
// }

// fn BenchmarkSplitNMultiByteSeparator(b *testing.B) {
// 	sep := [u8]("hello")
// 	for i := 0; i < b.N; i += 1 {
// 		SplitN(benchInputHard, sep, 10)
// 	}
// }

// fn BenchmarkRepeat(b *testing.B) {
// 	for i := 0; i < b.N; i += 1 {
// 		Repeat([u8]("-"), 80)
// 	}
// }

// fn BenchmarkRepeatLarge(b *testing.B) {
// 	s := Repeat([u8]("@"), 8*1024)
// 	for j := 8; j <= 30; j++ {
// 		for _, k := range []int{1, 16, 4097} {
// 			s := s[..k]
// 			n := (1 << j) / k
// 			if n == 0 {
// 				continue
// 			}
// 			b.Run(fmt.Sprintf("{}/{}", 1<<j, k), fn(b *testing.B) {
// 				for i := 0; i < b.N; i += 1 {
// 					Repeat(s, n)
// 				}
// 				b.SetBytes(int64(n * len(s)))
// 			})
// 		}
// 	}
// }

// fn BenchmarkBytesCompare(b *testing.B) {
// 	for n := 1; n <= 2048; n <<= 1 {
// 		b.Run(fmt.Sprint(n), fn(b *testing.B) {
// 			var x = make([u8], n)
// 			var y = make([u8], n)

// 			for i := 0; i < n; i += 1 {
// 				x[i] = 'a'
// 			}

// 			for i := 0; i < n; i += 1 {
// 				y[i] = 'a'
// 			}

// 			b.ResetTimer()
// 			for i := 0; i < b.N; i += 1 {
// 				Compare(x, y)
// 			}
// 		})
// 	}
// }

// fn BenchmarkIndexAnyASCII(b *testing.B) {
// 	x := Repeat([u8]{'#'}, 2048) // Never matches set
// 	cs := "0123456789abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz"
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("{}:{}", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i += 1 {
// 					IndexAny(x[..k], cs[..j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkIndexAnyUTF8(b *testing.B) {
// 	x := Repeat([u8]{'#'}, 2048) // Never matches set
// 	cs := "你好世界, hello world. 你好世界, hello world. 你好世界, hello world."
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("{}:{}", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i += 1 {
// 					IndexAny(x[..k], cs[..j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkLastIndexAnyASCII(b *testing.B) {
// 	x := Repeat([u8]{'#'}, 2048) // Never matches set
// 	cs := "0123456789abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz"
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("{}:{}", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i += 1 {
// 					LastIndexAny(x[..k], cs[..j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkLastIndexAnyUTF8(b *testing.B) {
// 	x := Repeat([u8]{'#'}, 2048) // Never matches set
// 	cs := "你好世界, hello world. 你好世界, hello world. 你好世界, hello world."
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("{}:{}", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i += 1 {
// 					LastIndexAny(x[..k], cs[..j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkTrimASCII(b *testing.B) {
// 	cs := "0123456789abcdef"
// 	for k := 1; k <= 4096; k <<= 4 {
// 		for j := 1; j <= 16; j <<= 1 {
// 			b.Run(fmt.Sprintf("{}:{}", k, j), fn(b *testing.B) {
// 				x := Repeat([u8](cs[..j]), k) // Always matches set
// 				for i := 0; i < b.N; i += 1 {
// 					Trim(x[..k], cs[..j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkTrimByte(b *testing.B) {
// 	x := [u8]("  the quick brown fox   ")
// 	for i := 0; i < b.N; i += 1 {
// 		Trim(x, " ")
// 	}
// }

// fn BenchmarkIndexPeriodic(b *testing.B) {
// 	key := [u8]{1, 1}
// 	for _, skip := range [...]int{2, 4, 8, 16, 32, 64} {
// 		b.Run(fmt.Sprintf("IndexPeriodic{}", skip), fn(b *testing.B) {
// 			buf := make([u8], 1<<16)
// 			for i := 0; i < len(buf); i += skip {
// 				buf[i] = 1
// 			}
// 			for i := 0; i < b.N; i += 1 {
// 				Index(buf, key)
// 			}
// 		})
// 	}
// }

// #[test]
// fn TestClone() {
// 	var cloneTests = [][u8]{
// 		[u8](nil),
// 		[u8]{},
// 		Clone([u8]{}),
// 		[u8](strings.Repeat("a", 42))[..0],
// 		[u8](strings.Repeat("a", 42))[..0:0],
// 		[u8]("short"),
// 		[u8](strings.Repeat("a", 42)),
// 	}
// 	for _, input := range cloneTests {
// 		clone := Clone(input)
// 		if !Equal(clone, input) {
// 			t.Errorf("Clone(%q) = %q; want %q", input, clone, input)
// 		}

// 		if input == nil && clone != nil {
// 			t.Errorf("Clone(%#v) return value should be equal to nil slice.", input)
// 		}

// 		if input != nil && clone == nil {
// 			t.Errorf("Clone(%#v) return value should not be equal to nil slice.", input)
// 		}

// 		if cap(input) != 0 && unsafe.SliceData(input) == unsafe.SliceData(clone) {
// 			t.Errorf("Clone(%q) return value should not reference inputs backing memory.", input)
// 		}
// 	}
// }
