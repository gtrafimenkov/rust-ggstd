// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package bytes implements functions for the manipulation of byte slices.
//! It is analogous to the facilities of the strings package.

mod buffer;

pub use buffer::{new_buffer, new_buffer_string, Buffer};

#[cfg(test)]
mod buffer_test;
