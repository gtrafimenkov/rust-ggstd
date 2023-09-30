// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::color::{self, ColorTrait};
use super::geom::{Point, Rectangle};

/// Config holds an image's color model and dimensions.
#[derive(Debug)]
pub struct Config {
    pub color_model: color::Model,
    pub width: usize,
    pub height: usize,
}

/// Image is a finite rectangular grid of color::Color values taken from a color
/// model.
pub trait Image {
    /// color_model returns the Image's color model.
    fn color_model(&self) -> color::Model;
    /// bounds returns the domain for which At can return non-zero color::
    /// The bounds do not necessarily contain the point (0, 0).
    fn bounds(&self) -> &Rectangle;
    /// at returns the color of the pixel at (x, y).
    /// at(bounds().min.x, bounds().min.y) returns the upper-left pixel of the grid.
    /// at(bounds().max.x-1, bounds().max.y-1) returns the lower-right one.
    fn at(&self, x: isize, y: isize) -> color::Color;
    // set color of the give pixel.  The color will be converted to the color model
    // of the image if necessary.  The conversion may be lossy.
    fn set(&mut self, x: isize, y: isize, c: &color::Color);
    /// opaque scans the entire image and reports whether it is fully opaque.
    fn opaque(&self) -> bool;
    /// stride returns the pix stride (in bytes) between vertically adjacent pixels.
    fn stride(&self) -> usize;
    /// pix returns the image pixel data.  Format of the data depends on the image type.
    fn pix(&self) -> &Vec<u8>;
    /// bytes_per_pixel return how many bytes is used per pixel
    fn bytes_per_pixel(&self) -> usize;

    /// get_pix_mutable returns the image data for editing.
    /// Format of the data depends on the image type.
    fn get_pix_mutable(&mut self) -> &mut Vec<u8>;

    /// deep_equal checks that two images are exactly the same
    fn deep_equal(&self, other: &dyn Image) -> bool {
        self.color_model() == other.color_model()
            && self.bounds() == other.bounds()
            && self.pix() == other.pix()
    }

    /// get_line_data returns data that encode one line of pixels.
    fn get_line_data(&self, y: isize) -> &[u8] {
        let b = self.bounds();
        let offset = (y - b.min.y) as usize * self.stride();
        &self.pix()[offset..offset + b.dx() * self.bytes_per_pixel()]
    }
}

// // RGBA64Image is an Image whose pixels can be converted directly to a
// // color::RGBA64.
// type RGBA64Image interface {
// 	// RGBA64At returns the RGBA64 color of the pixel at (x, y). It is
// 	// equivalent to calling At(x, y).rgba() and converting the resulting
// 	// 32-bit return values to a color::RGBA64, but it can avoid allocations
// 	// from converting concrete color types to the color::Color interface type.
// 	RGBA64At(x: isize, y: isize) color::RGBA64
// 	Image
// }

// // PalettedImage is an image whose colors may come from a limited palette.
// // If m is a PalettedImage and m.color_model() returns a color::Palette p,
// // undefined.
// type PalettedImage interface {
// 	Image
// }

/// pixel_buffer_length returns the length of the [u8] typed pix slice field
/// for the NewXxx functions.
// Conceptually, this is just (bpp * width * height),
// but this function panics if at least one of those is negative or if the
// computation would overflow the isize type.
//
// This panics instead of returning an error because of backwards
// compatibility. The NewXxx functions do not return an error.
fn pixel_buffer_length(bytes_per_pixel: usize, r: &Rectangle, _image_type_name: &str) -> usize {
    bytes_per_pixel * r.dx() * r.dy()
}

/// RGBA is an in-memory image whose At method returns color::RGBA values.
#[derive(Debug)]
pub struct RGBA {
    /// pix holds the image's pixels, in R, G, B, A order. The pixel at
    /// (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*4].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    pub rect: Rectangle,
}

