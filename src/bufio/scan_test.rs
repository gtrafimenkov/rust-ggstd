// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2013 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::{scan_bytes, scan_lines, Error, Scanner};
use crate::bytes;
use crate::strings;

// import (
// 	. "bufio"
// 	"bytes"
// 	"errors"
// 	"io"
// 	"strings"
// 	"testing"
// 	"unicode"
// 	"unicode/utf8"
// )

// const smallMaxTokenSize = 256 // Much smaller for more efficient testing.

// // Test white space table matches the Unicode definition.
// #[test]
// fn TestSpace() {
// 	for r := rune(0); r <= utf8.MAX_RUNE; r++ {
// 		if IsSpace(r) != unicode.IsSpace(r) {
// 			t.Fatalf("white space property disagrees: %#U should be %t", r, unicode.IsSpace(r))
// 		}
// 	}
// }

const SCAN_TESTS: &[&[u8]] = &[
    "".as_bytes(),
    "a".as_bytes(),
    "¼".as_bytes(),
    "☹".as_bytes(),
    b"\x81",               // UTF-8 error
    "\u{FFFD}".as_bytes(), // correctly encoded RUNE_ERROR
    "abcdefgh".as_bytes(),
    "abc def\n\t\tgh    ".as_bytes(),
    // "abc¼☹\x81\u{FFFD}日本語\x82abc".as_bytes(),
];

#[test]
fn test_scan_byte() {
    for (n, test) in SCAN_TESTS.iter().enumerate() {
        let mut buf = bytes::Reader::new(*test);
        let mut s = Scanner::new(&mut buf);
        s.split(scan_bytes);
        let mut i = 0;
        while s.scan() {
            let b = s.bytes();
            assert!(
                b.len() == 1 && b[0] == test[i],
                "#{}: {}: expected {:?} got {:?}",
                n,
                i,
                test,
                b
            );
            i += 1;
        }
        assert_eq!(
            i,
            test.len(),
            "#{}: termination expected at {}; got {}",
            n,
            test.len(),
            i
        );
        assert!(s.err().is_none(), "#{}: {:?}", n, s.err());
    }
}

// // Test that the rune splitter returns same sequence of runes (not bytes) as for range string.
// #[test]
// fn TestScanRune() {
// 	for n, test in SCAN_TESTS.iter().enumerate() {
// 		buf := strings::Reader::new(test)
// 		s := Scanner::new(buf)
// 		s.split(ScanRunes)
// 		var i, runeCount int
// 		var expect rune
// 		// Use a string range loop to validate the sequence of runes.
// 		for i, expect = range string(test) {
// 			if !s.scan() {
// 				break
// 			}
// 			runeCount++
// 			got, _ := utf8.decode_rune(s.bytes())
// 			if got != expect {
// 				t.Errorf("#{}: {}: expected %q got %q", n, i, expect, got)
// 			}
// 		}
// 		if s.scan() {
// 			t.Errorf("#{}: scan ran too long, got %q", n, s.Text())
// 		}
// 		testRuneCount := utf8.RuneCountInString(test)
// 		if runeCount != testRuneCount {
// 			t.Errorf("#{}: termination expected at {}; got {}", n, testRuneCount, runeCount)
// 		}
// 		err := s.Err()
// 		if err != nil {
// 			t.Errorf("#{}: {}", n, err)
// 		}
// 	}
// }

// var wordScanTests = []string{
// 	"",
// 	" ",
// 	"\n",
// 	"a",
// 	" a ",
// 	"abc def",
// 	" abc def ",
// 	" abc\tdef\nghi\rjkl\fmno\vpqr\u0085stu\u00a0\n",
// }

// // Test that the word splitter returns the same data as strings.Fields.
// #[test]
// fn TestScanWords() {
// 	for n, test := range wordScanTests {
// 		buf := strings::Reader::new(test)
// 		s := Scanner::new(buf)
// 		s.split(ScanWords)
// 		words := strings.Fields(test)
// 		var wordCount int
// 		for wordCount = 0; wordCount < len(words); wordCount++ {
// 			if !s.scan() {
// 				break
// 			}
// 			got := s.Text()
// 			if got != words[wordCount] {
// 				t.Errorf("#{}: {}: expected %q got %q", n, wordCount, words[wordCount], got)
// 			}
// 		}
// 		if s.scan() {
// 			t.Errorf("#{}: scan ran too long, got %q", n, s.Text())
// 		}
// 		if wordCount != len(words) {
// 			t.Errorf("#{}: termination expected at {}; got {}", n, len(words), wordCount)
// 		}
// 		err := s.Err()
// 		if err != nil {
// 			t.Errorf("#{}: {}", n, err)
// 		}
// 	}
// }

