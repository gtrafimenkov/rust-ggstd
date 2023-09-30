// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::bytes;
use crate::compress::zlib;
use crate::encoding::binary::{self, ByteOrder};
use crate::image::color::{self, Color, ColorTrait};
use crate::image::draw;
use crate::image::{self, png, Image, Img};
use crate::io as ggio;

fn diff(filename: &str, m0: &image::Img, m1: &image::Img) {
    let (b0, b1) = (m0.bounds(), m1.bounds());
    assert!(
        b0.eq(b1),
        "{}: dimensions differ: {:?} vs {:?}",
        filename,
        b0,
        b1
    );
    let dx = b1.min.x - b0.min.x;
    let dy = b1.min.y - b0.min.y;
    for y in b0.min.y..b0.max.y {
        for x in b0.min.x..b0.max.x {
            let c0 = m0.at(x, y);
            let c1 = m1.at(x + dx, y + dy);
            let (r0, g0, b0, a0) = c0.rgba();
            let (r1, g1, b1, a1) = c1.rgba();
            assert!(
                r0 == r1 && g0 == g1 && b0 == b1 && a0 == a1,
                "{}: colors differ at ({}, {}): {:?} vs {:?}",
                filename,
                x,
                y,
                c0,
                c1,
            );
        }
    }
}

fn encode_decode(m: &Img) -> Result<Box<Img>, png::Error> {
    let mut b = bytes::Buffer::new();
    png::encode(&mut b, m)?;
    match png::decode(&mut b) {
        Ok(img) => Ok(img),
        Err(err) => Err(png::Error::StdIo(err)),
    }
}

fn convert_to_nrgba(m: &Img) -> image::Img {
    let b = m.bounds();
    let mut ret = Img::new_nrgba(b);
    draw::draw(&mut ret, b, m, &b.min, draw::Op::Src);
    ret
}

#[test]
fn test_writer() {
    // The FILENAMES variable is declared in reader_test.go.
    let names = super::reader_test::FILENAMES;
    for filename in names {
        let qfn = format!("src/image/png/testdata/pngsuite/{}.png", filename);
        // Read the image.
        let m0 = super::reader_test::read_png(&qfn).unwrap();
        // Read the image again, encode it, and decode it.
        let m1 = super::reader_test::read_png(&qfn).unwrap();
        let m2 = encode_decode(m1.as_ref()).unwrap();
        // Compare the two.
        diff(filename, m0.as_ref(), m2.as_ref());
    }
}

#[test]
fn test_writer_paletted() {
    let (width, height) = (32, 16);

    struct TestCase {
        plen: usize,
        bitdepth: u8,
        datalen: usize,
    }

    impl TestCase {
        fn new(plen: usize, bitdepth: u8, datalen: usize) -> Self {
            Self {
                plen,
                bitdepth,
                datalen,
            }
        }
    }

    let test_cases = &[
        TestCase::new(256, 8, (1 + width) * height),
        TestCase::new(128, 8, (1 + width) * height),
        TestCase::new(16, 4, (1 + width / 2) * height),
        TestCase::new(4, 2, (1 + width / 4) * height),
        TestCase::new(2, 1, (1 + width / 8) * height),
    ];

    for tc in test_cases {
        // Create a paletted image with the correct palette length
        let mut palette = color::Palette::new(tc.plen);
        for i in 0..tc.plen {
            palette.colors[i] = Color::new_nrgba(i as u8, i as u8, i as u8, 255);
        }
        let mut m0 =
            image::Paletted::new(&image::rect(0, 0, width as isize, height as isize), palette);
        let mut i = 0;
        for y in 0..height {
            for x in 0..width {
                m0.set_color_index(x as isize, y as isize, (i % tc.plen) as u8);
                i += 1;
            }
        }

        // Encode the image
        let mut b = bytes::Buffer::new();
        png::encode(&mut b, &Img::Paletted(m0)).unwrap();
        let chunk_fields_length = 12; // 4 bytes for length, name and crc

        let data = b.bytes();
        let mut i = super::reader::PNG_HEADER.len();

        while i < data.len() - chunk_fields_length {
            let length = binary::BIG_ENDIAN.uint32(&data[i..i + 4]);
            let name = &data[i + 4..i + 8];
            match name {
                b"IHDR" => {
                    let bitdepth = data[i + 8 + 8];
                    assert_eq!(
                        bitdepth, tc.bitdepth,
                        "got bitdepth {}, want {}",
                        bitdepth, tc.bitdepth
                    );
                }
                b"IDAT" => {
                    // Uncompress the image data
                    let mut r0 = bytes::Reader::new(&data[i + 8..i + 8 + (length as usize)]);
                    let mut r = zlib::Reader::new(&mut r0).unwrap();
                    let (n, err) = ggio::copy(&mut ggio::Discard::new(), &mut r);
                    assert!(
                        err.is_none(),
                        "got error while reading image data: {:?}",
                        err
                    );
                    assert_eq!(
                        n as usize, tc.datalen,
                        "got uncompressed data length {}, want {}",
                        n, tc.datalen
                    );
                }
                _ => {}
            }

            i += chunk_fields_length + (length as usize);
        }
    }
}

