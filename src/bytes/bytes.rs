// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package bytes implements functions for the manipulation of byte slices.
//! It is analogous to the facilities of the strings package.

// import (
// 	"internal/bytealg"
// 	"unicode"
// 	"unicode/utf8"
// )

/// equal reports whether a and b
/// are the same length and contain the same bytes.
// A nil argument is equivalent to an empty slice.
pub fn equal(a: &[u8], b: &[u8]) -> bool {
    a == b
}

// // Compare returns an integer comparing two byte slices lexicographically.
// // The result will be 0 if a == b, -1 if a < b, and +1 if a > b.
// // A nil argument is equivalent to an empty slice.
// fn Compare(a, b: &[u8]) int {
// 	return bytealg.Compare(a, b)
// }

// // explode splits s into a slice of UTF-8 sequences, one per Unicode code point (still slices of bytes),
// // up to a maximum of n byte slices. Invalid UTF-8 sequences are chopped into individual bytes.
// fn explode(s [u8], n int) [][u8] {
// 	if n <= 0 || n > s.len() {
// 		n = s.len()
// 	}
// 	a := make([][u8], n)
// 	var size int
// 	na := 0
// 	for s.len() > 0 {
// 		if na+1 >= n {
// 			a[na] = s
// 			na++
// 			break
// 		}
// 		_, size = utf8.decode_rune(s)
// 		a[na] = s[0:size:size]
// 		s = s[size..]
// 		na++
// 	}
// 	return a[0:na]
// }

// // Count counts the number of non-overlapping instances of sep in s.
// // If sep is an empty slice, Count returns 1 + the number of UTF-8-encoded code points in s.
// fn Count(s, sep [u8]) int {
// 	// special case
// 	if len(sep) == 0 {
// 		return utf8.rune_count(s) + 1
// 	}
// 	if len(sep) == 1 {
// 		return bytealg.Count(s, sep[0])
// 	}
// 	n := 0
// 	for {
// 		i := Index(s, sep)
// 		if i == -1 {
// 			return n
// 		}
// 		n++
// 		s = s[i+len(sep)..]
// 	}
// }

// // Contains reports whether subslice is within b.
// fn Contains(b, subslice [u8]) -> bool {
// 	return Index(b, subslice) != -1
// }

// // ContainsAny reports whether any of the UTF-8-encoded code points in chars are within b.
// fn ContainsAny(b: &[u8], chars string) -> bool {
// 	return IndexAny(b, chars) >= 0
// }

// // ContainsRune reports whether the rune is contained in the UTF-8-encoded byte slice b.
// fn ContainsRune(b: &[u8], r rune) -> bool {
// 	return IndexRune(b, r) >= 0
// }

// // IndexByte returns the index of the first instance of c in b, or -1 if c is not present in b.
// fn IndexByte(b: &[u8], c byte) int {
// 	return bytealg.IndexByte(b, c)
// }

// fn indexBytePortable(s [u8], c byte) int {
// 	for i, b := range s {
// 		if b == c {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // LastIndex returns the index of the last instance of sep in s, or -1 if sep is not present in s.
// fn LastIndex(s, sep [u8]) int {
// 	n := len(sep)
// 	switch {
// 	case n == 0:
// 		return s.len()
// 	case n == 1:
// 		return LastIndexByte(s, sep[0])
// 	case n == s.len():
// 		if equal(s, sep) {
// 			return 0
// 		}
// 		return -1
// 	case n > s.len():
// 		return -1
// 	}
// 	// Rabin-Karp search from the end of the string
// 	hashss, pow := bytealg.HashStrRevBytes(sep)
// 	last := s.len() - n
// 	var h uint32
// 	for i := s.len() - 1; i >= last; i-- {
// 		h = h*bytealg.PrimeRK + uint32(s[i])
// 	}
// 	if h == hashss && equal(s[last..], sep) {
// 		return last
// 	}
// 	for i := last - 1; i >= 0; i-- {
// 		h *= bytealg.PrimeRK
// 		h += uint32(s[i])
// 		h -= pow * uint32(s[i+n])
// 		if h == hashss && equal(s[i:i+n], sep) {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present in s.
// fn LastIndexByte(s [u8], c byte) int {
// 	for i := s.len() - 1; i >= 0; i-- {
// 		if s[i] == c {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // IndexRune interprets s as a sequence of UTF-8-encoded code points.
// // It returns the byte index of the first occurrence in s of the given rune.
// // It returns -1 if rune is not present in s.
// // If r is utf8.RUNE_ERROR, it returns the first instance of any
// // invalid UTF-8 byte sequence.
// fn IndexRune(s [u8], r rune) int {
// 	switch {
// 	case 0 <= r && r < utf8::RUNE_SELF:
// 		return IndexByte(s, byte(r))
// 	case r == utf8.RUNE_ERROR:
// 		for i := 0; i < s.len(); {
// 			r1, n := utf8.decode_rune(s[i..])
// 			if r1 == utf8.RUNE_ERROR {
// 				return i
// 			}
// 			i += n
// 		}
// 		return -1
// 	case !utf8.ValidRune(r):
// 		return -1
// 	default:
// 		var b [utf8.UTFMAX]byte
// 		n := utf8.encode_rune(b[..], r)
// 		return Index(s, b[..n])
// 	}
// }

