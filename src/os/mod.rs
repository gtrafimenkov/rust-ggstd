// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod file;
mod proc;
mod tempfile;
pub mod user;

pub use file::{create, open, read_file, temp_dir};
pub use proc::get_uid;
pub use tempfile::{create_temp, is_err_pattern_has_separator, mkdir_temp, TempDir, TempFile};

#[cfg(not(windows))]
mod path_unix;
#[cfg(not(windows))]
pub use path_unix::is_path_separator;

#[cfg(windows)]
mod path_windows;
#[cfg(windows)]
pub use path_windows::is_path_separator;

#[cfg(test)]
mod path_test;

#[cfg(test)]
mod tempfile_test;
