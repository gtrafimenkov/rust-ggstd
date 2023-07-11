//! Package bits implements bit counting and manipulation
//! functions for the predeclared unsigned integer types.
//!
//! Functions in this package may be implemented directly by
//! the compiler, for better performance. For those functions
//! the code in this package will not be used. Which
//! functions are implemented by the compiler depends on the
//! architecture and the Go release.

mod bits;

pub use bits::rotate_left32;
