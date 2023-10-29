// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package time provides functionality for measuring and displaying time.
//!
//! The calendrical calculations always assume a Gregorian calendar, with
//! no leap seconds.

mod time;

pub use time::{date, now, unix, Month, Time, Weekday, HMS, YMD};

#[cfg(test)]
mod time_test;
