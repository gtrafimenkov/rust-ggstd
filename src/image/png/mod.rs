// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package png implements a PNG image decoder and encoder.
//!
//! The PNG specification is at <https://www.w3.org/TR/PNG/>.

mod paeth;
mod reader;
mod writer;

pub use reader::{decode, decode_config, Error};
pub use writer::{
    encode, Encoder, BEST_COMPRESSION, BEST_SPEED, DEFAULT_COMPRESSION, NO_COMPRESSION,
};

#[cfg(test)]
mod paeth_test;
#[cfg(test)]
mod reader_test;
#[cfg(test)]
mod writer_test;
