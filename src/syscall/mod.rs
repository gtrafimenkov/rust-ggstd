#[cfg(target_os = "windows")]
mod security_windows;

#[cfg(target_os = "windows")]
mod syscall_windows;

#[cfg(not(target_os = "windows"))]
mod syscall_clib;

#[cfg(target_os = "windows")]
pub use security_windows::{
    lookup_account, open_current_process_token, sid_to_string, translate_account_name, Token,
    NAME_DISPLAY, NAME_SAM_COMPATIBLE,
};

#[cfg(target_os = "windows")]
pub use syscall_windows::{get_uid, utf16_from_string, utf16_ptr_to_string, utf16_to_string};

#[cfg(not(target_os = "windows"))]
pub use syscall_clib::get_uid;
