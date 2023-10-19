// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright (c) 2014-2020 The Rust Project Developers
// Licensed under the terms of [MIT License](https://opensource.org/license/mit/).

// Original content of this file was copied from https://github.com/rust-lang/libc

#![allow(non_camel_case_types)]

pub type c_char = i8;
pub type c_int = i32;
pub type gid_t = u32;
pub type size_t = usize;
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

extern "C" {
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;

    pub fn getuid() -> uid_t;
}

pub const ERANGE: c_int = 34;
