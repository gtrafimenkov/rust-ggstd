// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright (c) 2014-2020 The Rust Project Developers
// SPDX-License-Identifier: MIT
// Parts of this file come from https://github.com/rust-lang/libc

#![allow(non_camel_case_types)]

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub type c_char = u8;

#[cfg(target_arch = "x86_64")]
pub type c_char = i8;

pub type c_int = i32;
pub type c_long = i64;
pub type c_uint = u32;
pub type c_void = std::os::raw::c_void;
pub type gid_t = u32;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uid_t = u32;

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

pub const ENOENT: c_int = 2;
pub const ERANGE: c_int = 34;
pub const ENOSYS: c_int = 38;

pub const AT_FDCWD: c_int = -100;

pub type time_t = i64;

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

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

    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const timespec,
        flag: c_int,
    ) -> c_int;

}

pub fn get_errno() -> c_int {
    unsafe { *__errno_location() }
}
