// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::paeth;
use super::reader::{self, FilterType, CB};
use super::Error;
use crate::bufio;
use crate::compat;
use crate::compress::zlib;
use crate::encoding::binary::{self, ByteOrder};
use crate::hash::crc32;
use crate::hash::Hash32;
use crate::image::{
    self,
    color::{self, ColorTrait},
    Image, Img,
};
use std::io::Write;

/// Encoder configures encoding PNG images.
pub struct Encoder {
    compression_level: CompressionLevel,
    // 	// BufferPool optionally specifies a buffer pool to get temporary
    // 	// EncoderBuffers when encoding an image.
    // 	BufferPool EncoderBufferPool
}

// // EncoderBufferPool is an interface for getting and returning temporary
// // instances of the EncoderBuffer struct. This can be used to reuse buffers
// // when encoding multiple images.
// type EncoderBufferPool interface {
// 	Get() *EncoderBuffer
// 	Put(*EncoderBuffer)
// }

/// EncoderBuffer holds the buffers used for encoding PNG images.
pub struct EncoderBuffer<'a> {
    enc: &'a Encoder,
    m: &'a image::Img,
    cb: CB,
    cr0: Vec<u8>,
    cr1: Vec<u8>,
    cr2: Vec<u8>,
    cr3: Vec<u8>,
    cr4: Vec<u8>,
    pr: Vec<u8>,
    zw_level: isize,
}

/// CompressionLevel indicates the compression level.
/// Positive CompressionLevel values are reserved to mean a numeric zlib
/// compression level, although that is not implemented yet.
type CompressionLevel = isize;

pub const DEFAULT_COMPRESSION: CompressionLevel = 0;
pub const NO_COMPRESSION: CompressionLevel = -1;
pub const BEST_SPEED: CompressionLevel = -2;
pub const BEST_COMPRESSION: CompressionLevel = -3;

// type opaquer interface {
// 	opaque() bool
// }

/// Returns whether or not the image is fully opaque.
fn opaque(m: &dyn image::Image) -> bool {
    m.opaque()
}

/// The absolute value of a byte interpreted as a signed int8.
fn abs8(d: u8) -> isize {
    if d < 128 {
        return d as isize;
    }
    return 256 - d as isize;
}

fn write_chunk(w: &mut dyn std::io::Write, b: &[u8], name: &str) -> std::io::Result<()> {
    // 	if self.err != nil {
    // 		return
    // 	}

    let n = b.len() as u32;
    if n as usize != b.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!("{} chunk is too large: {}", name, b.len()),
        ));
    }

    let mut header = [0_u8; 8];
    binary::BIG_ENDIAN.put_uint32(&mut header[..4], n);
    let name_bytes = name.as_bytes();
    header[4] = name_bytes[0];
    header[5] = name_bytes[1];
    header[6] = name_bytes[2];
    header[7] = name_bytes[3];
    let mut crc = crc32::new_ieee();
    crc.write_all(&header[4..8])?;
    crc.write_all(b)?;

    let mut footer = [0_u8; 4];
    binary::BIG_ENDIAN.put_uint32(&mut footer[..4], crc.sum32());

    w.write_all(&header[..8])?;
    w.write_all(b)?;
    w.write_all(&footer[..4])?;

    Ok(())
}

/// An encoder is an io.Writer that satisfies writes by writing PNG IDAT chunks,
/// including an 8-byte header and 4-byte CRC checksum per Write call. Such calls
/// should be relatively infrequent, since writeIDATs uses a bufio::Writer.
struct IDATWriter<'a> {
    w: &'a mut dyn std::io::Write,
}

impl std::io::Write for IDATWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        write_chunk(self.w, buf, "IDAT")?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