impl Image for RGBA {
    fn color_model(&self) -> color::Model {
        color::Model::RGBAModel
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    fn opaque(&self) -> bool {
        if self.rect.empty() {
            return true;
        }
        let mut i0 = 3;
        let mut i1 = self.rect.dx() * 4;
        for _y in self.rect.min.y..self.rect.max.y {
            let mut i = i0;
            while i < i1 {
                if self.pix[i] != 0xff {
                    return false;
                }
                i += 4;
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::RGBA(self.rgba_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::RGBA(c) = c {
            self.set_rbga(x, y, c);
        } else {
            self.set_rbga(x, y, &color::RGBA::new_from(c));
        };
    }

    fn bytes_per_pixel(&self) -> usize {
        4
    }

    // fn (p *RGBA) RGBA64At(x: isize, y: isize) color::RGBA64 {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return color::RGBA64{}
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
    // 	r := uint16(s[0])
    // 	g := uint16(s[1])
    // 	b := uint16(s[2])
    // 	a := uint16(s[3])
    // 	return color::RGBA64{
    // 		(r << 8) | r,
    // 		(g << 8) | g,
    // 		(b << 8) | b,
    // 		(a << 8) | a,
    // 	}
    // }
}

impl RGBA {
    /// new returns a new RGBA image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(4, r, "RGBA")],
            stride: 4 * r.dx(),
            rect: *r,
        }
    }

    pub fn rgba_at(&self, x: isize, y: isize) -> color::RGBA {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::RGBA::new(0, 0, 0, 0);
        }
        let i = self.pix_offset(x, y);
        let s = &self.pix[i..i + 4];
        color::RGBA::new(s[0], s[1], s[2], s[3])
    }

    // fn (p *RGBA) SetRGBA64(x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
    // 	s[0] = u8(c.r >> 8)
    // 	s[1] = u8(c.g >> 8)
    // 	s[2] = u8(c.b >> 8)
    // 	s[3] = u8(c.a >> 8)
    // }

    pub fn set_rbga(&mut self, x: isize, y: isize, c: &color::RGBA) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        let s = &mut self.pix[i..i + 4];
        s[0] = c.r;
        s[1] = c.g;
        s[2] = c.b;
        s[3] = c.a;
    }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn (p *RGBA) SubImage(r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &RGBA{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &RGBA{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize * 4
    }
}

/// RGBA64 is an in-memory image whose At method returns color::RGBA64 values.
#[derive(Debug)]
pub struct RGBA64 {
    // pix holds the image's pixels, in R, G, B, A order and big-endian format. The pixel at
    // (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*8].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
}

impl Image for RGBA64 {
    fn color_model(&self) -> color::Model {
        color::Model::RGBA64Model
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::RGBA64(self.rgba64_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::RGBA64(c) = c {
            self.set_rgba64(x, y, c);
        } else {
            self.set_rgba64(x, y, &color::RGBA64::new_from(c));
        };
    }

    // opaque scans the entire image and reports whether it is fully opaque.
    fn opaque(&self) -> bool {
        if self.rect.empty() {
            return true;
        }
        let (mut i0, mut i1) = (6, self.rect.dx() * 8);
        for _y in self.rect.min.y..self.rect.max.y {
            let mut i = i0;
            while i < i1 {
                if self.pix[i] != 0xff || self.pix[i + 1] != 0xff {
                    return false;
                }
                i += 8;
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn bytes_per_pixel(&self) -> usize {
        8
    }
}

impl RGBA64 {
    /// new returns a new RGBA64 image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(8, r, "RGBA64")],
            stride: 8 * r.dx(),
            rect: *r,
        }
    }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize * 8
    }

    fn rgba64_at(&self, x: isize, y: isize) -> color::RGBA64 {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::RGBA64::new(0, 0, 0, 0);
        }
        let i = self.pix_offset(x, y);
        let s = &self.pix[i..i + 8];
        color::RGBA64::new(
            ((s[0] as u16) << 8) | (s[1] as u16),
            ((s[2] as u16) << 8) | (s[3] as u16),
            ((s[4] as u16) << 8) | (s[5] as u16),
            ((s[6] as u16) << 8) | (s[7] as u16),
        )
    }

    fn set_rgba64(&mut self, x: isize, y: isize, c: &color::RGBA64) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        let s = &mut self.pix[i..i + 8];
        s[0] = (c.r >> 8) as u8;
        s[1] = c.r as u8;
        s[2] = (c.g >> 8) as u8;
        s[3] = c.g as u8;
        s[4] = (c.b >> 8) as u8;
        s[5] = c.b as u8;
        s[6] = (c.a >> 8) as u8;
        s[7] = c.a as u8;
    }
}

// // SubImage returns an image representing the portion of the image p visible
// // through r. The returned value shares pixels with the original image.
// fn (p *RGBA64) SubImage(r Rectangle) Image {
// 	r = r.intersect(self.rect)
// 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
// 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
// 	// this, the pix[i:] expression below can panic.
// 	if r.Empty() {
// 		return &RGBA64{}
// 	}
// 	let i = self.pix_offset(r.min.x, r.min.y)
// 	return &RGBA64{
// 		pix:    self.pix[i:],
// 		stride: self.stride,
// 		Rect:   r,
// 	}
// }

/// NRGBA is an in-memory image whose At method returns color::NRGBA values.
#[derive(Debug)]
pub struct NRGBA {
    /// pix holds the image's pixels, in R, G, B, A order. The pixel at
    /// (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*4].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    pub rect: Rectangle,
}

impl Image for NRGBA {
    fn color_model(&self) -> color::Model {
        color::Model::NRGBAModel
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    /// opaque scans the entire image and reports whether it is fully opaque.
    fn opaque(&self) -> bool {
        if self.rect.empty() {
            return true;
        }
        let mut i0 = 3;
        let mut i1 = self.rect.dx() * 4;
        for _y in self.rect.min.y..self.rect.max.y {
            let mut i = i0;
            while i < i1 {
                if self.pix[i] != 0xff {
                    return false;
                }
                i += 4;
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::NRGBA(self.nrgba_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::NRGBA(c) = c {
            self.set_nrgba(x, y, c);
        } else {
            self.set_nrgba(x, y, &color::NRGBA::new_from(c));
        };
    }

    fn bytes_per_pixel(&self) -> usize {
        4
    }
}

impl NRGBA {
    /// new returns a new NRGBA image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(4, r, "NRGBA")],
            stride: 4 * r.dx(),
            rect: *r,
        }
    }

    // fn RGBA64At(&self, x: isize, y: isize) color::RGBA64 {
    // 	r, g, b, a := self.nrgba_at(x, y).rgba()
    // 	return color::RGBA64{uint16(r), uint16(g), uint16(b), uint16(a)}
    // }

    fn nrgba_at(&self, x: isize, y: isize) -> color::NRGBA {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::NRGBA::new(0, 0, 0, 0);
        }
        let i = self.pix_offset(x, y);
        let s = &self.pix[i..i + 4];
        color::NRGBA::new(s[0], s[1], s[2], s[3])
    }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize * 4
    }

