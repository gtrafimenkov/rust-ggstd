// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

/// Color can convert itself to alpha-premultiplied 16-bits per channel RGBA.
/// The conversion may be lossy.
pub trait ColorTrait {
    /// rgba returns the alpha-premultiplied red, green, blue and alpha values
    /// for the color. Each value ranges within [0, 0xffff], but is represented
    /// by a u32 so that multiplying by a blend factor up to 0xffff will not
    /// overflow.
    ///
    /// An alpha-premultiplied color component c has been scaled by alpha (a),
    /// so has valid values 0 <= c <= a.
    fn rgba(&self) -> (u32, u32, u32, u32); // (r, g, b, a)
}

/// Color represents a color in different color models.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    /// RGBA is a traditional 32-bit alpha-premultiplied color, having 8
    /// bits for each of red, green, blue and alpha.
    RGBA(RGBA),
    /// NRGBA represents a non-alpha-premultiplied 32-bit color.
    NRGBA(NRGBA),
    /// RGBA64 represents a 64-bit alpha-premultiplied color, having 16 bits for
    /// each of red, green, blue and alpha.
    RGBA64(RGBA64),
    /// NRGBA64 represents a non-alpha-premultiplied 64-bit color,
    /// having 16 bits for each of red, green, blue and alpha.
    NRGBA64(NRGBA64),
    /// Gray represents an 8-bit grayscale color.
    Gray(Gray),
    /// Gray16 represents a 16-bit grayscale color.
    Gray16(Gray16),
    /// Alpha represents an 8-bit alpha color.
    Alpha(Alpha),
    /// Alpha16 represents a 16-bit alpha color.
    Alpha16(Alpha16),
}

impl Color {
    /// new_gray creates a new Gray color.
    pub const fn new_gray(y: u8) -> Self {
        Color::Gray(Gray::new(y))
    }
    /// new_rgba creates a new RGBA color.
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::RGBA(RGBA::new(r, g, b, a))
    }
    /// new_rgba64 creates a new RGBA64 color.
    pub const fn new_rgba64(r: u16, g: u16, b: u16, a: u16) -> Self {
        Color::RGBA64(RGBA64::new(r, g, b, a))
    }
    /// new_nrgba creates a new NRGBA color.
    pub const fn new_nrgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::NRGBA(NRGBA::new(r, g, b, a))
    }
    /// new_nrgba64 creates a new NRGBA64 color.
    pub const fn new_nrgba64(r: u16, g: u16, b: u16, a: u16) -> Self {
        Color::NRGBA64(NRGBA64::new(r, g, b, a))
    }
    /// to_gray16 converts color to Gray16
    pub fn to_gray16(&self) -> Gray16 {
        match self {
            Color::Gray16(c) => *c,
            _ => Gray16::new_from(self),
        }
    }
    /// to_nrgba converts color to NRGBA64
    pub fn to_nrgba(&self) -> NRGBA {
        match self {
            Color::NRGBA(c) => *c,
            _ => NRGBA::new_from(self),
        }
    }
    /// to_nrgba64 converts color to NRGBA64
    pub fn to_nrgba64(&self) -> NRGBA64 {
        match self {
            Color::NRGBA64(c) => *c,
            _ => NRGBA64::new_from(self),
        }
    }
}

impl ColorTrait for Color {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            Color::RGBA(c) => c.rgba(),
            Color::NRGBA(c) => c.rgba(),
            Color::Gray(c) => c.rgba(),
            Color::Gray16(c) => c.rgba(),
            Color::RGBA64(c) => c.rgba(),
            Color::NRGBA64(c) => c.rgba(),
            Color::Alpha(c) => c.rgba(),
            Color::Alpha16(c) => c.rgba(),
        }
    }
}

/// RGBA represents a traditional 32-bit alpha-premultiplied color, having 8
/// bits for each of red, green, blue and alpha.
///
/// An alpha-premultiplied color component C has been scaled by alpha (A), so
/// has valid values 0 <= C <= A.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn new_from(c: &Color) -> Self {
        if let Color::RGBA(c) = c {
            *c
        } else {
            let (r, g, b, a) = c.rgba();
            RGBA::new(
                (r >> 8) as u8,
                (g >> 8) as u8,
                (b >> 8) as u8,
                (a >> 8) as u8,
            )
        }
    }
}