/// SlowReader is a reader that returns only a few bytes at a time, to test the incremental
/// reads in Scanner.scan.
struct SlowReader<'a> {
    max: usize,
    r: &'a mut dyn std::io::Read,
}

impl<'a> SlowReader<'a> {
    fn new(max: usize, r: &'a mut dyn std::io::Read) -> Self {
        Self { max, r }
    }
}

impl std::io::Read for SlowReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.max.min(buf.len());
        self.r.read(&mut buf[0..len])
    }
}

// // genLine writes to buf a predictable but non-trivial line of text of length
// // n, including the terminal newline and an occasional carriage return.
// // If addNewline is false, the \r and \n are not emitted.
// fn genLine(buf *bytes::Buffer::new(), lineNum, n int, addNewline bool) {
// 	buf.Reset()
// 	doCR := lineNum%5 == 0
// 	if doCR {
// 		n--
// 	}
// 	for i := 0; i < n-1; i += 1 { // Stop early for \n.
// 		c := 'a' + byte(lineNum+i)
// 		if c == '\n' || c == '\r' { // Don't confuse us.
// 			c = 'N'
// 		}
// 		buf.write_byte(c)
// 	}
// 	if addNewline {
// 		if doCR {
// 			buf.write_byte('\r')
// 		}
// 		buf.write_byte('\n')
// 	}
// }

// // Test the line splitter, including some carriage returns but no long lines.
// #[test]
// fn TestScanLongLines() {
// 	// Build a buffer of lots of line lengths up to but not exceeding smallMaxTokenSize.
// 	tmp := new(bytes::Buffer::new())
// 	let mut buf = bytes::Buffer::new();
// 	lineNum := 0
// 	j := 0
// 	for i := 0; i < 2*smallMaxTokenSize; i += 1 {
// 		genLine(tmp, lineNum, j, true)
// 		if j < smallMaxTokenSize {
// 			j++
// 		} else {
// 			j--
// 		}
// 		buf.Write(tmp.bytes())
// 		lineNum++
// 	}
// 	s := Scanner::new(&SlowReader::new(1, buf))
// 	s.split(scan_lines)
// 	s.MaxTokenSize(smallMaxTokenSize)
// 	j = 0
// 	for lineNum := 0; s.scan(); lineNum++ {
// 		genLine(tmp, lineNum, j, false)
// 		if j < smallMaxTokenSize {
// 			j++
// 		} else {
// 			j--
// 		}
// 		line := tmp.String() // We use the string-valued token here, for variety.
// 		if s.Text() != line {
// 			t.Errorf("{}: bad line: {} {}\n%.100q\n%.100q\n", lineNum, len(s.bytes()), len(line), s.Text(), line)
// 		}
// 	}
// 	err := s.Err()
// 	if err != nil {
// 		t.Fatal(err)
// 	}
// }

// // Test that the line splitter errors out on a long line.
// #[test]
// fn TestScanLineTooLong() {
// 	const smallMaxTokenSize = 256 // Much smaller for more efficient testing.
// 	// Build a buffer of lots of line lengths up to but not exceeding smallMaxTokenSize.
// 	tmp := new(bytes::Buffer::new())
// 	let mut buf = bytes::Buffer::new();
// 	lineNum := 0
// 	j := 0
// 	for i := 0; i < 2*smallMaxTokenSize; i += 1 {
// 		genLine(tmp, lineNum, j, true)
// 		j++
// 		buf.Write(tmp.bytes())
// 		lineNum++
// 	}
// 	s := Scanner::new(&SlowReader::new(3, buf))
// 	s.split(scan_lines)
// 	s.MaxTokenSize(smallMaxTokenSize)
// 	j = 0
// 	for lineNum := 0; s.scan(); lineNum++ {
// 		genLine(tmp, lineNum, j, false)
// 		if j < smallMaxTokenSize {
// 			j++
// 		} else {
// 			j--
// 		}
// 		line := tmp.bytes()
// 		if !bytes::equal(s.bytes(), line) {
// 			t.Errorf("{}: bad line: {} {}\n%.100q\n%.100q\n", lineNum, len(s.bytes()), len(line), s.bytes(), line)
// 		}
// 	}
// 	err := s.Err()
// 	if err != ErrTooLong {
// 		t.Fatalf("expected ErrTooLong; got %s", err)
// 	}
// }

