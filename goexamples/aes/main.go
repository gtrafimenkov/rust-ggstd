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
	key := []byte("0123456789abcdef")
	plaintext := []byte("hello world.....hello world.....hello world.....")

	fmt.Printf("plaintext:      %v\n", string(plaintext))
	aes_basic(key, plaintext)
	aes_cbc(key, plaintext)

	// in CTR mode plaintext doesn't have to be a multiple of the block size
	plaintext = []byte("hello world.....hello world.....hello worl")
	fmt.Printf("plaintext:      %v\n", string(plaintext))
	aes_ctr(key, plaintext)
}

func aes_basic(key []byte, plaintext []byte) {
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

	for offset := 0; offset < len(plaintext); offset += aes.BlockSize {
		block.Encrypt(ciphertext[offset:offset+aes.BlockSize], plaintext[offset:offset+aes.BlockSize])
	}

	// decrypted := make([]byte, len(plaintext))
	// for offset := 0; offset < len(plaintext); offset += aes.BlockSize {
	// 	block.Decrypt(decrypted[offset:offset+aes.BlockSize], ciphertext[offset:offset+aes.BlockSize])
	// }

	// fmt.Printf("plaintext:      %v\n", string(plaintext))
	fmt.Printf("ECB encrypted:  %x\n", ciphertext)
	// fmt.Printf("decrypted:      %v\n", string(decrypted))
	// fmt.Println()
}

func aes_cbc(key []byte, plaintext []byte) {
	block, err := aes.NewCipher(key)
	if err != nil {
		fmt.Println("Error creating AES cipher block:", err)
		return
	}

	if len(plaintext)%aes.BlockSize != 0 {
		fmt.Println("Plaintext length must be a multiple of the block size")
		return
	}

	encrypted := make([]byte, len(plaintext))

	// Using the initialization vector of all zeroes for demo purposes.
	// In practice the IV should be unique for each encryption operation and upredictable,
	// for example, generated using a secure random generator.
	iv := make([]byte, aes.BlockSize)

	mode := cipher.NewCBCEncrypter(block, iv)

	mode.CryptBlocks(encrypted, plaintext)

	// decrypted := make([]byte, len(plaintext))
	// mode = cipher.NewCBCDecrypter(block, iv)
	// mode.CryptBlocks(decrypted, ciphertext)

	// fmt.Printf("plaintext:      %v\n", string(plaintext))
	fmt.Printf("CBC encrypted:  %x\n", encrypted)
	// fmt.Printf("decrypted:      %v\n", string(decrypted))
	// fmt.Println()
}

func aes_ctr(key []byte, plaintext []byte) {
	block, err := aes.NewCipher(key)
	if err != nil {
		fmt.Println("Error creating AES cipher:", err)
		return
	}

	// Using the initialization vector of all zeroes for demo purposes.
	// In practice the IV should be unique for each encryption operation and upredictable,
	// for example, generated using a secure random generator.
	iv := make([]byte, 16)

	stream := cipher.NewCTR(block, iv)

	encrypted := make([]byte, len(plaintext))
	stream.XORKeyStream(encrypted, plaintext)

	fmt.Printf("CTR encrypted:  %x\n", encrypted)

	// decrypted := make([]byte, len(encrypted))
	// stream = cipher.NewCTR(block, iv)
	// stream.XORKeyStream(decrypted, encrypted)
	// fmt.Printf("decrypted:      %v\n", string(decrypted))
}
