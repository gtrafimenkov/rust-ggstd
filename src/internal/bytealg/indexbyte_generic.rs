// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2018 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//go:build !386 && !amd64 && !s390x && !arm && !arm64 && !loong64 && !ppc64 && !ppc64le && !mips && !mipsle && !mips64 && !mips64le && !riscv64 && !wasm

pub fn index_byte(b: &[u8], c: u8) -> isize {
    for (i, x) in b.iter().enumerate() {
        if *x == c {
            return i as isize;
        }
    }
    return -1;
}

// fn IndexByteString(s string, c u8) -> isize {
// 	for i := 0; i < len(s); i++ {
// 		if s[i] == c {
// 			return i
// 		}
// 	}
// 	return -1
// }