    pub fn set_nrgba(&mut self, x: isize, y: isize, c: &color::NRGBA) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = c.r;
        self.pix[i + 1] = c.g;
        self.pix[i + 2] = c.b;
        self.pix[i + 3] = c.a;
    }
    // fn SetRGBA64(&self, x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	r, g, b, a := uint32(c.r), uint32(c.g), uint32(c.b), uint32(c.a)
    // 	if (a != 0) && (a != 0xffff) {
    // 		r = (r * 0xffff) / a
    // 		g = (g * 0xffff) / a
    // 		b = (b * 0xffff) / a
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
    // 	s[0] = u8(r >> 8)
    // 	s[1] = u8(g >> 8)
    // 	s[2] = u8(b >> 8)
    // 	s[3] = u8(a >> 8)
    // }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn SubImage(&self, r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &NRGBA{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &NRGBA{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }
}

/// NRGBA64 is an in-memory image whose At method returns color::NRGBA64 values.
#[derive(Debug)]
pub struct NRGBA64 {
    // pix holds the image's pixels, in R, G, B, A order and big-endian format. The pixel at
    // (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*8].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
}

impl Image for NRGBA64 {
    fn color_model(&self) -> color::Model {
        color::Model::NRGBA64Model
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    // fn at(&self, x: isize, y: isize) -> color::Color {
    //     color::Color::RGBA64(self.rgba64_at(x, y))
    // }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::NRGBA64(c) = c {
            self.set_nrgba64(x, y, c);
        } else {
            self.set_nrgba64(x, y, &color::NRGBA64::new_from(c));
        };
    }

    // // opaque scans the entire image and reports whether it is fully opaque.
    // fn opaque(&self) -> bool {
    //     if self.rect.empty() {
    //         return true;
    //     }
    //     let (mut i0, mut i1) = (6, self.rect.dx() * 8);
    //     for _y in self.rect.min.y..self.rect.max.y {
    //         let mut i = i0;
    //         while i < i1 {
    //             if self.pix[i ] != 0xff || self.pix[i + 1] != 0xff {
    //                 return false;
    //             }
    //             i += 8;
    //         }
    //         i0 += self.stride;
    //         i1 += self.stride;
    //     }
    //     return true;
    // }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::NRGBA64(self.nrgba64_at(x, y))
    }

    fn opaque(&self) -> bool {
        if self.rect.empty() {
            return true;
        }
        let (mut i0, mut i1) = (6, self.rect.dx() * 8);
        for _y in self.rect.min.y..self.rect.max.y {
            let mut i = i0;
            while i < i1 {
                if self.pix[i] != 0xff || self.pix[i + 1] != 0xff {
                    return false;
                }
                i += 8;
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        true
    }

    fn bytes_per_pixel(&self) -> usize {
        8
    }
}

impl NRGBA64 {
    /// new returns a new NRGBA64 image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(8, r, "NRGBA64")],
            stride: 8 * r.dx(),
            rect: *r,
        }
    }

    fn set_nrgba64(&mut self, x: isize, y: isize, c: &color::NRGBA64) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        let s = &mut self.pix[i..i + 8];
        s[0] = (c.r >> 8) as u8;
        s[1] = c.r as u8;
        s[2] = (c.g >> 8) as u8;
        s[3] = c.g as u8;
        s[4] = (c.b >> 8) as u8;
        s[5] = c.b as u8;
        s[6] = (c.a >> 8) as u8;
        s[7] = c.a as u8;
    }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize * 8
    }

    // fn (p *NRGBA64) RGBA64At(x: isize, y: isize) color::RGBA64 {
    // 	r, g, b, a := self.nrgba64_at(x, y).rgba()
    // 	return color::RGBA64{uint16(r), uint16(g), uint16(b), uint16(a)}
    // }

    fn nrgba64_at(&self, x: isize, y: isize) -> color::NRGBA64 {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::NRGBA64::new(0, 0, 0, 0);
        }
        let i = self.pix_offset(x, y);
        let s = &self.pix[i..i + 8];
        color::NRGBA64::new(
            (s[0] as u16) << 8 | (s[1] as u16),
            (s[2] as u16) << 8 | (s[3] as u16),
            (s[4] as u16) << 8 | (s[5] as u16),
            (s[6] as u16) << 8 | (s[7] as u16),
        )
    }

    // fn (p *NRGBA64) SetRGBA64(x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	r, g, b, a := uint32(c.r), uint32(c.g), uint32(c.b), uint32(c.a)
    // 	if (a != 0) && (a != 0xffff) {
    // 		r = (r * 0xffff) / a
    // 		g = (g * 0xffff) / a
    // 		b = (b * 0xffff) / a
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	s := self.pix[i..i+8 : i+8] // Small cap improves performance, see https://golang.org/issue/27857
    // 	s[0] = u8(r >> 8)
    // 	s[1] = u8(r)
    // 	s[2] = u8(g >> 8)
    // 	s[3] = u8(g)
    // 	s[4] = u8(b >> 8)
    // 	s[5] = u8(b)
    // 	s[6] = u8(a >> 8)
    // 	s[7] = u8(a)
    // }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn (p *NRGBA64) SubImage(r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &NRGBA64{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &NRGBA64{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }
}

