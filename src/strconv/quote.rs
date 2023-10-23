// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// //go:generate go run makeisprint.go -output isprint.go

use super::isprint::{IS_GRAPHIC, IS_NOT_PRINT16, IS_NOT_PRINT32, IS_PRINT16, IS_PRINT32};
use crate::unicode::utf8;

const LOWERHEX: &[u8; 16] = b"0123456789abcdef";
#[allow(unused)]
const UPPERHEX: &[u8; 16] = b"0123456789ABCDEF";

// // contains reports whether the string contains the byte c.
// fn contains(s string, c byte) bool {
// 	return index(s, c) != -1
// }

fn quote_with(s: &str, quote: u8, ascii_only: bool, graphic_only: bool) -> String {
    let mut buf = Vec::<u8>::with_capacity(3 * s.len() / 2);
    append_quoted_with(&mut buf, s.as_bytes(), quote, ascii_only, graphic_only);
    String::from_utf8_lossy(&buf).to_string()
}

// fn quoteRuneWith(r rune, quote char, ASCIIonly: bool, graphicOnly: bool) string {
// 	return string(appendQuotedRuneWith(nil, r, quote, ASCIIonly, graphicOnly))
// }

fn append_quoted_with(
    buf: &mut Vec<u8>,
    s: &[u8],
    quote: u8,
    ascii_only: bool,
    graphic_only: bool,
) {
    // Often called with big strings, so preallocate. If there's quoting,
    // this is conservative but still helps a lot.
    // 	if cap(buf)-len(buf) < s.len() {
    // 		nBuf := make([]byte, len(buf), len(buf)+1+s.len()+1)
    // 		copy(nBuf, buf)
    // 		buf = nBuf
    // 	}
    let mut s = s;
    buf.push(quote);
    while !s.is_empty() {
        let mut r = s[0] as char;
        let mut width = 1;
        if r >= utf8::RUNE_SELF {
            (r, width) = utf8::decode_rune(s);
        }
        if width == 1 && r == utf8::RUNE_ERROR {
            buf.extend_from_slice("\\x".as_bytes());
            buf.push(LOWERHEX[(s[0] >> 4) as usize]);
            buf.push(LOWERHEX[(s[0] & 0xF) as usize]);
        } else {
            append_escaped_rune(buf, r, quote, ascii_only, graphic_only);
        }
        s = &s[width..];
    }
    buf.push(quote);
}

// fn appendQuotedRuneWith(buf []byte, r rune, quote char, ASCIIonly: bool, graphicOnly: bool) []byte {
// 	buf = append(buf, quote)
// 	if !utf8::ValidRune(r) {
// 		r = utf8::RUNE_ERROR
// 	}
// 	buf = appendEscapedRune(buf, r, quote, ASCIIonly, graphicOnly)
// 	buf = append(buf, quote)
// 	return buf
// }

fn append_escaped_rune(
    buf: &mut Vec<u8>,
    r: char,
    quote: u8,
    ascii_only: bool,
    graphic_only: bool,
) {
    let mut rune_tmp = [0; utf8::UTFMAX];
    if r == quote as char || r == '\\' {
        // always backslashed
        buf.push(b'\\');
        buf.push(r as u8);
        return;
    }
    if ascii_only {
        if r < utf8::RUNE_SELF && is_print(r) {
            buf.push(r as u8);
            return;
        }
    } else if is_print(r) || graphic_only && is_in_graphic_list(r) {
        let n = utf8::encode_rune(&mut rune_tmp, r as u32);
        buf.extend_from_slice(&rune_tmp[..n]);
        return;
    }
    let mut r = r;
    match r {
        '\x07' => buf.extend_from_slice("\\a".as_bytes()),
        '\x08' => buf.extend_from_slice("\\b".as_bytes()),
        '\x0c' => buf.extend_from_slice("\\f".as_bytes()),
        '\n' => buf.extend_from_slice("\\n".as_bytes()),
        '\r' => buf.extend_from_slice("\\r".as_bytes()),
        '\t' => buf.extend_from_slice("\\t".as_bytes()),
        '\x0b' => buf.extend_from_slice("\\v".as_bytes()),
        _ => {
            if r < ' ' || r == 0x7f as char {
                buf.extend_from_slice("\\x".as_bytes());
                buf.push(LOWERHEX[(r as u8 >> 4) as usize]);
                buf.push(LOWERHEX[(r as u8 & 0xF) as usize]);
            } else {
                if !utf8::valid_rune(r as u32) {
                    r = '\u{FFFD}';
                }
                if (r as u32) < 0x10000 {
                    buf.extend_from_slice("\\u".as_bytes());
                    let mut s = 12;
                    while s >= 0 {
                        buf.push(LOWERHEX[(((r as u32) >> s) & 0xF) as usize]);
                        s -= 4;
                    }
                } else {
                    buf.extend_from_slice("\\U".as_bytes());
                    let mut s = 28;
                    while s >= 0 {
                        buf.push(LOWERHEX[(((r as u32) >> s) & 0xF) as usize]);
                        s -= 4;
                    }
                }
            }
        }
    }
}

