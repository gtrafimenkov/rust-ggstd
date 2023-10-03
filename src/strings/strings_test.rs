// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found input the LICENSE file.

use crate::strings::{contains, index};

// import (
// 	"bytes"
// 	"fmt"
// 	"io"
// 	"math"
// 	"math/rand"
// 	"reflect"
// 	"strconv"
// 	. "strings"
// 	"testing"
// 	"unicode"
// 	"unicode/utf8"
// 	"unsafe"
// )

// fn eq(a, b []string) bool {
// 	if len(a) != len(b) {
// 		return false
// 	}
// 	for i := 0; i < len(a); i++ {
// 		if a[i] != b[i] {
// 			return false
// 		}
// 	}
// 	return true
// }

// var abcd = "abcd"
// var faces = "☺☻☹"
// var commas = "1,2,3,4"
// var dots = "1....2....3....4"

struct IndexTest {
    s: &'static str,
    sep: &'static str,
    out: isize,
}

impl IndexTest {
    const fn new(s: &'static str, sep: &'static str, out: isize) -> Self {
        Self { s, sep, out }
    }
}

const INDEX_TESTS: &[IndexTest] = &[
    IndexTest::new("", "", 0),
    IndexTest::new("", "a", -1),
    IndexTest::new("", "foo", -1),
    IndexTest::new("fo", "foo", -1),
    IndexTest::new("foo", "foo", 0),
    IndexTest::new("oofofoofooo", "f", 2),
    IndexTest::new("oofofoofooo", "foo", 4),
    IndexTest::new("barfoobarfoo", "foo", 3),
    IndexTest::new("foo", "", 0),
    IndexTest::new("foo", "o", 1),
    IndexTest::new("abcABCabc", "A", 3),
    IndexTest::new("jrzm6jjhorimglljrea4w3rlgosts0w2gia17hno2td4qd1jz", "jz", 47),
    IndexTest::new("ekkuk5oft4eq0ocpacknhwouic1uua46unx12l37nioq9wbpnocqks6", "ks6", 52),
    IndexTest::new("999f2xmimunbuyew5vrkla9cpwhmxan8o98ec", "98ec", 33),
    IndexTest::new("9lpt9r98i04k8bz6c6dsrthb96bhi", "96bhi", 24),
    IndexTest::new("55u558eqfaod2r2gu42xxsu631xf0zobs5840vl", "5840vl", 33),
    // cases with one byte strings - test special case input Index()
    IndexTest::new("", "a", -1),
    IndexTest::new("x", "a", -1),
    IndexTest::new("x", "x", 0),
    IndexTest::new("abc", "a", 0),
    IndexTest::new("abc", "b", 1),
    IndexTest::new("abc", "c", 2),
    IndexTest::new("abc", "x", -1),
    // test special cases input Index() for short strings
    IndexTest::new("", "ab", -1),
    IndexTest::new("bc", "ab", -1),
    IndexTest::new("ab", "ab", 0),
    IndexTest::new("xab", "ab", 1),
    // IndexTest::new("xab"[..2], "ab", -1),
    IndexTest::new("", "abc", -1),
    IndexTest::new("xbc", "abc", -1),
    IndexTest::new("abc", "abc", 0),
    IndexTest::new("xabc", "abc", 1),
    // IndexTest::new("xabc"[..3], "abc", -1),
    IndexTest::new("xabxc", "abc", -1),
    IndexTest::new("", "abcd", -1),
    IndexTest::new("xbcd", "abcd", -1),
    IndexTest::new("abcd", "abcd", 0),
    IndexTest::new("xabcd", "abcd", 1),
    // IndexTest::new("xyabcd"[..5], "abcd", -1),
    IndexTest::new("xbcqq", "abcqq", -1),
    IndexTest::new("abcqq", "abcqq", 0),
    IndexTest::new("xabcqq", "abcqq", 1),
    // IndexTest::new("xyabcqq"[..6], "abcqq", -1),
    IndexTest::new("xabxcqq", "abcqq", -1),
    IndexTest::new("xabcqxq", "abcqq", -1),
    IndexTest::new("", "01234567", -1),
    IndexTest::new("32145678", "01234567", -1),
    IndexTest::new("01234567", "01234567", 0),
    IndexTest::new("x01234567", "01234567", 1),
    IndexTest::new("x0123456x01234567", "01234567", 9),
    // IndexTest::new("xx01234567"[..9], "01234567", -1),
    IndexTest::new("", "0123456789", -1),
    IndexTest::new("3214567844", "0123456789", -1),
    IndexTest::new("0123456789", "0123456789", 0),
    IndexTest::new("x0123456789", "0123456789", 1),
    IndexTest::new("x012345678x0123456789", "0123456789", 11),
    // IndexTest::new("xyz0123456789"[..12], "0123456789", -1),
    IndexTest::new("x01234567x89", "0123456789", -1),
    IndexTest::new("", "0123456789012345", -1),
    IndexTest::new("3214567889012345", "0123456789012345", -1),
    IndexTest::new("0123456789012345", "0123456789012345", 0),
    IndexTest::new("x0123456789012345", "0123456789012345", 1),
    IndexTest::new("x012345678901234x0123456789012345", "0123456789012345", 17),
    IndexTest::new("", "01234567890123456789", -1),
    IndexTest::new("32145678890123456789", "01234567890123456789", -1),
    IndexTest::new("01234567890123456789", "01234567890123456789", 0),
    IndexTest::new("x01234567890123456789", "01234567890123456789", 1),
    IndexTest::new("x0123456789012345678x01234567890123456789", "01234567890123456789", 21),
    // IndexTest::new("xyz01234567890123456789"[..22], "01234567890123456789", -1),
    IndexTest::new("", "0123456789012345678901234567890", -1),
    IndexTest::new("321456788901234567890123456789012345678911", "0123456789012345678901234567890", -1),
    IndexTest::new("0123456789012345678901234567890", "0123456789012345678901234567890", 0),
    IndexTest::new("x0123456789012345678901234567890", "0123456789012345678901234567890", 1),
    IndexTest::new("x012345678901234567890123456789x0123456789012345678901234567890", "0123456789012345678901234567890", 32),
    // IndexTest::new("xyz0123456789012345678901234567890"[..33], "0123456789012345678901234567890", -1),
    IndexTest::new("", "01234567890123456789012345678901", -1),
    IndexTest::new("32145678890123456789012345678901234567890211", "01234567890123456789012345678901", -1),
    IndexTest::new("01234567890123456789012345678901", "01234567890123456789012345678901", 0),
    IndexTest::new("x01234567890123456789012345678901", "01234567890123456789012345678901", 1),
    IndexTest::new("x0123456789012345678901234567890x01234567890123456789012345678901", "01234567890123456789012345678901", 33),
    // IndexTest::new("xyz01234567890123456789012345678901"[..34], "01234567890123456789012345678901", -1),
    IndexTest::new("xxxxxx012345678901234567890123456789012345678901234567890123456789012", "012345678901234567890123456789012345678901234567890123456789012", 6),
    IndexTest::new("", "0123456789012345678901234567890123456789", -1),
    IndexTest::new("xx012345678901234567890123456789012345678901234567890123456789012", "0123456789012345678901234567890123456789", 2),
    // IndexTest::new("xx012345678901234567890123456789012345678901234567890123456789012"[..41], "0123456789012345678901234567890123456789", -1),
    IndexTest::new("xx012345678901234567890123456789012345678901234567890123456789012", "0123456789012345678901234567890123456xxx", -1),
    IndexTest::new("xx0123456789012345678901234567890123456789012345678901234567890120123456789012345678901234567890123456xxx", "0123456789012345678901234567890123456xxx", 65),
    // test fallback to Rabin-Karp.
    IndexTest::new("oxoxoxoxoxoxoxoxoxoxoxoy", "oy", 22),
    IndexTest::new("oxoxoxoxoxoxoxoxoxoxoxox", "oy", -1),
];

// var lastIndexTests = []IndexTest{
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

// var indexAnyTests = []IndexTest{
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

// var lastIndexAnyTests = []IndexTest{
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

/// Execute f on each test case.  funcName should be the name of f; it's used
/// input failure reports.
fn run_index_tests(
    f: fn(s: &str, sep: &str) -> isize,
    func_name: &str,
    test_cases: &'static [IndexTest],
) {
    for test in test_cases {
        let actual = (f)(test.s, test.sep);
        assert_eq!(
            actual, test.out,
            "{}({:?},{:?}) = {}; want {}",
            func_name, test.s, test.sep, actual, test.out
        )
    }
}

#[test]
fn test_index() {
    run_index_tests(index, "index", INDEX_TESTS)
}
// #[test]
// fn TestLastIndex() { run_index_tests(t, LastIndex, "LastIndex", lastIndexTests) }
// #[test]
// fn TestIndexAny()  { run_index_tests(t, IndexAny, "IndexAny", indexAnyTests) }
// #[test]
// fn TestLastIndexAny() {
// 	run_index_tests(t, LastIndexAny, "LastIndexAny", lastIndexAnyTests)
// }