#[test]
fn test_writer_levels() {
    let m = Img::new_nrgba(&image::rect(0, 0, 100, 100));
    let mut b1 = bytes::Buffer::new();
    let mut b2 = bytes::Buffer::new();
    png::Encoder::new(png::DEFAULT_COMPRESSION)
        .encode(&mut b1, &m)
        .unwrap();

    png::Encoder::new(png::NO_COMPRESSION)
        .encode(&mut b2, &m)
        .unwrap();
    assert!(
        b1.len() < b2.len(),
        "DefaultCompression encoding was larger than NoCompression encoding"
    );
    let d1 = png::decode(&mut b1);
    let d2 = png::decode(&mut b2);
    assert!(d1.is_ok(), "cannot decode DefaultCompression");
    assert!(d2.is_ok(), "cannot decode NoCompression");
}

// #[test]
// fn TestSubImage() {
// 	m0 := image.NewRGBA(&image::rect(0, 0, 256, 256))
// 	for y := 0; y < 256; y++ {
// 		for x := 0; x < 256; x++ {
// 			m0.Set(x, y, color.RGBA{u8(x), u8(y), 0, 255})
// 		}
// 	}
// 	m0 = m0.SubImage(&image::rect(50, 30, 250, 130)).(*image.RGBA)
// 	m1, err := encode_decode(m0)
// 	if err != nil {
// 		t.Error(err)
// 		return
// 	}
// 	err = diff(m0, m1)
// 	if err != nil {
// 		t.Error(err)
// 		return
// 	}
// }

#[test]
fn test_write_rgba() {
    let (width, height) = (640, 480);
    let transparent_img = Img::new_nrgba(&image::rect(0, 0, width, height));
    let mut opaque_img = Img::new_nrgba(&image::rect(0, 0, width, height));
    let mut mixed_img = Img::new_nrgba(&image::rect(0, 0, width, height));
    let mut translucent_img = Img::new_nrgba(&image::rect(0, 0, width, height));
    for y in 0..height {
        for x in 0..width {
            let opaque_color = Color::new_rgba(x as u8, y as u8, (y + x) as u8, 255);
            let translucent_color =
                Color::new_rgba((x as u8) % 128, (y as u8) % 128, ((y + x) as u8) % 128, 128);
            opaque_img.set(x, y, &opaque_color);
            translucent_img.set(x, y, &translucent_color);
            if y % 2 == 0 {
                mixed_img.set(x, y, &opaque_color);
            }
        }
    }

    struct TestCase<'a> {
        name: &'a str,
        img: Img,
    }

    impl<'a> TestCase<'a> {
        fn new(name: &'a str, img: Img) -> Self {
            Self { name, img }
        }
    }

    let test_cases = &[
        TestCase::new("Transparent RGBA", transparent_img),
        TestCase::new("Opaque RGBA", opaque_img),
        TestCase::new("50/50 Transparent/Opaque RGBA", mixed_img),
        TestCase::new("RGBA with variable alpha", translucent_img),
    ];

    for tc in test_cases {
        let m0 = &tc.img;
        let m1 = encode_decode(m0).unwrap();
        diff(tc.name, &convert_to_nrgba(m0), &m1);
    }
}