/// quote returns a double-quoted Go string literal representing s. The
/// returned string uses Go escape sequences (\t, \n, \xFF, \u0100) for
/// control characters and non-printable characters as defined by
/// is_print.
pub fn quote(s: &str) -> String {
    quote_with(s, b'"', false, false)
}

/// quote_bytes_string is similar to Quote, but takes slice of bytes
/// instead of a string.  This is useful in cases when replacing Go Quote
/// function when the Go string contains arbitrary data (in Go strings
/// can contain arbitrary data, in Rust they cannot).
pub fn quote_bytes_string(s: &[u8]) -> String {
    let mut buf = Vec::<u8>::with_capacity(3 * s.len() / 2);
    append_quoted_with(&mut buf, s, b'"', false, false);
    String::from_utf8_lossy(&buf).to_string()
}

/// append_quote appends a double-quoted Go string literal representing s,
/// as generated by Quote, to dst and returns the extended buffer.
pub fn append_quote(dst: &mut Vec<u8>, s: &str) {
    append_quoted_with(dst, s.as_bytes(), b'"', false, false);
}

/// quote_to_ascii returns a double-quoted Go string literal representing s.
/// The returned string uses Go escape sequences (\t, \n, \xFF, \u0100) for
/// non-ASCII characters and non-printable characters as defined by is_print.
pub fn quote_to_ascii(s: &str) -> String {
    quote_with(s, b'"', true, false)
}

/// append_quote_to_ascii appends a double-quoted Go string literal representing s,
/// as generated by quote_to_ascii, to dst and returns the extended buffer.
pub fn append_quote_to_ascii(dst: &mut Vec<u8>, s: &str) {
    append_quoted_with(dst, s.as_bytes(), b'"', true, false)
}

#[allow(clippy::tabs_in_doc_comments)]
/// quote_to_graphic returns a double-quoted Go string literal representing s.
/// The returned string leaves Unicode graphic characters, as defined by
/// is_graphic, unchanged and uses Go escape sequences (\t, \n, \xFF, \u0100)
/// for non-graphic characters.
///
/// ```
/// use ggstd::strconv::quote_to_graphic;
///
/// let s = quote_to_graphic("☺");
/// assert_eq!(s, "\"☺\"");
///
/// // This string literal contains a tab character.
/// let s = quote_to_graphic("This is a \u{263a}	\u{000a}");
/// assert_eq!(s, r#""This is a ☺\t\n""#);
///
/// let s = quote_to_graphic(r#"" This is a ☺ \n ""#);
/// assert_eq!(s, r#""\" This is a ☺ \\n \"""#);
/// ```
pub fn quote_to_graphic(s: &str) -> String {
    quote_with(s, b'"', false, true)
}