// // IndexAny interprets s as a sequence of UTF-8-encoded Unicode code points.
// // It returns the byte index of the first occurrence in s of any of the Unicode
// // code points in chars. It returns -1 if chars is empty or if there is no code
// // point in common.
// fn IndexAny(s [u8], chars string) int {
// 	if chars == "" {
// 		// Avoid scanning all of s.
// 		return -1
// 	}
// 	if s.len() == 1 {
// 		r := rune(s[0])
// 		if r >= utf8::RUNE_SELF {
// 			// search utf8.RUNE_ERROR.
// 			for _, r = range chars {
// 				if r == utf8.RUNE_ERROR {
// 					return 0
// 				}
// 			}
// 			return -1
// 		}
// 		if bytealg.IndexByteString(chars, s[0]) >= 0 {
// 			return 0
// 		}
// 		return -1
// 	}
// 	if len(chars) == 1 {
// 		r := rune(chars[0])
// 		if r >= utf8::RUNE_SELF {
// 			r = utf8.RUNE_ERROR
// 		}
// 		return IndexRune(s, r)
// 	}
// 	if s.len() > 8 {
// 		if as, isASCII := makeASCIISet(chars); isASCII {
// 			for i, c := range s {
// 				if as.contains(c) {
// 					return i
// 				}
// 			}
// 			return -1
// 		}
// 	}
// 	var width int
// 	for i := 0; i < s.len(); i += width {
// 		r := rune(s[i])
// 		if r < utf8::RUNE_SELF {
// 			if bytealg.IndexByteString(chars, s[i]) >= 0 {
// 				return i
// 			}
// 			width = 1
// 			continue
// 		}
// 		r, width = utf8.decode_rune(s[i..])
// 		if r != utf8.RUNE_ERROR {
// 			// r is 2 to 4 bytes
// 			if len(chars) == width {
// 				if chars == string(r) {
// 					return i
// 				}
// 				continue
// 			}
// 			// Use bytealg.IndexString for performance if available.
// 			if bytealg.MaxLen >= width {
// 				if bytealg.IndexString(chars, string(r)) >= 0 {
// 					return i
// 				}
// 				continue
// 			}
// 		}
// 		for _, ch := range chars {
// 			if r == ch {
// 				return i
// 			}
// 		}
// 	}
// 	return -1
// }

// // LastIndexAny interprets s as a sequence of UTF-8-encoded Unicode code
// // points. It returns the byte index of the last occurrence in s of any of
// // the Unicode code points in chars. It returns -1 if chars is empty or if
// // there is no code point in common.
// fn LastIndexAny(s [u8], chars string) int {
// 	if chars == "" {
// 		// Avoid scanning all of s.
// 		return -1
// 	}
// 	if s.len() > 8 {
// 		if as, isASCII := makeASCIISet(chars); isASCII {
// 			for i := s.len() - 1; i >= 0; i-- {
// 				if as.contains(s[i]) {
// 					return i
// 				}
// 			}
// 			return -1
// 		}
// 	}
// 	if s.len() == 1 {
// 		r := rune(s[0])
// 		if r >= utf8::RUNE_SELF {
// 			for _, r = range chars {
// 				if r == utf8.RUNE_ERROR {
// 					return 0
// 				}
// 			}
// 			return -1
// 		}
// 		if bytealg.IndexByteString(chars, s[0]) >= 0 {
// 			return 0
// 		}
// 		return -1
// 	}
// 	if len(chars) == 1 {
// 		cr := rune(chars[0])
// 		if cr >= utf8::RUNE_SELF {
// 			cr = utf8.RUNE_ERROR
// 		}
// 		for i := s.len(); i > 0; {
// 			r, size := utf8.DecodeLastRune(s[..i])
// 			i -= size
// 			if r == cr {
// 				return i
// 			}
// 		}
// 		return -1
// 	}
// 	for i := s.len(); i > 0; {
// 		r := rune(s[i-1])
// 		if r < utf8::RUNE_SELF {
// 			if bytealg.IndexByteString(chars, s[i-1]) >= 0 {
// 				return i - 1
// 			}
// 			i--
// 			continue
// 		}
// 		r, size := utf8.DecodeLastRune(s[..i])
// 		i -= size
// 		if r != utf8.RUNE_ERROR {
// 			// r is 2 to 4 bytes
// 			if len(chars) == size {
// 				if chars == string(r) {
// 					return i
// 				}
// 				continue
// 			}
// 			// Use bytealg.IndexString for performance if available.
// 			if bytealg.MaxLen >= size {
// 				if bytealg.IndexString(chars, string(r)) >= 0 {
// 					return i
// 				}
// 				continue
// 			}
// 		}
// 		for _, ch := range chars {
// 			if r == ch {
// 				return i
// 			}
// 		}
// 	}
// 	return -1
// }

// // Generic split: splits after each instance of sep,
// // including sepSave bytes of sep in the subslices.
// fn genSplit(s, sep [u8], sepSave, n int) [][u8] {
// 	if n == 0 {
// 		return nil
// 	}
// 	if len(sep) == 0 {
// 		return explode(s, n)
// 	}
// 	if n < 0 {
// 		n = Count(s, sep) + 1
// 	}
// 	if n > s.len()+1 {
// 		n = s.len() + 1
// 	}

// 	a := make([][u8], n)
// 	n--
// 	i := 0
// 	for i < n {
// 		m := Index(s, sep)
// 		if m < 0 {
// 			break
// 		}
// 		a[i] = s[.. m+sepSave : m+sepSave]
// 		s = s[m+len(sep)..]
// 		i += 1
// 	}
// 	a[i] = s
// 	return a[..i+1]
// }

// // SplitN slices s into subslices separated by sep and returns a slice of
// // the subslices between those separators.
// // If sep is empty, SplitN splits after each UTF-8 sequence.
// // The count determines the number of subslices to return:
// //
// //	n > 0: at most n subslices; the last subslice will be the unsplit remainder.
// //	n == 0: the result is nil (zero subslices)
// //	n < 0: all subslices
// //
// // To split around the first instance of a separator, see Cut.
// fn SplitN(s, sep [u8], n int) [][u8] { return genSplit(s, sep, 0, n) }

