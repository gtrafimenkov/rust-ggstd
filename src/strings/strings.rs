// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// // Package strings implements simple functions to manipulate UTF-8 encoded strings.
// //
// // For information about UTF-8 strings in Go, see https://blog.golang.org/strings.
// package strings

// import (
// 	"internal/bytealg"
// 	"unicode"
// 	"unicode/utf8"
// )

// // explode splits s into a slice of UTF-8 strings,
// // one string per Unicode character up to a maximum of n (n < 0 means no limit).
// // Invalid UTF-8 bytes are sliced individually.
// fn explode(s string, n int) []string {
// 	l := utf8.RuneCountInString(s)
// 	if n < 0 || n > l {
// 		n = l
// 	}
// 	a := make([]string, n)
// 	for i := 0; i < n-1; i++ {
// 		_, size := utf8.decode_rune_in_string(s)
// 		a[i] = s[:size]
// 		s = s[size:]
// 	}
// 	if n > 0 {
// 		a[n-1] = s
// 	}
// 	return a
// }

// // Count counts the number of non-overlapping instances of substr in s.
// // If substr is an empty string, Count returns 1 + the number of Unicode code points in s.
// fn Count(s, substr string) int {
// 	// special case
// 	if len(substr) == 0 {
// 		return utf8.RuneCountInString(s) + 1
// 	}
// 	if len(substr) == 1 {
// 		return bytealg.CountString(s, substr[0])
// 	}
// 	n := 0
// 	for {
// 		i := index(s, substr)
// 		if i == -1 {
// 			return n
// 		}
// 		n++
// 		s = s[i+len(substr):]
// 	}
// }

/// contains reports whether substr is within s.
pub fn contains(s: &str, substr: &str) -> bool {
    // index(s, substr) >= 0
    s.contains(substr)
}

// // ContainsAny reports whether any Unicode code points in chars are within s.
// fn ContainsAny(s, chars string) -> bool {
// 	return IndexAny(s, chars) >= 0
// }

// // ContainsRune reports whether the Unicode code point r is within s.
// fn ContainsRune(s string, r rune) -> bool {
// 	return IndexRune(s, r) >= 0
// }

// // LastIndex returns the index of the last instance of substr in s, or -1 if substr is not present in s.
// fn LastIndex(s, substr string) int {
// 	n := len(substr)
// 	switch {
// 	case n == 0:
// 		return len(s)
// 	case n == 1:
// 		return LastIndexByte(s, substr[0])
// 	case n == len(s):
// 		if substr == s {
// 			return 0
// 		}
// 		return -1
// 	case n > len(s):
// 		return -1
// 	}
// 	// Rabin-Karp search from the end of the string
// 	hashss, pow := bytealg.HashStrRev(substr)
// 	last := len(s) - n
// 	var h uint32
// 	for i := len(s) - 1; i >= last; i-- {
// 		h = h*bytealg.PrimeRK + uint32(s[i])
// 	}
// 	if h == hashss && s[last:] == substr {
// 		return last
// 	}
// 	for i := last - 1; i >= 0; i-- {
// 		h *= bytealg.PrimeRK
// 		h += uint32(s[i])
// 		h -= pow * uint32(s[i+n])
// 		if h == hashss && s[i:i+n] == substr {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // index_byte returns the index of the first instance of c in s, or -1 if c is not present in s.
// fn index_byte(s string, c byte) int {
// 	return bytealg.IndexByteString(s, c)
// }

// // IndexRune returns the index of the first instance of the Unicode code point
// // r, or -1 if rune is not present in s.
// // If r is utf8.RUNE_ERROR, it returns the first instance of any
// // invalid UTF-8 byte sequence.
// fn IndexRune(s string, r rune) int {
// 	switch {
// 	case 0 <= r && r < utf8::RUNE_SELF:
// 		return index_byte(s, byte(r))
// 	case r == utf8.RUNE_ERROR:
// 		for i, r := range s {
// 			if r == utf8.RUNE_ERROR {
// 				return i
// 			}
// 		}
// 		return -1
// 	case !utf8.ValidRune(r):
// 		return -1
// 	default:
// 		return index(s, string(r))
// 	}
// }

