// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2013 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	if len(os.Args) > 1 {
		for _, arg := range os.Args[1:] {
			ExampleNew_file(arg)
		}
	} else {
		ExampleNew()
		ExampleSum()
	}
}

func ExampleNew() {
	h := md5.New()
	io.WriteString(h, "The fog is getting thicker!")
	io.WriteString(h, "And Leon's getting laaarger!")
	fmt.Printf("%x\n", h.Sum(nil))
	// Output: e2c569be17396eca2a2e3c11578123ed
}

func ExampleSum() {
	data := []byte("These pretzels are making me thirsty.")
	fmt.Printf("%x\n", md5.Sum(data))
	// Output: b0804ec967f48520697662a204f5fe72
}

func ExampleNew_file(path string) {
	f, err := os.Open(path)
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	h := md5.New()
	if _, err := io.Copy(h, f); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("%x\n", h.Sum(nil))
}