// // SplitAfterN slices s into subslices after each instance of sep and
// // returns a slice of those subslices.
// // If sep is empty, SplitAfterN splits after each UTF-8 sequence.
// // The count determines the number of subslices to return:
// //
// //	n > 0: at most n subslices; the last subslice will be the unsplit remainder.
// //	n == 0: the result is nil (zero subslices)
// //	n < 0: all subslices
// fn SplitAfterN(s, sep [u8], n int) [][u8] {
// 	return genSplit(s, sep, len(sep), n)
// }

// // Split slices s into all subslices separated by sep and returns a slice of
// // the subslices between those separators.
// // If sep is empty, Split splits after each UTF-8 sequence.
// // It is equivalent to SplitN with a count of -1.
// //
// // To split around the first instance of a separator, see Cut.
// fn Split(s, sep [u8]) [][u8] { return genSplit(s, sep, 0, -1) }

// // SplitAfter slices s into all subslices after each instance of sep and
// // returns a slice of those subslices.
// // If sep is empty, SplitAfter splits after each UTF-8 sequence.
// // It is equivalent to SplitAfterN with a count of -1.
// fn SplitAfter(s, sep [u8]) [][u8] {
// 	return genSplit(s, sep, len(sep), -1)
// }

// var asciiSpace = [256]uint8{'\t': 1, '\n': 1, '\v': 1, '\f': 1, '\r': 1, ' ': 1}

// // Fields interprets s as a sequence of UTF-8-encoded code points.
// // It splits the slice s around each instance of one or more consecutive white space
// // characters, as defined by unicode.IsSpace, returning a slice of subslices of s or an
// // empty slice if s contains only white space.
// fn Fields(s [u8]) [][u8] {
// 	// First count the fields.
// 	// This is an exact count if s is ASCII, otherwise it is an approximation.
// 	n := 0
// 	wasSpace := 1
// 	// setBits is used to track which bits are set in the bytes of s.
// 	setBits := uint8(0)
// 	for i := 0; i < s.len(); i += 1 {
// 		r := s[i]
// 		setBits |= r
// 		isSpace := int(asciiSpace[r])
// 		n += wasSpace & ^isSpace
// 		wasSpace = isSpace
// 	}

// 	if setBits >= utf8::RUNE_SELF {
// 		// Some runes in the input slice are not ASCII.
// 		return FieldsFunc(s, unicode.IsSpace)
// 	}

// 	// ASCII fast path
// 	a := make([][u8], n)
// 	na := 0
// 	fieldStart := 0
// 	i := 0
// 	// Skip spaces in the front of the input.
// 	for i < s.len() && asciiSpace[s[i]] != 0 {
// 		i += 1
// 	}
// 	fieldStart = i
// 	for i < s.len() {
// 		if asciiSpace[s[i]] == 0 {
// 			i += 1
// 			continue
// 		}
// 		a[na] = s[fieldStart:i:i]
// 		na++
// 		i += 1
// 		// Skip spaces in between fields.
// 		for i < s.len() && asciiSpace[s[i]] != 0 {
// 			i += 1
// 		}
// 		fieldStart = i
// 	}
// 	if fieldStart < s.len() { // Last field might end at EOF.
// 		a[na] = s[fieldStart:s.len():s.len()]
// 	}
// 	return a
// }

// // FieldsFunc interprets s as a sequence of UTF-8-encoded code points.
// // It splits the slice s at each run of code points c satisfying f(c) and
// // returns a slice of subslices of s. If all code points in s satisfy f(c), or
// // s.len() == 0, an empty slice is returned.
// //
// // FieldsFunc makes no guarantees about the order in which it calls f(c)
// // and assumes that f always returns the same value for a given c.
// fn FieldsFunc(s [u8], f fn(rune) bool) [][u8] {
// 	// A span is used to record a slice of s of the form s[start:end].
// 	// The start index is inclusive and the end index is exclusive.
// 	type span struct {
// 		start int
// 		end   int
// 	}
// 	spans := make([]span, 0, 32)

// 	// Find the field start and end indices.
// 	// Doing this in a separate pass (rather than slicing the string s
// 	// and collecting the result substrings right away) is significantly
// 	// more efficient, possibly due to cache effects.
// 	start := -1 // valid span start if >= 0
// 	for i := 0; i < s.len(); {
// 		size := 1
// 		r := rune(s[i])
// 		if r >= utf8::RUNE_SELF {
// 			r, size = utf8.decode_rune(s[i..])
// 		}
// 		if f(r) {
// 			if start >= 0 {
// 				spans = append(spans, span{start, i})
// 				start = -1
// 			}
// 		} else {
// 			if start < 0 {
// 				start = i
// 			}
// 		}
// 		i += size
// 	}

// 	// Last field might end at EOF.
// 	if start >= 0 {
// 		spans = append(spans, span{start, s.len()})
// 	}

// 	// Create subslices from recorded field indices.
// 	a := make([][u8], len(spans))
// 	for i, span := range spans {
// 		a[i] = s[span.start:span.end:span.end]
// 	}

// 	return a
// }

// // Join concatenates the elements of s to create a new byte slice. The separator
// // sep is placed between elements in the resulting slice.
// fn Join(s [][u8], sep [u8]) [u8] {
// 	if s.len() == 0 {
// 		return [u8]{}
// 	}
// 	if s.len() == 1 {
// 		// Just return a copy.
// 		return append([u8](nil), s[0]...)
// 	}
// 	n := len(sep) * (s.len() - 1)
// 	for _, v := range s {
// 		n += len(v)
// 	}

// 	b := make([u8], n)
// 	bp := copy(b, s[0])
// 	for _, v := range s[1..] {
// 		bp += copy(b[bp..], sep)
// 		bp += copy(b[bp..], v)
// 	}
// 	return b
// }

