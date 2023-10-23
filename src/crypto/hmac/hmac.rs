// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::compat;
use crate::crypto::subtle;
use crate::hash;
use std::cell::RefCell;

// FIPS 198-1:
// https://csrc.nist.gov/publications/fips/fips198-1/FIPS-198-1_final.pdf

// key is zero padded to the block size of the hash function
// ipad = 0x36 byte repeated for key length
// opad = 0x5c byte repeated for key length
// hmac = H([key ^ opad] H([key ^ ipad] text))

// // Marshalable is the combination of encoding.BinaryMarshaler and
// // encoding.BinaryUnmarshaler. Their method definitions are repeated here to
// // avoid a dependency on the encoding package.
// type marshalable interface {
// 	MarshalBinary() ([]byte, error)
// 	UnmarshalBinary([]byte) error
// }

/// HMAC implements the Keyed-Hash Message Authentication Code.
pub struct HMAC<H: hash::Hash> {
    opad: Vec<u8>,
    ipad: Vec<u8>,
    outer: RefCell<H>, // interior mutability to make HMAC compatible with hash::Hash
    inner: H,
    // 	// If marshaled is true, then opad and ipad do not contain a padded
    // 	// copy of the key, but rather the marshaled state of outer/inner after
    // 	// opad/ipad has been fed into it.
    // 	marshaled bool
}

impl<H: hash::Hash> hash::Hash for HMAC<H> {
    fn sum(&self, b: &[u8]) -> Vec<u8> {
        let orig_len = b.len();
        let b = self.inner.sum(b);

        let mut outer = self.outer.borrow_mut();
        // 	if self.marshaled {
        // 		if err := self.outer.(marshalable).UnmarshalBinary(self.opad); err != nil {
        // 			panic(err)
        // 		}
        // 	} else {
        outer.reset();
        _ = outer.write(&self.opad);
        // 	}
        _ = outer.write(&b[orig_len..]);
        outer.sum(&b[..orig_len])
    }

    fn size(&self) -> usize {
        self.inner.size()
    }

    fn block_size(&self) -> usize {
        self.inner.block_size()
    }

    fn reset(&mut self) {
        // 	if self.marshaled {
        // 		if err := self.inner.(marshalable).UnmarshalBinary(self.ipad); err != nil {
        // 			panic(err)
        // 		}
        // 		return
        // 	}

        self.inner.reset();
        _ = self.inner.write_all(&self.ipad);

        // 	// If the underlying hash is marshalable, we can save some time by
        // 	// saving a copy of the hash state now, and restoring it on future
        // 	// calls to Reset and Sum instead of writing ipad/opad every time.
        // 	//
        // 	// If either hash is unmarshalable for whatever reason,
        // 	// it's safe to bail out here.
        // 	marshalableInner, innerOK := self.inner.(marshalable)
        // 	if !innerOK {
        // 		return
        // 	}
        // 	marshalableOuter, outerOK := self.outer.(marshalable)
        // 	if !outerOK {
        // 		return
        // 	}

        // 	imarshal, err := marshalableInner.MarshalBinary()
        // 	if err != nil {
        // 		return
        // 	}

        // 	self.outer.Reset()
        // 	self.outer.Write(self.opad)
        // 	omarshal, err := marshalableOuter.MarshalBinary()
        // 	if err != nil {
        // 		return
        // 	}

        // 	// Marshaling succeeded; save the marshaled state for later
        // 	self.ipad = imarshal
        // 	self.opad = omarshal
        // 	self.marshaled = true
    }
}

impl<H: hash::Hash> HMAC<H> {
    /// new returns a new HMAC hash using the given hash.Hash type and key.
    /// new functions like `Digest::new` from `crypto::sha256` can be used as h.
    /// h must return a new Hash every time it is called.
    // Note that unlike other hash implementations in the standard library,
    // the returned Hash does not implement encoding.BinaryMarshaler
    // or encoding.BinaryUnmarshaler.
    pub fn new(h: fn() -> H, key: &[u8]) -> Self {
        // 	if boring.Enabled {
        // 		hm := boring.NewHMAC(h, key)
        // 		if hm != nil {
        // 			return hm
        // 		}
        // 		// BoringCrypto did not recognize h, so fall through to standard Go code.
        // 	}
        let mut outer = h();
        let mut inner = h();

        // 	unique := true
        // 	fn() {
        // 		defer fn() {
        // 			// The comparison might panic if the underlying types are not comparable.
        // 			_ = recover()
        // 		}()
        // 		if hm.outer == hm.inner {
        // 			unique = false
        // 		}
        // 	}()
        // 	if !unique {
        // 		panic("crypto/hmac: hash generation function does not produce unique values")
        // 	}
        let blocksize = inner.block_size();
        let mut ipad = vec![0; blocksize];
        let mut opad = vec![0; blocksize];
        if key.len() > blocksize {
            // If key is too big, hash it.
            _ = outer.write(key);
            let key = outer.sum(&[]);
            compat::copy(&mut ipad, &key);
            compat::copy(&mut opad, &key);
        } else {
            compat::copy(&mut ipad, key);
            compat::copy(&mut opad, key);
        }
        for i in 0..blocksize {
            ipad[i] ^= 0x36;
            opad[i] ^= 0x5c;
        }
        _ = inner.write(&ipad);
        Self {
            opad,
            ipad,
            outer: RefCell::new(outer),
            inner,
        }
    }
}

impl<H: hash::Hash> std::io::Write for HMAC<H> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// equal compares two MACs for equality without leaking timing information.
pub fn equal(mac1: &[u8], mac2: &[u8]) -> bool {
    // We don't have to be constant time if the lengths of the MACs are
    // different as that suggests that a completely different hash function
    // was used.
    subtle::constant_time_compare(mac1, mac2) == 1
}