/// Alpha is an in-memory image whose At method returns color::Alpha values.
#[derive(Debug)]
pub struct Alpha {
    // pix holds the image's pixels, as alpha values. The pixel at
    // (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*1].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
}

impl Image for Alpha {
    fn color_model(&self) -> color::Model {
        color::Model::AlphaModel
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::Alpha(self.alpha_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::Alpha(c) = c {
            self.set_alpha(x, y, c);
        } else {
            self.set_alpha(x, y, &color::Alpha::new_from(c));
        };
    }

    fn opaque(&self) -> bool {
        if self.rect.empty() {
            return true;
        }
        let (mut i0, mut i1) = (0, self.rect.dx());
        for _y in self.rect.min.y..self.rect.max.y {
            for i in i0..i1 {
                if self.pix[i] != 0xff {
                    return false;
                }
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn bytes_per_pixel(&self) -> usize {
        1
    }
}

impl Alpha {
    /// new returns a new Alpha image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(1, r, "Alpha")],
            stride: r.dx(),
            rect: *r,
        }
    }

    fn alpha_at(&self, x: isize, y: isize) -> color::Alpha {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::Alpha::new(0);
        }
        let i = self.pix_offset(x, y);
        color::Alpha::new(self.pix[i])
    }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize
    }

    // fn RGBA64At(&self, x: isize, y: isize) color::RGBA64 {
    // 	a := uint16(self.alpha_at(x, y).a)
    // 	a |= a << 8
    // 	return color::RGBA64{a, a, a, a}
    // }

    // fn SetRGBA64(&self, x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	self.pix[i] = u8(c.a >> 8)
    // }

    fn set_alpha(&mut self, x: isize, y: isize, c: &color::Alpha) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = c.a;
    }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn SubImage(&self, r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &Alpha{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &Alpha{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }
}

/// Alpha16 is an in-memory image whose At method returns color::Alpha16 values.
#[derive(Debug)]
pub struct Alpha16 {
    // pix holds the image's pixels, as alpha values. The pixel at
    // (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*1].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
}

impl Image for Alpha16 {
    fn color_model(&self) -> color::Model {
        color::Model::Alpha16Model
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::Alpha16(self.alpha16_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::Alpha16(c) = c {
            self.set_alpha16(x, y, c);
        } else {
            self.set_alpha16(x, y, &color::Alpha16::new_from(c));
        };
    }

    fn opaque(&self) -> bool {
        if self.rect.empty() {
            return true;
        }
        let (mut i0, mut i1) = (0, self.rect.dx() * 2);
        for _y in self.rect.min.y..self.rect.max.y {
            let mut i = i0;
            while i < i1 {
                if self.pix[i] != 0xff || self.pix[i + 1] != 0xff {
                    return false;
                }
                i += 2;
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn bytes_per_pixel(&self) -> usize {
        2
    }
}

impl Alpha16 {
    /// new returns a new Alpha16 image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(2, r, "Alpha16")],
            stride: 2 * r.dx(),
            rect: *r,
        }
    }

    fn alpha16_at(&self, x: isize, y: isize) -> color::Alpha16 {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::Alpha16::new(0);
        }
        let i = self.pix_offset(x, y);
        color::Alpha16::new((self.pix[i] as u16) << 8 | (self.pix[i + 1] as u16))
    }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize * 2
    }

    // fn (p *Alpha16) RGBA64At(x: isize, y: isize) color::RGBA64 {
    // 	a := self.Alpha16At(x, y).a
    // 	return color::RGBA64{a, a, a, a}
    // }

    // fn (p *Alpha16) SetRGBA64(x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	self.pix[i+0] = u8(c.a >> 8)
    // 	self.pix[i+1] = u8(c.a)
    // }

    fn set_alpha16(&mut self, x: isize, y: isize, c: &color::Alpha16) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = (c.a >> 8) as u8;
        self.pix[i + 1] = (c.a) as u8;
    }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn (p *Alpha16) SubImage(r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &Alpha16{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &Alpha16{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }
}

/// Gray is an in-memory image whose At method returns color::Gray values.
#[derive(Debug)]
pub struct Gray {
    /// pix holds the image's pixels, as gray values. The pixel at
    /// (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*1].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
}

impl Image for Gray {
    fn color_model(&self) -> color::Model {
        color::Model::GrayModel
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    /// opaque scans the entire image and reports whether it is fully opaque.
    fn opaque(&self) -> bool {
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::Gray(self.gray_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::Gray(c) = c {
            self.set_gray(x, y, c);
        } else {
            self.set_gray(x, y, &color::Gray::new_from(c));
        };
    }

    fn bytes_per_pixel(&self) -> usize {
        1
    }
}

impl Gray {
    /// new returns a new Gray image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(1, r, "Gray")],
            stride: r.dx(),
            rect: *r,
        }
    }

    // fn (p *Gray) RGBA64At(x: isize, y: isize) color::RGBA64 {
    // 	gray := uint16(self.GrayAt(x, y).y)
    // 	gray |= gray << 8
    // 	return color::RGBA64{gray, gray, gray, 0xffff}
    // }

    pub fn gray_at(&self, x: isize, y: isize) -> color::Gray {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::Gray::new(0);
        }
        let i = self.pix_offset(x, y);
        color::Gray::new(self.pix[i])
    }

    // pix_offset returns the index of the first element of pix that corresponds to
    // the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize
    }

    fn set_gray(&mut self, x: isize, y: isize, c: &color::Gray) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = c.y;
    }

    // fn (p *Gray) SetRGBA64(x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	// This formula is the same as in color::grayModel.
    // 	gray := (19595*uint32(c.r) + 38470*uint32(c.g) + 7471*uint32(c.b) + 1<<15) >> 24
    // 	let i = self.pix_offset(x, y);
    // 	self.pix[i] = u8(gray)
    // }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn (p *Gray) SubImage(r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &Gray{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &Gray{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }
}

