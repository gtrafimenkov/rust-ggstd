// // Copyright 2015 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package io_test

// import (
// 	"fmt"
// 	"io"
// 	"log"
// 	"os"
// 	"strings"
// )

// func ExampleCopy() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")

// 	if _, err := io.Copy(os.Stdout, r); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// some io.Reader stream to be read
// }

// func ExampleCopyBuffer() {
// 	r1 := strings::Reader::new("first reader\n")
// 	r2 := strings::Reader::new("second reader\n")
// 	buf := make([u8], 8)

// 	// buf is used here...
// 	if _, err := io.CopyBuffer(os.Stdout, r1, buf); err != nil {
// 		log.Fatal(err)
// 	}

// 	// ... reused here also. No need to allocate an extra buffer.
// 	if _, err := io.CopyBuffer(os.Stdout, r2, buf); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// first reader
// 	// second reader
// }

// func ExampleCopyN() {
// 	r := strings::Reader::new("some io.Reader stream to be read")

// 	if _, err := ggio::copy_n(os.Stdout, r, 4); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// some
// }

// func ExampleReadAtLeast() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")

// 	buf := make([u8], 14)
// 	if _, err := io.read_at_least(r, buf, 4); err != nil {
// 		log.Fatal(err)
// 	}
// 	fmt.Printf("%s\n", buf)

// 	// buffer smaller than minimal read size.
// 	shortBuf := make([u8], 3)
// 	if _, err := io.read_at_least(r, shortBuf, 4); err != nil {
// 		fmt.Println("error:", err)
// 	}

// 	// minimal read size bigger than io.Reader stream
// 	longBuf := make([u8], 64)
// 	if _, err := io.read_at_least(r, longBuf, 64); err != nil {
// 		fmt.Println("error:", err)
// 	}

// 	// Output:
// 	// some io.Reader
// 	// error: short buffer
// 	// error: unexpected EOF
// }

// func ExampleReadFull() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")

// 	buf := make([u8], 4)
// 	if _, err := io.read_full(r, buf); err != nil {
// 		log.Fatal(err)
// 	}
// 	fmt.Printf("%s\n", buf)

// 	// minimal read size bigger than io.Reader stream
// 	longBuf := make([u8], 64)
// 	if _, err := io.read_full(r, longBuf); err != nil {
// 		fmt.Println("error:", err)
// 	}

// 	// Output:
// 	// some
// 	// error: unexpected EOF
// }

// func ExampleWriteString() {
// 	if _, err := io.write_string(os.Stdout, "Hello World"); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output: Hello World
// }

// func ExampleLimitReader() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")
// 	lr := io.LimitReader(r, 4)

// 	if _, err := io.Copy(os.Stdout, lr); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// some
// }

// func ExampleMultiReader() {
// 	r1 := strings::Reader::new("first reader ")
// 	r2 := strings::Reader::new("second reader ")
// 	r3 := strings::Reader::new("third reader\n")
// 	r := io.MultiReader(r1, r2, r3)

// 	if _, err := io.Copy(os.Stdout, r); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// first reader second reader third reader
// }

// func ExampleTeeReader() {
// 	var r io.Reader = strings::Reader::new("some io.Reader stream to be read\n")

// 	r = io.TeeReader(r, os.Stdout)

// 	// Everything read from r will be copied to stdout.
// 	if _, err := ggio::read_all(r); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// some io.Reader stream to be read
// }

// func ExampleSectionReader() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")
// 	s := io.NewSectionReader(r, 5, 17)

// 	if _, err := io.Copy(os.Stdout, s); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// io.Reader stream
// }

// func ExampleSectionReader_Read() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")
// 	s := io.NewSectionReader(r, 5, 17)

// 	buf := make([u8], 9)
// 	if _, err := s.Read(buf); err != nil {
// 		log.Fatal(err)
// 	}

// 	fmt.Printf("%s\n", buf)

// 	// Output:
// 	// io.Reader
// }

// func ExampleSectionReader_ReadAt() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")
// 	s := io.NewSectionReader(r, 5, 17)

// 	buf := make([u8], 6)
// 	if _, err := s.ReadAt(buf, 10); err != nil {
// 		log.Fatal(err)
// 	}

// 	fmt.Printf("%s\n", buf)

// 	// Output:
// 	// stream
// }

// func ExampleSectionReader_Seek() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")
// 	s := io.NewSectionReader(r, 5, 17)

// 	if _, err := s.Seek(10, ggio::Seek::Start); err != nil {
// 		log.Fatal(err)
// 	}

// 	if _, err := io.Copy(os.Stdout, s); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// stream
// }

// func ExampleSectionReader_Size() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")
// 	s := io.NewSectionReader(r, 5, 17)

// 	fmt.Println(s.size())

// 	// Output:
// 	// 17
// }

// func ExampleSeeker_Seek() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")

// 	r.Seek(5, ggio::Seek::Start) // move to the 5th char from the start
// 	if _, err := io.Copy(os.Stdout, r); err != nil {
// 		log.Fatal(err)
// 	}

// 	r.Seek(-5, ggio::Seek::End)
// 	if _, err := io.Copy(os.Stdout, r); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// io.Reader stream to be read
// 	// read
// }

// func ExampleMultiWriter() {
// 	r := strings::Reader::new("some io.Reader stream to be read\n")

// 	var buf1, buf2 strings.Builder
// 	w := io.MultiWriter(&buf1, &buf2)

// 	if _, err := io.Copy(w, r); err != nil {
// 		log.Fatal(err)
// 	}

// 	fmt.Print(buf1.String())
// 	fmt.Print(buf2.String())

// 	// Output:
// 	// some io.Reader stream to be read
// 	// some io.Reader stream to be read
// }

// func ExamplePipe() {
// 	r, w := io.Pipe()

// 	go func() {
// 		fmt.Fprint(w, "some io.Reader stream to be read\n")
// 		w.close()
// 	}()

// 	if _, err := io.Copy(os.Stdout, r); err != nil {
// 		log.Fatal(err)
// 	}

// 	// Output:
// 	// some io.Reader stream to be read
// }

// func ExampleReadAll() {
// 	r := strings::Reader::new("Go is a general-purpose language designed with systems programming in mind.")

// 	b, err := ggio::read_all(r)
// 	if err != nil {
// 		log.Fatal(err)
// 	}

// 	fmt.Printf("%s", b)

// 	// Output:
// 	// Go is a general-purpose language designed with systems programming in mind.
// }
