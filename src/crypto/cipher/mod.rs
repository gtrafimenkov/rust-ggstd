// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package cipher implements standard block cipher modes that can be wrapped
//! around low-level block cipher implementations.
//! See <https://csrc.nist.gov/groups/ST/toolkit/BCM/current_modes.html>
//! and NIST Special Publication 800-38A.

mod cbc;
mod cipher;
mod ctr;

pub use cbc::{CBCDecrypter, CBCEncrypter};
pub use cipher::{Block, BlockMode, Stream};
pub use ctr::CTR;

#[cfg(test)]
mod cbc_aes_test;
#[cfg(test)]
mod cipher_test;
#[cfg(test)]
mod common_test;
#[cfg(test)]
mod ctr_aes_test;
#[cfg(test)]
mod ctr_test;
