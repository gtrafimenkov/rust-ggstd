// // Copyright 2011 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package bytes_test

// import (
// 	"bytes"
// 	"encoding/base64"
// 	"fmt"
// 	"io"
// 	"os"
// 	"sort"
// 	"strconv"
// 	"unicode"
// )

// func ExampleBuffer() {
// 	var b bytes::Buffer::new() // A Buffer needs no initialization.
// 	b.Write([u8]("Hello "))
// 	fmt.Fprintf(&b, "world!")
// 	b.WriteTo(os.Stdout)
// 	// Output: Hello world!
// }

// func ExampleBuffer_reader() {
// 	// A Buffer can turn a string or a [u8] into an io.Reader.
// 	buf := bytes.NewBufferString("R29waGVycyBydWxlIQ==")
// 	dec := base64.NewDecoder(base64.StdEncoding, buf)
// 	io.Copy(os.Stdout, dec)
// 	// Output: Gophers rule!
// }

// func ExampleBuffer_Bytes() {
// 	buf := bytes::Buffer::new(){}
// 	buf.Write([u8]{'h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'})
// 	os.Stdout.Write(buf.bytes())
// 	// Output: hello world
// }

// func ExampleBuffer_AvailableBuffer() {
// 	let mut buf = bytes::Buffer::new();
// 	for i := 0; i < 4; i += 1 {
// 		b := buf.AvailableBuffer()
// 		b = strconv.AppendInt(b, int64(i), 10)
// 		b = append(b, ' ')
// 		buf.Write(b)
// 	}
// 	os.Stdout.Write(buf.bytes())
// 	// Output: 0 1 2 3
// }

// func ExampleBuffer_Cap() {
// 	buf1 := bytes::new_buffer(make([u8], 10))
// 	buf2 := bytes::new_buffer(make([u8], 0, 10))
// 	fmt.Println(buf1.Cap())
// 	fmt.Println(buf2.Cap())
// 	// Output:
// 	// 10
// 	// 10
// }

// func ExampleBuffer_Grow() {
// 	var b bytes::Buffer::new()
// 	b.Grow(64)
// 	bb := b.bytes()
// 	b.Write([u8]("64 bytes or fewer"))
// 	fmt.Printf("%q", bb[..b.Len()])
// 	// Output: "64 bytes or fewer"
// }

// func ExampleBuffer_Len() {
// 	var b bytes::Buffer::new()
// 	b.Grow(64)
// 	b.Write([u8]("abcde"))
// 	fmt.Printf("{}", b.Len())
// 	// Output: 5
// }

// func ExampleBuffer_Next() {
// 	var b bytes::Buffer::new()
// 	b.Grow(64)
// 	b.Write([u8]("abcde"))
// 	fmt.Printf("%s\n", string(b.Next(2)))
// 	fmt.Printf("%s\n", string(b.Next(2)))
// 	fmt.Printf("%s", string(b.Next(2)))
// 	// Output:
// 	// ab
// 	// cd
// 	// e
// }

// func ExampleBuffer_Read() {
// 	var b bytes::Buffer::new()
// 	b.Grow(64)
// 	b.Write([u8]("abcde"))
// 	rdbuf := make([u8], 1)
// 	n, err := b.Read(rdbuf)
// 	if err != nil {
// 		panic(err)
// 	}
// 	fmt.Println(n)
// 	fmt.Println(b.String())
// 	fmt.Println(string(rdbuf))
// 	// Output
// 	// 1
// 	// bcde
// 	// a
// }

// func ExampleBuffer_ReadByte() {
// 	var b bytes::Buffer::new()
// 	b.Grow(64)
// 	b.Write([u8]("abcde"))
// 	c, err := b.ReadByte()
// 	if err != nil {
// 		panic(err)
// 	}
// 	fmt.Println(c)
// 	fmt.Println(b.String())
// 	// Output
// 	// 97
// 	// bcde
// }

