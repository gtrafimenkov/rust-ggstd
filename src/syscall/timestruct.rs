// Copyright 2023 The rust-ggstd authors.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::{set_timespec, Timespec};

// //go:build unix || (js && wasm)

// // timespec_to_nsec returns the time stored in ts as nanoseconds.
// fn timespec_to_nsec(ts Timespec) i64 { return ts.Nano() }

/// nsec_to_timespec converts a number of nanoseconds into a Timespec.
pub fn nsec_to_timespec(nsec: i64) -> Timespec {
    let mut sec = nsec / 1_000_000_000;
    let mut nsec = nsec % 1_000_000_000;
    if nsec < 0 {
        nsec += 1_000_000_000;
        sec -= 1;
    }
    set_timespec(sec, nsec)
}

// // TimevalToNsec returns the time stored in tv as nanoseconds.
// fn TimevalToNsec(tv Timeval) i64 { return tv.Nano() }

// // NsecToTimeval converts a number of nanoseconds into a Timeval.
// fn NsecToTimeval(nsec i64) Timeval {
// 	nsec += 999 // round up to microsecond
// 	usec := nsec % 1e9 / 1e3
// 	sec := nsec / 1e9
// 	if usec < 0 {
// 		usec += 1e6
// 		sec--
// 	}
// 	return setTimeval(sec, usec)
// }