// // HasPrefix tests whether the byte slice s begins with prefix.
// fn HasPrefix(s, prefix [u8]) -> bool {
// 	return s.len() >= len(prefix) && equal(s[0:len(prefix)], prefix)
// }

/// has_suffix tests whether the byte slice s ends with suffix.
pub fn has_suffix(s: &[u8], suffix: &[u8]) -> bool {
    s.len() >= suffix.len() && equal(&s[s.len() - suffix.len()..], suffix)
}

// // Map returns a copy of the byte slice s with all its characters modified
// // according to the mapping function. If mapping returns a negative value, the character is
// // dropped from the byte slice with no replacement. The characters in s and the
// // output are interpreted as UTF-8-encoded code points.
// fn Map(mapping fn(r rune) rune, s [u8]) [u8] {
// 	// In the worst case, the slice can grow when mapped, making
// 	// things unpleasant. But it's so rare we barge in assuming it's
// 	// fine. It could also shrink but that falls out naturally.
// 	b := make([u8], 0, s.len())
// 	for i := 0; i < s.len(); {
// 		wid := 1
// 		r := rune(s[i])
// 		if r >= utf8::RUNE_SELF {
// 			r, wid = utf8.decode_rune(s[i..])
// 		}
// 		r = mapping(r)
// 		if r >= 0 {
// 			b = utf8.AppendRune(b, r)
// 		}
// 		i += wid
// 	}
// 	return b
// }

// // Repeat returns a new byte slice consisting of count copies of b.
// //
// // It panics if count is negative or if the result of (b.len() * count)
// // overflows.
// fn Repeat(b: &[u8], count int) [u8] {
// 	if count == 0 {
// 		return [u8]{}
// 	}
// 	// Since we cannot return an error on overflow,
// 	// we should panic if the repeat will generate
// 	// an overflow.
// 	// See golang.org/issue/16237.
// 	if count < 0 {
// 		panic("bytes: negative Repeat count")
// 	} else if b.len()*count/count != b.len() {
// 		panic("bytes: Repeat count causes overflow")
// 	}

// 	if b.len() == 0 {
// 		return [u8]{}
// 	}

// 	n := b.len() * count

// 	// Past a certain chunk size it is counterproductive to use
// 	// larger chunks as the source of the write, as when the source
// 	// is too large we are basically just thrashing the CPU D-cache.
// 	// So if the result length is larger than an empirically-found
// 	// limit (8KB), we stop growing the source string once the limit
// 	// is reached and keep reusing the same source string - that
// 	// should therefore be always resident in the L1 cache - until we
// 	// have completed the construction of the result.
// 	// This yields significant speedups (up to +100%) in cases where
// 	// the result length is large (roughly, over L2 cache size).
// 	const chunkLimit = 8 * 1024
// 	chunkMax := n
// 	if chunkMax > chunkLimit {
// 		chunkMax = chunkLimit / b.len() * b.len()
// 		if chunkMax == 0 {
// 			chunkMax = b.len()
// 		}
// 	}
// 	nb := make([u8], n)
// 	bp := copy(nb, b)
// 	for bp < len(nb) {
// 		chunk := bp
// 		if chunk > chunkMax {
// 			chunk = chunkMax
// 		}
// 		bp += copy(nb[bp..], nb[..chunk])
// 	}
// 	return nb
// }

// // ToUpper returns a copy of the byte slice s with all Unicode letters mapped to
// // their upper case.
// fn ToUpper(s [u8]) [u8] {
// 	isASCII, hasLower := true, false
// 	for i := 0; i < s.len(); i += 1 {
// 		c := s[i]
// 		if c >= utf8::RUNE_SELF {
// 			isASCII = false
// 			break
// 		}
// 		hasLower = hasLower || ('a' <= c && c <= 'z')
// 	}

// 	if isASCII { // optimize for ASCII-only byte slices.
// 		if !hasLower {
// 			// Just return a copy.
// 			return append([u8](""), s...)
// 		}
// 		b := make([u8], s.len())
// 		for i := 0; i < s.len(); i += 1 {
// 			c := s[i]
// 			if 'a' <= c && c <= 'z' {
// 				c -= 'a' - 'A'
// 			}
// 			b[i] = c
// 		}
// 		return b
// 	}
// 	return Map(unicode.ToUpper, s)
// }

// // ToLower returns a copy of the byte slice s with all Unicode letters mapped to
// // their lower case.
// fn ToLower(s [u8]) [u8] {
// 	isASCII, hasUpper := true, false
// 	for i := 0; i < s.len(); i += 1 {
// 		c := s[i]
// 		if c >= utf8::RUNE_SELF {
// 			isASCII = false
// 			break
// 		}
// 		hasUpper = hasUpper || ('A' <= c && c <= 'Z')
// 	}

// 	if isASCII { // optimize for ASCII-only byte slices.
// 		if !hasUpper {
// 			return append([u8](""), s...)
// 		}
// 		b := make([u8], s.len())
// 		for i := 0; i < s.len(); i += 1 {
// 			c := s[i]
// 			if 'A' <= c && c <= 'Z' {
// 				c += 'a' - 'A'
// 			}
// 			b[i] = c
// 		}
// 		return b
// 	}
// 	return Map(unicode.ToLower, s)
// }

// // ToTitle treats s as UTF-8-encoded bytes and returns a copy with all the Unicode letters mapped to their title case.
// fn ToTitle(s [u8]) [u8] { return Map(unicode.ToTitle, s) }

// // ToUpperSpecial treats s as UTF-8-encoded bytes and returns a copy with all the Unicode letters mapped to their
// // upper case, giving priority to the special casing rules.
// fn ToUpperSpecial(c unicode.SpecialCase, s [u8]) [u8] {
// 	return Map(c.ToUpper, s)
// }

