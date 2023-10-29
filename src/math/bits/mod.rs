// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package bits implements bit counting and manipulation
//! functions for the predeclared unsigned integer types.
//!
//! Functions in this package may be implemented directly by
//! the compiler, for better performance. For those functions
//! the code in this package will not be used. Which
//! functions are implemented by the compiler depends on the
//! architecture and the Go release.

mod bits;
mod bits_tables;

pub use bits::{reverse16, reverse8, rotate_left32};

#[cfg(test)]
mod bits_test;
