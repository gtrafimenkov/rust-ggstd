mod getrandom;

#[cfg(not(windows))]
mod getrandom_linux;

pub use getrandom::get_random;