// #[test]
// fn TestIndexByte() {
// 	for _, tt := range INDEX_TESTS {
// 		if len(tt.sep) != 1 {
// 			continue
// 		}
// 		pos := index_byte(tt.s, tt.sep[0])
// 		if pos != tt.out {
// 			t.Errorf(`index_byte({:?}, {:?}) = {}; want {}`, tt.s, tt.sep[0], pos, tt.out)
// 		}
// 	}
// }

// #[test]
// fn TestLastIndexByte() {
// 	testCases := []IndexTest{
// 		{"", "q", -1},
// 		{"abcdef", "q", -1},
// 		{"abcdefabcdef", "a", len("abcdef")},      // something input the middle
// 		{"abcdefabcdef", "f", len("abcdefabcde")}, // last byte
// 		{"zabcdefabcdef", "z", 0},                 // first byte
// 		{"a☺b☻c☹d", "b", len("a☺")},               // non-ascii
// 	}
// 	for test in testCases {
// 		actual := LastIndexByte(test.s, test.sep[0])
// 		if actual != test.out {
// 			t.Errorf("LastIndexByte({:?},%c) = {}; want {}", test.s, test.sep[0], actual, test.out)
// 		}
// 	}
// }

// fn simpleIndex(s, sep string) isize {
// 	n := len(sep)
// 	for i := n; i <= len(s); i++ {
// 		if s[i-n:i] == sep {
// 			return i - n
// 		}
// 	}
// 	return -1
// }

// #[test]
// fn TestIndexRandom() {
// 	const chars = "abcdefghijklmnopqrstuvwxyz0123456789"
// 	for times := 0; times < 10; times++ {
// 		for strLen := 5 + rand.Intn(5); strLen < 140; strLen += 10 { // Arbitrary
// 			s1 := make([]byte, strLen)
// 			for i := range s1 {
// 				s1[i] = chars[rand.Intn(len(chars))]
// 			}
// 			s := string(s1)
// 			for i := 0; i < 50; i++ {
// 				begin := rand.Intn(len(s) + 1)
// 				end := begin + rand.Intn(len(s)+1-begin)
// 				sep := s[begin:end]
// 				if i%4 == 0 {
// 					pos := rand.Intn(len(sep) + 1)
// 					sep = sep[:pos] + "A" + sep[pos:]
// 				}
// 				want := simpleIndex(s, sep)
// 				res := Index(s, sep)
// 				if res != want {
// 					t.Errorf("Index({},{}) = %d; want %d", s, sep, res, want)
// 				}
// 			}
// 		}
// 	}
// }

// #[test]
// fn TestIndexRune() {
// 	tests := []struct {
// 		input   string
// 		rune rune
// 		want isize
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
// 	for _, tt := range tests {
// 		if got := IndexRune(tt.input, tt.rune); got != tt.want {
// 			t.Errorf("IndexRune({:?}, %d) = {}; want {}", tt.input, tt.rune, got, tt.want)
// 		}
// 	}

// 	haystack := "test世界"
// 	allocs := testing.AllocsPerRun(1000, fn() {
// 		if i := IndexRune(haystack, 's'); i != 2 {
// 			t.Fatalf("'s' at %d; want 2", i)
// 		}
// 		if i := IndexRune(haystack, '世'); i != 4 {
// 			t.Fatalf("'世' at %d; want 4", i)
// 		}
// 	})
// 	if allocs != 0 && testing.CoverMode() == "" {
// 		t.Errorf("expected no allocations, got %f", allocs)
// 	}
// }

// const benchmarkString = "some_text=some☺value"

// fn BenchmarkIndexRune(b *testing.B) {
// 	if got := IndexRune(benchmarkString, '☺'); got != 14 {
// 		b.Fatalf("wrong index: expected 14, got=%d", got)
// 	}
// 	for i := 0; i < b.N; i++ {
// 		IndexRune(benchmarkString, '☺')
// 	}
// }

// var benchmarkLongString = Repeat(" ", 100) + benchmarkString

// fn BenchmarkIndexRuneLongString(b *testing.B) {
// 	if got := IndexRune(benchmarkLongString, '☺'); got != 114 {
// 		b.Fatalf("wrong index: expected 114, got=%d", got)
// 	}
// 	for i := 0; i < b.N; i++ {
// 		IndexRune(benchmarkLongString, '☺')
// 	}
// }

// fn BenchmarkIndexRuneFastPath(b *testing.B) {
// 	if got := IndexRune(benchmarkString, 'v'); got != 17 {
// 		b.Fatalf("wrong index: expected 17, got=%d", got)
// 	}
// 	for i := 0; i < b.N; i++ {
// 		IndexRune(benchmarkString, 'v')
// 	}
// }

// fn BenchmarkIndex(b *testing.B) {
// 	if got := Index(benchmarkString, "v"); got != 17 {
// 		b.Fatalf("wrong index: expected 17, got=%d", got)
// 	}
// 	for i := 0; i < b.N; i++ {
// 		Index(benchmarkString, "v")
// 	}
// }

// fn BenchmarkLastIndex(b *testing.B) {
// 	if got := Index(benchmarkString, "v"); got != 17 {
// 		b.Fatalf("wrong index: expected 17, got=%d", got)
// 	}
// 	for i := 0; i < b.N; i++ {
// 		LastIndex(benchmarkString, "v")
// 	}
// }

// fn BenchmarkIndexByte(b *testing.B) {
// 	if got := index_byte(benchmarkString, 'v'); got != 17 {
// 		b.Fatalf("wrong index: expected 17, got=%d", got)
// 	}
// 	for i := 0; i < b.N; i++ {
// 		index_byte(benchmarkString, 'v')
// 	}
// }

// type SplitTest struct {
// 	s   string
// 	sep string
// 	n   isize
// 	a   []string
// }

// var splittests = []SplitTest{
// 	{"", "", -1, []string{}},
// 	{abcd, "", 2, []string{"a", "bcd"}},
// 	{abcd, "", 4, []string{"a", "b", "c", "d"}},
// 	{abcd, "", -1, []string{"a", "b", "c", "d"}},
// 	{faces, "", -1, []string{"☺", "☻", "☹"}},
// 	{faces, "", 3, []string{"☺", "☻", "☹"}},
// 	{faces, "", 17, []string{"☺", "☻", "☹"}},
// 	{"☺�☹", "", -1, []string{"☺", "�", "☹"}},
// 	{abcd, "a", 0, nil},
// 	{abcd, "a", -1, []string{"", "bcd"}},
// 	{abcd, "z", -1, []string{"abcd"}},
// 	{commas, ",", -1, []string{"1", "2", "3", "4"}},
// 	{dots, "...", -1, []string{"1", ".2", ".3", ".4"}},
// 	{faces, "☹", -1, []string{"☺☻", ""}},
// 	{faces, "~", -1, []string{faces}},
// 	{"1 2 3 4", " ", 3, []string{"1", "2", "3 4"}},
// 	{"1 2", " ", 3, []string{"1", "2"}},
// 	{"", "T", math.MaxInt / 4, []string{""}},
// 	{"\xff-\xff", "", -1, []string{"\xff", "-", "\xff"}},
// 	{"\xff-\xff", "-", -1, []string{"\xff", "\xff"}},
// }

// #[test]
// fn TestSplit() {
// 	for _, tt := range splittests {
// 		a := SplitN(tt.s, tt.sep, tt.n)
// 		if !eq(a, tt.a) {
// 			t.Errorf("Split({:?}, {:?}, %d) = {}; want {}", tt.s, tt.sep, tt.n, a, tt.a)
// 			continue
// 		}
// 		if tt.n == 0 {
// 			continue
// 		}
// 		s := Join(a, tt.sep)
// 		if s != tt.s {
// 			t.Errorf("Join(Split({:?}, {:?}, %d), {:?}) = {:?}", tt.s, tt.sep, tt.n, tt.sep, s)
// 		}
// 		if tt.n < 0 {
// 			b := Split(tt.s, tt.sep)
// 			if !reflect.DeepEqual(a, b) {
// 				t.Errorf("Split disagrees with SplitN({:?}, {:?}, %d) = {}; want {}", tt.s, tt.sep, tt.n, b, a)
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
// 	for _, tt := range splitaftertests {
// 		a := SplitAfterN(tt.s, tt.sep, tt.n)
// 		if !eq(a, tt.a) {
// 			t.Errorf(`Split({:?}, {:?}, %d) = {}; want {}`, tt.s, tt.sep, tt.n, a, tt.a)
// 			continue
// 		}
// 		s := Join(a, "")
// 		if s != tt.s {
// 			t.Errorf(`Join(Split({:?}, {:?}, %d), {:?}) = {:?}`, tt.s, tt.sep, tt.n, tt.sep, s)
// 		}
// 		if tt.n < 0 {
// 			b := SplitAfter(tt.s, tt.sep)
// 			if !reflect.DeepEqual(a, b) {
// 				t.Errorf("SplitAfter disagrees with SplitAfterN({:?}, {:?}, %d) = {}; want {}", tt.s, tt.sep, tt.n, b, a)
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
// 	{"\u2000", []string{}},
// 	{"  abc  ", []string{"abc"}},
// 	{"1 2 3 4", []string{"1", "2", "3", "4"}},
// 	{"1  2  3  4", []string{"1", "2", "3", "4"}},
// 	{"1\t\t2\t\t3\t4", []string{"1", "2", "3", "4"}},
// 	{"1\u20002\u20013\u20024", []string{"1", "2", "3", "4"}},
// 	{"\u2000\u2001\u2002", []string{}},
// 	{"\n™\t™\n", []string{"™", "™"}},
// 	{"\n\u20001™2\u2000 \u2001 ™", []string{"1™2", "™"}},
// 	{"\n1\uFFFD \uFFFD2\u20003\uFFFD4", []string{"1\uFFFD", "\uFFFD2", "3\uFFFD4"}},
// 	{"1\xFF\u2000\xFF2\xFF \xFF", []string{"1\xFF", "\xFF2\xFF", "\xFF"}},
// 	{faces, []string{faces}},
// }

