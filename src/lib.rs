#![allow(clippy::module_inception)]

pub mod bufio;
pub mod builtin;
pub mod bytes;
pub mod compat;
pub mod compress;
pub mod crypto;
pub mod encoding;
pub mod errors;
pub mod hash;
pub mod image;
pub mod internal;
pub mod io;
pub mod math;
pub mod os;
pub mod runtime;
pub mod strconv;
pub mod strings;
pub mod syscall;
pub mod time;
pub mod unicode;

#[cfg(windows)]
mod winapi_;

#[cfg(not(windows))]
mod libc_;
