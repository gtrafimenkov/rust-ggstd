// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2017 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

fn struct_passwd_for_negative_test() -> libc::passwd {
    let mut pwd = unsafe { std::mem::zeroed::<libc::passwd>() };
    pwd.pw_uid = ((1_u64 << 32) - 2) as u32;
    pwd.pw_gid = ((1_u64 << 32) - 3) as u32;
    pwd
}

/// Issue 22739
#[test]
fn test_negative_uid() {
    let sp = struct_passwd_for_negative_test();
    let u = super::cgo_lookup_unix::build_user(&sp);
    assert_eq!(u.uid, "4294967294");
    assert_eq!(u.gid, "4294967293");
}
