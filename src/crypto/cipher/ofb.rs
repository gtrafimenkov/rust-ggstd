// // Copyright 2011 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// // OFB (Output Feedback) Mode.

// package cipher

// import (
// 	"crypto/internal/alias"
// 	"crypto/subtle"
// )

// type ofb struct {
// 	b       Block
// 	cipher  []byte
// 	out     []byte
// 	outUsed int
// }

// // NewOFB returns a Stream that encrypts or decrypts using the block cipher b
// // in output feedback mode. The initialization vector iv's length must be equal
// // to b's block size.
// func NewOFB(b Block, iv []byte) Stream {
// 	blockSize := b.block_size()
// 	if len(iv) != blockSize {
// 		panic("cipher.NewOFB: IV length must equal block size")
// 	}
// 	bufSize := streamBufferSize
// 	if bufSize < blockSize {
// 		bufSize = blockSize
// 	}
// 	x := &ofb{
// 		b:       b,
// 		cipher:  make([]byte, blockSize),
// 		out:     make([]byte, 0, bufSize),
// 		outUsed: 0,
// 	}

// 	copy(x.cipher, iv)
// 	return x
// }

// func (x *ofb) refill() {
// 	bs := x.b.block_size()
// 	remain := len(x.out) - x.outUsed
// 	if remain > x.outUsed {
// 		return
// 	}
// 	copy(x.out, x.out[x.outUsed:])
// 	x.out = x.out[:cap(x.out)]
// 	for remain < len(x.out)-bs {
// 		x.b.encrypt(x.cipher, x.cipher)
// 		copy(x.out[remain:], x.cipher)
// 		remain += bs
// 	}
// 	x.out = x.out[:remain]
// 	x.outUsed = 0
// }

// func (x *ofb) xor_key_stream(dst, src []byte) {
// 	if dst.len() < src.len() {
// 		panic("crypto/cipher: output smaller than input")
// 	}
// 	if alias.InexactOverlap(dst[..src.len()], src) {
// 		panic("crypto/cipher: invalid buffer overlap")
// 	}
// 	for src.len() > 0 {
// 		if x.outUsed >= len(x.out)-x.b.block_size() {
// 			x.refill()
// 		}
// 		n := subtle.XORBytes(dst, src, x.out[x.outUsed:])
// 		dst = dst[n:]
// 		src = src[n:]
// 		x.outUsed += n
// 	}
// }