impl<'a> EncoderBuffer<'a> {
    fn write_ihdr(&mut self, w: &mut dyn std::io::Write) -> Result<(), Error> {
        let mut tmp = [0; 13];
        let b = self.m.bounds();
        binary::BIG_ENDIAN.put_uint32(&mut tmp[0..4], b.dx() as u32);
        binary::BIG_ENDIAN.put_uint32(&mut tmp[4..8], b.dy() as u32);
        // Set bit depth and color type.
        match self.cb {
            CB::G8 => {
                tmp[8] = 8;
                tmp[9] = reader::ColorType::Grayscale as u8;
            }
            CB::TC8 => {
                tmp[8] = 8;
                tmp[9] = reader::ColorType::TrueColor as u8;
            }
            CB::P8 => {
                tmp[8] = 8;
                tmp[9] = reader::ColorType::Paletted as u8;
            }
            CB::P4 => {
                tmp[8] = 4;
                tmp[9] = reader::ColorType::Paletted as u8;
            }
            CB::P2 => {
                tmp[8] = 2;
                tmp[9] = reader::ColorType::Paletted as u8;
            }
            CB::P1 => {
                tmp[8] = 1;
                tmp[9] = reader::ColorType::Paletted as u8;
            }
            CB::TCA8 => {
                tmp[8] = 8;
                tmp[9] = reader::ColorType::TrueColorAlpha as u8;
            }
            CB::G16 => {
                tmp[8] = 16;
                tmp[9] = reader::ColorType::Grayscale as u8;
            }
            CB::TC16 => {
                tmp[8] = 16;
                tmp[9] = reader::ColorType::TrueColor as u8;
            }
            CB::TCA16 => {
                tmp[8] = 16;
                tmp[9] = reader::ColorType::TrueColorAlpha as u8;
            }
            _ => panic!("unreachable"),
        }
        tmp[10] = 0; // default compression method
        tmp[11] = 0; // default filter method
        tmp[12] = 0; // non-interlaced
        write_chunk(w, &tmp[..13], "IHDR")?;
        Ok(())
    }

    fn write_plteand_trns(
        &mut self,
        w: &mut dyn std::io::Write,
        pal: &color::Palette,
    ) -> Result<(), Error> {
        let colors = pal.colors.len();
        if colors < 1 || colors > 256 {
            return Err(Error::FormatError(format!(
                "bad palette length: {}",
                colors
            )));
        }
        let mut tmp = [0_u8; 4 * 256];
        let mut last = None;
        for i in 0..colors {
            let c = pal.colors[i];
            let c1 = c.to_nrgba();
            tmp[3 * i + 0] = c1.r;
            tmp[3 * i + 1] = c1.g;
            tmp[3 * i + 2] = c1.b;
            if c1.a != 0xff {
                last = Some(i);
            }
            tmp[3 * 256 + i] = c1.a;
        }
        write_chunk(w, &tmp[..3 * colors], "PLTE")?;
        if let Some(last) = last {
            write_chunk(w, &tmp[3 * 256..3 * 256 + 1 + last], "tRNS")?;
        }
        Ok(())
    }