/// Gray16 is an in-memory image whose At method returns color::Gray16 values.
#[derive(Debug)]
pub struct Gray16 {
    /// pix holds the image's pixels, as gray values in big-endian format. The pixel at
    /// (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*2].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
}

impl Image for Gray16 {
    fn color_model(&self) -> color::Model {
        color::Model::Gray16Model
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    /// opaque scans the entire image and reports whether it is fully opaque.
    fn opaque(&self) -> bool {
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        color::Color::Gray16(self.gray16_at(x, y))
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if let color::Color::Gray16(c) = c {
            self.set_gray16(x, y, c);
        } else {
            self.set_gray16(x, y, &color::Gray16::new_from(c));
        };
    }

    fn bytes_per_pixel(&self) -> usize {
        2
    }
}

impl Gray16 {
    /// new returns a new Gray16 image with the given bounds.
    pub fn new(r: &Rectangle) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(2, r, "Gray16")],
            stride: 2 * r.dx(),
            rect: *r,
        }
    }

    pub fn gray16_at(&self, x: isize, y: isize) -> color::Gray16 {
        if !(Point::new(x, y).inside(&self.rect)) {
            return color::Gray16::new(0);
        }
        let i = self.pix_offset(x, y);
        color::Gray16::new(((self.pix[i] as u16) << 8) | (self.pix[i + 1] as u16))
    }

    // pix_offset returns the index of the first element of pix that corresponds to
    // the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize * 2
    }

    fn set_gray16(&mut self, x: isize, y: isize, c: &color::Gray16) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = (c.y >> 8) as u8;
        self.pix[i + 1] = (c.y) as u8;
    }

    // fn (p *Gray16) color_model() color::Model { return color::Gray16Model }

    // fn (p *Gray16) bounds() Rectangle { return self.rect }

    // fn (p *Gray16) At(x: isize, y: isize) color::Color {
    // 	return self.Gray16At(x, y)
    // }

    // fn (p *Gray16) RGBA64At(x: isize, y: isize) color::RGBA64 {
    // 	gray := self.Gray16At(x, y).y
    // 	return color::RGBA64{gray, gray, gray, 0xffff}
    // }

    // // pix_offset returns the index of the first element of pix that corresponds to
    // // the pixel at (x, y).
    // fn (p *Gray16) pix_offset(x: isize, y: isize) isize {
    // 	return (y-self.rect.min.y)*self.stride + (x-self.rect.min.x)*2
    // }

    // fn (p *Gray16) SetRGBA64(x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	// This formula is the same as in color::gray16Model.
    // 	gray := (19595*uint32(c.r) + 38470*uint32(c.g) + 7471*uint32(c.b) + 1<<15) >> 16
    // 	let i = self.pix_offset(x, y);
    // 	self.pix[i+0] = u8(gray >> 8)
    // 	self.pix[i+1] = u8(gray)
    // }

    // fn (p *Gray16) SetGray16(x: isize, y: isize, c color::Gray16) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	self.pix[i+0] = u8(c.y >> 8)
    // 	self.pix[i+1] = u8(c.y)
    // }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn (p *Gray16) SubImage(r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &Gray16{}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &Gray16{
    // 		pix:    self.pix[i:],
    // 		stride: self.stride,
    // 		Rect:   r,
    // 	}
    // }

    // // opaque scans the entire image and reports whether it is fully opaque.
    // fn (p *Gray16) opaque() bool {
    // 	return true
    // }
}

