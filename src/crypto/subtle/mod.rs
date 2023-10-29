// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package subtle implements functions that are often useful in cryptographic
//! code but require careful thought to use correctly.

mod constant_time;
mod xor;
mod xor_generic;

pub use constant_time::{constant_time_byte_eq, constant_time_compare};
pub use xor::xor_bytes;

#[cfg(test)]
mod constant_time_test;
