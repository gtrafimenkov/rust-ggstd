package main

import (
	"crypto/hmac"
	"crypto/sha256"
	"fmt"
)

func main() {
	key := []byte("this is a key")
	message := "hello there"
	m := hmac.New(sha256.New, key)
	m.Write([]byte(message))
	mac := m.Sum(nil)
	fmt.Printf("%x\n", mac)

	// Output:
	// 3f2b8ac8dc51f89e769930d8880312b639c1aa26c18a4c8bd3b3496a5de67540
}
