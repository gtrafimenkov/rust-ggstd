package main

import (
	"bytes"
	"compress/flate"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"log"
	"os"
)

func main() {
	content, err := os.ReadFile("src/testdata/Isaac.Newton-Opticks.txt")
	if err != nil {
		log.Fatal(err)
	}

	var buffer bytes.Buffer
	w, err := flate.NewWriter(&buffer, flate.BestCompression)
	if err != nil {
		log.Fatal(err)
	}

	repeats := 200
	for i := 0; i < repeats; i++ {
		if _, err := w.Write(content); err != nil {
			log.Fatal(err)
		}
	}

	if err := w.Close(); err != nil {
		log.Fatal(err)
	}

	compressed_data := buffer.Bytes()
	hash := sha256.Sum256(compressed_data)
	hash_str := hex.encode_to_string(hash[:])
	fmt.Println("compressed data hash:")
	fmt.Println(hash_str)
}
