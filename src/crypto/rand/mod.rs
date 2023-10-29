// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package rand implements a cryptographically secure
//! random number generator.

mod rand;

pub use rand::{read, Reader};

#[cfg(not(windows))]
mod rand_getrandom;

#[cfg(not(windows))]
mod rand_unix;
#[cfg(not(windows))]
use rand_unix::read_random;

#[cfg(not(windows))]
use rand_getrandom::{get_random, get_random_max_read};

#[cfg(windows)]
mod rand_windows;

#[cfg(windows)]
use rand_windows::read_random;

#[cfg(test)]
mod rand_test;