// #[test]
// fn TestFields() {
// 	for _, tt := range fieldstests {
// 		a := Fields(tt.s)
// 		if !eq(a, tt.a) {
// 			t.Errorf("Fields({:?}) = {}; want {}", tt.s, a, tt.a)
// 			continue
// 		}
// 	}
// }

// var FieldsFuncTests = []FieldsTest{
// 	{"", []string{}},
// 	{"XX", []string{}},
// 	{"XXhiXXX", []string{"hi"}},
// 	{"aXXbXXXcX", []string{"a", "b", "c"}},
// }

// #[test]
// fn TestFieldsFunc() {
// 	for _, tt := range fieldstests {
// 		a := FieldsFunc(tt.s, unicode.IsSpace)
// 		if !eq(a, tt.a) {
// 			t.Errorf("FieldsFunc({:?}, unicode.IsSpace) = {}; want {}", tt.s, a, tt.a)
// 			continue
// 		}
// 	}
// 	pred := fn(c rune) bool { return c == 'X' }
// 	for _, tt := range FieldsFuncTests {
// 		a := FieldsFunc(tt.s, pred)
// 		if !eq(a, tt.a) {
// 			t.Errorf("FieldsFunc({:?}) = {}, want {}", tt.s, a, tt.a)
// 		}
// 	}
// }

// // Test case for any function which accepts and returns a single string.
// type StringTest struct {
// 	input, out string
// }

// // Execute f on each test case.  funcName should be the name of f; it's used
// // input failure reports.
// #[test]
// fn runStringTests(, f fn(string) string, funcName string, testCases []StringTest) {
// 	for _, tc := range testCases {
// 		actual := f(tc.input)
// 		if actual != tc.out {
// 			t.Errorf("{}({:?}) = {:?}; want {:?}", funcName, tc.input, actual, tc.out)
// 		}
// 	}
// }

// var upperTests = []StringTest{
// 	{"", ""},
// 	{"ONLYUPPER", "ONLYUPPER"},
// 	{"abc", "ABC"},
// 	{"AbC123", "ABC123"},
// 	{"azAZ09_", "AZAZ09_"},
// 	{"longStrinGwitHmixofsmaLLandcAps", "LONGSTRINGWITHMIXOFSMALLANDCAPS"},
// 	{"RENAN BASTOS 93 AOSDAJDJAIDJAIDAJIaidsjjaidijadsjiadjiOOKKO", "RENAN BASTOS 93 AOSDAJDJAIDJAIDAJIAIDSJJAIDIJADSJIADJIOOKKO"},
// 	{"long\u0250string\u0250with\u0250nonascii\u2C6Fchars", "LONG\u2C6FSTRING\u2C6FWITH\u2C6FNONASCII\u2C6FCHARS"},
// 	{"\u0250\u0250\u0250\u0250\u0250", "\u2C6F\u2C6F\u2C6F\u2C6F\u2C6F"}, // grows one byte per char
// 	{"a\u0080\U0010FFFF", "A\u0080\U0010FFFF"},                           // test utf8::RUNE_SELF and utf8.MAX_RUNE
// }

// var lowerTests = []StringTest{
// 	{"", ""},
// 	{"abc", "abc"},
// 	{"AbC123", "abc123"},
// 	{"azAZ09_", "azaz09_"},
// 	{"longStrinGwitHmixofsmaLLandcAps", "longstringwithmixofsmallandcaps"},
// 	{"renan bastos 93 AOSDAJDJAIDJAIDAJIaidsjjaidijadsjiadjiOOKKO", "renan bastos 93 aosdajdjaidjaidajiaidsjjaidijadsjiadjiookko"},
// 	{"LONG\u2C6FSTRING\u2C6FWITH\u2C6FNONASCII\u2C6FCHARS", "long\u0250string\u0250with\u0250nonascii\u0250chars"},
// 	{"\u2C6D\u2C6D\u2C6D\u2C6D\u2C6D", "\u0251\u0251\u0251\u0251\u0251"}, // shrinks one byte per char
// 	{"A\u0080\U0010FFFF", "a\u0080\U0010FFFF"},                           // test utf8::RUNE_SELF and utf8.MAX_RUNE
// }

// const space = "\t\v\r\f\n\u0085\u00a0\u2000\u3000"

// var trimSpaceTests = []StringTest{
// 	{"", ""},
// 	{"abc", "abc"},
// 	{space + "abc" + space, "abc"},
// 	{" ", ""},
// 	{" \t\r\n \t\t\r\r\n\n ", ""},
// 	{" \t\r\n x\t\t\r\r\n\n ", "x"},
// 	{" \u2000\t\r\n x\t\t\r\r\ny\n \u3000", "x\t\t\r\r\ny"},
// 	{"1 \t\r\n2", "1 \t\r\n2"},
// 	{" x\x80", "x\x80"},
// 	{" x\xc0", "x\xc0"},
// 	{"x \xc0\xc0 ", "x \xc0\xc0"},
// 	{"x \xc0", "x \xc0"},
// 	{"x \xc0 ", "x \xc0"},
// 	{"x \xc0\xc0 ", "x \xc0\xc0"},
// 	{"x ☺\xc0\xc0 ", "x ☺\xc0\xc0"},
// 	{"x ☺ ", "x ☺"},
// }

// fn tenRunes(ch rune) string {
// 	r := make([]rune, 10)
// 	for i := range r {
// 		r[i] = ch
// 	}
// 	return string(r)
// }

// // User-defined self-inverse mapping function
// fn rot13(r rune) rune {
// 	step := rune(13)
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
// 	// 1.  Grow. This triggers two reallocations input Map.
// 	maxRune := fn(rune) rune { return unicode.MAX_RUNE }
// 	m := Map(maxRune, a)
// 	expect := tenRunes(unicode.MAX_RUNE)
// 	if m != expect {
// 		t.Errorf("growing: expected {:?} got {:?}", expect, m)
// 	}

// 	// 2. Shrink
// 	minRune := fn(rune) rune { return 'a' }
// 	m = Map(minRune, tenRunes(unicode.MAX_RUNE))
// 	expect = a
// 	if m != expect {
// 		t.Errorf("shrinking: expected {:?} got {:?}", expect, m)
// 	}

// 	// 3. Rot13
// 	m = Map(rot13, "a to zed")
// 	expect = "n gb mrq"
// 	if m != expect {
// 		t.Errorf("rot13: expected {:?} got {:?}", expect, m)
// 	}

// 	// 4. Rot13^2
// 	m = Map(rot13, Map(rot13, "a to zed"))
// 	expect = "a to zed"
// 	if m != expect {
// 		t.Errorf("rot13: expected {:?} got {:?}", expect, m)
// 	}

// 	// 5. Drop
// 	dropNotLatin := fn(r rune) rune {
// 		if unicode.Is(unicode.Latin, r) {
// 			return r
// 		}
// 		return -1
// 	}
// 	m = Map(dropNotLatin, "Hello, 세계")
// 	expect = "Hello"
// 	if m != expect {
// 		t.Errorf("drop: expected {:?} got {:?}", expect, m)
// 	}

// 	// 6. Identity
// 	identity := fn(r rune) rune {
// 		return r
// 	}
// 	orig := "Input string that we expect not to be copied."
// 	m = Map(identity, orig)
// 	if unsafe.StringData(orig) != unsafe.StringData(m) {
// 		t.Error("unexpected copy during identity map")
// 	}

// 	// 7. Handle invalid UTF-8 sequence
// 	replaceNotLatin := fn(r rune) rune {
// 		if unicode.Is(unicode.Latin, r) {
// 			return r
// 		}
// 		return utf8.RUNE_ERROR
// 	}
// 	m = Map(replaceNotLatin, "Hello\255World")
// 	expect = "Hello\uFFFDWorld"
// 	if m != expect {
// 		t.Errorf("replace invalid sequence: expected {:?} got {:?}", expect, m)
// 	}

// 	// 8. Check utf8::RUNE_SELF and utf8.MAX_RUNE encoding
// 	encode := fn(r rune) rune {
// 		switch r {
// 		case utf8::RUNE_SELF:
// 			return unicode.MAX_RUNE
// 		case unicode.MAX_RUNE:
// 			return utf8::RUNE_SELF
// 		}
// 		return r
// 	}
// 	s := string(rune(utf8::RUNE_SELF)) + string(utf8.MAX_RUNE)
// 	r := string(utf8.MAX_RUNE) + string(rune(utf8::RUNE_SELF)) // reverse of s
// 	m = Map(encode, s)
// 	if m != r {
// 		t.Errorf("encoding not handled correctly: expected {:?} got {:?}", r, m)
// 	}
// 	m = Map(encode, r)
// 	if m != s {
// 		t.Errorf("encoding not handled correctly: expected {:?} got {:?}", s, m)
// 	}