// // CMYK is an in-memory image whose At method returns color::CMYK values.
// type CMYK struct {
// 	// pix holds the image's pixels, in C, M, Y, K order. The pixel at
// 	// (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*4].
// 	pix [u8]
// 	// stride is the pix stride (in bytes) between vertically adjacent pixels.
// 	stride isize
// 	// Rect is the image's bounds.
// 	Rect Rectangle
// }

// fn (p *CMYK) color_model() color::Model { return color::CMYKModel }

// fn (p *CMYK) bounds() Rectangle { return self.rect }

// fn (p *CMYK) At(x: isize, y: isize) color::Color {
// 	return self.CMYKAt(x, y)
// }

// fn (p *CMYK) RGBA64At(x: isize, y: isize) color::RGBA64 {
// 	r, g, b, a := self.CMYKAt(x, y).rgba()
// 	return color::RGBA64{uint16(r), uint16(g), uint16(b), uint16(a)}
// }

// fn (p *CMYK) CMYKAt(x: isize, y: isize) color::CMYK {
// 	if !Point::new(x, y).inside(&self.rect) {
// 		return color::CMYK{}
// 	}
// 	let i = self.pix_offset(x, y);
// 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 	return color::CMYK{s[0], s[1], s[2], s[3]}
// }

// // pix_offset returns the index of the first element of pix that corresponds to
// // the pixel at (x, y).
// fn (p *CMYK) pix_offset(x: isize, y: isize) isize {
// 	return (y-self.rect.min.y)*self.stride + (x-self.rect.min.x)*4
// }

// fn (p *CMYK) Set(x: isize, y: isize, c color::Color) {
// 	if !Point::new(x, y).inside(&self.rect) {
// 		return
// 	}
// 	let i = self.pix_offset(x, y);
// 	c1 := color::CMYKModel.Convert(c).(color::CMYK)
// 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 	s[0] = c1.C
// 	s[1] = c1.M
// 	s[2] = c1.y
// 	s[3] = c1.K
// }

// fn (p *CMYK) SetRGBA64(x: isize, y: isize, c color::RGBA64) {
// 	if !Point::new(x, y).inside(&self.rect) {
// 		return
// 	}
// 	cc, mm, yy, kk := color::RGBToCMYK(u8(c.r>>8), u8(c.g>>8), u8(c.b>>8))
// 	let i = self.pix_offset(x, y);
// 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 	s[0] = cc
// 	s[1] = mm
// 	s[2] = yy
// 	s[3] = kk
// }

// fn (p *CMYK) SetCMYK(x: isize, y: isize, c color::CMYK) {
// 	if !Point::new(x, y).inside(&self.rect) {
// 		return
// 	}
// 	let i = self.pix_offset(x, y);
// 	s := self.pix[i..i+4 : i+4] // Small cap improves performance, see https://golang.org/issue/27857
// 	s[0] = c.C
// 	s[1] = c.M
// 	s[2] = c.y
// 	s[3] = c.K
// }

// // SubImage returns an image representing the portion of the image p visible
// // through r. The returned value shares pixels with the original image.
// fn (p *CMYK) SubImage(r Rectangle) Image {
// 	r = r.intersect(self.rect)
// 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
// 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
// 	// this, the pix[i:] expression below can panic.
// 	if r.Empty() {
// 		return &CMYK{}
// 	}
// 	let i = self.pix_offset(r.min.x, r.min.y)
// 	return &CMYK{
// 		pix:    self.pix[i:],
// 		stride: self.stride,
// 		Rect:   r,
// 	}
// }

// // opaque scans the entire image and reports whether it is fully opaque.
// fn (p *CMYK) opaque() bool {
// 	return true
// }

// // NewCMYK returns a new CMYK image with the given bounds.
// fn NewCMYK(r Rectangle) *CMYK {
// 	return &CMYK{
// 		pix:    make([u8], pixel_buffer_length(4, r, "CMYK")),
// 		stride: 4 * r.dx(),
// 		Rect:   r,
// 	}
// }

/// Paletted is an in-memory image of u8 indices into a given palette.
#[derive(Debug)]
pub struct Paletted {
    /// pix holds the image's pixels, as palette indices. The pixel at
    /// (x, y) starts at pix[(y-Rect.min.y)*stride + (x-Rect.min.x)*1].
    pub pix: Vec<u8>,
    /// stride is the pix stride (in bytes) between vertically adjacent pixels.
    pub stride: usize,
    /// rect is the image's bounds.
    rect: Rectangle,
    /// palette is the image's palette.
    pub(super) palette: color::Palette,
}

impl Image for Paletted {
    fn color_model(&self) -> color::Model {
        color::Model::Paletted(self.palette.clone())
    }

    fn bounds(&self) -> &Rectangle {
        &self.rect
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        if self.palette.colors.is_empty() {
            return color::Color::new_rgba(0, 0, 0, 0);
        }
        if !(Point::new(x, y).inside(&self.rect)) {
            return self.palette.colors[0];
        }
        let i = self.pix_offset(x, y);
        self.palette.colors[self.pix[i] as usize]
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        if !(Point::new(x, y).inside(&self.rect)) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = self.palette.index(c) as u8;
    }

