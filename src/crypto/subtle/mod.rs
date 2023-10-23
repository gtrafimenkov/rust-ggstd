//! Package subtle implements functions that are often useful in cryptographic
//! code but require careful thought to use correctly.

mod constant_time;

pub use constant_time::{constant_time_byte_eq, constant_time_compare};

#[cfg(test)]
mod constant_time_test;