// // IndexAny returns the index of the first instance of any Unicode code point
// // from chars in s, or -1 if no Unicode code point from chars is present in s.
// fn IndexAny(s, chars string) int {
// 	if chars == "" {
// 		// Avoid scanning all of s.
// 		return -1
// 	}
// 	if len(chars) == 1 {
// 		// Avoid scanning all of s.
// 		r := rune(chars[0])
// 		if r >= utf8::RUNE_SELF {
// 			r = utf8.RUNE_ERROR
// 		}
// 		return IndexRune(s, r)
// 	}
// 	if len(s) > 8 {
// 		if as, isASCII := makeASCIISet(chars); isASCII {
// 			for i := 0; i < len(s); i++ {
// 				if as.contains(s[i]) {
// 					return i
// 				}
// 			}
// 			return -1
// 		}
// 	}
// 	for i, c := range s {
// 		if IndexRune(chars, c) >= 0 {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // LastIndexAny returns the index of the last instance of any Unicode code
// // point from chars in s, or -1 if no Unicode code point from chars is
// // present in s.
// fn LastIndexAny(s, chars string) int {
// 	if chars == "" {
// 		// Avoid scanning all of s.
// 		return -1
// 	}
// 	if len(s) == 1 {
// 		rc := rune(s[0])
// 		if rc >= utf8::RUNE_SELF {
// 			rc = utf8.RUNE_ERROR
// 		}
// 		if IndexRune(chars, rc) >= 0 {
// 			return 0
// 		}
// 		return -1
// 	}
// 	if len(s) > 8 {
// 		if as, isASCII := makeASCIISet(chars); isASCII {
// 			for i := len(s) - 1; i >= 0; i-- {
// 				if as.contains(s[i]) {
// 					return i
// 				}
// 			}
// 			return -1
// 		}
// 	}
// 	if len(chars) == 1 {
// 		rc := rune(chars[0])
// 		if rc >= utf8::RUNE_SELF {
// 			rc = utf8.RUNE_ERROR
// 		}
// 		for i := len(s); i > 0; {
// 			r, size := utf8.DecodeLastRuneInString(s[:i])
// 			i -= size
// 			if rc == r {
// 				return i
// 			}
// 		}
// 		return -1
// 	}
// 	for i := len(s); i > 0; {
// 		r, size := utf8.DecodeLastRuneInString(s[:i])
// 		i -= size
// 		if IndexRune(chars, r) >= 0 {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present in s.
// fn LastIndexByte(s string, c byte) int {
// 	for i := len(s) - 1; i >= 0; i-- {
// 		if s[i] == c {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // Generic split: splits after each instance of sep,
// // including sepSave bytes of sep in the subarrays.
// fn genSplit(s, sep string, sepSave, n int) []string {
// 	if n == 0 {
// 		return nil
// 	}
// 	if sep == "" {
// 		return explode(s, n)
// 	}
// 	if n < 0 {
// 		n = Count(s, sep) + 1
// 	}

// 	if n > len(s)+1 {
// 		n = len(s) + 1
// 	}
// 	a := make([]string, n)
// 	n--
// 	i := 0
// 	for i < n {
// 		m := index(s, sep)
// 		if m < 0 {
// 			break
// 		}
// 		a[i] = s[:m+sepSave]
// 		s = s[m+len(sep):]
// 		i++
// 	}
// 	a[i] = s
// 	return a[:i+1]
// }

// // SplitN slices s into substrings separated by sep and returns a slice of
// // the substrings between those separators.
// //
// // The count determines the number of substrings to return:
// //
// //	n > 0: at most n substrings; the last substring will be the unsplit remainder.
// //	n == 0: the result is nil (zero substrings)
// //	n < 0: all substrings
// //
// // Edge cases for s and sep (for example, empty strings) are handled
// // as described in the documentation for Split.
// //
// // To split around the first instance of a separator, see Cut.
// fn SplitN(s, sep string, n int) []string { return genSplit(s, sep, 0, n) }

// // SplitAfterN slices s into substrings after each instance of sep and
// // returns a slice of those substrings.
// //
// // The count determines the number of substrings to return:
// //
// //	n > 0: at most n substrings; the last substring will be the unsplit remainder.
// //	n == 0: the result is nil (zero substrings)
// //	n < 0: all substrings
// //
// // Edge cases for s and sep (for example, empty strings) are handled
// // as described in the documentation for SplitAfter.
// fn SplitAfterN(s, sep string, n int) []string {
// 	return genSplit(s, sep, len(sep), n)
// }