// 	// 9. Check mapping occurs input the front, middle and back
// 	trimSpaces := fn(r rune) rune {
// 		if unicode.IsSpace(r) {
// 			return -1
// 		}
// 		return r
// 	}
// 	m = Map(trimSpaces, "   abc    123   ")
// 	expect = "abc123"
// 	if m != expect {
// 		t.Errorf("trimSpaces: expected {:?} got {:?}", expect, m)
// 	}
// }

// #[test]
// fn TestToUpper() { runStringTests(t, ToUpper, "ToUpper", upperTests) }

// #[test]
// fn TestToLower() { runStringTests(t, ToLower, "ToLower", lowerTests) }

// var toValidUTF8Tests = []struct {
// 	input   string
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
// 		got := ToValidUTF8(tc.input, tc.repl)
// 		if got != tc.out {
// 			t.Errorf("ToValidUTF8({:?}, {:?}) = {:?}; want {:?}", tc.input, tc.repl, got, tc.out)
// 		}
// 	}
// }

// fn BenchmarkToUpper(b *testing.B) {
// 	for _, tc := range upperTests {
// 		b.Run(tc.input, fn(b *testing.B) {
// 			for i := 0; i < b.N; i++ {
// 				actual := ToUpper(tc.input)
// 				if actual != tc.out {
// 					b.Errorf("ToUpper({:?}) = {:?}; want {:?}", tc.input, actual, tc.out)
// 				}
// 			}
// 		})
// 	}
// }

// fn BenchmarkToLower(b *testing.B) {
// 	for _, tc := range lowerTests {
// 		b.Run(tc.input, fn(b *testing.B) {
// 			for i := 0; i < b.N; i++ {
// 				actual := ToLower(tc.input)
// 				if actual != tc.out {
// 					b.Errorf("ToLower({:?}) = {:?}; want {:?}", tc.input, actual, tc.out)
// 				}
// 			}
// 		})
// 	}
// }

// fn BenchmarkMapNoChanges(b *testing.B) {
// 	identity := fn(r rune) rune {
// 		return r
// 	}
// 	for i := 0; i < b.N; i++ {
// 		Map(identity, "Some string that won't be modified.")
// 	}
// }

// #[test]
// fn TestSpecialCase() {
// 	lower := "abcçdefgğhıijklmnoöprsştuüvyz"
// 	upper := "ABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZ"
// 	u := ToUpperSpecial(unicode.TurkishCase, upper)
// 	if u != upper {
// 		t.Errorf("Upper(upper) is {} not {}", u, upper)
// 	}
// 	u = ToUpperSpecial(unicode.TurkishCase, lower)
// 	if u != upper {
// 		t.Errorf("Upper(lower) is {} not {}", u, upper)
// 	}
// 	l := ToLowerSpecial(unicode.TurkishCase, lower)
// 	if l != lower {
// 		t.Errorf("Lower(lower) is {} not {}", l, lower)
// 	}
// 	l = ToLowerSpecial(unicode.TurkishCase, upper)
// 	if l != lower {
// 		t.Errorf("Lower(upper) is {} not {}", l, lower)
// 	}
// }

// #[test]
// fn TestTrimSpace() { runStringTests(t, TrimSpace, "TrimSpace", trimSpaceTests) }

// var trimTests = []struct {
// 	f            string
// 	input, arg, out string
// }{
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

// #[test]
// fn TestTrim() {
// 	for _, tc := range trimTests {
// 		name := tc.f
// 		var f fn(string, string) string
// 		switch name {
// 		case "Trim":
// 			f = Trim
// 		case "TrimLeft":
// 			f = TrimLeft
// 		case "TrimRight":
// 			f = TrimRight
// 		case "TrimPrefix":
// 			f = TrimPrefix
// 		case "TrimSuffix":
// 			f = TrimSuffix
// 		default:
// 			t.Errorf("Undefined trim function {}", name)
// 		}
// 		actual := f(tc.input, tc.arg)
// 		if actual != tc.out {
// 			t.Errorf("{}({:?}, {:?}) = {:?}; want {:?}", name, tc.input, tc.arg, actual, tc.out)
// 		}
// 	}
// }

// fn BenchmarkTrim(b *testing.B) {
// 	b.ReportAllocs()

// 	for i := 0; i < b.N; i++ {
// 		for _, tc := range trimTests {
// 			name := tc.f
// 			var f fn(string, string) string
// 			switch name {
// 			case "Trim":
// 				f = Trim
// 			case "TrimLeft":
// 				f = TrimLeft
// 			case "TrimRight":
// 				f = TrimRight
// 			case "TrimPrefix":
// 				f = TrimPrefix
// 			case "TrimSuffix":
// 				f = TrimSuffix
// 			default:
// 				b.Errorf("Undefined trim function {}", name)
// 			}
// 			actual := f(tc.input, tc.arg)
// 			if actual != tc.out {
// 				b.Errorf("{}({:?}, {:?}) = {:?}; want {:?}", name, tc.input, tc.arg, actual, tc.out)
// 			}
// 		}
// 	}
// }

// fn BenchmarkToValidUTF8(b *testing.B) {
// 	tests := []struct {
// 		name  string
// 		input string
// 	}{
// 		{"Valid", "typical"},
// 		{"InvalidASCII", "foo\xffbar"},
// 		{"InvalidNonASCII", "日本語\xff日本語"},
// 	}
// 	replacement := "\uFFFD"
// 	b.ResetTimer()
// 	for test in tests {
// 		b.Run(test.name, fn(b *testing.B) {
// 			for i := 0; i < b.N; i++ {
// 				ToValidUTF8(test.input, replacement)
// 			}
// 		})
// 	}
// }

// type predicate struct {
// 	f    fn(rune) bool
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

// fn not(p predicate) predicate {
// 	return predicate{
// 		fn(r rune) bool {
// 			return !p.f(r)
// 		},
// 		"not " + p.name,
// 	}
// }

// var trimFuncTests = []struct {
// 	f        predicate
// 	input       string
// 	trimOut  string
// 	leftOut  string
// 	rightOut string
// }{
// 	{isSpace, space + " hello " + space,
// 		"hello",
// 		"hello " + space,
// 		space + " hello"},
// 	{isDigit, "\u0e50\u0e5212hello34\u0e50\u0e51",
// 		"hello",
// 		"hello34\u0e50\u0e51",
// 		"\u0e50\u0e5212hello"},
// 	{isUpper, "\u2C6F\u2C6F\u2C6F\u2C6FABCDhelloEF\u2C6F\u2C6FGH\u2C6F\u2C6F",
// 		"hello",
// 		"helloEF\u2C6F\u2C6FGH\u2C6F\u2C6F",
// 		"\u2C6F\u2C6F\u2C6F\u2C6FABCDhello"},
// 	{not(isSpace), "hello" + space + "hello",
// 		space,
// 		space + "hello",
// 		"hello" + space},
// 	{not(isDigit), "hello\u0e50\u0e521234\u0e50\u0e51helo",
// 		"\u0e50\u0e521234\u0e50\u0e51",
// 		"\u0e50\u0e521234\u0e50\u0e51helo",
// 		"hello\u0e50\u0e521234\u0e50\u0e51"},
// 	{isValidRune, "ab\xc0a\xc0cd",
// 		"\xc0a\xc0",
// 		"\xc0a\xc0cd",
// 		"ab\xc0a\xc0"},
// 	{not(isValidRune), "\xc0a\xc0",
// 		"a",
// 		"a\xc0",
// 		"\xc0a"},
// 	{isSpace, "",
// 		"",
// 		"",
// 		""},
// 	{isSpace, " ",
// 		"",
// 		"",
// 		""},
// }

// #[test]
// fn TestTrimFunc() {
// 	for _, tc := range trimFuncTests {
// 		trimmers := []struct {
// 			name string
// 			trim fn(s string, f fn(r rune) bool) string
// 			out  string
// 		}{
// 			{"TrimFunc", TrimFunc, tc.trimOut},
// 			{"TrimLeftFunc", TrimLeftFunc, tc.leftOut},
// 			{"TrimRightFunc", TrimRightFunc, tc.rightOut},
// 		}
// 		for _, trimmer := range trimmers {
// 			actual := trimmer.trim(tc.input, tc.f.f)
// 			if actual != trimmer.out {
// 				t.Errorf("{}({:?}, {:?}) = {:?}; want {:?}", trimmer.name, tc.input, tc.f.name, actual, trimmer.out)
// 			}
// 		}
// 	}
// }

// var indexFuncTests = []struct {
// 	input          string
// 	f           predicate
// 	first, last isize
// }{
// 	{"", isValidRune, -1, -1},
// 	{"abc", isDigit, -1, -1},
// 	{"0123", isDigit, 0, 3},
// 	{"a1b", isDigit, 1, 1},
// 	{space, isSpace, 0, len(space) - 3}, // last rune input space is 3 bytes
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
// 	{"\x80\x80\x80\x80", not(isValidRune), 0, 3},
// }

