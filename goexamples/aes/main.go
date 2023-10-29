// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package main

import (
	"crypto/aes"
	"crypto/cipher"
	"fmt"
)

func main() {
	aes_basic()
	aes_cbc()
}

var key = []byte("0123456789abcdef")
var plaintext = []byte("hello world.....hello world.....")

func aes_basic() {
	block, err := aes.NewCipher(key)
	if err != nil {
		fmt.Println("Error creating AES cipher block:", err)
		return
	}

	if len(plaintext)%aes.BlockSize != 0 {
		fmt.Println("Plaintext length must be a multiple of the block size")
		return
	}

	ciphertext := make([]byte, len(plaintext))

	block.Encrypt(ciphertext, plaintext)
	block.Encrypt(ciphertext[aes.BlockSize:], plaintext[aes.BlockSize:])

	fmt.Printf("aes basic: %x\n", ciphertext)
}

func aes_cbc() {
	block, err := aes.NewCipher(key)
	if err != nil {
		fmt.Println("Error creating AES cipher block:", err)
		return
	}

	if len(plaintext)%aes.BlockSize != 0 {
		fmt.Println("Plaintext length must be a multiple of the block size")
		return
	}

	ciphertext := make([]byte, len(plaintext))

	iv := make([]byte, aes.BlockSize)

	mode := cipher.NewCBCEncrypter(block, iv)
	mode.CryptBlocks(ciphertext, plaintext)

	fmt.Printf("aes cbc:   %x\n", ciphertext)
}
