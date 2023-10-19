// Copyright 2023 The rust-ggstd authors. All rights reserved.

use crate::libc_;

// get_uid returns the numeric user id of the caller.
//
// On Windows, it returns -1.
pub fn get_uid() -> isize {
    unsafe { libc_::getuid() as isize }
}