impl ColorTrait for RGBA {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let mut r = self.r as u32;
        r |= r << 8;
        let mut g = self.g as u32;
        g |= g << 8;
        let mut b = self.b as u32;
        b |= b << 8;
        let mut a = self.a as u32;
        a |= a << 8;
        (r, g, b, a)
    }
}

/// RGBA64 represents a 64-bit alpha-premultiplied color, having 16 bits for
/// each of red, green, blue and alpha.
///
/// An alpha-premultiplied color component C has been scaled by alpha (A), so
/// has valid values 0 <= C <= A.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGBA64 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl RGBA64 {
    pub const fn new(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self { r, g, b, a }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::RGBA64(c) => Self::new(c.r, c.g, c.b, c.a),
            _ => {
                let (r, g, b, a) = c.rgba();
                RGBA64::new(r as u16, g as u16, b as u16, a as u16)
            }
        }
    }
}

impl ColorTrait for RGBA64 {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        (self.r as u32, self.g as u32, self.b as u32, self.a as u32)
    }
}

/// NRGBA represents a non-alpha-premultiplied 32-bit color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl NRGBA {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::NRGBA(c) => Self::new(c.r, c.g, c.b, c.a),
            _ => {
                let (r, g, b, a) = c.rgba();
                if a == 0xffff {
                    return Self::new((r >> 8) as u8, (g >> 8) as u8, (b >> 8) as u8, 0xff);
                }
                if a == 0 {
                    return Self::new(0, 0, 0, 0);
                }
                // Since Color.RGBA returns an alpha-premultiplied color, we should have r <= a && g <= a && b <= a.
                let r = (r * 0xffff) / a;
                let g = (g * 0xffff) / a;
                let b = (b * 0xffff) / a;
                Self::new(
                    (r >> 8) as u8,
                    (g >> 8) as u8,
                    (b >> 8) as u8,
                    (a >> 8) as u8,
                )
            }
        }
    }
}

impl ColorTrait for NRGBA {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let mut r = self.r as u32;
        r |= r << 8;
        r *= self.a as u32;
        r /= 0xff;
        let mut g = self.g as u32;
        g |= g << 8;
        g *= self.a as u32;
        g /= 0xff;
        let mut b = self.b as u32;
        b |= b << 8;
        b *= self.a as u32;
        b /= 0xff;
        let mut a = self.a as u32;
        a |= a << 8;
        (r, g, b, a)
    }
}

/// NRGBA64 represents a non-alpha-premultiplied 64-bit color,
/// having 16 bits for each of red, green, blue and alpha.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NRGBA64 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl NRGBA64 {
    pub const fn new(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self { r, g, b, a }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::NRGBA64(c) => Self::new(c.r, c.g, c.b, c.a),
            _ => {
                let (r, g, b, a) = c.rgba();
                if a == 0xffff {
                    return NRGBA64::new(r as u16, g as u16, b as u16, 0xffff);
                }
                if a == 0 {
                    return NRGBA64::new(0, 0, 0, 0);
                }
                // Since Color.RGBA returns an alpha-premultiplied color, we should have r <= a && g <= a && b <= a.
                let r = (r * 0xffff) / a;
                let g = (g * 0xffff) / a;
                let b = (b * 0xffff) / a;
                NRGBA64::new(r as u16, g as u16, b as u16, a as u16)
            }
        }
    }
}

impl ColorTrait for NRGBA64 {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let mut r = self.r as u32;
        r *= self.a as u32;
        r /= 0xffff;
        let mut g = self.g as u32;
        g *= self.a as u32;
        g /= 0xffff;
        let mut b = self.b as u32;
        b *= self.a as u32;
        b /= 0xffff;
        let a = self.a as u32;
        (r, g, b, a)
    }
}

/// Alpha represents an 8-bit alpha color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Alpha {
    pub a: u8,
}

impl Alpha {
    pub const fn new(a: u8) -> Self {
        Self { a }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::Alpha(c) => Self::new(c.a),
            _ => {
                let (_, _, _, a) = c.rgba();
                Alpha::new((a >> 8) as u8)
            }
        }
    }
}