// // ToLowerSpecial treats s as UTF-8-encoded bytes and returns a copy with all the Unicode letters mapped to their
// // lower case, giving priority to the special casing rules.
// fn ToLowerSpecial(c unicode.SpecialCase, s [u8]) [u8] {
// 	return Map(c.ToLower, s)
// }

// // ToTitleSpecial treats s as UTF-8-encoded bytes and returns a copy with all the Unicode letters mapped to their
// // title case, giving priority to the special casing rules.
// fn ToTitleSpecial(c unicode.SpecialCase, s [u8]) [u8] {
// 	return Map(c.ToTitle, s)
// }

// // ToValidUTF8 treats s as UTF-8-encoded bytes and returns a copy with each run of bytes
// // representing invalid UTF-8 replaced with the bytes in replacement, which may be empty.
// fn ToValidUTF8(s, replacement [u8]) [u8] {
// 	b := make([u8], 0, s.len()+len(replacement))
// 	invalid := false // previous byte was from an invalid UTF-8 sequence
// 	for i := 0; i < s.len(); {
// 		c := s[i]
// 		if c < utf8::RUNE_SELF {
// 			i += 1
// 			invalid = false
// 			b = append(b, c)
// 			continue
// 		}
// 		_, wid := utf8.decode_rune(s[i..])
// 		if wid == 1 {
// 			i += 1
// 			if !invalid {
// 				invalid = true
// 				b = append(b, replacement...)
// 			}
// 			continue
// 		}
// 		invalid = false
// 		b = append(b, s[i:i+wid]...)
// 		i += wid
// 	}
// 	return b
// }

// // isSeparator reports whether the rune could mark a word boundary.
// // TODO: update when package unicode captures more of the properties.
// fn isSeparator(r rune) -> bool {
// 	// ASCII alphanumerics and underscore are not separators
// 	if r <= 0x7F {
// 		switch {
// 		case '0' <= r && r <= '9':
// 			return false
// 		case 'a' <= r && r <= 'z':
// 			return false
// 		case 'A' <= r && r <= 'Z':
// 			return false
// 		case r == '_':
// 			return false
// 		}
// 		return true
// 	}
// 	// Letters and digits are not separators
// 	if unicode.IsLetter(r) || unicode.IsDigit(r) {
// 		return false
// 	}
// 	// Otherwise, all we can do for now is treat spaces as separators.
// 	return unicode.IsSpace(r)
// }

// // Title treats s as UTF-8-encoded bytes and returns a copy with all Unicode letters that begin
// // words mapped to their title case.
// //
// // Deprecated: The rule Title uses for word boundaries does not handle Unicode
// // punctuation properly. Use golang.org/x/text/cases instead.
// fn Title(s [u8]) [u8] {
// 	// Use a closure here to remember state.
// 	// Hackish but effective. Depends on Map scanning in order and calling
// 	// the closure once per rune.
// 	prev := ' '
// 	return Map(
// 		fn(r rune) rune {
// 			if isSeparator(prev) {
// 				prev = r
// 				return unicode.ToTitle(r)
// 			}
// 			prev = r
// 			return r
// 		},
// 		s)
// }

// // TrimLeftFunc treats s as UTF-8-encoded bytes and returns a subslice of s by slicing off
// // all leading UTF-8-encoded code points c that satisfy f(c).
// fn TrimLeftFunc(s [u8], f fn(r rune) bool) [u8] {
// 	i := indexFunc(s, f, false)
// 	if i == -1 {
// 		return nil
// 	}
// 	return s[i..]
// }

// // TrimRightFunc returns a subslice of s by slicing off all trailing
// // UTF-8-encoded code points c that satisfy f(c).
// fn TrimRightFunc(s [u8], f fn(r rune) bool) [u8] {
// 	i := lastIndexFunc(s, f, false)
// 	if i >= 0 && s[i] >= utf8::RUNE_SELF {
// 		_, wid := utf8.decode_rune(s[i..])
// 		i += wid
// 	} else {
// 		i += 1
// 	}
// 	return s[0:i]
// }

// // TrimFunc returns a subslice of s by slicing off all leading and trailing
// // UTF-8-encoded code points c that satisfy f(c).
// fn TrimFunc(s [u8], f fn(r rune) bool) [u8] {
// 	return TrimRightFunc(TrimLeftFunc(s, f), f)
// }

// // TrimPrefix returns s without the provided leading prefix string.
// // If s doesn't start with prefix, s is returned unchanged.
// fn TrimPrefix(s, prefix [u8]) [u8] {
// 	if HasPrefix(s, prefix) {
// 		return s[len(prefix)..]
// 	}
// 	return s
// }

// // TrimSuffix returns s without the provided trailing suffix string.
// // If s doesn't end with suffix, s is returned unchanged.
// fn TrimSuffix(s, suffix: &[u8]) [u8] {
// 	if has_suffix(s, suffix) {
// 		return s[..s.len()-suffix.len()]
// 	}
// 	return s
// }

// // IndexFunc interprets s as a sequence of UTF-8-encoded code points.
// // It returns the byte index in s of the first Unicode
// // code point satisfying f(c), or -1 if none do.
// fn IndexFunc(s [u8], f fn(r rune) bool) int {
// 	return indexFunc(s, f, true)
// }

// // LastIndexFunc interprets s as a sequence of UTF-8-encoded code points.
// // It returns the byte index in s of the last Unicode
// // code point satisfying f(c), or -1 if none do.
// fn LastIndexFunc(s [u8], f fn(r rune) bool) int {
// 	return lastIndexFunc(s, f, true)
// }

