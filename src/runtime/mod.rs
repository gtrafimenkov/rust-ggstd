mod stubs;

pub use stubs::{fastrand, fastrand64, fastrandn, fastrandu};

#[cfg(windows)]
mod os_windows;
#[cfg(windows)]
pub use os_windows::get_random_data;

#[cfg(not(windows))]
mod os_linux;
#[cfg(not(windows))]
pub use os_linux::get_random_data;
