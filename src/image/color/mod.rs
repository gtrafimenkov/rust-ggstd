// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: BSD-3-Clause

mod color;

pub use color::{
    Alpha, Alpha16, Color, ColorTrait, Gray, Gray16, Model, Palette, BLACK, NRGBA, NRGBA64, OPAQUE,
    OPAQUE_BLACK, RGBA, RGBA64, TRANSPARENT, WHITE,
};

#[cfg(test)]
mod color_test;