// // Split slices s into all substrings separated by sep and returns a slice of
// // the substrings between those separators.
// //
// // If s does not contain sep and sep is not empty, Split returns a
// // slice of length 1 whose only element is s.
// //
// // If sep is empty, Split splits after each UTF-8 sequence. If both s
// // and sep are empty, Split returns an empty slice.
// //
// // It is equivalent to SplitN with a count of -1.
// //
// // To split around the first instance of a separator, see Cut.
// fn Split(s, sep string) []string { return genSplit(s, sep, 0, -1) }

// // SplitAfter slices s into all substrings after each instance of sep and
// // returns a slice of those substrings.
// //
// // If s does not contain sep and sep is not empty, SplitAfter returns
// // a slice of length 1 whose only element is s.
// //
// // If sep is empty, SplitAfter splits after each UTF-8 sequence. If
// // both s and sep are empty, SplitAfter returns an empty slice.
// //
// // It is equivalent to SplitAfterN with a count of -1.
// fn SplitAfter(s, sep string) []string {
// 	return genSplit(s, sep, len(sep), -1)
// }

// var asciiSpace = [256]uint8{'\t': 1, '\n': 1, '\v': 1, '\f': 1, '\r': 1, ' ': 1}

// // Fields splits the string s around each instance of one or more consecutive white space
// // characters, as defined by unicode.IsSpace, returning a slice of substrings of s or an
// // empty slice if s contains only white space.
// fn Fields(s string) []string {
// 	// First count the fields.
// 	// This is an exact count if s is ASCII, otherwise it is an approximation.
// 	n := 0
// 	wasSpace := 1
// 	// setBits is used to track which bits are set in the bytes of s.
// 	setBits := uint8(0)
// 	for i := 0; i < len(s); i++ {
// 		r := s[i]
// 		setBits |= r
// 		isSpace := int(asciiSpace[r])
// 		n += wasSpace & ^isSpace
// 		wasSpace = isSpace
// 	}

// 	if setBits >= utf8::RUNE_SELF {
// 		// Some runes in the input string are not ASCII.
// 		return FieldsFunc(s, unicode.IsSpace)
// 	}
// 	// ASCII fast path
// 	a := make([]string, n)
// 	na := 0
// 	fieldStart := 0
// 	i := 0
// 	// Skip spaces in the front of the input.
// 	for i < len(s) && asciiSpace[s[i]] != 0 {
// 		i++
// 	}
// 	fieldStart = i
// 	for i < len(s) {
// 		if asciiSpace[s[i]] == 0 {
// 			i++
// 			continue
// 		}
// 		a[na] = s[fieldStart:i]
// 		na++
// 		i++
// 		// Skip spaces in between fields.
// 		for i < len(s) && asciiSpace[s[i]] != 0 {
// 			i++
// 		}
// 		fieldStart = i
// 	}
// 	if fieldStart < len(s) { // Last field might end at EOF.
// 		a[na] = s[fieldStart:]
// 	}
// 	return a
// }

// // FieldsFunc splits the string s at each run of Unicode code points c satisfying f(c)
// // and returns an array of slices of s. If all code points in s satisfy f(c) or the
// // string is empty, an empty slice is returned.
// //
// // FieldsFunc makes no guarantees about the order in which it calls f(c)
// // and assumes that f always returns the same value for a given c.
// fn FieldsFunc(s string, f fn(rune) bool) []string {
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
// 	for end, rune := range s {
// 		if f(rune) {
// 			if start >= 0 {
// 				spans = append(spans, span{start, end})
// 				// Set start to a negative value.
// 				// Note: using -1 here consistently and reproducibly
// 				// slows down this code by a several percent on amd64.
// 				start = ^start
// 			}
// 		} else {
// 			if start < 0 {
// 				start = end
// 			}
// 		}
// 	}

// 	// Last field might end at EOF.
// 	if start >= 0 {
// 		spans = append(spans, span{start, len(s)})
// 	}

// 	// Create strings from recorded field indices.
// 	a := make([]string, len(spans))
// 	for i, span := range spans {
// 		a[i] = s[span.start:span.end]
// 	}

// 	return a
// }

// // Join concatenates the elements of its first argument to create a single string. The separator
// // string sep is placed between elements in the resulting string.
// fn Join(elems []string, sep string) string {
// 	switch len(elems) {
// 	case 0:
// 		return ""
// 	case 1:
// 		return elems[0]
// 	}
// 	n := len(sep) * (len(elems) - 1)
// 	for i := 0; i < len(elems); i++ {
// 		n += len(elems[i])
// 	}

