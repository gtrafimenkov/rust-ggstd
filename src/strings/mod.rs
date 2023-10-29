// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod builder;
mod reader;
mod strings;

pub use builder::Builder;
pub use reader::Reader;
pub use strings::{contains, cut, has_suffix, index, replace_all};

#[cfg(test)]
mod strings_test;
