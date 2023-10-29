// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod stubs;

pub use stubs::{fastrand, fastrand64, fastrandn, fastrandu};

#[cfg(target_os = "windows")]
pub(crate) mod os_windows;
#[cfg(target_os = "windows")]
use os_windows::get_random_data;

#[cfg(target_os = "linux")]
mod os_linux;
#[cfg(target_os = "linux")]
use os_linux::get_random_data;