impl ColorTrait for Alpha {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let mut a = self.a as u32;
        a |= a << 8;
        (a, a, a, a)
    }
}

/// Alpha16 represents a 16-bit alpha color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Alpha16 {
    pub a: u16,
}

impl Alpha16 {
    pub const fn new(a: u16) -> Self {
        Self { a }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::Alpha16(c) => Self::new(c.a),
            _ => {
                let (_, _, _, a) = c.rgba();
                Alpha16::new(a as u16)
            }
        }
    }
}

impl ColorTrait for Alpha16 {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let a = self.a as u32;
        (a, a, a, a)
    }
}

/// Gray represents an 8-bit grayscale color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gray {
    pub y: u8,
}

impl Gray {
    pub const fn new(y: u8) -> Self {
        Self { y }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::Gray(c) => Self::new(c.y),
            _ => {
                let (r, g, b, _) = c.rgba();

                // These coefficients (the fractions 0.299, 0.587 and 0.114) are the same
                // as those given by the JFIF specification and used by fn RGBToYCbCr in
                // ycbcr.go.
                //
                // Note that 19595 + 38470 + 7471 equals 65536.
                //
                // The 24 is 16 + 8. The 16 is the same as used in RGBToYCbCr. The 8 is
                // because the return value is 8 bit color, not 16 bit color.
                let y = (19595 * r + 38470 * g + 7471 * b + (1 << 15)) >> 24;

                Gray::new(y as u8)
            }
        }
    }
}

impl ColorTrait for Gray {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let mut y = self.y as u32;
        y |= y << 8;
        (y, y, y, 0xffff)
    }
}

/// Gray16 represents a 16-bit grayscale color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gray16 {
    pub y: u16,
}

impl Gray16 {
    pub const fn new(y: u16) -> Self {
        Self { y }
    }
    pub fn new_from(c: &Color) -> Self {
        match c {
            Color::Gray16(c) => Self::new(c.y),
            _ => {
                let (r, g, b, _) = c.rgba();
                // These coefficients (the fractions 0.299, 0.587 and 0.114) are the same
                // as those given by the JFIF specification and used by fn RGBToYCbCr in
                // ycbcr.go.
                //
                // Note that 19595 + 38470 + 7471 equals 65536.
                let y = (19595 * r + 38470 * g + 7471 * b + (1 << 15)) >> 16;
                Gray16::new(y as u16)
            }
        }
    }
}

impl ColorTrait for Gray16 {
    fn rgba(&self) -> (u32, u32, u32, u32) {
        let y = self.y as u32;
        (y, y, y, 0xffff)
    }
}

// /// Model can convert any Color to one from its own color model. The conversion
// /// may be lossy.
// pub trait Model {
//     fn Convert(c: dyn Color) -> dyn Color;
// }

// // ModelFunc returns a Model that invokes f to implement the conversion.
// fn ModelFunc(f fn(Color) Color) Model {
// 	// Note: using *modelFunc as the implementation
// 	// means that callers can still use comparisons
// 	// like m == RGBAModel. This is not possible if
// 	// we use the fn value directly, because funcs
// 	// are no longer comparable.
// 	return &modelFunc{f}
// }

// type modelFunc struct {
// 	f fn(Color) Color
// }

// fn (m *modelFunc) Convert(c Color) Color {
// 	return m.f(c)
// }

/// Models for the standard color types.
/// Model can convert any Color to one from its own color model. The conversion
/// may be lossy.
#[derive(Debug, PartialEq)]
pub enum Model {
    RGBAModel,
    RGBA64Model,
    NRGBAModel,
    NRGBA64Model,
    AlphaModel,
    Alpha16Model,
    GrayModel,
    Gray16Model,
    Paletted(Palette),
}