    // opaque scans the entire image and reports whether it is fully opaque.
    fn opaque(&self) -> bool {
        let mut present = [false; 256];
        let (mut i0, mut i1) = (0, self.rect.dx());
        for _y in self.rect.min.y..self.rect.max.y {
            for c in &self.pix[i0..i1] {
                present[*c as usize] = true;
            }
            i0 += self.stride;
            i1 += self.stride;
        }
        for (i, c) in self.palette.colors.iter().enumerate() {
            if !present[i] {
                continue;
            }
            let (_, _, _, a) = c.rgba();
            if a != 0xffff {
                return false;
            }
        }
        true
    }

    fn stride(&self) -> usize {
        self.stride
    }

    fn pix(&self) -> &Vec<u8> {
        &self.pix
    }

    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        &mut self.pix
    }

    fn bytes_per_pixel(&self) -> usize {
        1
    }
}

impl Paletted {
    /// new returns a new Paletted image with the given width, height and
    /// palette.
    pub fn new(r: &Rectangle, p: color::Palette) -> Self {
        Self {
            pix: vec![0; pixel_buffer_length(1, r, "Paletted")],
            stride: r.dx(),
            rect: *r,
            palette: p,
        }
    }

    // fn RGBA64At(&self, x: isize, y: isize) color::RGBA64 {
    // 	if self.palette.colors.len() == 0 {
    // 		return color::RGBA64{}
    // 	}
    // 	c := color::Color(nil)
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		c = self.palette.colors[0]
    // 	} else {
    // 		i := self.pix_offset(x, y)
    // 		c = self.palette.colors[self.pix[i]]
    // 	}
    // 	r, g, b, a := c.rgba()
    // 	return color::RGBA64{
    // 		uint16(r),
    // 		uint16(g),
    // 		uint16(b),
    // 		uint16(a),
    // 	}
    // }

    /// pix_offset returns the index of the first element of pix that corresponds to
    /// the pixel at (x, y).
    pub fn pix_offset(&self, x: isize, y: isize) -> usize {
        (y - self.rect.min.y) as usize * self.stride + (x - self.rect.min.x) as usize
    }

    // fn SetRGBA64(&self, x: isize, y: isize, c color::RGBA64) {
    // 	if !Point::new(x, y).inside(&self.rect) {
    // 		return
    // 	}
    // 	let i = self.pix_offset(x, y);
    // 	self.pix[i] = u8(self.palette.Index(c))
    // }

    pub fn color_index_at(&self, x: isize, y: isize) -> u8 {
        if !(Point::new(x, y).inside(&self.rect)) {
            return 0;
        }
        let i = self.pix_offset(x, y);
        self.pix[i]
    }

    pub fn set_color_index(&mut self, x: isize, y: isize, index: u8) {
        if !Point::new(x, y).inside(&self.rect) {
            return;
        }
        let i = self.pix_offset(x, y);
        self.pix[i] = index;
    }

    // // SubImage returns an image representing the portion of the image p visible
    // // through r. The returned value shares pixels with the original image.
    // fn SubImage(&self, r Rectangle) Image {
    // 	r = r.intersect(self.rect)
    // 	// If r1 and r2 are Rectangles, r1.intersect(r2) is not guaranteed to be inside
    // 	// either r1 or r2 if the intersection is empty. Without explicitly checking for
    // 	// this, the pix[i:] expression below can panic.
    // 	if r.Empty() {
    // 		return &Paletted{
    // 			Palette: self.palette,
    // 		}
    // 	}
    // 	let i = self.pix_offset(r.min.x, r.min.y)
    // 	return &Paletted{
    // 		pix:     self.pix[i:],
    // 		stride:  self.stride,
    // 		Rect:    self.rect.intersect(r),
    // 		Palette: self.palette,
    // 	}
    // }
}

/// Img is a combination of all possible image types.
#[derive(Debug)]
pub enum Img {
    Alpha(Alpha),
    Alpha16(Alpha16),
    Gray(Gray),
    Gray16(Gray16),
    NRGBA(NRGBA),
    NRGBA64(NRGBA64),
    Paletted(Paletted),
    RGBA(RGBA),
    RGBA64(RGBA64),
}

impl Img {
    /// new_rgba creates a new RGBA image.
    pub fn new_rgba(r: &Rectangle) -> Self {
        Self::RGBA(RGBA::new(r))
    }
    /// new_nrgba creates a new NRGBA image.
    pub fn new_nrgba(r: &Rectangle) -> Self {
        Self::NRGBA(NRGBA::new(r))
    }
    /// new_nrgba64 creates a new NRGBA64 image.
    pub fn new_nrgba64(r: &Rectangle) -> Img {
        Img::NRGBA64(NRGBA64::new(r))
    }
    /// new_rgba64 creates a new RGBA64 image.
    pub fn new_rgba64(r: &Rectangle) -> Img {
        Img::RGBA64(RGBA64::new(r))
    }
}

