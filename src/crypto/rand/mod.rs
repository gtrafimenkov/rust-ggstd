// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package rand implements a cryptographically secure
//! random number generator.

mod rand;

pub use rand::{read, Reader};

#[cfg(target_os = "linux")]
mod rand_getrandom;

#[cfg(target_os = "linux")]
mod rand_unix;
#[cfg(target_os = "linux")]
use rand_unix::read_random;

#[cfg(target_os = "linux")]
use rand_getrandom::{get_random, get_random_max_read};

#[cfg(target_os = "windows")]
mod rand_windows;

#[cfg(target_os = "windows")]
use rand_windows::read_random;

#[cfg(test)]
mod rand_test;