    /// Chooses the filter to use for encoding the current row, and applies it.
    /// The return value is the index of the filter and also of the row in cr that has had it applied.
    fn filter(&mut self, bpp: usize) -> FilterType {
        // We try all five filter types, and pick the one that minimizes the sum of absolute differences.
        // This is the same heuristic that libpng uses, although the filters are attempted in order of
        // estimated most likely to be minimal (ftUp, ftPaeth, ftNone, ftSub, ftAverage), rather than
        // in their enumeration order (ftNone, ftSub, ftUp, ftAverage, ftPaeth).
        let cdat0 = &mut self.cr0[1..];
        let cdat1 = &mut self.cr1[1..];
        let cdat2 = &mut self.cr2[1..];
        let cdat3 = &mut self.cr3[1..];
        let cdat4 = &mut self.cr4[1..];
        let pdat = &self.pr[1..];
        let n = cdat0.len();

        // The up filter.
        let mut sum = 0;
        for i in 0..n {
            cdat2[i] = cdat0[i].wrapping_sub(pdat[i]);
            sum += abs8(cdat2[i])
        }
        let mut best = sum;
        let mut filter = FilterType::Up;

        // The Paeth filter.
        let mut sum = 0;
        for i in 0..bpp {
            cdat4[i] = cdat0[i].wrapping_sub(pdat[i]);
            sum += abs8(cdat4[i]);
        }
        for i in bpp..n {
            cdat4[i] = cdat0[i].wrapping_sub(paeth::paeth(cdat0[i - bpp], pdat[i], pdat[i - bpp]));
            sum += abs8(cdat4[i]);
            if sum >= best {
                break;
            }
        }
        if sum < best {
            best = sum;
            filter = FilterType::Paeth;
        }

        // The none filter.
        let mut sum = 0;
        for i in 0..n {
            sum += abs8(cdat0[i]);
            if sum >= best {
                break;
            }
        }
        if sum < best {
            best = sum;
            filter = FilterType::None;
        }

        // The sub filter.
        let mut sum = 0;
        for i in 0..bpp {
            cdat1[i] = cdat0[i];
            sum += abs8(cdat1[i]);
        }
        for i in bpp..n {
            cdat1[i] = cdat0[i].wrapping_sub(cdat0[i - bpp]);
            sum += abs8(cdat1[i]);
            if sum >= best {
                break;
            }
        }
        if sum < best {
            best = sum;
            filter = FilterType::Sub;
        }

        // The average filter.
        let mut sum = 0;
        for i in 0..bpp {
            cdat3[i] = cdat0[i].wrapping_sub(pdat[i] / 2);
            sum += abs8(cdat3[i]);
        }
        for i in bpp..n {
            cdat3[i] =
                cdat0[i].wrapping_sub(((cdat0[i - bpp] as isize + pdat[i] as isize) / 2) as u8);
            sum += abs8(cdat3[i]);
            if sum >= best {
                break;
            }
        }
        if sum < best {
            filter = FilterType::Average;
        }

        return filter;
    }

