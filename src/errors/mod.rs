// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package errors implements functions to manipulate errors.

mod errors;

pub use errors::{
    copy_stdio_error, copy_stdio_option_error, iores_to_result, is_rust_eof, new_static,
    new_stdio_other_error, new_str, new_unexpected_eof, ErrorStaticString, ErrorString, RUST_EOF,
};