impl Model {
    /// convert any Color to one from its own color model. The conversion
    /// may be lossy.
    pub fn convert(&self, c: &Color) -> Color {
        match self {
            Model::RGBAModel => Color::RGBA(RGBA::new_from(c)),
            Model::NRGBAModel => Color::NRGBA(NRGBA::new_from(c)),
            Model::RGBA64Model => Color::RGBA64(RGBA64::new_from(c)),
            Model::NRGBA64Model => Color::NRGBA64(NRGBA64::new_from(c)),
            Model::GrayModel => Color::Gray(Gray::new_from(c)),
            Model::Gray16Model => Color::Gray16(Gray16::new_from(c)),
            Model::AlphaModel => Color::Alpha(Alpha::new_from(c)),
            Model::Alpha16Model => Color::Alpha16(Alpha16::new_from(c)),
            Model::Paletted(pal) => pal.convert(c),
        }
    }
    pub fn bitdepth(&self) -> usize {
        match self {
            Model::RGBAModel => 8,
            Model::RGBA64Model => 16,
            Model::NRGBAModel => 8,
            Model::NRGBA64Model => 16,
            Model::AlphaModel => 8,
            Model::Alpha16Model => 16,
            Model::GrayModel => 8,
            Model::Gray16Model => 16,
            Model::Paletted(pal) => match pal.colors.len() {
                0..=2 => 1,
                3..=4 => 2,
                5..=16 => 4,
                _ => 8,
            },
        }
    }
}

/// Palette is a palette of colors.
/// The number of colors is usually in the range of 1..=256.
#[derive(Debug, PartialEq, Clone)]
pub struct Palette {
    pub colors: Vec<Color>,
}

impl Palette {
    pub fn new(num_colors: usize) -> Self {
        Self {
            colors: vec![OPAQUE_BLACK; num_colors],
        }
    }

    /// Convert returns the palette color closest to c in Euclidean R,G,B space.
    pub fn convert(&self, c: &Color) -> Color {
        if self.colors.is_empty() {
            return Color::new_gray(0);
        }
        self.colors[self.index(c)]
    }

    /// index returns the index of the palette color closest to c in Euclidean
    /// R,G,B,A space.
    pub fn index(&self, c: &Color) -> usize {
        // A batch version of this computation is in image/draw/draw.go.
        let (cr, cg, cb, ca) = c.rgba();
        let (mut ret, mut best_sum) = (0, 0xffffffff);
        for (i, v) in self.colors.iter().enumerate() {
            let (vr, vg, vb, va) = v.rgba();
            let sum = sq_diff(cr, vr) + sq_diff(cg, vg) + sq_diff(cb, vb) + sq_diff(ca, va);
            if sum < best_sum {
                if sum == 0 {
                    return i;
                }
                (ret, best_sum) = (i, sum);
            }
        }
        ret
    }
}

/// sq_diff returns the squared-difference of x and y, shifted by 2 so that
/// adding four of those won't overflow a u32.
///
/// x and y are both assumed to be in the range [0, 0xffff].
pub(super) fn sq_diff(x: u32, y: u32) -> u32 {
    // The canonical code of this function looks as follows:
    //
    //	var d u32
    //	if x > y {
    //		d = x - y
    //	} else {
    //		d = y - x
    //	}
    //	return (d * d) >> 2
    //
    // Language spec guarantees the following properties of unsigned integer
    // values operations with respect to overflow/wrap around:
    //
    // > For unsigned integer values, the operations +, -, *, and << are
    // > computed modulo 2n, where n is the bit width of the unsigned
    // > integer's type. Loosely speaking, these unsigned integer operations
    // > discard high bits upon overflow, and programs may rely on ``wrap
    // > around''.
    //
    // Considering these properties and the fact that this function is
    // called in the hot paths (x,y loops), it is reduced to the below code
    // which is slightly faster. See TestSqDiff for correctness check.
    let d = x.wrapping_sub(y);
    d.wrapping_mul(d) >> 2
}

// Standard colors.
pub const BLACK: Color = Color::Gray16(Gray16::new(0));
pub const WHITE: Color = Color::Gray16(Gray16::new(0xffff));
pub const TRANSPARENT: Color = Color::Alpha16(Alpha16::new(0));
pub const OPAQUE: Color = Color::Alpha16(Alpha16::new(0xffff));
pub const OPAQUE_BLACK: Color = Color::new_rgba(0x00, 0x00, 0x00, 0xff);
