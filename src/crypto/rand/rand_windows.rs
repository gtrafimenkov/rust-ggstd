// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Windows cryptographically secure pseudorandom number
//! generator.

/// read_random reads the whole buffer of random data.
pub fn read_random(b: &mut [u8]) -> std::io::Result<()> {
    // RtlGenRandom only returns 1<<32-1 bytes at a time. We only read at
    // most 1<<31-1 bytes at a time so that  this works the same on 32-bit
    // and 64-bit systems.
    let read_max = 1 << 31 - 1;
    let mut out = b;
    while out.len() > 0 {
        let read = out.len().min(read_max);
        crate::runtime::os_windows::rtl_gen_random(&mut out[..read])?;
        out = &mut out[read..];
    }
    Ok(())
}