// // indexFunc is the same as IndexFunc except that if
// // truth==false, the sense of the predicate function is
// // inverted.
// fn indexFunc(s [u8], f fn(r rune) bool, truth bool) int {
// 	start := 0
// 	for start < s.len() {
// 		wid := 1
// 		r := rune(s[start])
// 		if r >= utf8::RUNE_SELF {
// 			r, wid = utf8.decode_rune(s[start..])
// 		}
// 		if f(r) == truth {
// 			return start
// 		}
// 		start += wid
// 	}
// 	return -1
// }

// // lastIndexFunc is the same as LastIndexFunc except that if
// // truth==false, the sense of the predicate function is
// // inverted.
// fn lastIndexFunc(s [u8], f fn(r rune) bool, truth bool) int {
// 	for i := s.len(); i > 0; {
// 		r, size := rune(s[i-1]), 1
// 		if r >= utf8::RUNE_SELF {
// 			r, size = utf8.DecodeLastRune(s[0:i])
// 		}
// 		i -= size
// 		if f(r) == truth {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // asciiSet is a 32-byte value, where each bit represents the presence of a
// // given ASCII character in the set. The 128-bits of the lower 16 bytes,
// // starting with the least-significant bit of the lowest word to the
// // most-significant bit of the highest word, map to the full range of all
// // 128 ASCII characters. The 128-bits of the upper 16 bytes will be zeroed,
// // ensuring that any non-ASCII character will be reported as not in the set.
// // This allocates a total of 32 bytes even though the upper half
// // is unused to avoid bounds checks in asciiSet.contains.
// type asciiSet [8]uint32

// // makeASCIISet creates a set of ASCII characters and reports whether all
// // characters in chars are ASCII.
// fn makeASCIISet(chars string) (as asciiSet, ok bool) {
// 	for i := 0; i < len(chars); i += 1 {
// 		c := chars[i]
// 		if c >= utf8::RUNE_SELF {
// 			return as, false
// 		}
// 		as[c/32] |= 1 << (c % 32)
// 	}
// 	return as, true
// }

// // contains reports whether c is inside the set.
// fn (as *asciiSet) contains(c byte) -> bool {
// 	return (as[c/32] & (1 << (c % 32))) != 0
// }

// // containsRune is a simplified version of strings.ContainsRune
// // to avoid importing the strings package.
// // We avoid bytes.ContainsRune to avoid allocating a temporary copy of s.
// fn containsRune(s string, r rune) -> bool {
// 	for _, c := range s {
// 		if c == r {
// 			return true
// 		}
// 	}
// 	return false
// }

// // Trim returns a subslice of s by slicing off all leading and
// // trailing UTF-8-encoded code points contained in cutset.
// fn Trim(s [u8], cutset string) [u8] {
// 	if s.len() == 0 {
// 		// This is what we've historically done.
// 		return nil
// 	}
// 	if cutset == "" {
// 		return s
// 	}
// 	if len(cutset) == 1 && cutset[0] < utf8::RUNE_SELF {
// 		return trimLeftByte(trimRightByte(s, cutset[0]), cutset[0])
// 	}
// 	if as, ok := makeASCIISet(cutset); ok {
// 		return trimLeftASCII(trimRightASCII(s, &as), &as)
// 	}
// 	return trimLeftUnicode(trimRightUnicode(s, cutset), cutset)
// }

// // TrimLeft returns a subslice of s by slicing off all leading
// // UTF-8-encoded code points contained in cutset.
// fn TrimLeft(s [u8], cutset string) [u8] {
// 	if s.len() == 0 {
// 		// This is what we've historically done.
// 		return nil
// 	}
// 	if cutset == "" {
// 		return s
// 	}
// 	if len(cutset) == 1 && cutset[0] < utf8::RUNE_SELF {
// 		return trimLeftByte(s, cutset[0])
// 	}
// 	if as, ok := makeASCIISet(cutset); ok {
// 		return trimLeftASCII(s, &as)
// 	}
// 	return trimLeftUnicode(s, cutset)
// }

// fn trimLeftByte(s [u8], c byte) [u8] {
// 	for s.len() > 0 && s[0] == c {
// 		s = s[1..]
// 	}
// 	if s.len() == 0 {
// 		// This is what we've historically done.
// 		return nil
// 	}
// 	return s
// }

// fn trimLeftASCII(s [u8], as *asciiSet) [u8] {
// 	for s.len() > 0 {
// 		if !as.contains(s[0]) {
// 			break
// 		}
// 		s = s[1..]
// 	}
// 	if s.len() == 0 {
// 		// This is what we've historically done.
// 		return nil
// 	}
// 	return s
// }

// fn trimLeftUnicode(s [u8], cutset string) [u8] {
// 	for s.len() > 0 {
// 		r, n := rune(s[0]), 1
// 		if r >= utf8::RUNE_SELF {
// 			r, n = utf8.decode_rune(s)
// 		}
// 		if !containsRune(cutset, r) {
// 			break
// 		}
// 		s = s[n..]
// 	}
// 	if s.len() == 0 {
// 		// This is what we've historically done.
// 		return nil
// 	}
// 	return s
// }

// // TrimRight returns a subslice of s by slicing off all trailing
// // UTF-8-encoded code points that are contained in cutset.
// fn TrimRight(s [u8], cutset string) [u8] {
// 	if s.len() == 0 || cutset == "" {
// 		return s
// 	}
// 	if len(cutset) == 1 && cutset[0] < utf8::RUNE_SELF {
// 		return trimRightByte(s, cutset[0])
// 	}
// 	if as, ok := makeASCIISet(cutset); ok {
// 		return trimRightASCII(s, &as)
// 	}
// 	return trimRightUnicode(s, cutset)
// }

// fn trimRightByte(s [u8], c byte) [u8] {
// 	for s.len() > 0 && s[s.len()-1] == c {
// 		s = s[..s.len()-1]
// 	}
// 	return s
// }

