package main

import (
	"bytes"
	"compress/flate"
	"encoding/hex"
	"fmt"
	"log"
)

func main() {
	msg := "Hello World! Hello Rust! Hello World! Hello Rust!"
	var buffer bytes.Buffer
	w, err := flate.NewWriter(&buffer, flate.BestCompression)
	if err != nil {
		log.Fatal(err)
	}

	if _, err := w.Write([]byte(msg)); err != nil {
		log.Fatal(err)
	}

	if err := w.Close(); err != nil {
		log.Fatal(err)
	}

	result_str := hex.encode_to_string(buffer.Bytes())
	fmt.Printf(
		"input string    (%02d bytes): %s\n", len([]byte(msg)), msg)
	fmt.Printf(
		"deflated string (%02d bytes): %s\n", len([]byte(result_str))/2, result_str)
}
