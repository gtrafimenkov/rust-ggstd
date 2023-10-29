// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

#[cfg(target_os = "windows")]
mod security_windows;

#[cfg(target_os = "windows")]
mod syscall_windows;

#[cfg(target_os = "windows")]
mod types_windows;

#[cfg(target_os = "windows")]
pub use types_windows::{nsec_to_filetime, Filetime};

#[cfg(target_os = "windows")]
pub use syscall_windows::{nsec_to_timespec, utimes_nano, Timespec};

#[cfg(target_os = "windows")]
pub use security_windows::{
    close_handle, get_error, lookup_account, open_current_process_token, sid_to_string,
    translate_account_name, Token, NAME_DISPLAY, NAME_SAM_COMPATIBLE,
};

#[cfg(target_os = "windows")]
pub use syscall_windows::{get_uid, utf16_from_string, utf16_ptr_to_string, utf16_to_string};

#[cfg(target_os = "linux")]
mod timestruct;
#[cfg(target_os = "linux")]
pub use timestruct::nsec_to_timespec;

#[cfg(target_os = "linux")]
mod syscall_clib;

#[cfg(target_os = "linux")]
mod syscall_linux;

#[cfg(target_os = "linux")]
pub use syscall_linux::utimes_nano;

#[cfg(target_os = "linux")]
mod types_linux;

#[cfg(target_os = "linux")]
pub use types_linux::{set_timespec, Timespec};

#[cfg(target_os = "linux")]
pub use syscall_clib::get_uid;
