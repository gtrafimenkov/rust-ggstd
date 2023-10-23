// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// constant_time_compare returns 1 if the two slices, x and y, have equal contents
/// and 0 otherwise. The time taken is a function of the length of the slices and
/// is independent of the contents. If the lengths of x and y do not match it
/// returns 0 immediately.
pub fn constant_time_compare(x: &[u8], y: &[u8]) -> isize {
    if x.len() != y.len() {
        return 0;
    }

    let mut v = 0;

    for i in 0..x.len() {
        v |= x[i] ^ y[i];
    }

    constant_time_byte_eq(v, 0)
}

// // ConstantTimeSelect returns x if v == 1 and y if v == 0.
// // Its behavior is undefined if v takes any other value.
// pub fn ConstantTimeSelect(v, x, y isize) ->isize { return ^(v-1)&x | (v-1)&y }

/// constant_time_byte_eq returns 1 if x == y and 0 otherwise.
pub fn constant_time_byte_eq(x: u8, y: u8) -> isize {
    ((((x ^ y) as u32).wrapping_sub(1)) >> 31) as isize
}

// // ConstantTimeEq returns 1 if x == y and 0 otherwise.
// pub fn ConstantTimeEq(x, y int32) ->isize {
// 	return isize((uint64(uint32(x^y)) - 1) >> 63)
// }

// // ConstantTimeCopy copies the contents of y into x (a slice of equal length)
// // if v == 1. If v == 0, x is left unchanged. Its behavior is undefined if v
// // takes any other value.
// pub fn ConstantTimeCopy(v isize, x, y &[u8]) {
// 	if x.len() != y.len() {
// 		panic("subtle: slices have different lengths")
// 	}

// 	xmask := byte(v - 1)
// 	ymask := byte(^(v - 1))
// 	for i := 0; i < x.len(); i++ {
// 		x[i] = x[i]&xmask | y[i]&ymask
// 	}
// }

// // ConstantTimeLessOrEq returns 1 if x <= y and 0 otherwise.
// // Its behavior is undefined if x or y are negative or > 2**31 - 1.
// pub fn ConstantTimeLessOrEq(x, y isize) ->isize {
// 	x32 := int32(x)
// 	y32 := int32(y)
// 	return isize(((x32 - y32 - 1) >> 31) & 1)
// }
