// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::{color, color::ColorTrait, image, rect, Image};
// import (
// 	"image/color"
// 	"image/color/palette"
// 	"testing"
// )

// type image interface {
// 	Image
// 	Opaque() bool
// 	Set(int, int, color::Color)
// 	SubImage(Rectangle) Image
// }

fn cmp(cm: color::Model, c0: &color::Color, c1: &color::Color) -> bool {
    let (r0, g0, b0, a0) = cm.convert(c0).rgba();
    let (r1, g1, b1, a1) = cm.convert(c1).rgba();
    r0 == r1 && g0 == g1 && b0 == b1 && a0 == a1
}

#[derive(Debug)]
struct TestCase {
    name: &'static str,
    image: fn() -> Box<dyn Image>,
}

const TEST_IMAGES: &[TestCase] = &[
    TestCase {
        name: "rgba",
        image: || return Box::new(super::RGBA::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "rgba64",
        image: || return Box::new(super::RGBA64::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "nrgba",
        image: || return Box::new(super::NRGBA::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "nrgba64",
        image: || return Box::new(super::NRGBA64::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "alpha",
        image: || return Box::new(super::Alpha::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "alpha16",
        image: || return Box::new(super::Alpha16::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "gray",
        image: || return Box::new(super::Gray::new(&rect(0, 0, 10, 10))),
    },
    TestCase {
        name: "gray16",
        image: || return Box::new(super::Gray16::new(&rect(0, 0, 10, 10))),
    },
    // TestCase	{name:"paletted", image:|| return
    // 		return NewPaletted(rect(0, 0, 10, 10), color::Palette{
    // 			TRANSPARENT,
    // 			&color::OPAQUE,
    // 		})
    // 	}},
];

#[test]
fn test_image() {
    for tc in TEST_IMAGES {
        let mut m = (tc.image)();
        assert!(
            rect(0, 0, 10, 10).eq(&m.bounds()),
            "{:?}: want bounds {:?}, got {:?}",
            tc.name,
            rect(0, 0, 10, 10),
            m.bounds()
        );
        assert!(
            cmp(m.color_model(), &color::TRANSPARENT, &m.at(6, 3)),
            "{}: at (6, 3), want a zero color, got {:?}",
            tc.name,
            &m.at(6, 3)
        );
        m.set(6, 3, &color::OPAQUE);
        assert!(
            cmp(m.color_model(), &color::OPAQUE, &m.at(6, 3)),
            "{}: at (6, 3), want a non-zero color, got {:?}",
            tc.name,
            &m.at(6, 3)
        );
        // 		if !m.SubImage(rect(6, 3, 7, 4)).(image).Opaque() {
        // 			t.Errorf("{}: at (6, 3) was not opaque", m)
        // 			continue
        // 		}
        // 		m = m.SubImage(rect(3, 2, 9, 8)).(image)
        // 		if !rect(3, 2, 9, 8).eq(&m.bounds()) {
        // 			t.Errorf("{}: sub-image want bounds {}, got {}", m, rect(3, 2, 9, 8), m.bounds())
        // 			continue
        // 		}
        // 		if !cmp(m.color_model(), &color::OPAQUE, m.at(6, 3)) {
        // 			t.Errorf("{}: sub-image at (6, 3), want a non-zero color, got {}", m, m.at(6, 3))
        // 			continue
        // 		}
        // 		if !cmp(m.color_model(), TRANSPARENT, m.at(3, 3)) {
        // 			t.Errorf("{}: sub-image at (3, 3), want a zero color, got {}", m, m.at(3, 3))
        // 			continue
        // 		}
        // 		m.set(3, 3, Opaque)
        // 		if !cmp(m.color_model(), &color::OPAQUE, m.at(3, 3)) {
        // 			t.Errorf("{}: sub-image at (3, 3), want a non-zero color, got {}", m, m.at(3, 3))
        // 			continue
        // 		}
        // 		// Test that taking an empty sub-image starting at a corner does not panic.
        // 		m.SubImage(rect(0, 0, 0, 0))
        // 		m.SubImage(rect(10, 0, 10, 0))
        // 		m.SubImage(rect(0, 10, 0, 10))
        // 		m.SubImage(rect(10, 10, 10, 10))
    }
}

// #[test]
// fn TestNewXxxBadRectangle() {
// 	// call calls f(r) and reports whether it ran without panicking.
// 	call := fn(f fn(Rectangle), r Rectangle) (ok bool) {
// 		defer fn() {
// 			if recover() != nil {
// 				ok = false
// 			}
// 		}()
// 		f(r)
// 		return true
// 	}

// 	testCases := []struct {
// 		name string
// 		f    fn(Rectangle)
// 	}{
// 		{"RGBA", fn(r Rectangle) { NewRGBA(r) }},
// 		{"RGBA64", fn(r Rectangle) { NewRGBA64(r) }},
// 		{"NRGBA", fn(r Rectangle) { NewNRGBA(r) }},
// 		{"NRGBA64", fn(r Rectangle) { NewNRGBA64(r) }},
// 		{"Alpha", fn(r Rectangle) { NewAlpha(r) }},
// 		{"Alpha16", fn(r Rectangle) { NewAlpha16(r) }},
// 		{"Gray", fn(r Rectangle) { NewGray(r) }},
// 		{"Gray16", fn(r Rectangle) { NewGray16(r) }},
// 		{"CMYK", fn(r Rectangle) { NewCMYK(r) }},
// 		{"Paletted", fn(r Rectangle) { NewPaletted(r, color::Palette{color::Black, color::White}) }},
// 		{"YCbCr", fn(r Rectangle) { NewYCbCr(r, YCbCrSubsampleRatio422) }},
// 		{"NYCbCrA", fn(r Rectangle) { NewNYCbCrA(r, YCbCrSubsampleRatio444) }},
// 	}

// 	for _, tc := range testCases {
// 		// Calling NewXxx(r) should fail (panic, since NewXxx doesn't return an
// 		// error) unless r's width and height are both non-negative.
// 		for _, negDx := range []bool{false, true} {
// 			for _, negDy := range []bool{false, true} {
// 				r := Rectangle{
// 					Min: Point{15, 28},
// 					Max: Point{16, 29},
// 				}
// 				if negDx {
// 					r.max.x = 14
// 				}
// 				if negDy {
// 					r.max.y = 27
// 				}

// 				got := call(tc.f, r)
// 				want := !negDx && !negDy
// 				if got != want {
// 					t.Errorf("New%s: negDx=%t, negDy=%t: got %t, want %t",
// 						tc.name, negDx, negDy, got, want)
// 				}
// 			}
// 		}

// 		// Passing a Rectangle whose width and height is MaxInt should also fail
// 		// (panic), due to overflow.
// 		{
// 			zeroAsUint := uint(0)
// 			maxUint := zeroAsUint - 1
// 			maxInt := int(maxUint / 2)
// 			got := call(tc.f, Rectangle{
// 				Min: Point{0, 0},
// 				Max: Point{maxInt, maxInt},
// 			})
// 			if got {
// 				t.Errorf("New%s: overflow: got ok, want !ok", tc.name)
// 			}
// 		}
// 	}
// }

#[test]
fn test16_bits_per_color_channel() {
    let test_color_model = &[
        color::Model::RGBA64Model,
        color::Model::NRGBA64Model,
        color::Model::Alpha16Model,
        color::Model::Gray16Model,
    ];
    for cm in test_color_model {
        let c = cm.convert(&color::Color::new_rgba64(0x1234, 0x1234, 0x1234, 0x1234));
        // 		c := cm.Convert(color::RGBA64::new(0x1234, 0x1234, 0x1234, 0x1234)) // Premultiplied alpha.
        let (r, _, _, _) = c.rgba();
        assert_eq!(
            r, 0x1234,
            "{:?}: want red value {:04x} got {:04x}",
            c, 0x1234, r
        );
    }
    let test_image: &mut [Box<dyn Image>] = &mut [
        Box::new(image::RGBA64::new(&rect(0, 0, 10, 10))),
        Box::new(image::NRGBA64::new(&rect(0, 0, 10, 10))),
        Box::new(image::Alpha16::new(&rect(0, 0, 10, 10))),
        Box::new(image::Gray16::new(&rect(0, 0, 10, 10))),
    ];
    for m in test_image.iter_mut() {
        m.set(
            1,
            2,
            // Non-premultiplied alpha.
            &color::Color::new_nrgba64(0xffff, 0xffff, 0xffff, 0x1357),
        );
        let (r, _, _, _) = m.at(1, 2).rgba();
        assert_eq!(
            r,
            0x1357,
            "{:?}: want red value {:04x} got {:04x}",
            m.color_model(),
            0x1357,
            r
        );
    }
}

// #[test]
// fn TestRGBA64Image() {
// 	// memset sets every element of s to v.
// 	memset := fn(s []byte, v byte) {
// 		for i := range s {
// 			s[i] = v
// 		}
// 	}

// 	r := rect(0, 0, 3, 2)
// 	testCases := []Image{
// 		NewAlpha(r),
// 		NewAlpha16(r),
// 		NewCMYK(r),
// 		NewGray(r),
// 		NewGray16(r),
// 		NewNRGBA(r),
// 		NewNRGBA64(r),
// 		NewNYCbCrA(r, YCbCrSubsampleRatio444),
// 		NewPaletted(r, palette.Plan9),
// 		NewRGBA(r),
// 		NewRGBA64(r),
// 		NewUniform(color::RGBA64{}),
// 		NewYCbCr(r, YCbCrSubsampleRatio444),
// 		r,
// 	}
// 	for _, tc := range testCases {
// 		switch tc := tc.(type) {
// 		// Most of the concrete image types in the testCases implement the
// 		// draw.RGBA64Image interface: they have a SetRGBA64 method. We use an
// 		// interface literal here, instead of importing "image/draw", to avoid
// 		// an import cycle.
// 		//
// 		// The YCbCr and NYCbCrA types are special-cased. Chroma subsampling
// 		// means that setting one pixel can modify neighboring pixels. They
// 		// don't have Set or SetRGBA64 methods because that side effect could
// 		// be surprising. Here, we just memset the channel buffers instead.
// 		//
// 		// The Uniform and Rectangle types are also special-cased, as they
// 		// don't have a Set or SetRGBA64 method.
// 		case interface {
// 			SetRGBA64(x, y int, c color::RGBA64)
// 		}:
// 			tc.SetRGBA64(1, 1, color::RGBA64{0x7FFF, 0x3FFF, 0x0000, 0x7FFF})

// 		case *NYCbCrA:
// 			memset(tc.YCbCr.y, 0x77)
// 			memset(tc.YCbCr.Cb, 0x88)
// 			memset(tc.YCbCr.Cr, 0x99)
// 			memset(tc.A, 0xAA)

// 		case *Uniform:
// 			tc.C = color::RGBA64{0x7FFF, 0x3FFF, 0x0000, 0x7FFF}

// 		case *YCbCr:
// 			memset(tc.y, 0x77)
// 			memset(tc.Cb, 0x88)
// 			memset(tc.Cr, 0x99)

// 		case Rectangle:
// 			// No-op. Rectangle pixels' colors are immutable. They're always
// 			// color::Opaque.

// 		default:
// 			t.Errorf("could not initialize pixels for {}", tc)
// 			continue
// 		}

// 		// Check that RGBA64At(x, y) is equivalent to At(x, y).rgba().
// 		rgba64Image, ok := tc.(RGBA64Image)
// 		if !ok {
// 			t.Errorf("{} is not an RGBA64Image", tc)
// 			continue
// 		}
// 		got := rgba64Image.RGBA64At(1, 1)
// 		wantR, wantG, wantB, wantA := tc.at(1, 1).rgba()
// 		if (uint32(got.R) != wantR) || (uint32(got.G) != wantG) ||
// 			(uint32(got.B) != wantB) || (uint32(got.A) != wantA) {
// 			t.Errorf("{}:\ngot  (0x%04X, 0x%04X, 0x%04X, 0x%04X)\n"+
// 				"want (0x%04X, 0x%04X, 0x%04X, 0x%04X)", tc,
// 				got.R, got.G, got.B, got.A,
// 				wantR, wantG, wantB, wantA)
// 			continue
// 		}
// 	}
// }

// fn BenchmarkAt(b *testing.B) {
// 	for _, tc := range TEST_IMAGES {
// 		b.Run(tc.name, fn(b *testing.B) {
// 			m := tc.image()
// 			b.ReportAllocs()
// 			b.ResetTimer()
// 			for i := 0; i < b.N; i++ {
// 				m.at(4, 5)
// 			}
// 		})
// 	}
// }

// fn BenchmarkSet(b *testing.B) {
// 	c := color::Gray{0xff}
// 	for _, tc := range TEST_IMAGES {
// 		b.Run(tc.name, fn(b *testing.B) {
// 			m := tc.image()
// 			b.ReportAllocs()
// 			b.ResetTimer()
// 			for i := 0; i < b.N; i++ {
// 				m.set(4, 5, c)
// 			}
// 		})
// 	}
// }

// fn Benchmarkrgba_at(b *testing.B) {
// 	m := NewRGBA(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.rgba_at(4, 5)
// 	}
// }

// fn BenchmarkRGBASetRGBA(b *testing.B) {
// 	m := NewRGBA(rect(0, 0, 10, 10))
// 	c := color::RGBA{0xff, 0xff, 0xff, 0x13}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetRGBA(4, 5, c)
// 	}
// }

// fn BenchmarkRGBA64At(b *testing.B) {
// 	m := NewRGBA64(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.RGBA64At(4, 5)
// 	}
// }

// fn BenchmarkRGBA64SetRGBA64(b *testing.B) {
// 	m := NewRGBA64(rect(0, 0, 10, 10))
// 	c := color::RGBA64{0xffff, 0xffff, 0xffff, 0x1357}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetRGBA64(4, 5, c)
// 	}
// }

// fn BenchmarkNrgba_at(b *testing.B) {
// 	m := NewNRGBA(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.Nrgba_at(4, 5)
// 	}
// }

// fn BenchmarkNRGBASetNRGBA(b *testing.B) {
// 	m := NewNRGBA(rect(0, 0, 10, 10))
// 	c := color::NRGBA{0xff, 0xff, 0xff, 0x13}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetNRGBA(4, 5, c)
// 	}
// }

// fn BenchmarkNRGBA64At(b *testing.B) {
// 	m := NewNRGBA64(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.NRGBA64At(4, 5)
// 	}
// }

// fn BenchmarkNRGBA64SetNRGBA64(b *testing.B) {
// 	m := NewNRGBA64(rect(0, 0, 10, 10))
// 	c := color::NRGBA64{0xffff, 0xffff, 0xffff, 0x1357}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetNRGBA64(4, 5, c)
// 	}
// }

// fn BenchmarkAlphaAt(b *testing.B) {
// 	m := NewAlpha(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.AlphaAt(4, 5)
// 	}
// }

// fn BenchmarkAlphaSetAlpha(b *testing.B) {
// 	m := NewAlpha(rect(0, 0, 10, 10))
// 	c := color::Alpha{0x13}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetAlpha(4, 5, c)
// 	}
// }

// fn BenchmarkAlpha16At(b *testing.B) {
// 	m := NewAlpha16(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.Alpha16At(4, 5)
// 	}
// }

// fn BenchmarkAlphaSetAlpha16(b *testing.B) {
// 	m := NewAlpha16(rect(0, 0, 10, 10))
// 	c := color::Alpha16{0x13}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetAlpha16(4, 5, c)
// 	}
// }

// fn BenchmarkGrayAt(b *testing.B) {
// 	m := NewGray(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.GrayAt(4, 5)
// 	}
// }

// fn BenchmarkGraySetGray(b *testing.B) {
// 	m := NewGray(rect(0, 0, 10, 10))
// 	c := color::Gray{0x13}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetGray(4, 5, c)
// 	}
// }

// fn BenchmarkGray16At(b *testing.B) {
// 	m := NewGray16(rect(0, 0, 10, 10))
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.Gray16At(4, 5)
// 	}
// }

// fn BenchmarkGraySetGray16(b *testing.B) {
// 	m := NewGray16(rect(0, 0, 10, 10))
// 	c := color::Gray16{0x13}
// 	b.ResetTimer()

// 	for i := 0; i < b.N; i++ {
// 		m.SetGray16(4, 5, c)
// 	}
// }
