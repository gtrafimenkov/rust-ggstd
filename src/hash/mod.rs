// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

//! Package hash provides interfaces for hash functions.

mod hash;

pub mod adler32;
pub mod crc32;

pub use hash::{Hash, Hash32, Hash64};
