// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// import (
// 	. "strconv"
// 	"strings"
// 	"testing"
// 	"unicode"
// )

// // Verify that our is_print agrees with unicode.is_print.
// #[test]
// fn TestIsPrint() {
// 	n := 0
// 	for r := rune(0); r <= unicode.MAX_RUNE; r++ {
// 		if is_print(r) != unicode.is_print(r) {
// 			t.Errorf("is_print(%U)=%t incorrect", r, is_print(r))
// 			n++
// 			if n > 10 {
// 				return
// 			}
// 		}
// 	}
// }

// // Verify that our is_graphic agrees with unicode.is_graphic.
// #[test]
// fn TestIsGraphic() {
// 	n := 0
// 	for r := rune(0); r <= unicode.MAX_RUNE; r++ {
// 		if is_graphic(r) != unicode.is_graphic(r) {
// 			t.Errorf("is_graphic(%U)=%t incorrect", r, is_graphic(r))
// 			n++
// 			if n > 10 {
// 				return
// 			}
// 		}
// 	}
// }

struct QuoteTest {
    input: &'static str,
    out: &'static str,
    ascii: &'static str,
    graphic: &'static str,
}

impl QuoteTest {
    const fn new(
        input: &'static str,
        out: &'static str,
        ascii: &'static str,
        graphic: &'static str,
    ) -> Self {
        Self {
            input,
            out,
            ascii,
            graphic,
        }
    }
}