    fn write_image(
        &mut self,
        w: &mut dyn std::io::Write,
        m: &image::Img,
        cb: CB,
        level: isize,
    ) -> std::io::Result<()> {
        // if self.zw.is_none() || self.zwLevel != level {
        //     self.zw = Some(zlib::new_writer_level(&mut w, level));
        //     self.zwLevel = level;
        // } else {
        //     self.zw.as_mut().unwrap().reset(&mut w);
        // }

        let mut zw = zlib::new_writer_level(w, level);
        self.zw_level = level;

        let bits_per_pixel = match cb {
            CB::G8 => 8,
            CB::TC8 => 24,
            CB::P8 => 8,
            CB::P4 => 4,
            CB::P2 => 2,
            CB::P1 => 1,
            CB::TCA8 => 32,
            CB::TC16 => 48,
            CB::TCA16 => 64,
            CB::G16 => 16,
            _ => {
                panic!("unreachable")
            }
        };

        // cr[*] and pr are the bytes for the current and previous row.
        // cr[0] is unfiltered (or equivalently, filtered with the ftNone filter).
        // cr[ft], for non-zero filter types ft, are buffers for transforming cr[0] under the
        // other PNG filter types. These buffers are allocated once and re-used for each row.
        // The +1 is for the per-row filter type, which is at cr[*][0].
        let b = m.bounds();
        let sz = 1 + (bits_per_pixel * b.dx() + 7) / 8;
        self.cr0.resize(sz, 0);
        self.cr1.resize(sz, 0);
        self.cr2.resize(sz, 0);
        self.cr3.resize(sz, 0);
        self.cr4.resize(sz, 0);
        self.cr0[0] = 0;
        self.cr1[0] = 1;
        self.cr2[0] = 2;
        self.cr3[0] = 3;
        self.cr4[0] = 4;
        self.pr.resize(sz, 0);
        self.pr.fill(0);

        for y in b.min.y..b.max.y {
            // Convert from colors to bytes.
            let mut i = 1;
            match cb {
                CB::G8 => {
                    if m.color_model() == color::Model::GrayModel {
                        compat::copy(&mut self.cr0[1..], m.get_line_data(y));
                    } else {
                        unimplemented!();
                        //     for x in b.min.x..b.max.x {
                        //         let c = color::Gray::new_from(&m.at(x, y));
                        //         self.cr0[i] = c.y;
                        //         i += 1;
                        //     }
                    }
                }
                CB::TC8 => {
                    // We have previously verified that the alpha value is fully opaque.
                    let stride = m.stride();
                    let pix = m.pix();
                    if stride != 0 {
                        let j0 = (y - b.min.y) as usize * stride;
                        let j1 = j0 + b.dx() * 4;
                        let mut j = j0;
                        while j < j1 {
                            self.cr0[i + 0] = pix[j + 0];
                            self.cr0[i + 1] = pix[j + 1];
                            self.cr0[i + 2] = pix[j + 2];
                            i += 3;
                            j += 4;
                        }
                    } else {
                        unimplemented!();
                        // 				for x in b.min.x..b.max.x {
                        // 					let (r, g, b, _) = m.at(x, y).rgba();
                        // 					self.cr0[i+0] = u8(r >> 8)
                        // 					self.cr0[i+1] = u8(g >> 8)
                        // 					self.cr0[i+2] = u8(b >> 8)
                        // 					i += 3
                        // 				}
                    }
                }
                CB::P8 => {
                    // 			if paletted != nil {
                    compat::copy(&mut self.cr0[1..], m.get_line_data(y));
                    // 			} else {
                    // 				pi := m.(image.PalettedImage)
                    // 				for x in b.min.x..b.max.x {
                    // 					self.cr0[i] = pi.color_index_at(x, y)
                    // 					i += 1
                    // 				}
                    // 			}
                }
                CB::P4 | CB::P2 | CB::P1 => match m {
                    Img::Paletted(img) => {
                        let mut a = 0;
                        let mut c = 0;
                        let pixels_per_byte = 8 / bits_per_pixel;
                        for x in b.min.x..b.max.x {
                            a = (a << bits_per_pixel) | img.color_index_at(x, y);
                            c += 1;
                            if c == pixels_per_byte {
                                self.cr0[i] = a;
                                i += 1;
                                a = 0;
                                c = 0;
                            }
                        }
                        if c != 0 {
                            while c != pixels_per_byte {
                                a = a << bits_per_pixel;
                                c += 1;
                            }
                            self.cr0[i] = a;
                        }
                    }
                    _ => panic!("unreacheable"),
                },
                CB::TCA8 => {
                    match m {
                        Img::NRGBA(_img) => {
                            compat::copy(&mut self.cr0[1..], m.get_line_data(y));
                        }
                        Img::RGBA(_img) => {
                            unimplemented!();
                            // 				dst := self.cr0[1..]
                            // 				src := img.pix[img.pix_offset(b.min.x, y):img.pix_offset(b.max.x, y)]
                            // 				for ; len(src) >= 4; dst, src = dst[4..], src[4..] {
                            // 					d := (*[4]byte)(dst)
                            // 					s := (*[4]byte)(src)
                            // 					if s[3] == 0x00 {
                            // 						d[0] = 0
                            // 						d[1] = 0
                            // 						d[2] = 0
                            // 						d[3] = 0
                            // 					} else if s[3] == 0xff {
                            // 						copy(d[..], s[..])
                            // 					} else {
                            // 						// This code does the same as color::Model::NRGBAModel.Convert(
                            // 						// img.At(x, y)).(color.NRGBA) but with no extra memory
                            // 						// allocations or interface/function call overhead.
                            // 						//
                            // 						// The multiplier m combines 0x101 (which converts
                            // 						// 8-bit color to 16-bit color) and 0xffff (which, when
                            // 						// combined with the division-by-a, converts from
                            // 						// alpha-premultiplied to non-alpha-premultiplied).
                            // 						const m = 0x101 * 0xffff
                            // 						a := uint32(s[3]) * 0x101
                            // 						d[0] = u8((uint32(s[0]) * m / a) >> 8)
                            // 						d[1] = u8((uint32(s[1]) * m / a) >> 8)
                            // 						d[2] = u8((uint32(s[2]) * m / a) >> 8)
                            // 						d[3] = s[3]
                            // 					}
                            // 				}
                        }
                        _ => {
                            unimplemented!();
                            // 				// Convert from image.Image (which is alpha-premultiplied) to PNG's non-alpha-premultiplied.
                            // 				for x in b.min.x..b.max.x {
                            // 					c := color::Model::NRGBAModel.Convert(m.at(x, y)).(color.NRGBA)
                            // 					self.cr0[i+0] = c.r;
                            // 					self.cr0[i+1] = c.g;
                            // 					self.cr0[i+2] = c.b;
                            // 					self.cr0[i+3] = c.a;
                            // 					i += 4;
                            // 				}
                        }
                    }
                }
                CB::G16 => {
                    for x in b.min.x..b.max.x {
                        let c = m.at(x, y).to_gray16();
                        self.cr0[i + 0] = (c.y >> 8) as u8;
                        self.cr0[i + 1] = (c.y) as u8;
                        i += 2;
                    }
                }
                CB::TC16 => {
                    // We have previously verified that the alpha value is fully opaque.
                    for x in b.min.x..b.max.x {
                        let (r, g, b, _) = m.at(x, y).rgba();
                        self.cr0[i + 0] = (r >> 8) as u8;
                        self.cr0[i + 1] = (r) as u8;
                        self.cr0[i + 2] = (g >> 8) as u8;
                        self.cr0[i + 3] = (g) as u8;
                        self.cr0[i + 4] = (b >> 8) as u8;
                        self.cr0[i + 5] = (b) as u8;
                        i += 6;
                    }
                }
                CB::TCA16 => {
                    // Convert from image.Image (which is alpha-premultiplied) to PNG's non-alpha-premultiplied.
                    for x in b.min.x..b.max.x {
                        let c = m.at(x, y).to_nrgba64();
                        self.cr0[i + 0] = (c.r >> 8) as u8;
                        self.cr0[i + 1] = (c.r) as u8;
                        self.cr0[i + 2] = (c.g >> 8) as u8;
                        self.cr0[i + 3] = (c.g) as u8;
                        self.cr0[i + 4] = (c.b >> 8) as u8;
                        self.cr0[i + 5] = (c.b) as u8;
                        self.cr0[i + 6] = (c.a >> 8) as u8;
                        self.cr0[i + 7] = (c.a) as u8;
                        i += 8;
                    }
                }
                _ => {
                    panic!("unreachable")
                }
            }

            // Apply the filter.
            // Skip filter for NoCompression and paletted images (CB::P8) as
            // "filters are rarely useful on palette images" and will result
            // in larger files (see http://www.libpng.org/pub/png/book/chapter09.html).
            let mut f = FilterType::None;
            if level != zlib::NO_COMPRESSION && !cb.paletted() {
                // Since we skip paletted images we don't have to worry about
                // bits_per_pixel not being a multiple of 8
                let bpp = bits_per_pixel / 8;
                f = self.filter(bpp);
            }

            // Write the compressed bytes.
            let row_data = match f {
                FilterType::None => &self.cr0,
                FilterType::Sub => &self.cr1,
                FilterType::Up => &self.cr2,
                FilterType::Average => &self.cr3,
                FilterType::Paeth => &self.cr4,
            };
            zw.write(row_data)?;

            // The current row for y is the previous row for y+1.
            // (self.pr, self.cr0) = (self.cr0, self.pr);
            std::mem::swap(&mut self.cr0, &mut self.pr);
        }
        zw.close()?;
        Ok(())
    }