// func ExampleClone() {
// 	b := [u8]("abc")
// 	clone := bytes.Clone(b)
// 	fmt.Printf("%s\n", clone)
// 	clone[0] = 'd'
// 	fmt.Printf("%s\n", b)
// 	fmt.Printf("%s\n", clone)
// 	// Output:
// 	// abc
// 	// abc
// 	// dbc
// }

// func ExampleCompare() {
// 	// Interpret Compare's result by comparing it to zero.
// 	var a, b [u8]
// 	if bytes.Compare(a, b) < 0 {
// 		// a less b
// 	}
// 	if bytes.Compare(a, b) <= 0 {
// 		// a less or equal b
// 	}
// 	if bytes.Compare(a, b) > 0 {
// 		// a greater b
// 	}
// 	if bytes.Compare(a, b) >= 0 {
// 		// a greater or equal b
// 	}

// 	// Prefer Equal to Compare for equality comparisons.
// 	if bytes::equal(a, b) {
// 		// a equal b
// 	}
// 	if !bytes::equal(a, b) {
// 		// a not equal b
// 	}
// }

// func ExampleCompare_search() {
// 	// Binary search to find a matching byte slice.
// 	var needle [u8]
// 	var haystack [][u8] // Assume sorted
// 	i := sort.Search(len(haystack), func(i int) bool {
// 		// Return haystack[i] >= needle.
// 		return bytes.Compare(haystack[i], needle) >= 0
// 	})
// 	if i < len(haystack) && bytes::equal(haystack[i], needle) {
// 		// Found it!
// 	}
// }

// func ExampleContains() {
// 	fmt.Println(bytes.Contains([u8]("seafood"), [u8]("foo")))
// 	fmt.Println(bytes.Contains([u8]("seafood"), [u8]("bar")))
// 	fmt.Println(bytes.Contains([u8]("seafood"), [u8]("")))
// 	fmt.Println(bytes.Contains([u8](""), [u8]("")))
// 	// Output:
// 	// true
// 	// false
// 	// true
// 	// true
// }

// func ExampleContainsAny() {
// 	fmt.Println(bytes.ContainsAny([u8]("I like seafood."), "fÄo!"))
// 	fmt.Println(bytes.ContainsAny([u8]("I like seafood."), "去是伟大的."))
// 	fmt.Println(bytes.ContainsAny([u8]("I like seafood."), ""))
// 	fmt.Println(bytes.ContainsAny([u8](""), ""))
// 	// Output:
// 	// true
// 	// true
// 	// false
// 	// false
// }

// func ExampleContainsRune() {
// 	fmt.Println(bytes.ContainsRune([u8]("I like seafood."), 'f'))
// 	fmt.Println(bytes.ContainsRune([u8]("I like seafood."), 'ö'))
// 	fmt.Println(bytes.ContainsRune([u8]("去是伟大的!"), '大'))
// 	fmt.Println(bytes.ContainsRune([u8]("去是伟大的!"), '!'))
// 	fmt.Println(bytes.ContainsRune([u8](""), '@'))
// 	// Output:
// 	// true
// 	// false
// 	// true
// 	// true
// 	// false
// }

// func ExampleCount() {
// 	fmt.Println(bytes.Count([u8]("cheese"), [u8]("e")))
// 	fmt.Println(bytes.Count([u8]("five"), [u8](""))) // before & after each rune
// 	// Output:
// 	// 3
// 	// 5
// }

// func ExampleCut() {
// 	show := func(s, sep string) {
// 		before, after, found := bytes.Cut([u8](s), [u8](sep))
// 		fmt.Printf("Cut(%q, %q) = %q, %q, {}\n", s, sep, before, after, found)
// 	}
// 	show("Gopher", "Go")
// 	show("Gopher", "ph")
// 	show("Gopher", "er")
// 	show("Gopher", "Badger")
// 	// Output:
// 	// Cut("Gopher", "Go") = "", "pher", true
// 	// Cut("Gopher", "ph") = "Go", "er", true
// 	// Cut("Gopher", "er") = "Goph", "", true
// 	// Cut("Gopher", "Badger") = "Gopher", "", false
// }