// fn trimRightASCII(s [u8], as *asciiSet) [u8] {
// 	for s.len() > 0 {
// 		if !as.contains(s[s.len()-1]) {
// 			break
// 		}
// 		s = s[..s.len()-1]
// 	}
// 	return s
// }

// fn trimRightUnicode(s [u8], cutset string) [u8] {
// 	for s.len() > 0 {
// 		r, n := rune(s[s.len()-1]), 1
// 		if r >= utf8::RUNE_SELF {
// 			r, n = utf8.DecodeLastRune(s)
// 		}
// 		if !containsRune(cutset, r) {
// 			break
// 		}
// 		s = s[..s.len()-n]
// 	}
// 	return s
// }

// // TrimSpace returns a subslice of s by slicing off all leading and
// // trailing white space, as defined by Unicode.
// fn TrimSpace(s [u8]) [u8] {
// 	// Fast path for ASCII: look for the first ASCII non-space byte
// 	start := 0
// 	for ; start < s.len(); start++ {
// 		c := s[start]
// 		if c >= utf8::RUNE_SELF {
// 			// If we run into a non-ASCII byte, fall back to the
// 			// slower unicode-aware method on the remaining bytes
// 			return TrimFunc(s[start..], unicode.IsSpace)
// 		}
// 		if asciiSpace[c] == 0 {
// 			break
// 		}
// 	}

// 	// Now look for the first ASCII non-space byte from the end
// 	stop := s.len()
// 	for ; stop > start; stop-- {
// 		c := s[stop-1]
// 		if c >= utf8::RUNE_SELF {
// 			return TrimFunc(s[start:stop], unicode.IsSpace)
// 		}
// 		if asciiSpace[c] == 0 {
// 			break
// 		}
// 	}

// 	// At this point s[start:stop] starts and ends with an ASCII
// 	// non-space bytes, so we're done. Non-ASCII cases have already
// 	// been handled above.
// 	if start == stop {
// 		// Special case to preserve previous TrimLeftFunc behavior,
// 		// returning nil instead of empty slice if all spaces.
// 		return nil
// 	}
// 	return s[start:stop]
// }

// // Runes interprets s as a sequence of UTF-8-encoded code points.
// // It returns a slice of runes (Unicode code points) equivalent to s.
// fn Runes(s [u8]) []rune {
// 	t := make([]rune, utf8.rune_count(s))
// 	i := 0
// 	for s.len() > 0 {
// 		r, l := utf8.decode_rune(s)
// 		t[i] = r
// 		i += 1
// 		s = s[l..]
// 	}
// 	return t
// }

// // Replace returns a copy of the slice s with the first n
// // non-overlapping instances of old replaced by new.
// // If old is empty, it matches at the beginning of the slice
// // and after each UTF-8 sequence, yielding up to k+1 replacements
// // for a k-rune slice.
// // If n < 0, there is no limit on the number of replacements.
// fn Replace(s, old, new [u8], n int) [u8] {
// 	m := 0
// 	if n != 0 {
// 		// Compute number of replacements.
// 		m = Count(s, old)
// 	}
// 	if m == 0 {
// 		// Just return a copy.
// 		return append([u8](nil), s...)
// 	}
// 	if n < 0 || m < n {
// 		n = m
// 	}

// 	// Apply replacements to buffer.
// 	t := make([u8], s.len()+n*(len(new)-len(old)))
// 	w := 0
// 	start := 0
// 	for i := 0; i < n; i += 1 {
// 		j := start
// 		if len(old) == 0 {
// 			if i > 0 {
// 				_, wid := utf8.decode_rune(s[start..])
// 				j += wid
// 			}
// 		} else {
// 			j += Index(s[start..], old)
// 		}
// 		w += copy(t[w..], s[start:j])
// 		w += copy(t[w..], new)
// 		start = j + len(old)
// 	}
// 	w += copy(t[w..], s[start..])
// 	return t[0:w]
// }

// // ReplaceAll returns a copy of the slice s with all
// // non-overlapping instances of old replaced by new.
// // If old is empty, it matches at the beginning of the slice
// // and after each UTF-8 sequence, yielding up to k+1 replacements
// // for a k-rune slice.
// fn ReplaceAll(s, old, new [u8]) [u8] {
// 	return Replace(s, old, new, -1)
// }

// // EqualFold reports whether s and t, interpreted as UTF-8 strings,
// // are equal under simple Unicode case-folding, which is a more general
// // form of case-insensitivity.
// fn EqualFold(s, t [u8]) -> bool {
// 	// ASCII fast path
// 	i := 0
// 	for ; i < s.len() && i < len(t); i += 1 {
// 		sr := s[i]
// 		tr := t[i]
// 		if sr|tr >= utf8::RUNE_SELF {
// 			goto hasUnicode
// 		}

// 		// Easy case.
// 		if tr == sr {
// 			continue
// 		}

// 		// Make sr < tr to simplify what follows.
// 		if tr < sr {
// 			tr, sr = sr, tr
// 		}
// 		// ASCII only, sr/tr must be upper/lower case
// 		if 'A' <= sr && sr <= 'Z' && tr == sr+'a'-'A' {
// 			continue
// 		}
// 		return false
// 	}
// 	// Check if we've exhausted both strings.
// 	return s.len() == len(t)

// hasUnicode:
// 	s = s[i..]
// 	t = t[i..]
// 	for s.len() != 0 && len(t) != 0 {
// 		// Extract first rune from each.
// 		var sr, tr rune
// 		if s[0] < utf8::RUNE_SELF {
// 			sr, s = rune(s[0]), s[1..]
// 		} else {
// 			r, size := utf8.decode_rune(s)
// 			sr, s = r, s[size..]
// 		}
// 		if t[0] < utf8::RUNE_SELF {
// 			tr, t = rune(t[0]), t[1..]
// 		} else {
// 			r, size := utf8.decode_rune(t)
// 			tr, t = r, t[size..]
// 		}