/// append_quote_to_graphic appends a double-quoted Go string literal representing s,
/// as generated by quote_to_graphic, to dst and returns the extended buffer.
pub fn append_quote_to_graphic(dst: &mut Vec<u8>, s: &str) {
    append_quoted_with(dst, s.as_bytes(), b'"', false, true)
}

// // QuoteRune returns a single-quoted Go character literal representing the
// // rune. The returned string uses Go escape sequences (\t, \n, \xFF, \u0100)
// // for control characters and non-printable characters as defined by is_print.
// // If r is not a valid Unicode code point, it is interpreted as the Unicode
// // replacement character U+FFFD.
// fn QuoteRune(r rune) string {
// 	return quoteRuneWith(r, '\'', false, false)
// }

// // AppendQuoteRune appends a single-quoted Go character literal representing the rune,
// // as generated by QuoteRune, to dst and returns the extended buffer.
// fn AppendQuoteRune(dst: &mut Vec<u8>, r rune) []byte {
// 	return appendQuotedRuneWith(dst, r, '\'', false, false)
// }

// // QuoteRuneToASCII returns a single-quoted Go character literal representing
// // the rune. The returned string uses Go escape sequences (\t, \n, \xFF,
// // \u0100) for non-ASCII characters and non-printable characters as defined
// // by is_print.
// // If r is not a valid Unicode code point, it is interpreted as the Unicode
// // replacement character U+FFFD.
// fn QuoteRuneToASCII(r rune) string {
// 	return quoteRuneWith(r, '\'', true, false)
// }

// // AppendQuoteRuneToASCII appends a single-quoted Go character literal representing the rune,
// // as generated by QuoteRuneToASCII, to dst and returns the extended buffer.
// fn AppendQuoteRuneToASCII(dst: &mut Vec<u8>, r rune) []byte {
// 	return appendQuotedRuneWith(dst, r, '\'', true, false)
// }

// // QuoteRuneToGraphic returns a single-quoted Go character literal representing
// // the rune. If the rune is not a Unicode graphic character,
// // as defined by is_graphic, the returned string will use a Go escape sequence
// // (\t, \n, \xFF, \u0100).
// // If r is not a valid Unicode code point, it is interpreted as the Unicode
// // replacement character U+FFFD.
// fn QuoteRuneToGraphic(r rune) string {
// 	return quoteRuneWith(r, '\'', false, true)
// }

// // AppendQuoteRuneToGraphic appends a single-quoted Go character literal representing the rune,
// // as generated by QuoteRuneToGraphic, to dst and returns the extended buffer.
// fn AppendQuoteRuneToGraphic(dst: &mut Vec<u8>, r rune) []byte {
// 	return appendQuotedRuneWith(dst, r, '\'', false, true)
// }

// // CanBackquote reports whether the string s can be represented
// // unchanged as a single-line backquoted string without control
// // characters other than tab.
// fn CanBackquote(s string) bool {
// 	for s.len() > 0 {
// 		r, wid := utf8::decode_rune_in_string(s)
// 		s = s[wid..]
// 		if wid > 1 {
// 			if r == '\ufeff' {
// 				return false // BOMs are invisible and should not be quoted.
// 			}
// 			continue // All other multibyte runes are correctly encoded and assumed printable.
// 		}
// 		if r == utf8::RUNE_ERROR {
// 			return false
// 		}
// 		if (r < ' ' && r != '\t') || r == '`' || r == '\u007F' {
// 			return false
// 		}
// 	}
// 	return true
// }

// fn unhex(b byte) (v rune, ok bool) {
// 	c := rune(b)
// 	switch {
// 	case '0' <= c && c <= '9':
// 		return c - '0', true
// 	case 'a' <= c && c <= 'f':
// 		return c - 'a' + 10, true
// 	case 'A' <= c && c <= 'F':
// 		return c - 'A' + 10, true
// 	}
// 	return
// }