impl Image for Img {
    fn bytes_per_pixel(&self) -> usize {
        match self {
            Img::Paletted(img) => img.bytes_per_pixel(),
            Img::RGBA(img) => img.bytes_per_pixel(),
            Img::NRGBA(img) => img.bytes_per_pixel(),
            Img::RGBA64(img) => img.bytes_per_pixel(),
            Img::NRGBA64(img) => img.bytes_per_pixel(),
            Img::Alpha(img) => img.bytes_per_pixel(),
            Img::Alpha16(img) => img.bytes_per_pixel(),
            Img::Gray(img) => img.bytes_per_pixel(),
            Img::Gray16(img) => img.bytes_per_pixel(),
        }
    }
    fn stride(&self) -> usize {
        match self {
            Img::Paletted(img) => img.stride(),
            Img::RGBA(img) => img.stride(),
            Img::NRGBA(img) => img.stride(),
            Img::RGBA64(img) => img.stride(),
            Img::NRGBA64(img) => img.stride(),
            Img::Alpha(img) => img.stride(),
            Img::Alpha16(img) => img.stride(),
            Img::Gray(img) => img.stride(),
            Img::Gray16(img) => img.stride(),
        }
    }
    fn bounds(&self) -> &Rectangle {
        match self {
            Img::Paletted(img) => img.bounds(),
            Img::RGBA(img) => img.bounds(),
            Img::NRGBA(img) => img.bounds(),
            Img::RGBA64(img) => img.bounds(),
            Img::NRGBA64(img) => img.bounds(),
            Img::Alpha(img) => img.bounds(),
            Img::Alpha16(img) => img.bounds(),
            Img::Gray(img) => img.bounds(),
            Img::Gray16(img) => img.bounds(),
        }
    }
    fn pix(&self) -> &Vec<u8> {
        match self {
            Img::Paletted(img) => img.pix(),
            Img::RGBA(img) => img.pix(),
            Img::NRGBA(img) => img.pix(),
            Img::RGBA64(img) => img.pix(),
            Img::NRGBA64(img) => img.pix(),
            Img::Alpha(img) => img.pix(),
            Img::Alpha16(img) => img.pix(),
            Img::Gray(img) => img.pix(),
            Img::Gray16(img) => img.pix(),
        }
    }
    fn get_pix_mutable(&mut self) -> &mut Vec<u8> {
        match self {
            Img::Paletted(img) => img.get_pix_mutable(),
            Img::RGBA(img) => img.get_pix_mutable(),
            Img::NRGBA(img) => img.get_pix_mutable(),
            Img::RGBA64(img) => img.get_pix_mutable(),
            Img::NRGBA64(img) => img.get_pix_mutable(),
            Img::Alpha(img) => img.get_pix_mutable(),
            Img::Alpha16(img) => img.get_pix_mutable(),
            Img::Gray(img) => img.get_pix_mutable(),
            Img::Gray16(img) => img.get_pix_mutable(),
        }
    }

    fn color_model(&self) -> color::Model {
        match self {
            Img::Paletted(img) => img.color_model(),
            Img::RGBA(img) => img.color_model(),
            Img::NRGBA(img) => img.color_model(),
            Img::RGBA64(img) => img.color_model(),
            Img::NRGBA64(img) => img.color_model(),
            Img::Alpha(img) => img.color_model(),
            Img::Alpha16(img) => img.color_model(),
            Img::Gray(img) => img.color_model(),
            Img::Gray16(img) => img.color_model(),
        }
    }

    fn at(&self, x: isize, y: isize) -> color::Color {
        match self {
            Img::Paletted(img) => img.at(x, y),
            Img::RGBA(img) => img.at(x, y),
            Img::NRGBA(img) => img.at(x, y),
            Img::RGBA64(img) => img.at(x, y),
            Img::NRGBA64(img) => img.at(x, y),
            Img::Alpha(img) => img.at(x, y),
            Img::Alpha16(img) => img.at(x, y),
            Img::Gray(img) => img.at(x, y),
            Img::Gray16(img) => img.at(x, y),
        }
    }

    fn set(&mut self, x: isize, y: isize, c: &color::Color) {
        match self {
            Img::Paletted(img) => img.set(x, y, c),
            Img::RGBA(img) => img.set(x, y, c),
            Img::NRGBA(img) => img.set(x, y, c),
            Img::RGBA64(img) => img.set(x, y, c),
            Img::NRGBA64(img) => img.set(x, y, c),
            Img::Alpha(img) => img.set(x, y, c),
            Img::Alpha16(img) => img.set(x, y, c),
            Img::Gray(img) => img.set(x, y, c),
            Img::Gray16(img) => img.set(x, y, c),
        }
    }

    fn opaque(&self) -> bool {
        match self {
            Img::Paletted(img) => img.opaque(),
            Img::RGBA(img) => img.opaque(),
            Img::NRGBA(img) => img.opaque(),
            Img::RGBA64(img) => img.opaque(),
            Img::NRGBA64(img) => img.opaque(),
            Img::Alpha(img) => img.opaque(),
            Img::Alpha16(img) => img.opaque(),
            Img::Gray(img) => img.opaque(),
            Img::Gray16(img) => img.opaque(),
        }
    }
}
