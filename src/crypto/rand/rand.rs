// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// Reader is a cryptographically secure random number generator
/// that implements `std::io::Read` and `create::io::Reader` traits.
/// The implementation uses `read` function.
pub struct Reader {}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader {
    pub fn new() -> Self {
        Self {}
    }
}

impl std::io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        read(buf)?;
        Ok(buf.len())
    }
}

impl crate::io::Reader for Reader {
    fn read(&mut self, p: &mut [u8]) -> crate::io::IoRes {
        match read(p) {
            Ok(_) => (p.len(), None),
            Err(err) => (0, Some(err)),
        }
    }
}

/// read reads the whole buffer of cryptographically secure random data.
/// On Linux read uses getrandom(2) if available, /dev/urandom otherwise.
/// On Windows systems, read uses the RtlGenRandom API.
pub fn read(b: &mut [u8]) -> std::io::Result<()> {
    super::read_random(b)
}
