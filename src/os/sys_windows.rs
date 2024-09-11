// Copyright 2024 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// package os

use crate::winapi_;

use crate::syscall::utf16_to_string;

pub fn hostname() -> std::io::Result<String> {
    // Use PhysicalDnsHostname to uniquely identify host in a cluster
    let format = winapi_::ComputerNamePhysicalDnsHostname;

    let mut n: u32 = 64;
    loop {
        let mut b = vec![0_u16; n as usize];
        let res = unsafe { winapi_::GetComputerNameExW(format, b.as_mut_ptr(), &mut n) };
        if res != 0 {
            // success
            return Ok(utf16_to_string(&b));
        }

        let err = std::io::Error::last_os_error();
        if err.raw_os_error().unwrap() != winapi_::ERROR_MORE_DATA as i32 {
            // 			return "", NewSyscallError("ComputerNameEx", err)
            return Err(err);
        }

        // If we received an ERROR_MORE_DATA, but n doesn't get larger,
        // something has gone wrong and we may be in an infinite loop
        if n as usize <= b.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ComputerNameEx",
            ));
        }
    }
}