// // UnquoteChar decodes the first character or byte in the escaped string
// // or character literal represented by the string s.
// // It returns four values:
// //
// //  1. value, the decoded Unicode code point or byte value;
// //  2. multibyte, a boolean indicating whether the decoded character requires a multibyte UTF-8 representation;
// //  3. tail, the remainder of the string after the character; and
// //  4. an error that will be nil if the character is syntactically valid.
// //
// // The second argument, quote, specifies the type of literal being parsed
// // and therefore which escaped quote character is permitted.
// // If set to a single quote, it permits the sequence \' and disallows unescaped '.
// // If set to a double quote, it permits \" and disallows unescaped ".
// // If set to zero, it does not permit either escape and allows both quote characters to appear unescaped.
// fn UnquoteChar(s string, quote char) (value rune, multibyte bool, tail string, err error) {
// 	// easy cases
// 	if s.len() == 0 {
// 		err = ErrSyntax
// 		return
// 	}
// 	switch c := s[0]; {
// 	case c == quote && (quote == '\'' || quote == '"'):
// 		err = ErrSyntax
// 		return
// 	case c >= utf8::RUNE_SELF:
// 		r, size := utf8::decode_rune_in_string(s)
// 		return r, true, s[size..], nil
// 	case c != '\\':
// 		return rune(s[0]), false, s[1..], nil
// 	}

// 	// hard case: c is backslash
// 	if s.len() <= 1 {
// 		err = ErrSyntax
// 		return
// 	}
// 	c := s[1]
// 	s = s[2..]

// 	switch c {
// 	case 'a':
// 		value = '\a'
// 	case 'b':
// 		value = '\b'
// 	case 'f':
// 		value = '\f'
// 	case 'n':
// 		value = '\n'
// 	case 'r':
// 		value = '\r'
// 	case 't':
// 		value = '\t'
// 	case 'v':
// 		value = '\v'
// 	case 'x', 'u', 'U':
// 		n := 0
// 		switch c {
// 		case 'x':
// 			n = 2
// 		case 'u':
// 			n = 4
// 		case 'U':
// 			n = 8
// 		}
// 		var v rune
// 		if s.len() < n {
// 			err = ErrSyntax
// 			return
// 		}
// 		for j := 0; j < n; j++ {
// 			x, ok := unhex(s[j])
// 			if !ok {
// 				err = ErrSyntax
// 				return
// 			}
// 			v = v<<4 | x
// 		}
// 		s = s[n..]
// 		if c == 'x' {
// 			// single-byte string, possibly not UTF-8
// 			value = v
// 			break
// 		}
// 		if !utf8::ValidRune(v) {
// 			err = ErrSyntax
// 			return
// 		}
// 		value = v
// 		multibyte = true
// 	case '0', '1', '2', '3', '4', '5', '6', '7':
// 		v := rune(c) - '0'
// 		if s.len() < 2 {
// 			err = ErrSyntax
// 			return
// 		}
// 		for j := 0; j < 2; j++ { // one digit already; two more
// 			x := rune(s[j]) - '0'
// 			if x < 0 || x > 7 {
// 				err = ErrSyntax
// 				return
// 			}
// 			v = (v << 3) | x
// 		}
// 		s = s[2..]
// 		if v > 255 {
// 			err = ErrSyntax
// 			return
// 		}
// 		value = v
// 	case '\\':
// 		value = '\\'
// 	case '\'', '"':
// 		if c != quote {
// 			err = ErrSyntax
// 			return
// 		}
// 		value = rune(c)
// 	default:
// 		err = ErrSyntax
// 		return
// 	}
// 	tail = s
// 	return
// }

// // QuotedPrefix returns the quoted string (as understood by Unquote) at the prefix of s.
// // If s does not start with a valid quoted string, QuotedPrefix returns an error.
// fn QuotedPrefix(s string) (string, error) {
// 	out, _, err := unquote(s, false)
// 	return out, err
// }

// // Unquote interprets s as a single-quoted, double-quoted,
// // or backquoted Go string literal, returning the string value
// // that s quotes.  (If s is single-quoted, it would be a Go
// // character literal; Unquote returns the corresponding
// // one-character string.)
// fn Unquote(s string) (string, error) {
// 	out, rem, err := unquote(s, true)
// 	if len(rem) > 0 {
// 		return "", ErrSyntax
// 	}
// 	return out, err
// }

