// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Copyright 2017 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// import (
// 	"testing"
// 	"testing/quick"
// )

#[test]
fn test_sq_diff() {
    // canonical sqDiff implementation
    fn orig(x: u32, y: u32) -> u32 {
        let d = if x > y { x - y } else { y - x };
        d.wrapping_mul(d) >> 2
    }
    let test_cases = &[
        0_u32, 1, 2, 0x0fffd, 0x0fffe, 0x0ffff, 0x10000, 0x10001, 0x10002, 0xfffffffd, 0xfffffffe,
        0xffffffff,
    ];
    for x in test_cases {
        for y in test_cases {
            let (got, want) = (super::color::sq_diff(*x, *y), orig(*x, *y));
            assert_eq!(
                got, want,
                "sqDiff({:x}, {:x}): got {}, want {}",
                x, y, got, want
            );
        }
    }
    // 	if err := quick.CheckEqual(orig, sqDiff, &quick.Config{MaxCountScale: 10}); err != nil {
    // 		t.Fatal(err)
    // 	}
}