// fn BenchmarkEncodeGray(b *testing.B) {
// 	img := image.NewGray(&image::rect(0, 0, 640, 480))
// 	b.SetBytes(640 * 480 * 1)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		Encode(io.Discard, img)
// 	}
// }

// type pool struct {
// 	b *EncoderBuffer
// }

// fn (p *pool) Get() *EncoderBuffer {
// 	return p.b
// }

// fn (p *pool) Put(b *EncoderBuffer) {
// 	p.b = b
// }

// fn BenchmarkEncodeGrayWithBufferPool(b *testing.B) {
// 	img := image.NewGray(&image::rect(0, 0, 640, 480))
// 	e := Encoder{
// 		BufferPool: &pool{},
// 	}
// 	b.SetBytes(640 * 480 * 1)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		e.Encode(io.Discard, img)
// 	}
// }

// fn BenchmarkEncodeNRGBOpaque(b *testing.B) {
// 	img := image::NewNRGBA(&image::rect(0, 0, 640, 480))
// 	// Set all pixels to 0xFF alpha to force opaque mode.
// 	bo := img.bounds()
// 	for y := bo.min.y; y < bo.max.y; y++ {
// 		for x := bo.min.x; x < bo.max.x; x++ {
// 			img.Set(x, y, color.NRGBA{0, 0, 0, 255})
// 		}
// 	}
// 	if !img.Opaque() {
// 		b.Fatal("expected image to be opaque")
// 	}
// 	b.SetBytes(640 * 480 * 4)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		Encode(io.Discard, img)
// 	}
// }

// fn BenchmarkEncodeNRGBA(b *testing.B) {
// 	img := image::NewNRGBA(&image::rect(0, 0, 640, 480))
// 	if img.Opaque() {
// 		b.Fatal("expected image not to be opaque")
// 	}
// 	b.SetBytes(640 * 480 * 4)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		Encode(io.Discard, img)
// 	}
// }

// fn BenchmarkEncodePaletted(b *testing.B) {
// 	img := image.NewPaletted(&image::rect(0, 0, 640, 480), color.Palette{
// 		color.RGBA{0, 0, 0, 255},
// 		color.RGBA{255, 255, 255, 255},
// 	})
// 	b.SetBytes(640 * 480 * 1)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		Encode(io.Discard, img)
// 	}
// }

// fn BenchmarkEncodeRGBOpaque(b *testing.B) {
// 	img := image.NewRGBA(&image::rect(0, 0, 640, 480))
// 	// Set all pixels to 0xFF alpha to force opaque mode.
// 	bo := img.bounds()
// 	for y := bo.min.y; y < bo.max.y; y++ {
// 		for x := bo.min.x; x < bo.max.x; x++ {
// 			img.Set(x, y, color.RGBA{0, 0, 0, 255})
// 		}
// 	}
// 	if !img.Opaque() {
// 		b.Fatal("expected image to be opaque")
// 	}
// 	b.SetBytes(640 * 480 * 4)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		Encode(io.Discard, img)
// 	}
// }

// fn BenchmarkEncodeRGBA(b *testing.B) {
// 	const width, height = 640, 480
// 	img := image.NewRGBA(&image::rect(0, 0, width, height))
// 	for y := 0; y < height; y++ {
// 		for x := 0; x < width; x++ {
// 			percent := (x + y) % 100
// 			switch {
// 			case percent < 10: // 10% of pixels are translucent (have alpha >0 and <255)
// 				img.Set(x, y, color.NRGBA{u8(x), u8(y), u8(x * y), u8(percent)})
// 			case percent < 40: // 30% of pixels are transparent (have alpha == 0)
// 				img.Set(x, y, color.NRGBA{u8(x), u8(y), u8(x * y), 0})
// 			default: // 60% of pixels are opaque (have alpha == 255)
// 				img.Set(x, y, color.NRGBA{u8(x), u8(y), u8(x * y), 255})
// 			}
// 		}
// 	}
// 	if img.Opaque() {
// 		b.Fatal("expected image not to be opaque")
// 	}
// 	b.SetBytes(width * height * 4)
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		Encode(io.Discard, img)
// 	}
// }