/// Test that the line splitter handles a final line without a newline.
fn test_no_newline(text: &str, lines: &[&str]) {
    let mut buf = strings::Reader::new(text);
    let mut sr = SlowReader::new(7, &mut buf);
    let mut s = Scanner::new(&mut sr);
    s.split(scan_lines);
    let mut line_num = 0;
    while s.scan() {
        let line = lines[line_num];
        assert_eq!(
            s.text(),
            line,
            "{}: bad line: {:?} {:?}",
            line_num,
            s.text(),
            line
        );
        line_num += 1;
    }
    assert!(s.err().is_none());
}

/// Test that the line splitter handles a final line without a newline.
#[test]
fn test_scan_line_no_newline() {
    let text = "abcdefghijklmn\nopqrstuvwxyz";
    let lines = &["abcdefghijklmn", "opqrstuvwxyz"];
    test_no_newline(text, lines);
}

/// Test that the line splitter handles a final line with a carriage return but no newline.
#[test]
fn test_scan_line_return_but_no_newline() {
    let text = "abcdefghijklmn\nopqrstuvwxyz\r";
    let lines = &["abcdefghijklmn", "opqrstuvwxyz"];
    test_no_newline(text, lines);
}

/// Test that the line splitter handles a final empty line.
#[test]
fn test_scan_line_empty_final_line() {
    test_no_newline(
        "abcdefghijklmn\nopqrstuvwxyz\n\n",
        &["abcdefghijklmn", "opqrstuvwxyz", ""],
    );
}

/// Test that the line splitter handles a final empty line with a carriage return but no newline.
#[test]
fn test_scan_line_empty_final_line_with_cr() {
    test_no_newline(
        "abcdefghijklmn\nopqrstuvwxyz\r\n",
        &["abcdefghijklmn", "opqrstuvwxyz", ""],
    );
}

// var testError = errors.New("testError")

// // Test the correct error is returned when the split function errors out.
// #[test]
// fn TestSplitError() {
// 	// Create a split function that delivers a little data, then a predictable error.
// 	numSplits := 0
// 	const okCount = 7
// 	errorSplit := fn(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 		if atEOF {
// 			panic("didn't get enough data")
// 		}
// 		if numSplits >= okCount {
// 			return 0, nil, testError
// 		}
// 		numSplits++
// 		return 1, data[0:1], nil
// 	}
// 	// Read the data.
// 	const text = "abcdefghijklmnopqrstuvwxyz"
// 	buf := strings::Reader::new(text)
// 	s := Scanner::new(&SlowReader::new(1, buf))
// 	s.split(errorSplit)
// 	var i int
// 	for i = 0; s.scan(); i += 1 {
// 		if len(s.bytes()) != 1 || text[i] != s.bytes()[0] {
// 			t.Errorf("#{}: expected %q got %q", i, text[i], s.bytes()[0])
// 		}
// 	}
// 	// Check correct termination location and error.
// 	if i != okCount {
// 		t.Errorf("unexpected termination; expected {} tokens got {}", okCount, i)
// 	}
// 	err := s.Err()
// 	if err != testError {
// 		t.Fatalf("expected %q got {}", testError, err)
// 	}
// }

// // Test that an EOF is overridden by a user-generated scan error.
// #[test]
// fn TestErrAtEOF() {
// 	s := Scanner::new(strings::Reader::new("1 2 33"))
// 	// This splitter will fail on last entry, after s.err==EOF.
// 	split := fn(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 		advance, token, err = ScanWords(data, atEOF)
// 		if len(token) > 1 {
// 			if s.ErrOrEOF() != io.EOF {
// 				t.Fatal("not testing EOF")
// 			}
// 			err = testError
// 		}
// 		return
// 	}
// 	s.split(split)
// 	for s.scan() {
// 	}
// 	if s.Err() != testError {
// 		t.Fatal("wrong error:", s.Err())
// 	}
// }

