// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package draw provides image composition functions.
//!
//! See "The Go image/draw package" for an introduction to this package:
//! <https://golang.org/doc/articles/image_draw.html>

mod draw;

pub use draw::{draw, Op};