// 		// If they match, keep going; if not, return false.

// 		// Easy case.
// 		if tr == sr {
// 			continue
// 		}

// 		// Make sr < tr to simplify what follows.
// 		if tr < sr {
// 			tr, sr = sr, tr
// 		}
// 		// Fast check for ASCII.
// 		if tr < utf8::RUNE_SELF {
// 			// ASCII only, sr/tr must be upper/lower case
// 			if 'A' <= sr && sr <= 'Z' && tr == sr+'a'-'A' {
// 				continue
// 			}
// 			return false
// 		}

// 		// General case. SimpleFold(x) returns the next equivalent rune > x
// 		// or wraps around to smaller values.
// 		r := unicode.SimpleFold(sr)
// 		for r != sr && r < tr {
// 			r = unicode.SimpleFold(r)
// 		}
// 		if r == tr {
// 			continue
// 		}
// 		return false
// 	}

// 	// One string is empty. Are both?
// 	return s.len() == len(t)
// }

// // Index returns the index of the first instance of sep in s, or -1 if sep is not present in s.
// fn Index(s, sep [u8]) int {
// 	n := len(sep)
// 	switch {
// 	case n == 0:
// 		return 0
// 	case n == 1:
// 		return IndexByte(s, sep[0])
// 	case n == s.len():
// 		if equal(sep, s) {
// 			return 0
// 		}
// 		return -1
// 	case n > s.len():
// 		return -1
// 	case n <= bytealg.MaxLen:
// 		// Use brute force when s and sep both are small
// 		if s.len() <= bytealg.MaxBruteForce {
// 			return bytealg.Index(s, sep)
// 		}
// 		c0 := sep[0]
// 		c1 := sep[1]
// 		i := 0
// 		t := s.len() - n + 1
// 		fails := 0
// 		for i < t {
// 			if s[i] != c0 {
// 				// IndexByte is faster than bytealg.Index, so use it as long as
// 				// we're not getting lots of false positives.
// 				o := IndexByte(s[i+1:t], c0)
// 				if o < 0 {
// 					return -1
// 				}
// 				i += o + 1
// 			}
// 			if s[i+1] == c1 && equal(s[i:i+n], sep) {
// 				return i
// 			}
// 			fails++
// 			i += 1
// 			// Switch to bytealg.Index when IndexByte produces too many false positives.
// 			if fails > bytealg.Cutover(i) {
// 				r := bytealg.Index(s[i..], sep)
// 				if r >= 0 {
// 					return r + i
// 				}
// 				return -1
// 			}
// 		}
// 		return -1
// 	}
// 	c0 := sep[0]
// 	c1 := sep[1]
// 	i := 0
// 	fails := 0
// 	t := s.len() - n + 1
// 	for i < t {
// 		if s[i] != c0 {
// 			o := IndexByte(s[i+1:t], c0)
// 			if o < 0 {
// 				break
// 			}
// 			i += o + 1
// 		}
// 		if s[i+1] == c1 && equal(s[i:i+n], sep) {
// 			return i
// 		}
// 		i += 1
// 		fails++
// 		if fails >= 4+i>>4 && i < t {
// 			// Give up on IndexByte, it isn't skipping ahead
// 			// far enough to be better than Rabin-Karp.
// 			// Experiments (using IndexPeriodic) suggest
// 			// the cutover is about 16 byte skips.
// 			// TODO: if large prefixes of sep are matching
// 			// we should cutover at even larger average skips,
// 			// because equal becomes that much more expensive.
// 			// This code does not take that effect into account.
// 			j := bytealg.IndexRabinKarpBytes(s[i..], sep)
// 			if j < 0 {
// 				return -1
// 			}
// 			return i + j
// 		}
// 	}
// 	return -1
// }

// // Cut slices s around the first instance of sep,
// // returning the text before and after sep.
// // The found result reports whether sep appears in s.
// // If sep does not appear in s, cut returns s, nil, false.
// //
// // Cut returns slices of the original slice s, not copies.
// fn Cut(s, sep [u8]) (before, after [u8], found bool) {
// 	if i := Index(s, sep); i >= 0 {
// 		return s[..i], s[i+len(sep)..], true
// 	}
// 	return s, nil, false
// }

// // Clone returns a copy of b[..b.len()].
// // The result may have additional unused capacity.
// // Clone(nil) returns nil.
// fn Clone(b: &[u8]) [u8] {
// 	if b == nil {
// 		return nil
// 	}
// 	return append([u8]{}, b...)
// }

// // CutPrefix returns s without the provided leading prefix byte slice
// // and reports whether it found the prefix.
// // If s doesn't start with prefix, CutPrefix returns s, false.
// // If prefix is the empty byte slice, CutPrefix returns s, true.
// //
// // CutPrefix returns slices of the original slice s, not copies.
// fn CutPrefix(s, prefix [u8]) (after [u8], found bool) {
// 	if !HasPrefix(s, prefix) {
// 		return s, false
// 	}
// 	return s[len(prefix)..], true
// }

// // CutSuffix returns s without the provided ending suffix byte slice
// // and reports whether it found the suffix.
// // If s doesn't end with suffix, CutSuffix returns s, false.
// // If suffix is the empty byte slice, CutSuffix returns s, true.
// //
// // CutSuffix returns slices of the original slice s, not copies.
// fn CutSuffix(s, suffix: &[u8]) (before [u8], found bool) {
// 	if !has_suffix(s, suffix) {
// 		return s, false
// 	}
// 	return s[..s.len()-suffix.len()], true
// }