    /// Write the actual image data to one or more IDAT chunks.
    fn write_idats(&mut self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        // 	if self.err != nil {
        // 		return
        // 	}

        // match self.bw.as_mut() {
        //     Some(bw) => bw.reset(e),
        //     None => self.bw = Some(bufio::new_writer_size(e, 1 << 15)),
        // }

        // if self.bw == nil {
        //     self.bw = bufio::new_writer_size(e, 1 << 15);
        // } else {
        //     self.bw.Reset(e);
        // }

        let mut w = IDATWriter { w };
        // ggstd TODO: try to implement optimizations present in the original code:
        //   - caching buf writer
        //   - caching zlib compressor
        let mut bw = bufio::new_writer_size(&mut w, 1 << 15);
        self.write_image(
            &mut bw,
            &self.m,
            self.cb,
            level_to_zlib(self.enc.compression_level),
        )?;
        bw.flush()?;
        Ok(())
    }

    fn write_iend(&mut self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write_chunk(w, &[], "IEND")
    }
}

// This function is required because we want the zero value of
// Encoder.CompressionLevel to map to zlib::DEFAULT_COMPRESSION.
fn level_to_zlib(l: CompressionLevel) -> isize {
    match l {
        DEFAULT_COMPRESSION => zlib::DEFAULT_COMPRESSION,
        NO_COMPRESSION => zlib::NO_COMPRESSION,
        BEST_SPEED => zlib::BEST_SPEED,
        BEST_COMPRESSION => zlib::BEST_COMPRESSION,
        _ => zlib::DEFAULT_COMPRESSION,
    }
}