// func ExampleCutPrefix() {
// 	show := func(s, sep string) {
// 		after, found := bytes.CutPrefix([u8](s), [u8](sep))
// 		fmt.Printf("CutPrefix(%q, %q) = %q, {}\n", s, sep, after, found)
// 	}
// 	show("Gopher", "Go")
// 	show("Gopher", "ph")
// 	// Output:
// 	// CutPrefix("Gopher", "Go") = "pher", true
// 	// CutPrefix("Gopher", "ph") = "Gopher", false
// }

// func ExampleCutSuffix() {
// 	show := func(s, sep string) {
// 		before, found := bytes.CutSuffix([u8](s), [u8](sep))
// 		fmt.Printf("CutSuffix(%q, %q) = %q, {}\n", s, sep, before, found)
// 	}
// 	show("Gopher", "Go")
// 	show("Gopher", "er")
// 	// Output:
// 	// CutSuffix("Gopher", "Go") = "Gopher", false
// 	// CutSuffix("Gopher", "er") = "Goph", true
// }

// func ExampleEqual() {
// 	fmt.Println(bytes::equal([u8]("Go"), [u8]("Go")))
// 	fmt.Println(bytes::equal([u8]("Go"), [u8]("C++")))
// 	// Output:
// 	// true
// 	// false
// }

// func ExampleEqualFold() {
// 	fmt.Println(bytes.EqualFold([u8]("Go"), [u8]("go")))
// 	// Output: true
// }

// func ExampleFields() {
// 	fmt.Printf("Fields are: %q", bytes.Fields([u8]("  foo bar  baz   ")))
// 	// Output: Fields are: ["foo" "bar" "baz"]
// }

// func ExampleFieldsFunc() {
// 	f := func(c rune) bool {
// 		return !unicode.IsLetter(c) && !unicode.IsNumber(c)
// 	}
// 	fmt.Printf("Fields are: %q", bytes.FieldsFunc([u8]("  foo1;bar2,baz3..."), f))
// 	// Output: Fields are: ["foo1" "bar2" "baz3"]
// }

// func ExampleHasPrefix() {
// 	fmt.Println(bytes.HasPrefix([u8]("Gopher"), [u8]("Go")))
// 	fmt.Println(bytes.HasPrefix([u8]("Gopher"), [u8]("C")))
// 	fmt.Println(bytes.HasPrefix([u8]("Gopher"), [u8]("")))
// 	// Output:
// 	// true
// 	// false
// 	// true
// }

// func ExampleHasSuffix() {
// 	fmt.Println(bytes::has_suffix([u8]("Amigo"), [u8]("go")))
// 	fmt.Println(bytes::has_suffix([u8]("Amigo"), [u8]("O")))
// 	fmt.Println(bytes::has_suffix([u8]("Amigo"), [u8]("Ami")))
// 	fmt.Println(bytes::has_suffix([u8]("Amigo"), [u8]("")))
// 	// Output:
// 	// true
// 	// false
// 	// false
// 	// true
// }

// func ExampleIndex() {
// 	fmt.Println(bytes.Index([u8]("chicken"), [u8]("ken")))
// 	fmt.Println(bytes.Index([u8]("chicken"), [u8]("dmr")))
// 	// Output:
// 	// 4
// 	// -1
// }

// func ExampleIndexByte() {
// 	fmt.Println(bytes.IndexByte([u8]("chicken"), byte('k')))
// 	fmt.Println(bytes.IndexByte([u8]("chicken"), byte('g')))
// 	// Output:
// 	// 4
// 	// -1
// }

// func ExampleIndexFunc() {
// 	f := func(c rune) bool {
// 		return unicode.Is(unicode.Han, c)
// 	}
// 	fmt.Println(bytes.IndexFunc([u8]("Hello, 世界"), f))
// 	fmt.Println(bytes.IndexFunc([u8]("Hello, world"), f))
// 	// Output:
// 	// 7
// 	// -1
// }

