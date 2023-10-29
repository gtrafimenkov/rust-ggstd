// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod bufio;
mod scan;

pub use bufio::{Reader, Writer};
pub use scan::{scan_bytes, scan_lines, Error, Scanner};

#[cfg(test)]
mod bufio_test;
#[cfg(test)]
mod scan_test;