/// Test for issue 5268.
struct AlwaysError {}

impl std::io::Read for AlwaysError {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
    }
}

#[test]
fn test_non_eofwith_empty_read() {
    let mut r = AlwaysError {};
    let mut scanner = Scanner::new(&mut r);
    assert_eq!(false, scanner.scan(), "read should fail");
    assert_eq!(false, scanner.scan(), "read should fail");
    assert_eq!(false, scanner.scan(), "read should fail");
    assert_eq!(false, scanner.scan(), "read should fail");
    let err = scanner.err();
    assert!(err.is_some(), "unexpected error: {:?}", err);
    let err = err.as_ref().unwrap();
    if let Error::ErrRead(err) = err {
        assert_eq!(
            err.kind(),
            std::io::ErrorKind::UnexpectedEof,
            "expecting std::io::ErrorKind::UnexpectedEof, got {:?}",
            err
        );
    } else {
        assert!(false, "expecting Error::ErrRead, got {:?}", err);
    }
}

// // Test that Scan finishes if we have endless empty reads.
// type endlessZeros struct{}

// fn (endlessZeros) Read(p [u8]) (int, error) {
// 	return 0, nil
// }

// #[test]
// fn TestBadReader() {
// 	scanner := Scanner::new(endlessZeros{})
// 	for scanner.scan() {
// 		t.Fatal("read should fail")
// 	}
// 	err := scanner.err()
// 	if err != io.ErrNoProgress {
// 		t.Errorf("unexpected error: {}", err)
// 	}
// }

// #[test]
// fn TestScanWordsExcessiveWhiteSpace() {
// 	const word = "ipsum"
// 	s := strings.Repeat(" ", 4*smallMaxTokenSize) + word
// 	scanner := Scanner::new(strings::Reader::new(s))
// 	scanner.MaxTokenSize(smallMaxTokenSize)
// 	scanner.split(ScanWords)
// 	if !scanner.scan() {
// 		t.Fatalf("scan failed: {}", scanner.err())
// 	}
// 	if token := scanner.Text(); token != word {
// 		t.Fatalf("unexpected token: {}", token)
// 	}
// }

// // Test that empty tokens, including at end of line or end of file, are found by the scanner.
// // Issue 8672: Could miss final empty token.

// fn commaSplit(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 	for i := 0; i < len(data); i += 1 {
// 		if data[i] == ',' {
// 			return i + 1, data[..i], nil
// 		}
// 	}
// 	return 0, data, ErrFinalToken
// }

// #[test]
// fn testEmptyTokens(, text string, values []string) {
// 	s := Scanner::new(strings::Reader::new(text))
// 	s.split(commaSplit)
// 	var i int
// 	for i = 0; s.scan(); i += 1 {
// 		if i >= len(values) {
// 			t.Fatalf("got {} fields, expected {}", i+1, len(values))
// 		}
// 		if s.Text() != values[i] {
// 			t.Errorf("{}: expected %q got %q", i, values[i], s.Text())
// 		}
// 	}
// 	if i != len(values) {
// 		t.Fatalf("got {} fields, expected {}", i, len(values))
// 	}
// 	if err := s.Err(); err != nil {
// 		t.Fatal(err)
// 	}
// }

// #[test]
// fn TestEmptyTokens() {
// 	testEmptyTokens(t, "1,2,3,", []string{"1", "2", "3", ""})
// }

// #[test]
// fn TestWithNoEmptyTokens() {
// 	testEmptyTokens(t, "1,2,3", []string{"1", "2", "3"})
// }

// fn loopAtEOFSplit(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 	if len(data) > 0 {
// 		return 1, data[..1], nil
// 	}
// 	return 0, data, nil
// }