// func ExampleIndexAny() {
// 	fmt.Println(bytes.IndexAny([u8]("chicken"), "aeiouy"))
// 	fmt.Println(bytes.IndexAny([u8]("crwth"), "aeiouy"))
// 	// Output:
// 	// 2
// 	// -1
// }

// func ExampleIndexRune() {
// 	fmt.Println(bytes.IndexRune([u8]("chicken"), 'k'))
// 	fmt.Println(bytes.IndexRune([u8]("chicken"), 'd'))
// 	// Output:
// 	// 4
// 	// -1
// }

// func ExampleJoin() {
// 	s := [][u8]{[u8]("foo"), [u8]("bar"), [u8]("baz")}
// 	fmt.Printf("%s", bytes.Join(s, [u8](", ")))
// 	// Output: foo, bar, baz
// }

// func ExampleLastIndex() {
// 	fmt.Println(bytes.Index([u8]("go gopher"), [u8]("go")))
// 	fmt.Println(bytes.LastIndex([u8]("go gopher"), [u8]("go")))
// 	fmt.Println(bytes.LastIndex([u8]("go gopher"), [u8]("rodent")))
// 	// Output:
// 	// 0
// 	// 3
// 	// -1
// }

// func ExampleLastIndexAny() {
// 	fmt.Println(bytes.LastIndexAny([u8]("go gopher"), "MüQp"))
// 	fmt.Println(bytes.LastIndexAny([u8]("go 地鼠"), "地大"))
// 	fmt.Println(bytes.LastIndexAny([u8]("go gopher"), "z,!."))
// 	// Output:
// 	// 5
// 	// 3
// 	// -1
// }

// func ExampleLastIndexByte() {
// 	fmt.Println(bytes.LastIndexByte([u8]("go gopher"), byte('g')))
// 	fmt.Println(bytes.LastIndexByte([u8]("go gopher"), byte('r')))
// 	fmt.Println(bytes.LastIndexByte([u8]("go gopher"), byte('z')))
// 	// Output:
// 	// 3
// 	// 8
// 	// -1
// }

// func ExampleLastIndexFunc() {
// 	fmt.Println(bytes.LastIndexFunc([u8]("go gopher!"), unicode.IsLetter))
// 	fmt.Println(bytes.LastIndexFunc([u8]("go gopher!"), unicode.IsPunct))
// 	fmt.Println(bytes.LastIndexFunc([u8]("go gopher!"), unicode.IsNumber))
// 	// Output:
// 	// 8
// 	// 9
// 	// -1
// }

// func ExampleMap() {
// 	rot13 := func(r rune) rune {
// 		switch {
// 		case r >= 'A' && r <= 'Z':
// 			return 'A' + (r-'A'+13)%26
// 		case r >= 'a' && r <= 'z':
// 			return 'a' + (r-'a'+13)%26
// 		}
// 		return r
// 	}
// 	fmt.Printf("%s\n", bytes.Map(rot13, [u8]("'Twas brillig and the slithy gopher...")))
// 	// Output:
// 	// 'Gjnf oevyyvt naq gur fyvgul tbcure...
// }

// func ExampleReader_Len() {
// 	fmt.Println(bytes::Reader::new([u8]("Hi!")).Len())
// 	fmt.Println(bytes::Reader::new([u8]("こんにちは!")).Len())
// 	// Output:
// 	// 3
// 	// 16
// }

// func ExampleRepeat() {
// 	fmt.Printf("ba%s", bytes.Repeat([u8]("na"), 2))
// 	// Output: banana
// }

// func ExampleReplace() {
// 	fmt.Printf("%s\n", bytes.Replace([u8]("oink oink oink"), [u8]("k"), [u8]("ky"), 2))
// 	fmt.Printf("%s\n", bytes.Replace([u8]("oink oink oink"), [u8]("oink"), [u8]("moo"), -1))
// 	// Output:
// 	// oinky oinky oink
// 	// moo moo moo
// }

// func ExampleReplaceAll() {
// 	fmt.Printf("%s\n", bytes.ReplaceAll([u8]("oink oink oink"), [u8]("oink"), [u8]("moo")))
// 	// Output:
// 	// moo moo moo
// }

