// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package gzip implements reading and writing of gzip format compressed files,
//! as specified in RFC 1952.

mod gunzip;

pub use gunzip::{Header, Reader, ERR_CHECKSUM_MSG, ERR_INVALID_HEADER};

#[cfg(test)]
mod gunzip_test;