// // unquote parses a quoted string at the start of the input,
// // returning the parsed prefix, the remaining suffix, and any parse errors.
// // If unescape is true, the parsed prefix is unescaped,
// // otherwise the input prefix is provided verbatim.
// fn unquote(in string, unescape bool) (out, rem string, err error) {
// 	// Determine the quote form and optimistically find the terminating quote.
// 	if len(in) < 2 {
// 		return "", in, ErrSyntax
// 	}
// 	quote := in[0]
// 	end := index(in[1..], quote)
// 	if end < 0 {
// 		return "", in, ErrSyntax
// 	}
// 	end += 2 // position after terminating quote; may be wrong if escape sequences are present

// 	switch quote {
// 	case '`':
// 		switch {
// 		case !unescape:
// 			out = in[:end] // include quotes
// 		case !contains(in[:end], '\r'):
// 			out = in[len("`") : end-len("`")] // exclude quotes
// 		default:
// 			// Carriage return characters ('\r') inside raw string literals
// 			// are discarded from the raw string value.
// 			buf := make([]byte, 0, end-len("`")-len("\r")-len("`"))
// 			for i := len("`"); i < end-len("`"); i++ {
// 				if in[i] != '\r' {
// 					buf = append(buf, in[i])
// 				}
// 			}
// 			out = string(buf)
// 		}
// 		// NOTE: Prior implementations did not verify that raw strings consist
// 		// of valid UTF-8 characters and we continue to not verify it as such.
// 		// The Go specification does not explicitly require valid UTF-8,
// 		// but only mention that it is implicitly valid for Go source code
// 		// (which must be valid UTF-8).
// 		return out, in[end..], nil
// 	case '"', '\'':
// 		// Handle quoted strings without any escape sequences.
// 		if !contains(in[:end], '\\') && !contains(in[:end], '\n') {
// 			var valid bool
// 			switch quote {
// 			case '"':
// 				valid = utf8::ValidString(in[len(`"`) : end-len(`"`)])
// 			case '\'':
// 				r, n := utf8::decode_rune_in_string(in[len("'") : end-len("'")])
// 				valid = len("'")+n+len("'") == end && (r != utf8::RUNE_ERROR || n != 1)
// 			}
// 			if valid {
// 				out = in[:end]
// 				if unescape {
// 					out = out[1 : end-1] // exclude quotes
// 				}
// 				return out, in[end..], nil
// 			}
// 		}

// 		// Handle quoted strings with escape sequences.
// 		var buf []byte
// 		in0 := in
// 		in = in[1..] // skip starting quote
// 		if unescape {
// 			buf = make([]byte, 0, 3*end/2) // try to avoid more allocations
// 		}
// 		for len(in) > 0 && in[0] != quote {
// 			// Process the next character,
// 			// rejecting any unescaped newline characters which are invalid.
// 			r, multibyte, rem, err := UnquoteChar(in, quote)
// 			if in[0] == '\n' || err != nil {
// 				return "", in0, ErrSyntax
// 			}
// 			in = rem

// 			// Append the character if unescaping the input.
// 			if unescape {
// 				if r < utf8::RUNE_SELF || !multibyte {
// 					buf = append(buf, byte(r))
// 				} else {
// 					var arr [utf8::UTFMAX]byte
// 					n := utf8::encode_rune(arr[..], r)
// 					buf = append(buf, arr[:n]...)
// 				}
// 			}

// 			// Single quoted strings must be a single character.
// 			if quote == '\'' {
// 				break
// 			}
// 		}

// 		// Verify that the string ends with a terminating quote.
// 		if !(len(in) > 0 && in[0] == quote) {
// 			return "", in0, ErrSyntax
// 		}
// 		in = in[1..] // skip terminating quote

