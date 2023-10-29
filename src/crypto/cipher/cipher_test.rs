// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2013 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::crypto::aes;
use crate::crypto::cipher::{self, Block, BlockMode};

// import (
// 	"bytes"
// 	"crypto/aes"
// 	"crypto/cipher"
// 	"crypto/des"
// 	"testing"
// )

// #[test]
// fn TestCryptBlocks() {
// 	buf := make([]byte, 16)
// 	block, _ := aes.NewCipher(buf)

// 	mode := cipher.NewCBCDecrypter(block, buf)
// 	mustPanic(t, "crypto/cipher: input not full blocks", fn() { mode.crypt_blocks(buf, buf[..3]) })
// 	mustPanic(t, "crypto/cipher: output smaller than input", fn() { mode.crypt_blocks(buf[..3], buf) })

// 	mode = cipher.NewCBCEncrypter(block, buf)
// 	mustPanic(t, "crypto/cipher: input not full blocks", fn() { mode.crypt_blocks(buf, buf[..3]) })
// 	mustPanic(t, "crypto/cipher: output smaller than input", fn() { mode.crypt_blocks(buf[..3], buf) })
// }

// #[test]
// fn mustPanic(, msg string, f fn()) {
// 	defer fn() {
// 		err := recover()
// 		if err == nil {
// 			t.Errorf("function did not panic, wanted %q", msg)
// 		} else if err != msg {
// 			t.Errorf("got panic %v, wanted %q", err, msg)
// 		}
// 	}()
// 	f()
// }

#[test]
fn test_empty_plaintext() {
    let key = [0; 16];
    let a = aes::Cipher::new(&key[..16]).unwrap();
    // 	d, err := des.NewCipher(key[..8])
    // 	if err != nil {
    // 		t.Fatal(err)
    // 	}

    let s = 16;
    let mut pt = vec![0; s];
    let mut ct = vec![0; s];
    for i in 0..s {
        (pt[i], ct[i]) = (i as u8, i as u8);
    }

    // 	assertEqual := fn(name string, got, want []byte) {
    // 		if !bytes.Equal(got, want) {
    // 			t.Fatalf("%s: got %v, want %v", name, got, want)
    // 		}
    // 	}

    let ciphers = &[
        a,
        // d
    ];
    for b in ciphers {
        let iv = vec![0; b.block_size()];
        let mut cbce = cipher::CBCEncrypter::new(b, &iv);
        cbce.crypt_blocks(&mut ct, &pt[..0]);
        assert_eq!(&ct, &pt, "CBC encrypt");

        let mut cbcd = cipher::CBCDecrypter::new(b, &iv);
        cbcd.crypt_blocks(&mut ct, &pt[..0]);
        assert_eq!(&ct, &pt, "CBC decrypt");

        // 		cfbe := cipher.NewCFBEncrypter(b, iv)
        // 		cfbe.xor_key_stream(ct, pt[..0])
        // assert_eq!(&ct, &pt, "CFB encrypt");

        // 		cfbd := cipher.NewCFBDecrypter(b, iv)
        // 		cfbd.xor_key_stream(ct, pt[..0])
        // assert_eq!(&ct, &pt, "CFB decrypt");

        // 		ctr := cipher.NewCTR(b, iv)
        // 		ctr.xor_key_stream(ct, pt[..0])
        // assert_eq!(&ct, &pt, "CTR");

        // 		ofb := cipher.NewOFB(b, iv)
        // 		ofb.xor_key_stream(ct, pt[..0])
        // assert_eq!(&ct, &pt, "OFB");
    }
}