// func ExampleRunes() {
// 	rs := bytes.Runes([u8]("go gopher"))
// 	for _, r := range rs {
// 		fmt.Printf("%#U\n", r)
// 	}
// 	// Output:
// 	// U+0067 'g'
// 	// U+006F 'o'
// 	// U+0020 ' '
// 	// U+0067 'g'
// 	// U+006F 'o'
// 	// U+0070 'p'
// 	// U+0068 'h'
// 	// U+0065 'e'
// 	// U+0072 'r'
// }

// func ExampleSplit() {
// 	fmt.Printf("%q\n", bytes.Split([u8]("a,b,c"), [u8](",")))
// 	fmt.Printf("%q\n", bytes.Split([u8]("a man a plan a canal panama"), [u8]("a ")))
// 	fmt.Printf("%q\n", bytes.Split([u8](" xyz "), [u8]("")))
// 	fmt.Printf("%q\n", bytes.Split([u8](""), [u8]("Bernardo O'Higgins")))
// 	// Output:
// 	// ["a" "b" "c"]
// 	// ["" "man " "plan " "canal panama"]
// 	// [" " "x" "y" "z" " "]
// 	// [""]
// }

// func ExampleSplitN() {
// 	fmt.Printf("%q\n", bytes.SplitN([u8]("a,b,c"), [u8](","), 2))
// 	z := bytes.SplitN([u8]("a,b,c"), [u8](","), 0)
// 	fmt.Printf("%q (nil = {})\n", z, z == nil)
// 	// Output:
// 	// ["a" "b,c"]
// 	// [] (nil = true)
// }

// func ExampleSplitAfter() {
// 	fmt.Printf("%q\n", bytes.SplitAfter([u8]("a,b,c"), [u8](",")))
// 	// Output: ["a," "b," "c"]
// }

// func ExampleSplitAfterN() {
// 	fmt.Printf("%q\n", bytes.SplitAfterN([u8]("a,b,c"), [u8](","), 2))
// 	// Output: ["a," "b,c"]
// }

// func ExampleTitle() {
// 	fmt.Printf("%s", bytes.Title([u8]("her royal highness")))
// 	// Output: Her Royal Highness
// }

// func ExampleToTitle() {
// 	fmt.Printf("%s\n", bytes.ToTitle([u8]("loud noises")))
// 	fmt.Printf("%s\n", bytes.ToTitle([u8]("хлеб")))
// 	// Output:
// 	// LOUD NOISES
// 	// ХЛЕБ
// }

// func ExampleToTitleSpecial() {
// 	str := [u8]("ahoj vývojári golang")
// 	totitle := bytes.ToTitleSpecial(unicode.AzeriCase, str)
// 	fmt.Println("Original : " + string(str))
// 	fmt.Println("ToTitle : " + string(totitle))
// 	// Output:
// 	// Original : ahoj vývojári golang
// 	// ToTitle : AHOJ VÝVOJÁRİ GOLANG
// }

// func ExampleToValidUTF8() {
// 	fmt.Printf("%s\n", bytes.ToValidUTF8([u8]("abc"), [u8]("\uFFFD")))
// 	fmt.Printf("%s\n", bytes.ToValidUTF8([u8]("a\xffb\xC0\xAFc\xff"), [u8]("")))
// 	fmt.Printf("%s\n", bytes.ToValidUTF8([u8]("\xed\xa0\x80"), [u8]("abc")))
// 	// Output:
// 	// abc
// 	// abc
// 	// abc
// }

// func ExampleTrim() {
// 	fmt.Printf("[%q]", bytes.Trim([u8](" !!! Achtung! Achtung! !!! "), "! "))
// 	// Output: ["Achtung! Achtung"]
// }