// #[test]
// fn TestDontLoopForever() {
// 	s := Scanner::new(strings::Reader::new("abc"))
// 	s.split(loopAtEOFSplit)
// 	// Expect a panic
// 	defer fn() {
// 		err := recover()
// 		if err == nil {
// 			t.Fatal("should have panicked")
// 		}
// 		if msg, ok := err.(string); !ok || !strings::contains(msg, "empty tokens") {
// 			panic(err)
// 		}
// 	}()
// 	for count := 0; s.scan(); count++ {
// 		if count > 1000 {
// 			t.Fatal("looping")
// 		}
// 	}
// 	if s.Err() != nil {
// 		t.Fatal("after scan:", s.Err())
// 	}
// }

// #[test]
// fn TestBlankLines() {
// 	s := Scanner::new(strings::Reader::new(strings.Repeat("\n", 1000)))
// 	for count := 0; s.scan(); count++ {
// 		if count > 2000 {
// 			t.Fatal("looping")
// 		}
// 	}
// 	if s.Err() != nil {
// 		t.Fatal("after scan:", s.Err())
// 	}
// }

// type countdown int

// fn (c *countdown) split(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 	if *c > 0 {
// 		*c--
// 		return 1, data[..1], nil
// 	}
// 	return 0, nil, nil
// }

// // Check that the looping-at-EOF check doesn't trigger for merely empty tokens.
// #[test]
// fn TestEmptyLinesOK() {
// 	c := countdown(10000)
// 	s := Scanner::new(strings::Reader::new(strings.Repeat("\n", 10000)))
// 	s.split(c.split)
// 	for s.scan() {
// 	}
// 	if s.Err() != nil {
// 		t.Fatal("after scan:", s.Err())
// 	}
// 	if c != 0 {
// 		t.Fatalf("stopped with {} left to process", c)
// 	}
// }

// // Make sure we can read a huge token if a big enough buffer is provided.
// #[test]
// fn TestHugeBuffer() {
// 	text := strings.Repeat("x", 2*MAX_SCAN_TOKEN_SIZE)
// 	s := Scanner::new(strings::Reader::new(text + "\n"))
// 	s.Buffer(make([u8], 100), 3*MAX_SCAN_TOKEN_SIZE)
// 	for s.scan() {
// 		token := s.Text()
// 		if token != text {
// 			t.Errorf("scan got incorrect token of length {}", len(token))
// 		}
// 	}
// 	if s.Err() != nil {
// 		t.Fatal("after scan:", s.Err())
// 	}
// }

// // negativeEOFReader returns an invalid -1 at the end, as though it
// // were wrapping the read system call.
// type negativeEOFReader int

// fn (r *negativeEOFReader) Read(p [u8]) (int, error) {
// 	if *r > 0 {
// 		c := int(*r)
// 		if c > len(p) {
// 			c = len(p)
// 		}
// 		for i := 0; i < c; i += 1 {
// 			p[i] = 'a'
// 		}
// 		p[c-1] = '\n'
// 		*r -= negativeEOFReader(c)
// 		return c, nil
// 	}
// 	return -1, io.EOF
// }

// // Test that the scanner doesn't panic and returns ErrBadReadCount
// // on a reader that returns a negative count of bytes read (issue 38053).
// #[test]
// fn TestNegativeEOFReader() {
// 	r := negativeEOFReader(10)
// 	scanner := Scanner::new(&r)
// 	c := 0
// 	for scanner.scan() {
// 		c++
// 		if c > 1 {
// 			t.Error("read too many lines")
// 			break
// 		}
// 	}
// 	if got, want := scanner.err(), ErrBadReadCount; got != want {
// 		t.Errorf("scanner.err: got {}, want {}", got, want)
// 	}
// }

// // largeReader returns an invalid count that is larger than the number
// // of bytes requested.
// type largeReader struct{}

// fn (largeReader) Read(p [u8]) (int, error) {
// 	return len(p) + 1, nil
// }

// // Test that the scanner doesn't panic and returns ErrBadReadCount
// // on a reader that returns an impossibly large count of bytes read (issue 38053).
// #[test]
// fn TestLargeReader() {
// 	scanner := Scanner::new(largeReader{})
// 	for scanner.scan() {
// 	}
// 	if got, want := scanner.err(), ErrBadReadCount; got != want {
// 		t.Errorf("scanner.err: got {}, want {}", got, want)
// 	}
// }