// 		if unescape {
// 			return string(buf), in, nil
// 		}
// 		return in0[:len(in0)-len(in)], in, nil
// 	default:
// 		return "", in, ErrSyntax
// 	}
// }

/// bsearch16 returns the smallest i such that a[i] >= x.
/// If there is no such i, bsearch16 returns a.len().
fn bsearch16(a: &[u16], x: u16) -> usize {
    let (mut i, mut j) = (0, a.len());
    while i < j {
        let h = i + ((j - i) >> 1);
        if a[h] < x {
            i = h + 1;
        } else {
            j = h;
        }
    }
    i
}

/// bsearch32 returns the smallest i such that a[i] >= x.
/// If there is no such i, bsearch32 returns a.len().
fn bsearch32(a: &[u32], x: u32) -> usize {
    let (mut i, mut j) = (0, a.len());
    while i < j {
        let h = i + ((j - i) >> 1);
        if a[h] < x {
            i = h + 1;
        } else {
            j = h;
        }
    }
    i
}

// // TODO: is_print is a local implementation of unicode.is_print, verified by the tests
// // to give the same answer. It allows this package not to depend on unicode,
// // and therefore not pull in all the Unicode tables. If the linker were better
// // at tossing unused tables, we could get rid of this implementation.
// // That would be nice.

/// is_print reports whether the rune is defined as printable by Go, with
/// the same definition as unicode.is_print: letters, numbers, punctuation,
/// symbols and ASCII space.
pub fn is_print(r: char) -> bool {
    let r = r as u32;

    // Fast check for Latin-1
    if r <= 0xFF {
        if (0x20..=0x7E).contains(&r) {
            // All the ASCII is printable from space through DEL-1.
            return true;
        }
        if (0xA1..=0xFF).contains(&r) {
            // Similarly for ¡ through ÿ...
            return r != 0xAD; // ...except for the bizarre soft hyphen.
        }
        return false;
    }

    // Same algorithm, either on u16 or uint32 value.
    // First, find first i such that isPrint[i] >= x.
    // This is the index of either the start or end of a pair that might span x.
    // The start is even (isPrint[i&^1]) and the end is odd (isPrint[i|1]).
    // If we find x in a range, make sure x is not in isNotPrint list.

    if r < (1 << 16) {
        let (rr, is_print, is_not_print) = (r as u16, IS_PRINT16, IS_NOT_PRINT16);
        let i = bsearch16(&is_print, rr);
        if i >= is_print.len() || rr < is_print[i & !1] || is_print[i | 1] < rr {
            return false;
        }
        let j = bsearch16(&is_not_print, rr);
        return j >= is_not_print.len() || is_not_print[j] != rr;
    }

    let (rr, is_print, is_not_print) = (r, IS_PRINT32, IS_NOT_PRINT32);
    let i = bsearch32(&is_print, rr);
    if i >= is_print.len() || rr < is_print[i & !1] || is_print[i | 1] < rr {
        return false;
    }
    if r >= 0x20000 {
        return true;
    }
    let r = (r - 0x10000) as u16;
    let j = bsearch16(&is_not_print, r);
    j >= is_not_print.len() || is_not_print[j] != r
}

/// is_graphic reports whether the rune is defined as a Graphic by Unicode. Such
/// characters include letters, marks, numbers, punctuation, symbols, and
/// spaces, from categories L, M, N, P, S, and Zs.
pub fn is_graphic(r: char) -> bool {
    if is_print(r) {
        return true;
    }
    is_in_graphic_list(r)
}

/// is_in_graphic_list reports whether the rune is in the IS_GRAPHIC list. This separation
/// from is_graphic allows quote_with to avoid two calls to is_print.
/// Should be called only if is_print fails.
fn is_in_graphic_list(r: char) -> bool {
    // We know r must fit in 16 bits - see makeisprint.go.
    if r as u32 > 0xFFFF {
        return false;
    }
    let rr = r as u16;
    let i = bsearch16(&IS_GRAPHIC, rr);
    i < IS_GRAPHIC.len() && rr == IS_GRAPHIC[i]
}
