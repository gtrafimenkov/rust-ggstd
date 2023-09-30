mod builder;
mod reader;
mod strings;

pub use builder::Builder;
pub use reader::Reader;
pub use strings::{contains, has_suffix, index, replace_all};

#[cfg(test)]
mod strings_test;
