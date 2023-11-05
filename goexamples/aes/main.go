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

	ciphertext1 := make([]byte, len(plaintext))

	block.Encrypt(ciphertext1, plaintext)
	block.Encrypt(ciphertext1[aes.BlockSize:], plaintext[aes.BlockSize:])

	ciphertext2 := make([]byte, len(plaintext))
	block.Encrypt(ciphertext2, plaintext)
	block.Encrypt(ciphertext2[aes.BlockSize:], plaintext[aes.BlockSize:])

	decrypted1 := make([]byte, len(plaintext))
	block.Decrypt(decrypted1, ciphertext1)
	block.Decrypt(decrypted1[aes.BlockSize:], ciphertext1[aes.BlockSize:])

	decrypted2 := make([]byte, len(plaintext))
	block.Decrypt(decrypted2, ciphertext2)
	block.Decrypt(decrypted2[aes.BlockSize:], ciphertext2[aes.BlockSize:])

	fmt.Printf("plaintext:        %v\n", string(plaintext))
	fmt.Printf("aes basic 1:      %x\n", ciphertext1)
	fmt.Printf("aes basic 2:      %x\n", ciphertext2)
	fmt.Printf("decrypted 1:      %v\n", string(decrypted1))
	fmt.Printf("decrypted 2:      %v\n", string(decrypted1))
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

	iv := make([]byte, aes.BlockSize)
	mode := cipher.NewCBCEncrypter(block, iv)

	ciphertext1 := make([]byte, len(plaintext))
	mode.CryptBlocks(ciphertext1, plaintext)
	ciphertext2 := make([]byte, len(plaintext))
	mode.CryptBlocks(ciphertext2, plaintext)

	mode = cipher.NewCBCDecrypter(block, iv)

	decrypted1 := make([]byte, len(plaintext))
	mode.CryptBlocks(decrypted1, ciphertext1)
	decrypted2 := make([]byte, len(plaintext))
	mode.CryptBlocks(decrypted2, ciphertext2)

	fmt.Printf("plaintext:        %v\n", string(plaintext))
	fmt.Printf("aes cbc (iv=0) 1: %x\n", ciphertext1)
	fmt.Printf("aes cbc (iv=0) 2: %x\n", ciphertext2)
	fmt.Printf("decrypted 1:      %v\n", string(decrypted1))
	fmt.Printf("decrypted 2:      %v\n", string(decrypted2))
}
