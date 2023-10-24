// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2021 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::libc_;
use std::sync::atomic;

//go:build dragonfly || freebsd || linux

static GETRANDOM_UNSUPPORTED: atomic::AtomicI32 = atomic::AtomicI32::new(0);

// GetRandomFlag is a flag supported by the getrandom system call.
type GetRandomFlag = usize;

/// GetRandom calls the getrandom system call.
pub fn get_random(p: &mut [u8], flags: GetRandomFlag) -> std::io::Result<usize> {
    if p.len() == 0 {
        return Ok(0);
    }
    if GETRANDOM_UNSUPPORTED.load(std::sync::atomic::Ordering::SeqCst) != 0 {
        // if atomic.LoadInt32(&getrandomUnsupported) != 0 {
        // 	return 0, syscall.ENOSYS
        return Err(std::io::Error::from(std::io::ErrorKind::Unsupported));
    }
    let r = unsafe {
        libc_::getrandom(
            p.as_mut_ptr() as *mut std::ffi::c_void,
            p.len(),
            flags as libc_::c_uint,
        )
    };
    if r < 0 {
        let errno = libc_::get_errno();
        if errno == libc_::ENOSYS {
            // syscall is not supported
            GETRANDOM_UNSUPPORTED.store(1, std::sync::atomic::Ordering::SeqCst);
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("errno: {}", errno),
        ));
    }
    Ok(r as usize)
}
