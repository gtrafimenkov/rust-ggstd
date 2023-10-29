// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod file;
mod proc;
mod tempfile;
pub mod user;

pub use file::{create, open, read_file, temp_dir};
pub use proc::get_uid;
pub use tempfile::{create_temp, is_err_pattern_has_separator, mkdir_temp, TempDir, TempFile};

#[cfg(target_os = "linux")]
pub use crate::libc_::timespec as Timespec;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod file_posix;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use file_posix::chtimes;

#[cfg(target_os = "linux")]
mod file_unix;
#[cfg(target_os = "linux")]
use file_unix::fix_long_path;

#[cfg(target_os = "linux")]
mod path_unix;
#[cfg(target_os = "linux")]
pub use path_unix::is_path_separator;

#[cfg(target_os = "windows")]
mod path_windows;
#[cfg(target_os = "windows")]
use path_windows::fix_long_path;
#[cfg(target_os = "windows")]
pub use path_windows::is_path_separator;

#[cfg(test)]
mod path_test;

#[cfg(test)]
mod tempfile_test;

#[cfg(test)]
mod os_test;

#[cfg(all(test, target_os = "windows"))]
mod path_windows_test;
