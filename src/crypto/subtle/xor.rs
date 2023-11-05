// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2022 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// xor_bytes sets dst[i] = x[i] ^ y[i] for all i < n = min(len(x), len(y)),
/// returning n, the number of bytes written to dst.
/// If dst does not have length at least n,
/// XORBytes panics without writing anything to dst.
pub fn xor_bytes(dst: &mut [u8], x: &[u8], y: &[u8]) -> usize {
    let n = x.len().min(y.len());
    if n == 0 {
        return 0;
    }
    if n > dst.len() {
        panic!("subtle.XORBytes: dst too short")
    }
    // 	xorBytes(&dst[0], &x[0], &y[0], n) // arch-specific
    super::xor_generic::xor_loop(dst, x, y);
    n
}

/// xor_bytes_inplace sets dst[i] = dst[i] ^ y[i] for all i < n = min(len(dst), len(y)),
/// returning n, the number of bytes written to dst.
pub fn xor_bytes_inplace(dst: &mut [u8], y: &[u8]) -> usize {
    let n = dst.len().min(y.len());
    if n == 0 {
        return 0;
    }
    super::xor_generic::xor_loop_inplace(dst, y);
    n
}
