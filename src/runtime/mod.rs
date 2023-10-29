// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod stubs;

pub use stubs::{fastrand, fastrand64, fastrandn, fastrandu};

#[cfg(windows)]
pub(crate) mod os_windows;
#[cfg(windows)]
use os_windows::get_random_data;

#[cfg(not(windows))]
mod os_linux;
#[cfg(not(windows))]
use os_linux::get_random_data;