// #[test]
// fn TestIndexFunc() {
// 	for _, tc := range indexFuncTests {
// 		first := IndexFunc(tc.input, tc.f.f)
// 		if first != tc.first {
// 			t.Errorf("IndexFunc({:?}, {}) = %d; want %d", tc.input, tc.f.name, first, tc.first)
// 		}
// 		last := LastIndexFunc(tc.input, tc.f.f)
// 		if last != tc.last {
// 			t.Errorf("LastIndexFunc({:?}, {}) = %d; want %d", tc.input, tc.f.name, last, tc.last)
// 		}
// 	}
// }

// #[test]
// fn equal(m string, s1, s2 string, ) bool {
// 	if s1 == s2 {
// 		return true
// 	}
// 	e1 := Split(s1, "")
// 	e2 := Split(s2, "")
// 	for i, c1 := range e1 {
// 		if i >= len(e2) {
// 			break
// 		}
// 		r1, _ := utf8.decode_rune_in_string(c1)
// 		r2, _ := utf8.decode_rune_in_string(e2[i])
// 		if r1 != r2 {
// 			t.Errorf("{} diff at %d: U+%04X U+%04X", m, i, r1, r2)
// 		}
// 	}
// 	return false
// }

// #[test]
// fn TestCaseConsistency() {
// 	// Make a string of all the runes.
// 	numRunes := isize(unicode.MAX_RUNE + 1)
// 	if testing.Short() {
// 		numRunes = 1000
// 	}
// 	a := make([]rune, numRunes)
// 	for i := range a {
// 		a[i] = rune(i)
// 	}
// 	s := string(a)
// 	// convert the cases.
// 	upper := ToUpper(s)
// 	lower := ToLower(s)

// 	// Consistency checks
// 	if n := utf8.RuneCountInString(upper); n != numRunes {
// 		t.Error("rune count wrong input upper:", n)
// 	}
// 	if n := utf8.RuneCountInString(lower); n != numRunes {
// 		t.Error("rune count wrong input lower:", n)
// 	}
// 	if !equal("ToUpper(upper)", ToUpper(upper), upper, t) {
// 		t.Error("ToUpper(upper) consistency fail")
// 	}
// 	if !equal("ToLower(lower)", ToLower(lower), lower, t) {
// 		t.Error("ToLower(lower) consistency fail")
// 	}
// 	/*
// 		  These fail because of non-one-to-oneness of the data, such as multiple
// 		  upper case 'I' mapping to 'i'.  We comment them out but keep them for
// 		  interest.
// 		  For instance: CAPITAL LETTER I WITH DOT ABOVE:
// 			unicode.ToUpper(unicode.ToLower('\u0130')) != '\u0130'

// 		if !equal("ToUpper(lower)", ToUpper(lower), upper, t) {
// 			t.Error("ToUpper(lower) consistency fail");
// 		}
// 		if !equal("ToLower(upper)", ToLower(upper), lower, t) {
// 			t.Error("ToLower(upper) consistency fail");
// 		}
// 	*/
// }

// var longString = "a" + string(make([]byte, 1<<16)) + "z"

// var RepeatTests = []struct {
// 	input, out string
// 	count   isize
// }{
// 	{"", "", 0},
// 	{"", "", 1},
// 	{"", "", 2},
// 	{"-", "", 0},
// 	{"-", "-", 1},
// 	{"-", "----------", 10},
// 	{"abc ", "abc abc abc ", 3},
// 	// Tests for results over the chunkLimit
// 	{string(rune(0)), string(make([]byte, 1<<16)), 1 << 16},
// 	{longString, longString + longString, 2},
// }

// #[test]
// fn TestRepeat() {
// 	for _, tt := range RepeatTests {
// 		a := Repeat(tt.input, tt.count)
// 		if !equal("Repeat(s)", a, tt.out, t) {
// 			t.Errorf("Repeat({}, %d) = {}; want {}", tt.input, tt.count, a, tt.out)
// 			continue
// 		}
// 	}
// }

// fn repeat(s string, count isize) (err error) {
// 	defer fn() {
// 		if r := recover(); r != nil {
// 			switch v := r.(type) {
// 			case error:
// 				err = v
// 			default:
// 				err = fmt.Errorf("{}", v)
// 			}
// 		}
// 	}()

// 	Repeat(s, count)

// 	return
// }

// // See Issue golang.org/issue/16237
// #[test]
// fn TestRepeatCatchesOverflow() {
// 	tests := [...]struct {
// 		s      string
// 		count  isize
// 		errStr string
// 	}{
// 		0: {"--", -2147483647, "negative"},
// 		1: {"", isize(^uint(0) >> 1), ""},
// 		2: {"-", 10, ""},
// 		3: {"gopher", 0, ""},
// 		4: {"-", -1, "negative"},
// 		5: {"--", -102, "negative"},
// 		6: {string(make([]byte, 255)), isize((^uint(0))/255 + 1), "overflow"},
// 	}

// 	for i, tt := range tests {
// 		err := repeat(tt.s, tt.count)
// 		if tt.errStr == "" {
// 			if err != nil {
// 				t.Errorf("#%d panicked {}", i, err)
// 			}
// 			continue
// 		}

// 		if err == nil || !contains(err.Error(), tt.errStr) {
// 			t.Errorf("#%d expected {:?} got {:?}", i, tt.errStr, err)
// 		}
// 	}
// }

// fn runesEqual(a, b []rune) bool {
// 	if len(a) != len(b) {
// 		return false
// 	}
// 	for i, r := range a {
// 		if r != b[i] {
// 			return false
// 		}
// 	}
// 	return true
// }

// var RunesTests = []struct {
// 	input    string
// 	out   []rune
// 	lossy bool
// }{
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
// 	for _, tt := range RunesTests {
// 		a := []rune(tt.input)
// 		if !runesEqual(a, tt.out) {
// 			t.Errorf("[]rune({:?}) = {}; want {}", tt.input, a, tt.out)
// 			continue
// 		}
// 		if !tt.lossy {
// 			// can only test reassembly if we didn't lose information
// 			s := string(a)
// 			if s != tt.input {
// 				t.Errorf("string([]rune({:?})) = %x; want %x", tt.input, s, tt.input)
// 			}
// 		}
// 	}
// }

// #[test]
// fn TestReadByte() {
// 	testStrings := []string{"", abcd, faces, commas}
// 	for _, s := range testStrings {
// 		reader := NewReader(s)
// 		if e := reader.UnreadByte(); e == nil {
// 			t.Errorf("Unreading {:?} at beginning: expected error", s)
// 		}
// 		var res bytes.Buffer
// 		for {
// 			b, e := reader.ReadByte()
// 			if e == io.EOF {
// 				break
// 			}
// 			if e != nil {
// 				t.Errorf("Reading {:?}: {}", s, e)
// 				break
// 			}
// 			res.WriteByte(b)
// 			// unread and read again
// 			e = reader.UnreadByte()
// 			if e != nil {
// 				t.Errorf("Unreading {:?}: {}", s, e)
// 				break
// 			}
// 			b1, e := reader.ReadByte()
// 			if e != nil {
// 				t.Errorf("Reading {:?} after unreading: {}", s, e)
// 				break
// 			}
// 			if b1 != b {
// 				t.Errorf("Reading {:?} after unreading: want byte {:?}, got {:?}", s, b, b1)
// 				break
// 			}
// 		}
// 		if res.String() != s {
// 			t.Errorf("Reader({:?}).ReadByte() produced {:?}", s, res.String())
// 		}
// 	}
// }

// #[test]
// fn TestReadRune() {
// 	testStrings := []string{"", abcd, faces, commas}
// 	for _, s := range testStrings {
// 		reader := NewReader(s)
// 		if e := reader.UnreadRune(); e == nil {
// 			t.Errorf("Unreading {:?} at beginning: expected error", s)
// 		}
// 		res := ""
// 		for {
// 			r, z, e := reader.ReadRune()
// 			if e == io.EOF {
// 				break
// 			}
// 			if e != nil {
// 				t.Errorf("Reading {:?}: {}", s, e)
// 				break
// 			}
// 			res += string(r)
// 			// unread and read again
// 			e = reader.UnreadRune()
// 			if e != nil {
// 				t.Errorf("Unreading {:?}: {}", s, e)
// 				break
// 			}
// 			r1, z1, e := reader.ReadRune()
// 			if e != nil {
// 				t.Errorf("Reading {:?} after unreading: {}", s, e)
// 				break
// 			}
// 			if r1 != r {
// 				t.Errorf("Reading {:?} after unreading: want rune {:?}, got {:?}", s, r, r1)
// 				break
// 			}
// 			if z1 != z {
// 				t.Errorf("Reading {:?} after unreading: want size %d, got %d", s, z, z1)
// 				break
// 			}
// 		}
// 		if res != s {
// 			t.Errorf("Reader({:?}).ReadRune() produced {:?}", s, res)
// 		}
// 	}
// }

