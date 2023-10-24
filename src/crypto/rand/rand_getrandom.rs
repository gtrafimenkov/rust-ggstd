// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2014 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// //go:build linux || freebsd || dragonfly || solaris

pub fn get_random_max_read() -> usize {
    if std::env::consts::OS == "linux" || std::env::consts::OS == "android" {
        // Per the manpage:
        //     When reading from the urandom source, a maximum of 33554431 bytes
        //     is returned by a single call to getrandom() on systems where int
        //     has a size of 32 bits.
        (1 << 25) - 1
    } else {
        1 << 8
    }
}

/// If the kernel is too old to support the getrandom syscall(),
/// unix::get_random will immediately return ErrorKind::Unsupported and we will then fall back to
/// reading from /dev/urandom in rand_unix.go. unix::get_random caches the ErrorKind::Unsupported
/// result so we only suffer the syscall overhead once in this case.
/// If the kernel supports the getrandom() syscall, unix::get_random will block
/// until the kernel has sufficient randomness (as we don't use GRND_NONBLOCK).
/// In this case, unix::get_random will not return an error.
pub fn get_random(p: &mut [u8]) -> std::io::Result<()> {
    match crate::internal::syscall::unix::get_random(p, 0) {
        Ok(n) => {
            if n != p.len() {
                Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
            } else {
                Ok(())
            }
        }
        Err(err) => Err(err),
    }
}
