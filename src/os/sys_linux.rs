// Copyright 2024 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fs::File;
use std::io::Read;

use crate::libc_;

pub fn hostname() -> std::io::Result<String> {
    // Try uname first, as it's only one system call and reading
    // from /proc is not allowed on Android.
    let mut un = unsafe { std::mem::zeroed::<libc_::utsname>() };
    let err = unsafe { libc_::uname(&mut un) };
    let nodename = unsafe { std::ffi::CStr::from_ptr(un.nodename.as_ptr()) };
    let nodename = nodename.to_string_lossy().to_string();
    if err == 0 && !nodename.is_empty() {
        return Ok(nodename);
    }

    // 	if runtime.GOOS == "android" {
    // 		if name != "" {
    // 			return name, nil
    // 		}
    // 		return "localhost", nil
    // 	}

    let mut f = File::open("/proc/sys/kernel/hostname")?;
    let mut buf: [u8; 512] = [0; 512];
    let mut n = f.read(&mut buf)?;
    if n > 0 && buf[n - 1] == b'\n' {
        n -= 1;
    }

    Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}