// var UnreadRuneErrorTests = []struct {
// 	name string
// 	f    fn(*Reader)
// }{
// 	{"Read", fn(r *Reader) { r.Read([]byte{0}) }},
// 	{"ReadByte", fn(r *Reader) { r.ReadByte() }},
// 	{"UnreadRune", fn(r *Reader) { r.UnreadRune() }},
// 	{"Seek", fn(r *Reader) { r.Seek(0, io.SeekCurrent) }},
// 	{"WriteTo", fn(r *Reader) { r.WriteTo(&bytes.Buffer{}) }},
// }

// #[test]
// fn TestUnreadRuneError() {
// 	for _, tt := range UnreadRuneErrorTests {
// 		reader := NewReader("0123456789")
// 		if _, _, err := reader.ReadRune(); err != nil {
// 			// should not happen
// 			t.Fatal(err)
// 		}
// 		tt.f(reader)
// 		err := reader.UnreadRune()
// 		if err == nil {
// 			t.Errorf("Unreading after {}: expected error", tt.name)
// 		}
// 	}
// }

struct ReplaceTestCase {
    input: &'static str,
    old: &'static str,
    new: &'static str,
    n: isize,
    out: &'static str,
}

static REPLACE_TESTS: [ReplaceTestCase; 19] = [
    ReplaceTestCase {
        input: "hello",
        old: "l",
        new: "L",
        n: 0,
        out: "hello",
    },
    ReplaceTestCase {
        input: "hello",
        old: "l",
        new: "L",
        n: -1,
        out: "heLLo",
    },
    ReplaceTestCase {
        input: "hello",
        old: "x",
        new: "X",
        n: -1,
        out: "hello",
    },
    ReplaceTestCase {
        input: "",
        old: "x",
        new: "X",
        n: -1,
        out: "",
    },
    ReplaceTestCase {
        input: "radar",
        old: "r",
        new: "<r>",
        n: -1,
        out: "<r>ada<r>",
    },
    ReplaceTestCase {
        input: "",
        old: "",
        new: "<>",
        n: -1,
        out: "<>",
    },
    ReplaceTestCase {
        input: "banana",
        old: "a",
        new: "<>",
        n: -1,
        out: "b<>n<>n<>",
    },
    ReplaceTestCase {
        input: "banana",
        old: "a",
        new: "<>",
        n: 1,
        out: "b<>nana",
    },
    ReplaceTestCase {
        input: "banana",
        old: "a",
        new: "<>",
        n: 1000,
        out: "b<>n<>n<>",
    },
    ReplaceTestCase {
        input: "banana",
        old: "an",
        new: "<>",
        n: -1,
        out: "b<><>a",
    },
    ReplaceTestCase {
        input: "banana",
        old: "ana",
        new: "<>",
        n: -1,
        out: "b<>na",
    },
    ReplaceTestCase {
        input: "banana",
        old: "",
        new: "<>",
        n: -1,
        out: "<>b<>a<>n<>a<>n<>a<>",
    },
    ReplaceTestCase {
        input: "banana",
        old: "",
        new: "<>",
        n: 10,
        out: "<>b<>a<>n<>a<>n<>a<>",
    },
    ReplaceTestCase {
        input: "banana",
        old: "",
        new: "<>",
        n: 6,
        out: "<>b<>a<>n<>a<>n<>a",
    },
    ReplaceTestCase {
        input: "banana",
        old: "",
        new: "<>",
        n: 5,
        out: "<>b<>a<>n<>a<>na",
    },
    ReplaceTestCase {
        input: "banana",
        old: "",
        new: "<>",
        n: 1,
        out: "<>banana",
    },
    ReplaceTestCase {
        input: "banana",
        old: "a",
        new: "a",
        n: -1,
        out: "banana",
    },
    ReplaceTestCase {
        input: "banana",
        old: "a",
        new: "a",
        n: 1,
        out: "banana",
    },
    ReplaceTestCase {
        input: "☺☻☹",
        old: "",
        new: "<>",
        n: -1,
        out: "<>☺<>☻<>☹<>",
    },
];

#[test]
fn test_replace() {
    for tt in &REPLACE_TESTS {
        // let s = Replace(tt.input, tt.old, tt.new, tt.n);
        // 		if  s != tt.out {
        // 			t.Errorf("Replace({:?}, {:?}, {:?}, %d) = {:?}, want {:?}", tt.input, tt.old, tt.new, tt.n, s, tt.out)
        // 		}
        if tt.n == -1 {
            let s = super::replace_all(tt.input, tt.old, tt.new);
            assert_eq!(
                s,
                tt.out.to_string(),
                "replace_all({:?}, {:?}, {:?}) = {:?}, want {:?}",
                tt.input,
                tt.old,
                tt.new,
                s,
                tt.out
            );
        }
    }
}

// var TitleTests = []struct {
// 	input, out string
// }{
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
// 	for _, tt := range TitleTests {
// 		if s := Title(tt.input); s != tt.out {
// 			t.Errorf("Title({:?}) = {:?}, want {:?}", tt.input, s, tt.out)
// 		}
// 	}
// }

struct ContainsTest {
    s: &'static str,
    substr: &'static str,
    expected: bool,
}

impl ContainsTest {
    const fn new(s: &'static str, substr: &'static str, expected: bool) -> Self {
        Self {
            s,
            substr,
            expected,
        }
    }
}

const CONTAINS_TESTS: &[ContainsTest] = &[
    ContainsTest::new("abc", "bc", true),
    ContainsTest::new("abc", "bcd", false),
    ContainsTest::new("abc", "", true),
    ContainsTest::new("", "a", false),
    // cases to cover code input runtime/asm_amd64.s:indexShortStr
    // 2-byte needle
    ContainsTest::new("xxxxxx", "01", false),
    ContainsTest::new("01xxxx", "01", true),
    ContainsTest::new("xx01xx", "01", true),
    ContainsTest::new("xxxx01", "01", true),
    // ContainsTest::new("01xxxxx"[1:], "01", false),
    // ContainsTest::new("xxxxx01"[:6], "01", false),
    // 3-byte needle
    ContainsTest::new("xxxxxxx", "012", false),
    ContainsTest::new("012xxxx", "012", true),
    ContainsTest::new("xx012xx", "012", true),
    ContainsTest::new("xxxx012", "012", true),
    // ContainsTest::new("012xxxxx"[1:], "012", false),
    // ContainsTest::new("xxxxx012"[:7], "012", false),
    // 4-byte needle
    ContainsTest::new("xxxxxxxx", "0123", false),
    ContainsTest::new("0123xxxx", "0123", true),
    ContainsTest::new("xx0123xx", "0123", true),
    ContainsTest::new("xxxx0123", "0123", true),
    // ContainsTest::new("0123xxxxx"[1:], "0123", false),
    // ContainsTest::new("xxxxx0123"[:8], "0123", false),
    // 5-7-byte needle
    ContainsTest::new("xxxxxxxxx", "01234", false),
    ContainsTest::new("01234xxxx", "01234", true),
    ContainsTest::new("xx01234xx", "01234", true),
    ContainsTest::new("xxxx01234", "01234", true),
    // ContainsTest::new("01234xxxxx"[1:], "01234", false),
    // ContainsTest::new("xxxxx01234"[:9], "01234", false),
    // 8-byte needle
    ContainsTest::new("xxxxxxxxxxxx", "01234567", false),
    ContainsTest::new("01234567xxxx", "01234567", true),
    ContainsTest::new("xx01234567xx", "01234567", true),
    ContainsTest::new("xxxx01234567", "01234567", true),
    // ContainsTest::new("01234567xxxxx"[1:], "01234567", false),
    // ContainsTest::new("xxxxx01234567"[:12], "01234567", false),
    // 9-15-byte needle
    ContainsTest::new("xxxxxxxxxxxxx", "012345678", false),
    ContainsTest::new("012345678xxxx", "012345678", true),
    ContainsTest::new("xx012345678xx", "012345678", true),
    ContainsTest::new("xxxx012345678", "012345678", true),
    // ContainsTest::new("012345678xxxxx"[1:], "012345678", false),
    // ContainsTest::new("xxxxx012345678"[:13], "012345678", false),
    // 16-byte needle
    ContainsTest::new("xxxxxxxxxxxxxxxxxxxx", "0123456789ABCDEF", false),
    ContainsTest::new("0123456789ABCDEFxxxx", "0123456789ABCDEF", true),
    ContainsTest::new("xx0123456789ABCDEFxx", "0123456789ABCDEF", true),
    ContainsTest::new("xxxx0123456789ABCDEF", "0123456789ABCDEF", true),
    // ContainsTest::new("0123456789ABCDEFxxxxx"[1:], "0123456789ABCDEF", false),
    // ContainsTest::new("xxxxx0123456789ABCDEF"[:20], "0123456789ABCDEF", false),
    // 17-31-byte needle
    ContainsTest::new("xxxxxxxxxxxxxxxxxxxxx", "0123456789ABCDEFG", false),
    ContainsTest::new("0123456789ABCDEFGxxxx", "0123456789ABCDEFG", true),
    ContainsTest::new("xx0123456789ABCDEFGxx", "0123456789ABCDEFG", true),
    ContainsTest::new("xxxx0123456789ABCDEFG", "0123456789ABCDEFG", true),
    // ContainsTest::new("0123456789ABCDEFGxxxxx"[1:], "0123456789ABCDEFG", false),
    // ContainsTest::new("xxxxx0123456789ABCDEFG"[:21], "0123456789ABCDEFG", false),

    // partial match cases
    ContainsTest::new("xx01x", "012", false),     // 3
    ContainsTest::new("xx0123x", "01234", false), // 5-7
    ContainsTest::new("xx01234567x", "012345678", false), // 9-15
    ContainsTest::new("xx0123456789ABCDEFx", "0123456789ABCDEFG", false), // 17-31, issue 15679
];

