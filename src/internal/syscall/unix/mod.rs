// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod getrandom;

#[cfg(target_os = "linux")]
mod getrandom_linux;

pub use getrandom::get_random;