// func ExampleTrimFunc() {
// 	fmt.Println(string(bytes.TrimFunc([u8]("go-gopher!"), unicode.IsLetter)))
// 	fmt.Println(string(bytes.TrimFunc([u8]("\"go-gopher!\""), unicode.IsLetter)))
// 	fmt.Println(string(bytes.TrimFunc([u8]("go-gopher!"), unicode.IsPunct)))
// 	fmt.Println(string(bytes.TrimFunc([u8]("1234go-gopher!567"), unicode.IsNumber)))
// 	// Output:
// 	// -gopher!
// 	// "go-gopher!"
// 	// go-gopher
// 	// go-gopher!
// }

// func ExampleTrimLeft() {
// 	fmt.Print(string(bytes.TrimLeft([u8]("453gopher8257"), "0123456789")))
// 	// Output:
// 	// gopher8257
// }

// func ExampleTrimLeftFunc() {
// 	fmt.Println(string(bytes.TrimLeftFunc([u8]("go-gopher"), unicode.IsLetter)))
// 	fmt.Println(string(bytes.TrimLeftFunc([u8]("go-gopher!"), unicode.IsPunct)))
// 	fmt.Println(string(bytes.TrimLeftFunc([u8]("1234go-gopher!567"), unicode.IsNumber)))
// 	// Output:
// 	// -gopher
// 	// go-gopher!
// 	// go-gopher!567
// }

// func ExampleTrimPrefix() {
// 	var b = [u8]("Goodbye,, world!")
// 	b = bytes.TrimPrefix(b, [u8]("Goodbye,"))
// 	b = bytes.TrimPrefix(b, [u8]("See ya,"))
// 	fmt.Printf("Hello%s", b)
// 	// Output: Hello, world!
// }

// func ExampleTrimSpace() {
// 	fmt.Printf("%s", bytes.TrimSpace([u8](" \t\n a lone gopher \n\t\r\n")))
// 	// Output: a lone gopher
// }

// func ExampleTrimSuffix() {
// 	var b = [u8]("Hello, goodbye, etc!")
// 	b = bytes.TrimSuffix(b, [u8]("goodbye, etc!"))
// 	b = bytes.TrimSuffix(b, [u8]("gopher"))
// 	b = append(b, bytes.TrimSuffix([u8]("world!"), [u8]("x!"))...)
// 	os.Stdout.Write(b)
// 	// Output: Hello, world!
// }

// func ExampleTrimRight() {
// 	fmt.Print(string(bytes.TrimRight([u8]("453gopher8257"), "0123456789")))
// 	// Output:
// 	// 453gopher
// }

// func ExampleTrimRightFunc() {
// 	fmt.Println(string(bytes.TrimRightFunc([u8]("go-gopher"), unicode.IsLetter)))
// 	fmt.Println(string(bytes.TrimRightFunc([u8]("go-gopher!"), unicode.IsPunct)))
// 	fmt.Println(string(bytes.TrimRightFunc([u8]("1234go-gopher!567"), unicode.IsNumber)))
// 	// Output:
// 	// go-
// 	// go-gopher
// 	// 1234go-gopher!
// }

// func ExampleToLower() {
// 	fmt.Printf("%s", bytes.ToLower([u8]("Gopher")))
// 	// Output: gopher
// }

// func ExampleToLowerSpecial() {
// 	str := [u8]("AHOJ VÝVOJÁRİ GOLANG")
// 	totitle := bytes.ToLowerSpecial(unicode.AzeriCase, str)
// 	fmt.Println("Original : " + string(str))
// 	fmt.Println("ToLower : " + string(totitle))
// 	// Output:
// 	// Original : AHOJ VÝVOJÁRİ GOLANG
// 	// ToLower : ahoj vývojári golang
// }

// func ExampleToUpper() {
// 	fmt.Printf("%s", bytes.ToUpper([u8]("Gopher")))
// 	// Output: GOPHER
// }

// func ExampleToUpperSpecial() {
// 	str := [u8]("ahoj vývojári golang")
// 	totitle := bytes.ToUpperSpecial(unicode.AzeriCase, str)
// 	fmt.Println("Original : " + string(str))
// 	fmt.Println("ToUpper : " + string(totitle))
// 	// Output:
// 	// Original : ahoj vývojári golang
// 	// ToUpper : AHOJ VÝVOJÁRİ GOLANG
// }