/// encode writes the image m to w in PNG format. Any Image may be
/// encoded, but images that are not image.NRGBA might be encoded lossily.
pub fn encode(w: &mut dyn std::io::Write, m: &image::Img) -> Result<(), Error> {
    Encoder::new(DEFAULT_COMPRESSION).encode(w, m)
}

impl Encoder {
    pub fn new(compression_level: CompressionLevel) -> Self {
        Self { compression_level }
    }

    /// Encode writes the Image m to w in PNG format.
    pub fn encode(&mut self, w: &mut dyn std::io::Write, m: &image::Img) -> Result<(), Error> {
        // Obviously, negative widths and heights are invalid. Furthermore, the PNG
        // spec section 11.2.2 says that zero is invalid. Excessively large images are
        // also rejected.
        let bounds = m.bounds();
        let (mw, mh) = (bounds.dx() as i64, bounds.dy() as i64);
        if mw <= 0 || mh <= 0 || mw >= 1 << 32 || mh >= 1 << 32 {
            return Err(Error::FormatError(format!(
                "invalid image size: {}x{}",
                mw, mh
            )));
        }

        // 	var e *encoder
        // 	if self.BufferPool != nil {
        // 		buffer := self.BufferPool.Get()
        // 		e = (*encoder)(buffer)
        // 	}

        // 	if e == nil {
        // 		e = &encoder{}
        // 	}
        // 	if self.BufferPool != nil {
        // 		defer self.BufferPool.Put((*EncoderBuffer)(e))
        // 	}

        // 	e.enc = enc
        // 	e.w = w
        // 	e.m = m

        let cb = match m {
            image::Img::Paletted(img) => match img.palette.colors.len() {
                0..=2 => CB::P1,
                3..=4 => CB::P2,
                5..=16 => CB::P4,
                _ => CB::P8,
            },
            _ => match m.color_model() {
                color::Model::GrayModel => CB::G8,
                color::Model::Gray16Model => CB::G16,
                color::Model::RGBAModel | color::Model::NRGBAModel | color::Model::AlphaModel => {
                    if opaque(m) {
                        CB::TC8
                    } else {
                        CB::TCA8
                    }
                }
                _ => {
                    if opaque(m) {
                        CB::TC16
                    } else {
                        CB::TCA16
                    }
                }
            },
        };

        let mut e = EncoderBuffer {
            enc: self,
            m,
            cb,
            zw_level: 0,
            cr0: Vec::new(),
            cr1: Vec::new(),
            cr2: Vec::new(),
            cr3: Vec::new(),
            cr4: Vec::new(),
            pr: Vec::new(),
        };
        w.write_all(reader::PNG_HEADER)?;
        e.write_ihdr(w)?;
        if let Img::Paletted(img) = m {
            e.write_plteand_trns(w, &img.palette)?;
        }
        e.write_idats(w)?;
        e.write_iend(w)?;
        Ok(())
    }
}
