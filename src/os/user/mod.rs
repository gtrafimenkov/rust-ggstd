// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package user allows user account lookups by name or id.
//!
//! For most Unix systems, this package has two internal implementations of
//! resolving user and group ids to names, and listing supplementary group IDs.
//! One is written in pure Go and parses /etc/passwd and /etc/group. The other
//! is cgo-based and relies on the standard C library (libc) routines such as
//! getpwuid_r, getgrnam_r, and getgrouplist.
//!
//! When cgo is available, and the required routines are implemented in libc
//! for a particular platform, cgo-based (libc-backed) code is used.
//! This can be overridden by using osusergo build tag, which enforces
//! the pure Go implementation.

#[cfg(not(windows))]
mod cgo_lookup_unix;
mod lookup;
mod user;

#[cfg(windows)]
mod lookup_windows;

pub use lookup::current;
pub use user::{unknown_user_id_error, Group, User};

#[cfg(not(windows))]
use cgo_lookup_unix::current_internal;
#[cfg(windows)]
use lookup_windows::current_internal;

#[cfg(test)]
mod user_test;

#[cfg(test)]
#[cfg(not(windows))]
mod cgo_unix_test;
