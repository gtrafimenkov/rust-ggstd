//! Package time provides functionality for measuring and displaying time.
//!
//! The calendrical calculations always assume a Gregorian calendar, with
//! no leap seconds.

mod time;

pub use time::{date, now, unix, Month, Time, Weekday, HMS, YMD};

#[cfg(test)]
mod time_test;