#[test]
fn test_contains() {
    for ct in CONTAINS_TESTS {
        assert_eq!(
            contains(ct.s, ct.substr),
            ct.expected,
            "contains({}, {}) = {}, want {}",
            ct.s,
            ct.substr,
            !ct.expected,
            ct.expected,
        );
    }
}

// var ContainsAnyTests = []struct {
// 	str, substr string
// 	expected    bool
// }{
// 	{"", "", false},
// 	{"", "a", false},
// 	{"", "abc", false},
// 	{"a", "", false},
// 	{"a", "a", true},
// 	{"aaa", "a", true},
// 	{"abc", "xyz", false},
// 	{"abc", "xcz", true},
// 	{"a☺b☻c☹d", "uvw☻xyz", true},
// 	{"aRegExp*", ".(|)*+?^$[]", true},
// 	{dots + dots + dots, " ", false},
// }

// #[test]
// fn TestContainsAny() {
// 	for _, ct := range ContainsAnyTests {
// 		if ContainsAny(ct.str, ct.substr) != ct.expected {
// 			t.Errorf("ContainsAny({}, {}) = {}, want {}",
// 				ct.str, ct.substr, !ct.expected, ct.expected)
// 		}
// 	}
// }

// var ContainsRuneTests = []struct {
// 	str      string
// 	r        rune
// 	expected bool
// }{
// 	{"", 'a', false},
// 	{"a", 'a', true},
// 	{"aaa", 'a', true},
// 	{"abc", 'y', false},
// 	{"abc", 'c', true},
// 	{"a☺b☻c☹d", 'x', false},
// 	{"a☺b☻c☹d", '☻', true},
// 	{"aRegExp*", '*', true},
// }

// #[test]
// fn TestContainsRune() {
// 	for _, ct := range ContainsRuneTests {
// 		if ContainsRune(ct.str, ct.r) != ct.expected {
// 			t.Errorf("ContainsRune({:?}, {:?}) = {}, want {}",
// 				ct.str, ct.r, !ct.expected, ct.expected)
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
// 	{"1", "2", false},
// 	{"utf-8", "US-ASCII", false},
// }

// #[test]
// fn TestEqualFold() {
// 	for _, tt := range EqualFoldTests {
// 		if out := EqualFold(tt.s, tt.t); out != tt.out {
// 			t.Errorf("EqualFold(%#q, %#q) = {}, want {}", tt.s, tt.t, out, tt.out)
// 		}
// 		if out := EqualFold(tt.t, tt.s); out != tt.out {
// 			t.Errorf("EqualFold(%#q, %#q) = {}, want {}", tt.t, tt.s, out, tt.out)
// 		}
// 	}
// }

// fn BenchmarkEqualFold(b *testing.B) {
// 	b.Run("Tests", fn(b *testing.B) {
// 		for i := 0; i < b.N; i++ {
// 			for _, tt := range EqualFoldTests {
// 				if out := EqualFold(tt.s, tt.t); out != tt.out {
// 					b.Fatal("wrong result")
// 				}
// 			}
// 		}
// 	})

// 	const s1 = "abcdefghijKz"
// 	const s2 = "abcDefGhijKz"

// 	b.Run("ASCII", fn(b *testing.B) {
// 		for i := 0; i < b.N; i++ {
// 			EqualFold(s1, s2)
// 		}
// 	})

// 	b.Run("UnicodePrefix", fn(b *testing.B) {
// 		for i := 0; i < b.N; i++ {
// 			EqualFold("αβδ"+s1, "ΑΒΔ"+s2)
// 		}
// 	})

// 	b.Run("UnicodeSuffix", fn(b *testing.B) {
// 		for i := 0; i < b.N; i++ {
// 			EqualFold(s1+"αβδ", s2+"ΑΒΔ")
// 		}
// 	})
// }

// var CountTests = []struct {
// 	s, sep string
// 	num    isize
// }{
// 	{"", "", 1},
// 	{"", "notempty", 0},
// 	{"notempty", "", 9},
// 	{"smaller", "not smaller", 0},
// 	{"12345678987654321", "6", 2},
// 	{"611161116", "6", 3},
// 	{"notequal", "NotEqual", 0},
// 	{"equal", "equal", 1},
// 	{"abc1231231123q", "123", 3},
// 	{"11111", "11", 2},
// }

// #[test]
// fn TestCount() {
// 	for _, tt := range CountTests {
// 		if num := Count(tt.s, tt.sep); num != tt.num {
// 			t.Errorf("Count({:?}, {:?}) = %d, want %d", tt.s, tt.sep, num, tt.num)
// 		}
// 	}
// }

struct CutTest {
    s: &'static str,
    sep: &'static str,
    before: &'static str,
    after: &'static str,
    found: bool,
}

impl CutTest {
    const fn new(
        s: &'static str,
        sep: &'static str,
        before: &'static str,
        after: &'static str,
        found: bool,
    ) -> Self {
        Self {
            s,
            sep,
            before,
            after,
            found,
        }
    }
}

const CUT_TESTS: &[CutTest] = &[
    CutTest::new("abc", "b", "a", "c", true),
    CutTest::new("abc", "a", "", "bc", true),
    CutTest::new("abc", "c", "ab", "", true),
    CutTest::new("abc", "abc", "", "", true),
    CutTest::new("abc", "", "", "abc", true),
    CutTest::new("abc", "d", "abc", "", false),
    CutTest::new("", "d", "", "", false),
    CutTest::new("", "", "", "", true),
    CutTest::new("привет,мир", ",", "привет", "мир", true),
    CutTest::new("привет🙂мир", "🙂", "привет", "мир", true),
];

#[test]
fn test_cut() {
    for tt in CUT_TESTS {
        let (before, after, found) = super::cut(tt.s, tt.sep);
        assert_eq!(before.to_string(), tt.before);
        assert_eq!(after.to_string(), tt.after);
        assert_eq!(found, tt.found);
    }
}

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
// 	for _, tt := range cutPrefixTests {
// 		if after, found := CutPrefix(tt.s, tt.sep); after != tt.after || found != tt.found {
// 			t.Errorf("CutPrefix({:?}, {:?}) = {:?}, {}, want {:?}, {}", tt.s, tt.sep, after, found, tt.after, tt.found)
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
// 	for _, tt := range cutSuffixTests {
// 		if after, found := CutSuffix(tt.s, tt.sep); after != tt.after || found != tt.found {
// 			t.Errorf("CutSuffix({:?}, {:?}) = {:?}, {}, want {:?}, {}", tt.s, tt.sep, after, found, tt.after, tt.found)
// 		}
// 	}
// }

// fn makeBenchInputHard() string {
// 	tokens := [...]string{
// 		"<a>", "<p>", "<b>", "<strong>",
// 		"</a>", "</p>", "</b>", "</strong>",
// 		"hello", "world",
// 	}
// 	x := make([]byte, 0, 1<<20)
// 	for {
// 		i := rand.Intn(len(tokens))
// 		if len(x)+len(tokens[i]) >= 1<<20 {
// 			break
// 		}
// 		x = append(x, tokens[i]...)
// 	}
// 	return string(x)
// }

// var benchInputHard = makeBenchInputHard()

// fn benchmarkIndexHard(b *testing.B, sep string) {
// 	for i := 0; i < b.N; i++ {
// 		Index(benchInputHard, sep)
// 	}
// }

// fn benchmarkLastIndexHard(b *testing.B, sep string) {
// 	for i := 0; i < b.N; i++ {
// 		LastIndex(benchInputHard, sep)
// 	}
// }

// fn benchmarkCountHard(b *testing.B, sep string) {
// 	for i := 0; i < b.N; i++ {
// 		Count(benchInputHard, sep)
// 	}
// }

// fn BenchmarkIndexHard1(b *testing.B) { benchmarkIndexHard(b, "<>") }
// fn BenchmarkIndexHard2(b *testing.B) { benchmarkIndexHard(b, "</pre>") }
// fn BenchmarkIndexHard3(b *testing.B) { benchmarkIndexHard(b, "<b>hello world</b>") }
// fn BenchmarkIndexHard4(b *testing.B) {
// 	benchmarkIndexHard(b, "<pre><b>hello</b><strong>world</strong></pre>")
// }

// fn BenchmarkLastIndexHard1(b *testing.B) { benchmarkLastIndexHard(b, "<>") }
// fn BenchmarkLastIndexHard2(b *testing.B) { benchmarkLastIndexHard(b, "</pre>") }
// fn BenchmarkLastIndexHard3(b *testing.B) { benchmarkLastIndexHard(b, "<b>hello world</b>") }

