// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package unicode provides data and functions to test some properties of
//! Unicode code points.

mod graphic;
mod letter;
mod rune;
mod tables;
pub mod utf8;

pub use letter::{MAX_ASCII, MAX_LATIN1, MAX_RUNE, REPLACEMENT_CHAR};
pub use rune::Rune;