static QUOTE_TESTS: &[QuoteTest] = &[
    QuoteTest::new(
        "\x07\x08\x0C\r\n\x09\x0B",
        r#""\a\b\f\r\n\t\v""#,
        r#""\a\b\f\r\n\t\v""#,
        r#""\a\b\f\r\n\t\v""#,
    ),
    QuoteTest::new("\\", r#""\\""#, r#""\\""#, r#""\\""#),
    // TODO: test with quote_bytes_string
    // QuoteTest::new("abc\xffdef", r#""abc\xffdef""#, r#""abc\xffdef""#, r#""abc\xffdef""#),
    QuoteTest::new("\u{263a}", r#""☺""#, r#""\u263a""#, r#""☺""#),
    QuoteTest::new(
        "\u{10ffff}",
        r#""\U0010ffff""#,
        r#""\U0010ffff""#,
        r#""\U0010ffff""#,
    ),
    QuoteTest::new("\x04", r#""\x04""#, r#""\x04""#, r#""\x04""#),
    // Some non-printable but graphic runes. Final column is double-quoted.
    QuoteTest::new(
        "!\u{00a0}!\u{2000}!\u{3000}!",
        r#""!\u00a0!\u2000!\u3000!""#,
        r#""!\u00a0!\u2000!\u3000!""#,
        "\"!\u{00a0}!\u{2000}!\u{3000}!\"",
    ),
    QuoteTest::new("\x7f", r#""\x7f""#, r#""\x7f""#, r#""\x7f""#),
];

#[test]
fn test_quote() {
    for tt in QUOTE_TESTS {
        let out = super::quote(tt.input);
        assert_eq!(
            out, tt.out,
            "quote({}) = {}, want {}",
            tt.input, out, tt.out
        );
        let mut out = "abc".as_bytes().to_vec();
        super::append_quote(&mut out, tt.input);
        let mut expect = "abc".as_bytes().to_vec();
        expect.extend_from_slice(tt.out.as_bytes());
        assert_eq!(
            out, expect,
            "append_quote({:?}, {:?}) = {:?}, want {:?}",
            "abc", tt.input, out, expect
        );
    }
}

#[test]
fn test_quote_to_ascii() {
    for tt in QUOTE_TESTS {
        let out = super::quote_to_ascii(tt.input);
        assert_eq!(
            out, tt.ascii,
            "quote_to_ascii({}) = {}, want {}",
            tt.input, out, tt.ascii
        );
        let mut out = "abc".as_bytes().to_vec();
        super::append_quote_to_ascii(&mut out, tt.input);
        let mut expect = "abc".as_bytes().to_vec();
        expect.extend_from_slice(tt.ascii.as_bytes());
        assert_eq!(
            out, expect,
            "append_quote_to_ascii({:?}, {:?}) = {:?}, want {:?}",
            "abc", tt.input, out, expect
        );
    }
}

#[test]
fn test_quote_to_graphic() {
    for tt in QUOTE_TESTS {
        let out = super::quote_to_graphic(tt.input);
        assert_eq!(
            out, tt.graphic,
            "quote_to_graphic({}) = {}, want {}",
            tt.input, out, tt.graphic
        );
        let mut out = "abc".as_bytes().to_vec();
        super::append_quote_to_graphic(&mut out, tt.input);
        let mut expect = "abc".as_bytes().to_vec();
        expect.extend_from_slice(tt.graphic.as_bytes());
        assert_eq!(
            out, expect,
            "append_quote_to_graphic({:?}, {:?}) = {:?}, want {:?}",
            "abc", tt.input, out, expect
        );
    }
}

// fn BenchmarkQuote(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		quote("\x07\x08\x0C\r\n\t\v\x07\x08\x0C\r\n\t\v\x07\x08\x0C\r\n\t\v")
// 	}
// }

// fn BenchmarkQuoteRune(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		QuoteRune('\x07')
// 	}
// }

// var benchQuoteBuf []byte

// fn BenchmarkAppendQuote(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		benchQuoteBuf = append_quote(benchQuoteBuf[:0], "\x07\x08\x0C\r\n\t\v\x07\x08\x0C\r\n\t\v\x07\x08\x0C\r\n\t\v")
// 	}
// }

// var benchQuoteRuneBuf []byte

// fn BenchmarkAppendQuoteRune(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		benchQuoteRuneBuf = AppendQuoteRune(benchQuoteRuneBuf[:0], '\x07')
// 	}
// }

// type quoteRuneTest struct {
// 	in      rune
// 	out     string
// 	ascii   string
// 	graphic string
// }

// var quoterunetests = []quoteRuneTest{
// 	{'a', r#"'a'"#, r#"'a'"#, r#"'a'`},
// 	{'\x07', r#"'\x07'"#, r#"'\x07'"#, r#"'\x07'`},
// 	{'\\', r#"'\\'"#, r#"'\\'"#, r#"'\\'`},
// 	{0xFF, r#"'ÿ'"#, r#"'\u00ff'"#, r#"'ÿ'`},
// 	{0x263a, r#"'☺'"#, r#"'\u263a'"#, r#"'☺'`},
// 	{0xdead, r#"'�'"#, r#"'\ufffd'"#, r#"'�'`},
// 	{0xfffd, r#"'�'"#, r#"'\ufffd'"#, r#"'�'`},
// 	{0x0010ffff, r#"'\U0010ffff'"#, r#"'\U0010ffff'"#, r#"'\U0010ffff'`},
// 	{0x0010ffff + 1, r#"'�'"#, r#"'\ufffd'"#, r#"'�'`},
// 	{0x04, r#"'\x04'"#, r#"'\x04'"#, r#"'\x04'`},
// 	// Some differences between graphic and printable. Note the last column is double-quoted.
// 	{'\u00a0', r#"'\u00a0'"#, r#"'\u00a0'"#, "'\u00a0'"},
// 	{'\u2000', r#"'\u2000'"#, r#"'\u2000'"#, "'\u2000'"},
// 	{'\u3000', r#"'\u3000'"#, r#"'\u3000'"#, "'\u3000'"},
// }

// #[test]
// fn TestQuoteRune() {
// 	for _, tt := range quoterunetests {
// 		if out := QuoteRune(tt.input); out != tt.out {
// 			t.Errorf("QuoteRune(%U) = %s, want %s", tt.input, out, tt.out)
// 		}
// 		if out := AppendQuoteRune([]byte("abc"), tt.input); string(out) != "abc"+tt.out {
// 			t.Errorf("AppendQuoteRune(%q, %U) = %s, want %s", "abc", tt.input, out, "abc"+tt.out)
// 		}
// 	}
// }

// #[test]
// fn TestQuoteRuneToASCII() {
// 	for _, tt := range quoterunetests {
// 		if out := QuoteRuneToASCII(tt.input); out != tt.ascii {
// 			t.Errorf("QuoteRuneToASCII(%U) = %s, want %s", tt.input, out, tt.ascii)
// 		}
// 		if out := AppendQuoteRuneToASCII([]byte("abc"), tt.input); string(out) != "abc"+tt.ascii {
// 			t.Errorf("AppendQuoteRuneToASCII(%q, %U) = %s, want %s", "abc", tt.input, out, "abc"+tt.ascii)
// 		}
// 	}
// }

// #[test]
// fn TestQuoteRuneToGraphic() {
// 	for _, tt := range quoterunetests {
// 		if out := QuoteRuneToGraphic(tt.input); out != tt.graphic {
// 			t.Errorf("QuoteRuneToGraphic(%U) = %s, want %s", tt.input, out, tt.graphic)
// 		}
// 		if out := AppendQuoteRuneToGraphic([]byte("abc"), tt.input); string(out) != "abc"+tt.graphic {
// 			t.Errorf("AppendQuoteRuneToGraphic(%q, %U) = %s, want %s", "abc", tt.input, out, "abc"+tt.graphic)
// 		}
// 	}
// }

// type canBackquoteTest struct {
// 	in  string
// 	out bool
// }

// var canbackquotetests = []canBackquoteTest{
// 	{"`", false},
// 	{string(rune(0)), false},
// 	{string(rune(1)), false},
// 	{string(rune(2)), false},
// 	{string(rune(3)), false},
// 	{string(rune(4)), false},
// 	{string(rune(5)), false},
// 	{string(rune(6)), false},
// 	{string(rune(7)), false},
// 	{string(rune(8)), false},
// 	{string(rune(9)), true}, // \t
// 	{string(rune(10)), false},
// 	{string(rune(11)), false},
// 	{string(rune(12)), false},
// 	{string(rune(13)), false},
// 	{string(rune(14)), false},
// 	{string(rune(15)), false},
// 	{string(rune(16)), false},
// 	{string(rune(17)), false},
// 	{string(rune(18)), false},
// 	{string(rune(19)), false},
// 	{string(rune(20)), false},
// 	{string(rune(21)), false},
// 	{string(rune(22)), false},
// 	{string(rune(23)), false},
// 	{string(rune(24)), false},
// 	{string(rune(25)), false},
// 	{string(rune(26)), false},
// 	{string(rune(27)), false},
// 	{string(rune(28)), false},
// 	{string(rune(29)), false},
// 	{string(rune(30)), false},
// 	{string(rune(31)), false},
// 	{string(rune(0x7F)), false},
// 	{`' !"#$%&'()*+,-./:;<=>?@[\]^_{|}~"#, true},
// 	{`0123456789"#, true},
// 	{`ABCDEFGHIJKLMNOPQRSTUVWXYZ"#, true},
// 	{`abcdefghijklmnopqrstuvwxyz"#, true},
// 	{`☺"#, true},
// 	{"\x80", false},
// 	{"a\xe0\xa0z", false},
// 	{"\ufeffabc", false},
// 	{"a\ufeffz", false},
// }

// #[test]
// fn TestCanBackquote() {
// 	for _, tt := range canbackquotetests {
// 		if out := CanBackquote(tt.input); out != tt.out {
// 			t.Errorf("CanBackquote(%q) = %v, want %v", tt.input, out, tt.out)
// 		}
// 	}
// }

// type unQuoteTest struct {
// 	in  string
// 	out string
// }

// var unquotetests = []unQuoteTest{
// 	{`"""#, ""},
// 	{`"a""#, "a"},
// 	{`"abc""#, "abc"},
// 	{`"☺""#, "☺"},
// 	{`"hello world""#, "hello world"},
// 	{`"\xFF""#, "\xFF"},
// 	{`"\377""#, "\377"},
// 	{`"\u1234""#, "\u1234"},
// 	{`"\U00010111""#, "\U00010111"},
// 	{`"\U0001011111""#, "\U0001011111"},
// 	{`"\x07\x08\x0C\n\r\t\v\\\"""#, "\x07\x08\x0C\n\r\t\v\\\""},
// 	{`"'""#, "'"},

// 	{`'a'"#, "a"},
// 	{`'☹'"#, "☹"},
// 	{`'\x07'"#, "\x07"},
// 	{`'\x10'"#, "\x10"},
// 	{`'\377'"#, "\377"},
// 	{`'\u1234'"#, "\u1234"},
// 	{`'\U00010111'"#, "\U00010111"},
// 	{`'\t'"#, "\t"},
// 	{`' '"#, " "},
// 	{`'\''"#, "'"},
// 	{`'"'"#, "\""},

// 	{"``", r#"`},
// 	{"`a`", r#"a`},
// 	{"`abc`", r#"abc`},
// 	{"`☺`", r#"☺`},
// 	{"`hello world`", r#"hello world`},
// 	{"`\\xFF`", r#"\xFF`},
// 	{"`\\377`", r#"\377`},
// 	{"`\\`", r#"\`},
// 	{"`\n`", "\n"},
// 	{"`	`", r#"	`},
// 	{"` `", r#" `},
// 	{"`a\rb`", "ab"},
// }

// var misquoted = []string{
// 	`"#,
// 	`""#,
// 	`"a"#,
// 	`"'"#,
// 	`b""#,
// 	`"\""#,
// 	`"\9""#,
// 	`"\19""#,
// 	`"\129""#,
// 	`'\'"#,
// 	`'\9'"#,
// 	`'\19'"#,
// 	`'\129'"#,
// 	`'ab'"#,
// 	`"\x1!""#,
// 	`"\U12345678""#,
// 	`"\z""#,
// 	"`",
// 	"`xxx",
// 	"``x\r",
// 	"`\"",
// 	`"\'""#,
// 	`'\"'"#,
// 	"\"\n\"",
// 	"\"\\n\n\"",
// 	"'\n'",
// 	`"\udead""#,
// 	`"\ud83d\ude4f""#,
// }

// #[test]
// fn TestUnquote() {
// 	for _, tt := range unquotetests {
// 		testUnquote(t, tt.input, tt.out, nil)
// 	}
// 	for tt in QUOTE_TESTS {
// 		testUnquote(t, tt.out, tt.input, nil)
// 	}
// 	for _, s := range misquoted {
// 		testUnquote(t, s, "", ErrSyntax)
// 	}
// }

// // Issue 23685: invalid UTF-8 should not go through the fast path.
// #[test]
// fn TestUnquoteInvalidUTF8() {
// 	tests := []struct {
// 		in string

// 		// one of:
// 		want    string
// 		wantErr error
// 	}{
// 		{in: `"foo""#, want: "foo"},
// 		{in: `"foo"#, wantErr: ErrSyntax},
// 		{in: `"` + "\xc0" + `""#, want: "\xef\xbf\xbd"},
// 		{in: `"a` + "\xc0" + `""#, want: "a\xef\xbf\xbd"},
// 		{in: `"\t` + "\xc0" + `""#, want: "\t\xef\xbf\xbd"},
// 	}
// 	for _, tt := range tests {
// 		testUnquote(t, tt.input, tt.want, tt.wantErr)
// 	}
// }

// #[test]
// fn testUnquote(, in, want string, wantErr error) {
// 	// Test Unquote.
// 	got, gotErr := Unquote(in)
// 	if got != want || gotErr != wantErr {
// 		t.Errorf("Unquote(%q) = (%q, %v), want (%q, %v)", in, got, gotErr, want, wantErr)
// 	}

// 	// Test QuotedPrefix.
// 	// Adding an arbitrary suffix should not change the result of QuotedPrefix
// 	// assume that the suffix doesn't accidentally terminate a truncated input.
// 	if gotErr == nil {
// 		want = in
// 	}
// 	suffix := "\n\r\\\"`'" // special characters for quoted strings
// 	if len(in) > 0 {
// 		suffix = strings.ReplaceAll(suffix, in[:1], "")
// 	}
// 	in += suffix
// 	got, gotErr = QuotedPrefix(in)
// 	if gotErr == nil && wantErr != nil {
// 		_, wantErr = Unquote(got) // original input had trailing junk, reparse with only valid prefix
// 		want = got
// 	}
// 	if got != want || gotErr != wantErr {
// 		t.Errorf("QuotedPrefix(%q) = (%q, %v), want (%q, %v)", in, got, gotErr, want, wantErr)
// 	}
// }

// fn BenchmarkUnquoteEasy(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Unquote(`"Give me a rock, paper and scissors and I will move the world."`)
// 	}
// }

// fn BenchmarkUnquoteHard(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Unquote(`"\x47ive me a \x72ock, \x70aper and \x73cissors and \x49 will move the world."`)
// 	}
// }