// 	var b Builder
// 	b.Grow(n)
// 	b.WriteString(elems[0])
// 	for _, s := range elems[1:] {
// 		b.WriteString(sep)
// 		b.WriteString(s)
// 	}
// 	return b.String()
// }

// // HasPrefix tests whether the string s begins with prefix.
// fn HasPrefix(s, prefix string) -> bool {
// 	return len(s) >= len(prefix) && s[0:len(prefix)] == prefix
// }

/// has_suffix tests whether the string s ends with suffix.
pub fn has_suffix(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

// // Map returns a copy of the string s with all its characters modified
// // according to the mapping function. If mapping returns a negative value, the character is
// // dropped from the string with no replacement.
// fn Map(mapping fn(rune) rune, s string) string {
// 	// In the worst case, the string can grow when mapped, making
// 	// things unpleasant. But it's so rare we barge in assuming it's
// 	// fine. It could also shrink but that falls out naturally.

// 	// The output buffer b is initialized on demand, the first
// 	// time a character differs.
// 	var b Builder

// 	for i, c := range s {
// 		r := mapping(c)
// 		if r == c && c != utf8.RUNE_ERROR {
// 			continue
// 		}

// 		var width int
// 		if c == utf8.RUNE_ERROR {
// 			c, width = utf8.decode_rune_in_string(s[i:])
// 			if width != 1 && r == c {
// 				continue
// 			}
// 		} else {
// 			width = utf8.rune_len(c)
// 		}

// 		b.Grow(len(s) + utf8.UTFMAX)
// 		b.WriteString(s[:i])
// 		if r >= 0 {
// 			b.WriteRune(r)
// 		}

// 		s = s[i+width:]
// 		break
// 	}

// 	// Fast path for unchanged input
// 	if b.Cap() == 0 { // didn't call b.Grow above
// 		return s
// 	}

// 	for _, c := range s {
// 		r := mapping(c)

// 		if r >= 0 {
// 			// common case
// 			// Due to inlining, it is more performant to determine if WriteByte should be
// 			// invoked rather than always call WriteRune
// 			if r < utf8::RUNE_SELF {
// 				b.WriteByte(byte(r))
// 			} else {
// 				// r is not a ASCII rune.
// 				b.WriteRune(r)
// 			}
// 		}
// 	}

// 	return b.String()
// }

// // Repeat returns a new string consisting of count copies of the string s.
// //
// // It panics if count is negative or if the result of (len(s) * count)
// // overflows.
// fn Repeat(s string, count int) string {
// 	switch count {
// 	case 0:
// 		return ""
// 	case 1:
// 		return s
// 	}

// 	// Since we cannot return an error on overflow,
// 	// we should panic if the repeat will generate
// 	// an overflow.
// 	// See golang.org/issue/16237.
// 	if count < 0 {
// 		panic("strings: negative Repeat count")
// 	} else if len(s)*count/count != len(s) {
// 		panic("strings: Repeat count causes overflow")
// 	}

// 	if len(s) == 0 {
// 		return ""
// 	}

// 	n := len(s) * count

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
// 	if n > chunkLimit {
// 		chunkMax = chunkLimit / len(s) * len(s)
// 		if chunkMax == 0 {
// 			chunkMax = len(s)
// 		}
// 	}

// 	var b Builder
// 	b.Grow(n)
// 	b.WriteString(s)
// 	for b.Len() < n {
// 		chunk := n - b.Len()
// 		if chunk > b.Len() {
// 			chunk = b.Len()
// 		}
// 		if chunk > chunkMax {
// 			chunk = chunkMax
// 		}
// 		b.WriteString(b.String()[:chunk])
// 	}
// 	return b.String()
// }

// // ToUpper returns s with all Unicode letters mapped to their upper case.
// fn ToUpper(s string) string {
// 	isASCII, hasLower := true, false
// 	for i := 0; i < len(s); i++ {
// 		c := s[i]
// 		if c >= utf8::RUNE_SELF {
// 			isASCII = false
// 			break
// 		}
// 		hasLower = hasLower || ('a' <= c && c <= 'z')
// 	}

// 	if isASCII { // optimize for ASCII-only strings.
// 		if !hasLower {
// 			return s
// 		}
// 		var (
// 			b   Builder
// 			pos int
// 		)
// 		b.Grow(len(s))
// 		for i := 0; i < len(s); i++ {
// 			c := s[i]
// 			if 'a' <= c && c <= 'z' {
// 				c -= 'a' - 'A'
// 				if pos < i {
// 					b.WriteString(s[pos:i])
// 				}
// 				b.WriteByte(c)
// 				pos = i + 1
// 			}
// 		}
// 		if pos < len(s) {
// 			b.WriteString(s[pos:])
// 		}
// 		return b.String()
// 	}
// 	return Map(unicode.ToUpper, s)
// }

// // ToLower returns s with all Unicode letters mapped to their lower case.
// fn ToLower(s string) string {
// 	isASCII, hasUpper := true, false
// 	for i := 0; i < len(s); i++ {
// 		c := s[i]
// 		if c >= utf8::RUNE_SELF {
// 			isASCII = false
// 			break
// 		}
// 		hasUpper = hasUpper || ('A' <= c && c <= 'Z')
// 	}

// 	if isASCII { // optimize for ASCII-only strings.
// 		if !hasUpper {
// 			return s
// 		}
// 		var (
// 			b   Builder
// 			pos int
// 		)
// 		b.Grow(len(s))
// 		for i := 0; i < len(s); i++ {
// 			c := s[i]
// 			if 'A' <= c && c <= 'Z' {
// 				c += 'a' - 'A'
// 				if pos < i {
// 					b.WriteString(s[pos:i])
// 				}
// 				b.WriteByte(c)
// 				pos = i + 1
// 			}
// 		}
// 		if pos < len(s) {
// 			b.WriteString(s[pos:])
// 		}
// 		return b.String()
// 	}
// 	return Map(unicode.ToLower, s)
// }

// // ToTitle returns a copy of the string s with all Unicode letters mapped to
// // their Unicode title case.
// fn ToTitle(s string) string { return Map(unicode.ToTitle, s) }

// // ToUpperSpecial returns a copy of the string s with all Unicode letters mapped to their
// // upper case using the case mapping specified by c.
// fn ToUpperSpecial(c unicode.SpecialCase, s string) string {
// 	return Map(c.ToUpper, s)
// }

// // ToLowerSpecial returns a copy of the string s with all Unicode letters mapped to their
// // lower case using the case mapping specified by c.
// fn ToLowerSpecial(c unicode.SpecialCase, s string) string {
// 	return Map(c.ToLower, s)
// }

// // ToTitleSpecial returns a copy of the string s with all Unicode letters mapped to their
// // Unicode title case, giving priority to the special casing rules.
// fn ToTitleSpecial(c unicode.SpecialCase, s string) string {
// 	return Map(c.ToTitle, s)
// }

// // ToValidUTF8 returns a copy of the string s with each run of invalid UTF-8 byte sequences
// // replaced by the replacement string, which may be empty.
// fn ToValidUTF8(s, replacement string) string {
// 	var b Builder

// 	for i, c := range s {
// 		if c != utf8.RUNE_ERROR {
// 			continue
// 		}

// 		_, wid := utf8.decode_rune_in_string(s[i:])
// 		if wid == 1 {
// 			b.Grow(len(s) + len(replacement))
// 			b.WriteString(s[:i])
// 			s = s[i:]
// 			break
// 		}
// 	}

// 	// Fast path for unchanged input
// 	if b.Cap() == 0 { // didn't call b.Grow above
// 		return s
// 	}

// 	invalid := false // previous byte was from an invalid UTF-8 sequence
// 	for i := 0; i < len(s); {
// 		c := s[i]
// 		if c < utf8::RUNE_SELF {
// 			i++
// 			invalid = false
// 			b.WriteByte(c)
// 			continue
// 		}
// 		_, wid := utf8.decode_rune_in_string(s[i:])
// 		if wid == 1 {
// 			i++
// 			if !invalid {
// 				invalid = true
// 				b.WriteString(replacement)
// 			}
// 			continue
// 		}
// 		invalid = false
// 		b.WriteString(s[i : i+wid])
// 		i += wid
// 	}

// 	return b.String()
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

// // Title returns a copy of the string s with all Unicode letters that begin words
// // mapped to their Unicode title case.
// //
// // Deprecated: The rule Title uses for word boundaries does not handle Unicode
// // punctuation properly. Use golang.org/x/text/cases instead.
// fn Title(s string) string {
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

// // TrimLeftFunc returns a slice of the string s with all leading
// // Unicode code points c satisfying f(c) removed.
// fn TrimLeftFunc(s string, f fn(rune) bool) string {
// 	i := indexFunc(s, f, false)
// 	if i == -1 {
// 		return ""
// 	}
// 	return s[i:]
// }

// // TrimRightFunc returns a slice of the string s with all trailing
// // Unicode code points c satisfying f(c) removed.
// fn TrimRightFunc(s string, f fn(rune) bool) string {
// 	i := lastIndexFunc(s, f, false)
// 	if i >= 0 && s[i] >= utf8::RUNE_SELF {
// 		_, wid := utf8.decode_rune_in_string(s[i:])
// 		i += wid
// 	} else {
// 		i++
// 	}
// 	return s[0:i]
// }

// // TrimFunc returns a slice of the string s with all leading
// // and trailing Unicode code points c satisfying f(c) removed.
// fn TrimFunc(s string, f fn(rune) bool) string {
// 	return TrimRightFunc(TrimLeftFunc(s, f), f)
// }

// // IndexFunc returns the index into s of the first Unicode
// // code point satisfying f(c), or -1 if none do.
// fn IndexFunc(s string, f fn(rune) bool) int {
// 	return indexFunc(s, f, true)
// }

// // LastIndexFunc returns the index into s of the last
// // Unicode code point satisfying f(c), or -1 if none do.
// fn LastIndexFunc(s string, f fn(rune) bool) int {
// 	return lastIndexFunc(s, f, true)
// }

// // indexFunc is the same as IndexFunc except that if
// // truth==false, the sense of the predicate function is
// // inverted.
// fn indexFunc(s string, f fn(rune) bool, truth bool) int {
// 	for i, r := range s {
// 		if f(r) == truth {
// 			return i
// 		}
// 	}
// 	return -1
// }

// // lastIndexFunc is the same as LastIndexFunc except that if
// // truth==false, the sense of the predicate function is
// // inverted.
// fn lastIndexFunc(s string, f fn(rune) bool, truth bool) int {
// 	for i := len(s); i > 0; {
// 		r, size := utf8.DecodeLastRuneInString(s[0:i])
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
// 	for i := 0; i < len(chars); i++ {
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

// // Trim returns a slice of the string s with all leading and
// // trailing Unicode code points contained in cutset removed.
// fn Trim(s, cutset string) string {
// 	if s == "" || cutset == "" {
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

// // TrimLeft returns a slice of the string s with all leading
// // Unicode code points contained in cutset removed.
// //
// // To remove a prefix, use TrimPrefix instead.
// fn TrimLeft(s, cutset string) string {
// 	if s == "" || cutset == "" {
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

// fn trimLeftByte(s string, c byte) string {
// 	for len(s) > 0 && s[0] == c {
// 		s = s[1:]
// 	}
// 	return s
// }

// fn trimLeftASCII(s string, as *asciiSet) string {
// 	for len(s) > 0 {
// 		if !as.contains(s[0]) {
// 			break
// 		}
// 		s = s[1:]
// 	}
// 	return s
// }

// fn trimLeftUnicode(s, cutset string) string {
// 	for len(s) > 0 {
// 		r, n := rune(s[0]), 1
// 		if r >= utf8::RUNE_SELF {
// 			r, n = utf8.decode_rune_in_string(s)
// 		}
// 		if !ContainsRune(cutset, r) {
// 			break
// 		}
// 		s = s[n:]
// 	}
// 	return s
// }

// // TrimRight returns a slice of the string s, with all trailing
// // Unicode code points contained in cutset removed.
// //
// // To remove a suffix, use TrimSuffix instead.
// fn TrimRight(s, cutset string) string {
// 	if s == "" || cutset == "" {
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

// fn trimRightByte(s string, c byte) string {
// 	for len(s) > 0 && s[len(s)-1] == c {
// 		s = s[:len(s)-1]
// 	}
// 	return s
// }

// fn trimRightASCII(s string, as *asciiSet) string {
// 	for len(s) > 0 {
// 		if !as.contains(s[len(s)-1]) {
// 			break
// 		}
// 		s = s[:len(s)-1]
// 	}
// 	return s
// }

// fn trimRightUnicode(s, cutset string) string {
// 	for len(s) > 0 {
// 		r, n := rune(s[len(s)-1]), 1
// 		if r >= utf8::RUNE_SELF {
// 			r, n = utf8.DecodeLastRuneInString(s)
// 		}
// 		if !ContainsRune(cutset, r) {
// 			break
// 		}
// 		s = s[:len(s)-n]
// 	}
// 	return s
// }

// // TrimSpace returns a slice of the string s, with all leading
// // and trailing white space removed, as defined by Unicode.
// fn TrimSpace(s string) string {
// 	// Fast path for ASCII: look for the first ASCII non-space byte
// 	start := 0
// 	for ; start < len(s); start++ {
// 		c := s[start]
// 		if c >= utf8::RUNE_SELF {
// 			// If we run into a non-ASCII byte, fall back to the
// 			// slower unicode-aware method on the remaining bytes
// 			return TrimFunc(s[start:], unicode.IsSpace)
// 		}
// 		if asciiSpace[c] == 0 {
// 			break
// 		}
// 	}

// 	// Now look for the first ASCII non-space byte from the end
// 	stop := len(s)
// 	for ; stop > start; stop-- {
// 		c := s[stop-1]
// 		if c >= utf8::RUNE_SELF {
// 			// start has been already trimmed above, should trim end only
// 			return TrimRightFunc(s[start:stop], unicode.IsSpace)
// 		}
// 		if asciiSpace[c] == 0 {
// 			break
// 		}
// 	}

// 	// At this point s[start:stop] starts and ends with an ASCII
// 	// non-space bytes, so we're done. Non-ASCII cases have already
// 	// been handled above.
// 	return s[start:stop]
// }

// // TrimPrefix returns s without the provided leading prefix string.
// // If s doesn't start with prefix, s is returned unchanged.
// fn TrimPrefix(s, prefix string) string {
// 	if HasPrefix(s, prefix) {
// 		return s[len(prefix):]
// 	}
// 	return s
// }

// // TrimSuffix returns s without the provided trailing suffix string.
// // If s doesn't end with suffix, s is returned unchanged.
// fn TrimSuffix(s: &str, suffix: &str) string {
// 	if has_suffix(s, suffix) {
// 		return s[:len(s)-len(suffix)]
// 	}
// 	return s
// }

// // Replace returns a copy of the string s with the first n
// // non-overlapping instances of old replaced by new.
// // If old is empty, it matches at the beginning of the string
// // and after each UTF-8 sequence, yielding up to k+1 replacements
// // for a k-rune string.
// // If n < 0, there is no limit on the number of replacements.
// fn Replace(s, old, new string, n int) string {
// 	if old == new || n == 0 {
// 		return s // avoid allocation
// 	}

// 	// Compute number of replacements.
// 	if m := Count(s, old); m == 0 {
// 		return s // avoid allocation
// 	} else if n < 0 || m < n {
// 		n = m
// 	}

// 	// Apply replacements to buffer.
// 	var b Builder
// 	b.Grow(len(s) + n*(len(new)-len(old)))
// 	start := 0
// 	for i := 0; i < n; i++ {
// 		j := start
// 		if len(old) == 0 {
// 			if i > 0 {
// 				_, wid := utf8.decode_rune_in_string(s[start:])
// 				j += wid
// 			}
// 		} else {
// 			j += index(s[start:], old)
// 		}
// 		b.WriteString(s[start:j])
// 		b.WriteString(new)
// 		start = j + len(old)
// 	}
// 	b.WriteString(s[start:])
// 	return b.String()
// }

/// ReplaceAll returns a copy of the string s with all
/// non-overlapping instances of old replaced by new.
/// If old is empty, it matches at the beginning of the string
/// and after each UTF-8 sequence, yielding up to k+1 replacements
/// for a k-rune string.
pub fn replace_all(s: &str, old: &str, new: &str) -> String {
    s.replace(old, new)
}

// // EqualFold reports whether s and t, interpreted as UTF-8 strings,
// // are equal under simple Unicode case-folding, which is a more general
// // form of case-insensitivity.
// fn EqualFold(s, t string) -> bool {
// 	// ASCII fast path
// 	i := 0
// 	for ; i < len(s) && i < len(t); i++ {
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
// 	return len(s) == len(t)

// hasUnicode:
// 	s = s[i:]
// 	t = t[i:]
// 	for _, sr := range s {
// 		// If t is exhausted the strings are not equal.
// 		if len(t) == 0 {
// 			return false
// 		}

// 		// Extract first rune from second string.
// 		var tr rune
// 		if t[0] < utf8::RUNE_SELF {
// 			tr, t = rune(t[0]), t[1:]
// 		} else {
// 			r, size := utf8.decode_rune_in_string(t)
// 			tr, t = r, t[size:]
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

// 	// First string is empty, so check if the second one is also empty.
// 	return len(t) == 0
// }

/// index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
pub fn index(s: &str, substr: &str) -> isize {
    match s.find(substr) {
        Some(n) => n as isize,
        None => -1,
    }
    // 	n := len(substr)
    // 	switch {
    // 	case n == 0:
    // 		return 0
    // 	case n == 1:
    // 		return index_byte(s, substr[0])
    // 	case n == len(s):
    // 		if substr == s {
    // 			return 0
    // 		}
    // 		return -1
    // 	case n > len(s):
    // 		return -1
    // 	case n <= bytealg.MaxLen:
    // 		// Use brute force when s and substr both are small
    // 		if len(s) <= bytealg.MaxBruteForce {
    // 			return bytealg.IndexString(s, substr)
    // 		}
    // 		c0 := substr[0]
    // 		c1 := substr[1]
    // 		i := 0
    // 		t := len(s) - n + 1
    // 		fails := 0
    // 		for i < t {
    // 			if s[i] != c0 {
    // 				// index_byte is faster than bytealg.IndexString, so use it as long as
    // 				// we're not getting lots of false positives.
    // 				o := index_byte(s[i+1:t], c0)
    // 				if o < 0 {
    // 					return -1
    // 				}
    // 				i += o + 1
    // 			}
    // 			if s[i+1] == c1 && s[i:i+n] == substr {
    // 				return i
    // 			}
    // 			fails++
    // 			i++
    // 			// Switch to bytealg.IndexString when index_byte produces too many false positives.
    // 			if fails > bytealg.Cutover(i) {
    // 				r := bytealg.IndexString(s[i:], substr)
    // 				if r >= 0 {
    // 					return r + i
    // 				}
    // 				return -1
    // 			}
    // 		}
    // 		return -1
    // 	}
    // 	c0 := substr[0]
    // 	c1 := substr[1]
    // 	i := 0
    // 	t := len(s) - n + 1
    // 	fails := 0
    // 	for i < t {
    // 		if s[i] != c0 {
    // 			o := index_byte(s[i+1:t], c0)
    // 			if o < 0 {
    // 				return -1
    // 			}
    // 			i += o + 1
    // 		}
    // 		if s[i+1] == c1 && s[i:i+n] == substr {
    // 			return i
    // 		}
    // 		i++
    // 		fails++
    // 		if fails >= 4+i>>4 && i < t {
    // 			// See comment in ../bytes/bytes.go.
    // 			j := bytealg.IndexRabinKarp(s[i:], substr)
    // 			if j < 0 {
    // 				return -1
    // 			}
    // 			return i + j
    // 		}
    // 	}
    // 	return -1
}

/// Cut slices s around the first instance of sep,
/// returning the text before and after sep.
/// The found result reports whether sep appears in s.
/// If sep does not appear in s, cut returns s, "", false.
pub fn cut<'a>(s: &'a str, sep: &'_ str) -> (&'a str, &'a str, bool) {
    if let Some(index) = s.find(sep) {
        let before = &s[..index];
        let after = &s[index + sep.len()..];
        (before, after, true)
    } else {
        (s, "", false)
    }
}

// // CutPrefix returns s without the provided leading prefix string
// // and reports whether it found the prefix.
// // If s doesn't start with prefix, CutPrefix returns s, false.
// // If prefix is the empty string, CutPrefix returns s, true.
// fn CutPrefix(s, prefix string) (after string, found bool) {
// 	if !HasPrefix(s, prefix) {
// 		return s, false
// 	}
// 	return s[len(prefix):], true
// }

// // CutSuffix returns s without the provided ending suffix string
// // and reports whether it found the suffix.
// // If s doesn't end with suffix, CutSuffix returns s, false.
// // If suffix is the empty string, CutSuffix returns s, true.
// fn CutSuffix(s: &str, suffix: &str) (before string, found bool) {
// 	if !has_suffix(s, suffix) {
// 		return s, false
// 	}
// 	return s[:len(s)-len(suffix)], true
// }