// fn BenchmarkCountHard1(b *testing.B) { benchmarkCountHard(b, "<>") }
// fn BenchmarkCountHard2(b *testing.B) { benchmarkCountHard(b, "</pre>") }
// fn BenchmarkCountHard3(b *testing.B) { benchmarkCountHard(b, "<b>hello world</b>") }

// var benchInputTorture = Repeat("ABC", 1<<10) + "123" + Repeat("ABC", 1<<10)
// var benchNeedleTorture = Repeat("ABC", 1<<10+1)

// fn BenchmarkIndexTorture(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Index(benchInputTorture, benchNeedleTorture)
// 	}
// }

// fn BenchmarkCountTorture(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Count(benchInputTorture, benchNeedleTorture)
// 	}
// }

// fn BenchmarkCountTortureOverlapping(b *testing.B) {
// 	A := Repeat("ABC", 1<<20)
// 	B := Repeat("ABC", 1<<10)
// 	for i := 0; i < b.N; i++ {
// 		Count(A, B)
// 	}
// }

// fn BenchmarkCountByte(b *testing.B) {
// 	indexSizes := []isize{10, 32, 4 << 10, 4 << 20, 64 << 20}
// 	benchStr := Repeat(benchmarkString,
// 		(indexSizes[len(indexSizes)-1]+len(benchmarkString)-1)/len(benchmarkString))
// 	benchFunc := fn(b *testing.B, benchStr string) {
// 		b.SetBytes(int64(len(benchStr)))
// 		for i := 0; i < b.N; i++ {
// 			Count(benchStr, "=")
// 		}
// 	}
// 	for _, size := range indexSizes {
// 		b.Run(fmt.Sprintf("%d", size), fn(b *testing.B) {
// 			benchFunc(b, benchStr[:size])
// 		})
// 	}

// }

// var makeFieldsInput = fn() string {
// 	x := make([]byte, 1<<20)
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
// 	return string(x)
// }

// var makeFieldsInputASCII = fn() string {
// 	x := make([]byte, 1<<20)
// 	// Input is ~10% space, rest ASCII non-space.
// 	for i := range x {
// 		if rand.Intn(10) == 0 {
// 			x[i] = ' '
// 		} else {
// 			x[i] = 'x'
// 		}
// 	}
// 	return string(x)
// }

// var stringdata = []struct{ name, data string }{
// 	{"ASCII", makeFieldsInputASCII()},
// 	{"Mixed", makeFieldsInput()},
// }

// fn BenchmarkFields(b *testing.B) {
// 	for _, sd := range stringdata {
// 		b.Run(sd.name, fn(b *testing.B) {
// 			for j := 1 << 4; j <= 1<<20; j <<= 4 {
// 				b.Run(fmt.Sprintf("%d", j), fn(b *testing.B) {
// 					b.ReportAllocs()
// 					b.SetBytes(int64(j))
// 					data := sd.data[:j]
// 					for i := 0; i < b.N; i++ {
// 						Fields(data)
// 					}
// 				})
// 			}
// 		})
// 	}
// }

// fn BenchmarkFieldsFunc(b *testing.B) {
// 	for _, sd := range stringdata {
// 		b.Run(sd.name, fn(b *testing.B) {
// 			for j := 1 << 4; j <= 1<<20; j <<= 4 {
// 				b.Run(fmt.Sprintf("%d", j), fn(b *testing.B) {
// 					b.ReportAllocs()
// 					b.SetBytes(int64(j))
// 					data := sd.data[:j]
// 					for i := 0; i < b.N; i++ {
// 						FieldsFunc(data, unicode.IsSpace)
// 					}
// 				})
// 			}
// 		})
// 	}
// }

// fn BenchmarkSplitEmptySeparator(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Split(benchInputHard, "")
// 	}
// }

// fn BenchmarkSplitSingleByteSeparator(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Split(benchInputHard, "/")
// 	}
// }

// fn BenchmarkSplitMultiByteSeparator(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Split(benchInputHard, "hello")
// 	}
// }

// fn BenchmarkSplitNSingleByteSeparator(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		SplitN(benchInputHard, "/", 10)
// 	}
// }

// fn BenchmarkSplitNMultiByteSeparator(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		SplitN(benchInputHard, "hello", 10)
// 	}
// }

// fn BenchmarkRepeat(b *testing.B) {
// 	s := "0123456789"
// 	for _, n := range []isize{5, 10} {
// 		for _, c := range []isize{0, 1, 2, 6} {
// 			b.Run(fmt.Sprintf("%dx%d", n, c), fn(b *testing.B) {
// 				for i := 0; i < b.N; i++ {
// 					Repeat(s[:n], c)
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkRepeatLarge(b *testing.B) {
// 	s := Repeat("@", 8*1024)
// 	for j := 8; j <= 30; j++ {
// 		for _, k := range []isize{1, 16, 4097} {
// 			s := s[:k]
// 			n := (1 << j) / k
// 			if n == 0 {
// 				continue
// 			}
// 			b.Run(fmt.Sprintf("%d/%d", 1<<j, k), fn(b *testing.B) {
// 				for i := 0; i < b.N; i++ {
// 					Repeat(s, n)
// 				}
// 				b.SetBytes(int64(n * len(s)))
// 			})
// 		}
// 	}
// }

// fn BenchmarkIndexAnyASCII(b *testing.B) {
// 	x := Repeat("#", 2048) // Never matches set
// 	cs := "0123456789abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz"
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("%d:%d", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i++ {
// 					IndexAny(x[:k], cs[:j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkIndexAnyUTF8(b *testing.B) {
// 	x := Repeat("#", 2048) // Never matches set
// 	cs := "你好世界, hello world. 你好世界, hello world. 你好世界, hello world."
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("%d:%d", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i++ {
// 					IndexAny(x[:k], cs[:j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkLastIndexAnyASCII(b *testing.B) {
// 	x := Repeat("#", 2048) // Never matches set
// 	cs := "0123456789abcdefghijklmnopqrstuvwxyz0123456789abcdefghijklmnopqrstuvwxyz"
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("%d:%d", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i++ {
// 					LastIndexAny(x[:k], cs[:j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkLastIndexAnyUTF8(b *testing.B) {
// 	x := Repeat("#", 2048) // Never matches set
// 	cs := "你好世界, hello world. 你好世界, hello world. 你好世界, hello world."
// 	for k := 1; k <= 2048; k <<= 4 {
// 		for j := 1; j <= 64; j <<= 1 {
// 			b.Run(fmt.Sprintf("%d:%d", k, j), fn(b *testing.B) {
// 				for i := 0; i < b.N; i++ {
// 					LastIndexAny(x[:k], cs[:j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkTrimASCII(b *testing.B) {
// 	cs := "0123456789abcdef"
// 	for k := 1; k <= 4096; k <<= 4 {
// 		for j := 1; j <= 16; j <<= 1 {
// 			b.Run(fmt.Sprintf("%d:%d", k, j), fn(b *testing.B) {
// 				x := Repeat(cs[:j], k) // Always matches set
// 				for i := 0; i < b.N; i++ {
// 					Trim(x[:k], cs[:j])
// 				}
// 			})
// 		}
// 	}
// }

// fn BenchmarkTrimByte(b *testing.B) {
// 	x := "  the quick brown fox   "
// 	for i := 0; i < b.N; i++ {
// 		Trim(x, " ")
// 	}
// }

// fn BenchmarkIndexPeriodic(b *testing.B) {
// 	key := "aa"
// 	for _, skip := range [...]isize{2, 4, 8, 16, 32, 64} {
// 		b.Run(fmt.Sprintf("IndexPeriodic%d", skip), fn(b *testing.B) {
// 			s := Repeat("a"+Repeat(" ", skip-1), 1<<16/skip)
// 			for i := 0; i < b.N; i++ {
// 				Index(s, key)
// 			}
// 		})
// 	}
// }

// fn BenchmarkJoin(b *testing.B) {
// 	vals := []string{"red", "yellow", "pink", "green", "purple", "orange", "blue"}
// 	for l := 0; l <= len(vals); l++ {
// 		b.Run(strconv.Itoa(l), fn(b *testing.B) {
// 			b.ReportAllocs()
// 			vals := vals[:l]
// 			for i := 0; i < b.N; i++ {
// 				Join(vals, " and ")
// 			}
// 		})
// 	}
// }

// fn BenchmarkTrimSpace(b *testing.B) {
// 	tests := []struct{ name, input string }{
// 		{"NoTrim", "typical"},
// 		{"ASCII", "  foo bar  "},
// 		{"SomeNonASCII", "    \u2000\t\r\n x\t\t\r\r\ny\n \u3000    "},
// 		{"JustNonASCII", "\u2000\u2000\u2000☺☺☺☺\u3000\u3000\u3000"},
// 	}
// 	for test in tests {
// 		b.Run(test.name, fn(b *testing.B) {
// 			for i := 0; i < b.N; i++ {
// 				TrimSpace(test.input)
// 			}
// 		})
// 	}
// }

// var stringSink string

// fn BenchmarkReplaceAll(b *testing.B) {
// 	b.ReportAllocs()
// 	for i := 0; i < b.N; i++ {
// 		stringSink = replace_all("banana", "a", "<>")
// 	}
// }
