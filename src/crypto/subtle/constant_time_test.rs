// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// import (
// 	"testing"
// 	"testing/quick"
// )

struct TestConstantTimeCompare(&'static [u8], &'static [u8], isize);

const TEST_CONSTANT_TIME_COMPARE_DATA: &[TestConstantTimeCompare] = &[
    TestConstantTimeCompare(&[], &[], 1),
    TestConstantTimeCompare(&[0x11], &[0x11], 1),
    TestConstantTimeCompare(&[0x12], &[0x11], 0),
    TestConstantTimeCompare(&[0x11], &[0x11, 0x12], 0),
    TestConstantTimeCompare(&[0x11, 0x12], &[0x11], 0),
];

#[test]
fn test_constant_time_compare() {
    for (i, test) in TEST_CONSTANT_TIME_COMPARE_DATA.iter().enumerate() {
        let r = super::constant_time_compare(test.0, test.1);
        assert_eq!(
            r, test.2,
            "#{} bad result (got {:x}, want {:x})",
            i, r, test.2
        );
    }
}

struct TestConstantTimeByteEq(u8, u8, isize);

const TEST_CONSTAND_TIME_BYTE_EQ_DATA: &[TestConstantTimeByteEq] = &[
    TestConstantTimeByteEq(0, 0, 1),
    TestConstantTimeByteEq(0, 1, 0),
    TestConstantTimeByteEq(1, 0, 0),
    TestConstantTimeByteEq(0xff, 0xff, 1),
    TestConstantTimeByteEq(0xff, 0xfe, 0),
];

// fn byte_eq(a: u8, b: u8) -> isize {
//     if a == b {
//         1
//     } else {
//         0
//     }
// }

#[test]
fn test_constant_time_byte_eq() {
    for (i, test) in TEST_CONSTAND_TIME_BYTE_EQ_DATA.iter().enumerate() {
        let r = super::constant_time_byte_eq(test.0, test.1);
        assert_eq!(
            r, test.2,
            "#{} bad result (got {:x}, want {:x})",
            i, r, test.2
        );
    }
    // 	err := quick.CheckEqual(constant_time_byte_eq, byte_eq, nil)
    // 	if err != nil {
    // 		t.Error(err)
    // 	}
}

// fn eq(a, b int32) isize {
// 	if a == b {
// 		return 1
// 	}
// 	return 0
// }

// #[test]
// fn TestConstantTimeEq() {
// 	err := quick.CheckEqual(ConstantTimeEq, eq, nil)
// 	if err != nil {
// 		t.Error(err)
// 	}
// }

// fn makeCopy(v isize, x, y []byte) []byte {
// 	if len(x) > len(y) {
// 		x = x[0:len(y)]
// 	} else {
// 		y = y[0:len(x)]
// 	}
// 	if v == 1 {
// 		copy(x, y)
// 	}
// 	return x
// }

// fn constantTimeCopyWrapper(v isize, x, y []byte) []byte {
// 	if len(x) > len(y) {
// 		x = x[0:len(y)]
// 	} else {
// 		y = y[0:len(x)]
// 	}
// 	v &= 1
// 	ConstantTimeCopy(v, x, y)
// 	return x
// }

// #[test]
// fn TestConstantTimeCopy() {
// 	err := quick.CheckEqual(constantTimeCopyWrapper, makeCopy, nil)
// 	if err != nil {
// 		t.Error(err)
// 	}
// }

// var lessOrEqTests = []struct {
// 	x, y, result isize
// }{
// 	{0, 0, 1},
// 	{1, 0, 0},
// 	{0, 1, 1},
// 	{10, 20, 1},
// 	{20, 10, 0},
// 	{10, 10, 1},
// }

// #[test]
// fn TestConstantTimeLessOrEq() {
// 	for i, test in lessOrEqTests {
// 		result := ConstantTimeLessOrEq(test.x, test.y)
// 		if result != test.result {
// 			t.Errorf("#{}: {} <= {} gave {}, expected {}", i, test.x, test.y, result, test.result)
// 		}
// 	}
// }

// var benchmarkGlobal u8

// fn Benchmarkconstant_time_byte_eq(b *testing.B) {
// 	var x, y u8

// 	for i := 0; i < b.N; i++ {
// 		x, y = u8(constant_time_byte_eq(x, y)), x
// 	}

// 	benchmarkGlobal = x
// }

// fn BenchmarkConstantTimeEq(b *testing.B) {
// 	var x, y isize

// 	for i := 0; i < b.N; i++ {
// 		x, y = ConstantTimeEq(int32(x), int32(y)), x
// 	}

// 	benchmarkGlobal = u8(x)
// }

// fn BenchmarkConstantTimeLessOrEq(b *testing.B) {
// 	var x, y isize

// 	for i := 0; i < b.N; i++ {
// 		x, y = ConstantTimeLessOrEq(x, y), x
// 	}

// 	benchmarkGlobal = u8(x)
// }
