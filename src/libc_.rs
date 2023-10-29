// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright (c) 2014-2020 The Rust Project Developers
// SPDX-License-Identifier: MIT
// Parts of this file come from https://github.com/rust-lang/libc

#![allow(non_camel_case_types)]

pub type c_char = i8;
pub type c_int = i32;
pub type gid_t = u32;
pub type size_t = usize;
pub type uid_t = u32;
pub type c_uint = u32;
pub type c_void = std::os::raw::c_void;
pub type ssize_t = isize;

#[repr(C)]
pub struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}

pub const ERANGE: c_int = 34;
pub const ENOSYS: c_int = 38;

extern "C" {
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;

    pub fn getuid() -> uid_t;

    pub fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t;

    pub fn __errno_location() -> *mut c_int;
}

pub fn get_errno() -> c_int {
    unsafe { *__errno_location() }
}
