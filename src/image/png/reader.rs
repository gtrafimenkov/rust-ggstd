// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::paeth::filter_paeth;
use crate::compat;
use crate::compress::zlib;
use crate::encoding::binary::{self, ByteOrder};
use crate::hash::{crc32, Hash, Hash32};
use crate::image::{
    self,
    color::{self, Color, Gray, Gray16, Model, Palette, RGBA},
    Image, Img,
};
use std::io::Write;

/// Color type, as per the PNG spec.
pub(super) enum ColorType {
    Grayscale = 0,
    TrueColor = 2,
    Paletted = 3,
    GrayscaleAlpha = 4,
    TrueColorAlpha = 6,
}

impl ColorType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ColorType::Grayscale),
            2 => Some(ColorType::TrueColor),
            3 => Some(ColorType::Paletted),
            4 => Some(ColorType::GrayscaleAlpha),
            6 => Some(ColorType::TrueColorAlpha),
            _ => None,
        }
    }
}

/// A CB is a combination of color type and bit depth.
#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum CB {
    G1,
    G2,
    G4,
    G8,
    GA8,
    TC8,
    P1,
    P2,
    P4,
    P8,
    TCA8,
    G16,
    GA16,
    TC16,
    TCA16,
}

impl CB {
    pub(super) fn paletted(self) -> bool {
        match self {
            CB::P1 => true,
            CB::P2 => true,
            CB::P4 => true,
            CB::P8 => true,
            _ => false,
        }
    }

    pub(super) fn true_color(&self) -> bool {
        match self {
            CB::TC8 => true,
            CB::TC16 => true,
            _ => false,
        }
    }
}

/// Filter type, as per the PNG spec.
#[derive(Debug)]
pub(super) enum FilterType {
    None = 0,
    Sub = 1,
    Up = 2,
    Average = 3,
    Paeth = 4,
}

const FT_NONE: u8 = FilterType::None as u8;
const FT_SUB: u8 = FilterType::Sub as u8;
const FT_UP: u8 = FilterType::Up as u8;
const FT_AVERAGE: u8 = FilterType::Average as u8;
const FT_PAETH: u8 = FilterType::Paeth as u8;

/// Interlace type.
#[derive(Debug, PartialEq, Copy, Clone)]
enum InterlaceType {
    None = 0,
    Adam7 = 1,
}

/// InterlaceScan defines the placement and size of a pass for Adam7 interlacing.
struct InterlaceScan {
    x_factor: isize,
    y_factor: isize,
    x_offset: isize,
    y_offset: isize,
}

impl InterlaceScan {
    const fn new(xf: isize, yf: isize, xo: isize, yo: isize) -> Self {
        Self {
            x_factor: xf,
            y_factor: yf,
            x_offset: xo,
            y_offset: yo,
        }
    }
}

/// INTERLACING defines Adam7 interlacing, with 7 passes of reduced images.
/// See https://www.w3.org/TR/PNG/#8Interlace
const INTERLACING: &[InterlaceScan] = &[
    InterlaceScan::new(8, 8, 0, 0),
    InterlaceScan::new(8, 8, 4, 0),
    InterlaceScan::new(4, 8, 0, 4),
    InterlaceScan::new(4, 4, 2, 0),
    InterlaceScan::new(2, 4, 0, 2),
    InterlaceScan::new(2, 2, 1, 0),
    InterlaceScan::new(1, 2, 0, 1),
];

/// Decoding stage.
/// The PNG specification says that the IHDR, PLTE (if present), tRNS (if
/// present), IDAT and IEND chunks must appear in that order. There may be
/// multiple IDAT chunks, and IDAT chunks must be sequential (i.e. they may not
/// have any other chunks between them).
/// https://www.w3.org/TR/PNG/#5ChunkOrdering
#[derive(PartialEq, PartialOrd)]
enum DecodingStage {
    Start = 0,
    SeenIHDR,
    SeenPLTE,
    SeentRNS,
    SeenIDAT,
    SeenIEND,
}

pub(super) const PNG_HEADER: &[u8] = b"\x89PNG\r\n\x1a\n";

struct Decoder {
    img: Option<Box<Img>>,
    crc: crc32::Digest,
    width: usize,
    height: usize,
    depth: u8,
    palette_vec: Vec<Color>,
    palette: Option<Palette>,
    cb: CB,
    stage: DecodingStage,
    interlace: InterlaceType,
    // use_transparent and transparent are used for grayscale and truecolor
    // transparency, as opposed to palette transparency.
    use_transparent: bool,
    transparent: [u8; 6],
}

impl Default for Decoder {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            stage: DecodingStage::Start,
            crc: crc32::new_ieee(),
            depth: 0,
            cb: CB::TC8, // placeholder until the header is decoded
            interlace: InterlaceType::None,
            use_transparent: false,
            transparent: [0; 6],
            img: None,
            palette_vec: Vec::with_capacity(0),
            palette: None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// A FormatError reports that the input is not a valid PNG.
    FormatError(String),
    /// Input-output error
    StdIo(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::StdIo(error)
    }
}

impl Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FormatError(v) => write!(f, "FormatError({})", v),
            Error::StdIo(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for Error {}

fn new_chunk_order_error() -> std::io::Error {
    new_format_error("chunk out of order")
}

fn new_unsupported_error(msg: &str) -> std::io::Error {
    new_format_error(&format!("png: unsupported feature: {}", msg))
}

fn new_format_error(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message.to_string())
}

fn decode_cb(depth: u8, color_type: u8) -> std::io::Result<CB> {
    let ct = match ColorType::from_u8(color_type) {
        Some(ct) => ct,
        None => return Err(new_unsupported_error(&format!("color type {}", color_type))),
    };
    match depth {
        1 => match ct {
            ColorType::Grayscale => return Ok(CB::G1),
            ColorType::Paletted => return Ok(CB::P1),
            _ => {}
        },
        2 => match ct {
            ColorType::Grayscale => return Ok(CB::G2),
            ColorType::Paletted => return Ok(CB::P2),
            _ => {}
        },
        4 => match ct {
            ColorType::Grayscale => return Ok(CB::G4),
            ColorType::Paletted => return Ok(CB::P4),
            _ => {}
        },
        8 => match ct {
            ColorType::Grayscale => return Ok(CB::G8),
            ColorType::TrueColor => return Ok(CB::TC8),
            ColorType::Paletted => return Ok(CB::P8),
            ColorType::GrayscaleAlpha => return Ok(CB::GA8),
            ColorType::TrueColorAlpha => return Ok(CB::TCA8),
        },
        16 => match ct {
            ColorType::Grayscale => return Ok(CB::G16),
            ColorType::TrueColor => return Ok(CB::TC16),
            ColorType::GrayscaleAlpha => return Ok(CB::GA16),
            ColorType::TrueColorAlpha => return Ok(CB::TCA16),
            _ => {}
        },
        _ => {}
    }
    return Err(new_unsupported_error(&format!(
        "bit depth {}, color type {}",
        depth, color_type
    )));
}

impl Decoder {
    fn parse_ihdr(&mut self, r: &mut dyn std::io::Read, length: u32) -> std::io::Result<()> {
        if length != 13 {
            return Err(new_format_error("bad IHDR length"));
        }
        let mut tmp = [0; 13];
        r.read_exact(&mut tmp)?;
        self.crc.write(&tmp[..13])?;
        if tmp[10] != 0 {
            return Err(new_unsupported_error("compression method"));
        }
        if tmp[11] != 0 {
            return Err(new_unsupported_error("filter method"));
        }
        const IT_NONE: u8 = InterlaceType::None as u8;
        const IT_ADAM7: u8 = InterlaceType::Adam7 as u8;
        self.interlace = match tmp[12] {
            IT_NONE => InterlaceType::None,
            IT_ADAM7 => InterlaceType::Adam7,
            _ => return Err(new_format_error("invalid interlace method")),
        };

        let w = binary::BIG_ENDIAN.uint32(&tmp[0..4]) as i32;
        let h = binary::BIG_ENDIAN.uint32(&tmp[4..8]) as i32;
        if w <= 0 || h <= 0 {
            return Err(new_format_error("non-positive dimension"));
        }
        let n_pixels64 = (w as i64) * (h as i64);
        let n_pixels = n_pixels64 as isize;
        if n_pixels64 != (n_pixels as i64) {
            return Err(new_unsupported_error("dimension overflow"));
        }
        // There can be up to 8 bytes per pixel, for 16 bits per channel RGBA.
        if n_pixels.overflowing_mul(8).1 {
            return Err(new_unsupported_error("dimension overflow"));
        }

        self.depth = tmp[8];
        self.cb = decode_cb(self.depth, tmp[9])?;
        self.width = w as usize;
        self.height = h as usize;
        verify_checksum(r, &self.crc)
    }

    fn parse_plte(&mut self, r: &mut dyn std::io::Read, length: u32) -> std::io::Result<()> {
        let np = (length / 3) as usize; // The number of palette entries.

        if length % 3 != 0 || np == 0 || np > 256 || np > (1 << (self.depth)) {
            return Err(new_format_error("bad PLTE length"));
        }
        let mut tmp_buf = [0; 3 * 256];
        let n = 3 * np;
        r.read_exact(&mut tmp_buf[..n])?;
        self.crc.write_all(&tmp_buf[..n])?;
        match self.cb {
            CB::P1 | CB::P2 | CB::P4 | CB::P8 => {
                self.palette_vec.reserve(256 - self.palette_vec.capacity());
                for i in 0..np {
                    let c = Color::new_rgba(
                        tmp_buf[3 * i + 0],
                        tmp_buf[3 * i + 1],
                        tmp_buf[3 * i + 2],
                        0xff,
                    );
                    self.palette_vec.push(c);
                }
            }
            CB::TC8 | CB::TCA8 | CB::TC16 | CB::TCA16 => {
                // As per the PNG spec, a PLTE chunk is optional (and for practical purposes,
                // ignorable) for the TrueColor and TrueColorAlpha color types (section 4.1.2).
            }
            _ => {
                return Err(new_format_error("PLTE, color type mismatch"));
            }
        }
        verify_checksum(r, &self.crc)
    }

    fn parset_rns(&mut self, r: &mut dyn std::io::Read, length: u32) -> std::io::Result<()> {
        let length = length as usize;
        match self.cb {
            CB::G1 | CB::G2 | CB::G4 | CB::G8 | CB::G16 => {
                if length != 2 {
                    return Err(new_format_error("bad tRNS length"));
                }
                let mut tmp = [0; 2];
                r.read_exact(&mut tmp[0..length])?;
                self.crc.write(&tmp[..length])?;

                compat::copy(&mut self.transparent, &tmp[..length]);
                match self.cb {
                    CB::G1 => self.transparent[1] *= 0xff,
                    CB::G2 => self.transparent[1] *= 0x55,
                    CB::G4 => self.transparent[1] *= 0x11,
                    _ => {}
                }
                self.use_transparent = true;
            }
            CB::TC8 | CB::TC16 => {
                if length != 6 {
                    return Err(new_format_error("bad tRNS length"));
                }
                let mut tmp = [0; 6];
                r.read_exact(&mut tmp[0..length])?;
                self.crc.write(&tmp[..length])?;

                compat::copy(&mut self.transparent, &tmp[..length]);
                self.use_transparent = true;
            }
            CB::P1 | CB::P2 | CB::P4 | CB::P8 => {
                if length > 256 {
                    return Err(new_format_error("bad tRNS length"));
                }
                let mut tmp = [0; 256];
                r.read_exact(&mut tmp[0..length])?;
                self.crc.write(&tmp[..length])?;
                if self.palette_vec.len() < length {
                    // Initialize the rest of the palette to opaque black. The spec (section
                    // 11.2.3) says that "any out-of-range pixel value found in the image data
                    // is an error", but some real-world PNG files have out-of-range pixel
                    // values. We fall back to opaque black, the same as libpng 1.5.13;
                    // ImageMagick 6.5.7 returns an error.
                    self.palette_vec.resize(length, color::OPAQUE_BLACK);
                }
                for i in 0..length {
                    let c = RGBA::new_from(&self.palette_vec[i]);
                    self.palette_vec[i] = Color::new_nrgba(c.r, c.g, c.b, tmp[i]);
                }
            }
            _ => return Err(new_format_error("tRNS, color type mismatch")),
        }
        verify_checksum(r, &self.crc)
    }

    fn parse_idat(&mut self, r: &mut dyn std::io::Read, length: u32) -> std::io::Result<()> {
        let mut idat_reader = IdatReader {
            idat_length: length,
            r,
            crc: &mut self.crc,
        };

        let dp = DecodeParams {
            width: self.width,
            height: self.height,
            depth: self.depth,
            cb: self.cb,
            interlace: self.interlace,
            use_transparent: self.use_transparent,
            transparent: self.transparent,
        };
        let pal = convert_palette(&self.palette_vec);

        let img = {
            let mut zlib_reader = match zlib::Reader::new(&mut idat_reader) {
                Ok(reader) => reader,
                Err(err) => return Err(err.to_stdio_error()),
            };
            let img = match self.interlace {
                InterlaceType::None => {
                    let mut img = allocate_image(&dp, &pal);
                    read_image_pass(&dp, &mut zlib_reader, 0, &mut img)?;
                    img
                }
                InterlaceType::Adam7 => {
                    let mut img = allocate_image(&dp, &pal);
                    for pass in 0..7 {
                        let mut img_pass = allocate_image(&dp, &pal);
                        if read_image_pass(&dp, &mut zlib_reader, pass, &mut img_pass)? {
                            merge_pass_into(&mut img, &img_pass, pass);
                        }
                    }
                    img
                }
            };

            // Check for EOF, to verify the zlib checksum.
            let mut tmp = [0; 1];
            if let Err(err) = zlib_reader.read(&mut tmp) {
                return Err(err.to_stdio_error());
            }
            img
        };

        self.palette = if self.cb.paletted() { Some(pal) } else { None };

        if idat_reader.idat_length != 0 {
            return Err(new_format_error("too much pixel data"));
        }

        self.img = Some(Box::new(img));

        verify_checksum(r, &self.crc)
    }

    fn parse_iend(&mut self, r: &mut dyn std::io::Read, length: u32) -> std::io::Result<()> {
        if length != 0 {
            return Err(new_format_error("bad IEND length"));
        }
        verify_checksum(r, &self.crc)
    }

    fn parse_chunk(&mut self, r: &mut dyn std::io::Read, config_only: bool) -> std::io::Result<()> {
        // Read the length and chunk type.
        let mut tmp = [0; 8];
        r.read_exact(&mut tmp)?;
        let length = binary::BIG_ENDIAN.uint32(&tmp[..4]);
        self.crc.reset();
        self.crc.write(&tmp[4..8])?;

        // Read the chunk data.
        let chunk_type = &tmp[4..8];
        match chunk_type {
            b"IHDR" => {
                if self.stage != DecodingStage::Start {
                    return Err(new_chunk_order_error());
                }
                self.stage = DecodingStage::SeenIHDR;
                return self.parse_ihdr(r, length);
            }
            b"PLTE" => {
                if self.stage != DecodingStage::SeenIHDR {
                    return Err(new_chunk_order_error());
                }
                self.stage = DecodingStage::SeenPLTE;
                return self.parse_plte(r, length);
            }
            b"tRNS" => {
                if self.cb.paletted() {
                    if self.stage != DecodingStage::SeenPLTE {
                        return Err(new_chunk_order_error());
                    }
                } else if self.cb.true_color() {
                    if self.stage != DecodingStage::SeenIHDR
                        && self.stage != DecodingStage::SeenPLTE
                    {
                        return Err(new_chunk_order_error());
                    }
                } else if self.stage != DecodingStage::SeenIHDR {
                    return Err(new_chunk_order_error());
                }
                self.stage = DecodingStage::SeentRNS;
                return self.parset_rns(r, length);
            }
            b"IDAT" => {
                if self.stage < DecodingStage::SeenIHDR
                    || self.stage > DecodingStage::SeenIDAT
                    || (self.stage == DecodingStage::SeenIHDR && self.cb.paletted())
                {
                    return Err(new_chunk_order_error());
                } else if self.stage == DecodingStage::SeenIDAT {
                    // Ignore trailing zero-length or garbage IDAT chunks.
                    //
                    // This does not affect valid PNG images that contain multiple IDAT
                    // chunks, since the first call to parseIDAT below will consume all
                    // consecutive IDAT chunks required for decoding the image.
                    // break;

                    // the chank will be consumed below
                    // return Ok(());
                } else {
                    self.stage = DecodingStage::SeenIDAT;
                    if config_only {
                        return Ok(());
                    }
                    return self.parse_idat(r, length);
                }
            }
            b"IEND" => {
                if self.stage != DecodingStage::SeenIDAT {
                    return Err(new_chunk_order_error());
                }
                self.stage = DecodingStage::SeenIEND;
                return self.parse_iend(r, length);
            }
            _ => {}
        }

        // consuming unrecognized chunk
        if length > 0x7fffffff {
            return Err(new_format_error(&format!("Bad chunk length: {}", length)));
        }
        // Ignore this chunk (of a known length).
        let mut ignored = [0_u8; 4096];
        let mut length = length as usize;
        while length > 0 {
            let size = length.min(ignored.len());
            r.read_exact(&mut ignored[..size])?;
            self.crc.write(&ignored[..size])?;
            length -= size;
        }
        verify_checksum(r, &self.crc)
    }
}

// merge_pass_into merges a single pass into a full sized image.
fn merge_pass_into(dst: &mut Img, src: &Img, pass: usize) {
    let bytes_per_pixel = dst.bytes_per_pixel();
    let src_pix = src.pix();
    let stride = dst.stride();
    let rect = *dst.bounds();
    let dst_pix = dst.get_pix_mutable();
    let dst_pix_len = dst_pix.len();
    let mut s = 0;
    let bounds = src.bounds();
    let p = &INTERLACING[pass];
    for y in bounds.min.y..bounds.max.y {
        let d_base = (y * p.y_factor + p.y_offset - rect.min.y) * stride as isize
            + (p.x_offset - rect.min.x) * bytes_per_pixel as isize;
        for x in bounds.min.x..bounds.max.x {
            let d = (d_base + x * p.x_factor * bytes_per_pixel as isize) as usize;
            if d <= dst_pix_len {
                compat::copy(&mut dst_pix[d..], &src_pix[s..s + bytes_per_pixel]);
            }
            s += bytes_per_pixel;
        }
    }
    if let Img::Paletted(target) = dst {
        if let Img::Paletted(source) = src {
            if target.palette.colors.len() < source.palette.colors.len() {
                // readImagePass can return a paletted image whose implicit palette
                // length (one more than the maximum Pix value) is larger than the
                // explicit palette length (what's in the PLTE chunk). Make the
                // same adjustment here.
                target
                    .palette
                    .colors
                    .resize(source.palette.colors.len(), color::OPAQUE_BLACK);
            }
        }
    }
}

fn verify_checksum(r: &mut dyn std::io::Read, crc: &crc32::Digest) -> std::io::Result<()> {
    let mut tmp = [0_u8; 4];
    r.read_exact(&mut tmp)?;

    if binary::BIG_ENDIAN.uint32(&tmp) != crc.sum32() {
        return Err(new_format_error("invalid checksum"));
    }
    Ok(())
}

fn check_header(r: &mut dyn std::io::Read) -> std::io::Result<()> {
    let mut tmp = [0_u8; PNG_HEADER.len()];
    r.read_exact(&mut tmp)?;
    if tmp != PNG_HEADER {
        return Err(new_format_error("not a PNG file"));
    }
    Ok(())
}

/// decode reads a PNG image from r and returns it as an image.Image.
/// The type of Image returned depends on the PNG contents.
pub fn decode(r: &mut dyn std::io::Read) -> std::io::Result<Box<Img>> {
    let mut d = Decoder::default();
    check_header(r)?;
    while d.stage != DecodingStage::SeenIEND {
        d.parse_chunk(r, false)?;
    }
    Ok(d.img.unwrap())
}

/// decode_config returns the color model and dimensions of a PNG image without
/// decoding the entire image.
pub fn decode_config(r: &mut dyn std::io::Read) -> std::io::Result<image::Config> {
    let mut d = Decoder::default();
    check_header(r)?;

    loop {
        d.parse_chunk(r, true)?;
        if d.cb.paletted() {
            if d.stage >= DecodingStage::SeentRNS {
                break;
            }
        } else {
            if d.stage >= DecodingStage::SeenIHDR {
                break;
            }
        }
    }

    let cm = match d.cb {
        CB::G1 | CB::G2 | CB::G4 | CB::G8 => Model::GrayModel,
        CB::GA8 => Model::NRGBAModel,
        CB::TC8 => Model::RGBAModel,
        CB::P1 | CB::P2 | CB::P4 | CB::P8 => Model::Paletted(convert_palette(&d.palette_vec)),
        CB::TCA8 => Model::NRGBAModel,
        CB::G16 => Model::Gray16Model,
        CB::GA16 => Model::NRGBA64Model,
        CB::TC16 => Model::RGBA64Model,
        CB::TCA16 => Model::NRGBA64Model,
    };
    Ok(image::Config {
        color_model: cm,
        width: d.width,
        height: d.height,
    })
}

// fn init() {
// 	image.RegisterFormat("png", PNG_HEADER, decode, decode_config)
// }

/// IdatReader presents one or more IDAT chunks as one continuous stream (minus the
/// intermediate chunk headers and footers). If the PNG data looked like:
///
///	... len0 IDAT xxx crc0 len1 IDAT yy crc1 len2 IEND crc2
///
/// then this reader presents xxxyy. For well-formed PNG data, the decoder state
/// immediately before the first Read call is that d.r is positioned between the
/// first IDAT and xxx, and the decoder state immediately after the last Read
/// call is that d.r is positioned between yy and crc1.
struct IdatReader<'a> {
    idat_length: u32,
    r: &'a mut dyn std::io::Read,
    crc: &'a mut crc32::Digest,
}

impl std::io::Read for IdatReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        let mut tmp = [0_u8; 8];
        while self.idat_length == 0 {
            // We have exhausted an IDAT chunk. Verify the checksum of that chunk.
            verify_checksum(self.r, self.crc)?;
            // Read the length and chunk type of the next chunk, and check that
            // it is an IDAT chunk.
            self.r.read_exact(&mut tmp[..8])?;
            self.idat_length = binary::BIG_ENDIAN.uint32(&tmp[..4]);
            if &tmp[4..8] != b"IDAT" {
                return Err(new_format_error("not enough pixel data"));
            }
            self.crc.reset();
            self.crc.write_all(&tmp[4..8])?;
        }
        if self.idat_length >= 0x80000000 {
            return Err(new_unsupported_error("IDAT chunk length overflow"));
        }
        let bytes_to_read = buf.len().min(self.idat_length as usize);
        let n = self.r.read(&mut buf[0..bytes_to_read])?;
        self.crc.write_all(&buf[..n])?;
        self.idat_length -= n as u32;
        return Ok(n);
    }
}

struct DecodeParams {
    width: usize,
    height: usize,
    depth: u8,
    cb: CB,
    interlace: InterlaceType,
    use_transparent: bool,
    transparent: [u8; 6],
}

fn convert_palette(palette: &Vec<Color>) -> Palette {
    Palette {
        colors: palette.clone(),
    }
}
fn allocate_image<'a>(dp: &'a DecodeParams, palette: &Palette) -> Img {
    let r = image::rect(0, 0, dp.width as isize, dp.height as isize);

    match dp.cb {
        CB::G1 | CB::G2 | CB::G4 | CB::G8 => {
            if dp.use_transparent {
                Img::new_nrgba(&r)
            } else {
                Img::Gray(image::Gray::new(&r))
            }
        }
        CB::GA8 => Img::new_nrgba(&r),
        CB::TC8 => {
            if dp.use_transparent {
                Img::new_nrgba(&r)
            } else {
                Img::new_rgba(&r)
            }
        }
        CB::P1 | CB::P2 | CB::P4 | CB::P8 => {
            Img::Paletted(image::Paletted::new(&r, palette.clone()))
        }
        CB::TCA8 => Img::new_nrgba(&r),
        CB::G16 => {
            if dp.use_transparent {
                Img::new_nrgba64(&r)
            } else {
                Img::Gray16(image::Gray16::new(&r))
            }
        }
        CB::GA16 => Img::new_nrgba64(&r),
        CB::TC16 => {
            if dp.use_transparent {
                Img::new_nrgba64(&r)
            } else {
                Img::new_rgba64(&r)
            }
        }
        CB::TCA16 => Img::new_nrgba64(&r),
    }
}

/// read_image_pass reads a single image pass, sized according to the pass number.
fn read_image_pass<'a>(
    dp: &'a DecodeParams,
    r: &mut dyn std::io::Read,
    pass: usize,
    img: &mut Img,
) -> std::io::Result<bool> {
    let bits_per_pixel: usize;
    let mut pix_offset = 0;
    let mut width = dp.width;
    let mut height = dp.height;

    if dp.interlace == InterlaceType::Adam7 {
        let p = &INTERLACING[pass];
        // Add the multiplication factor and subtract one, effectively rounding up.
        width = ((width as isize + p.x_factor - p.x_offset - 1) / p.x_factor) as usize;
        height = ((height as isize + p.y_factor - p.y_offset - 1) / p.y_factor) as usize;
        // A PNG image can't have zero width or height, but for an interlaced
        // image, an individual pass might have zero width or height. If so, we
        // shouldn't even read a per-row filter type byte, so return early.
        if width == 0 || height == 0 {
            return Ok(false);
        }
    }

    // let mut img/* : Box<dyn image::Image>*/ =
    bits_per_pixel = match dp.cb {
        CB::G1 | CB::G2 | CB::G4 | CB::G8 => dp.depth as usize,
        CB::GA8 => 16,
        CB::TC8 => 24,
        CB::P1 | CB::P2 | CB::P4 | CB::P8 => dp.depth as usize,
        CB::TCA8 => 32,
        CB::G16 => 16,
        CB::GA16 => 32,
        CB::TC16 => 48,
        CB::TCA16 => 64,
    };
    let bytes_per_pixel = (bits_per_pixel + 7) / 8;

    // The +1 is for the per-row filter type, which is at cr[0].
    let row_size = 1 + ((bits_per_pixel as i64) * (width as i64) + 7) / 8;
    if row_size != (row_size as isize) as i64 {
        return Err(new_unsupported_error("dimension overflow"));
    }
    // cr and pr are the bytes for the current and previous row.
    let mut cr = vec![0_u8; row_size as usize];
    let mut pr = vec![0_u8; row_size as usize];

    let img_stride = img.stride();

    for y in 0..height {
        // Read the decompressed bytes.
        // 		_, err := io.ReadFull(r, cr)
        // 		if err != nil {
        // 			if err == io.EOF || err == io.ErrUnexpectedEOF {
        // 				return nil, FormatError("not enough pixel data")
        // 			}
        // 			return nil, err
        // 		}
        r.read_exact(&mut cr)?;

        // Apply the filter.
        let filter_type = cr[0];
        let cdat = &mut cr[1..];
        let pdat = &pr[1..];
        match filter_type {
            FT_NONE => {
                // No-op.
            }
            FT_SUB => {
                for i in bytes_per_pixel..cdat.len() {
                    cdat[i] = cdat[i].wrapping_add(cdat[i - bytes_per_pixel]);
                }
            }
            FT_UP => {
                for i in 0..pdat.len() {
                    cdat[i] = cdat[i].wrapping_add(pdat[i]);
                }
            }
            FT_AVERAGE => {
                // The first column has no column to the left of it, so it is a
                // special case. We know that the first column exists because we
                // check above that width != 0, and so len(cdat) != 0.
                for i in 0..bytes_per_pixel {
                    cdat[i] = cdat[i].wrapping_add(pdat[i] / 2);
                }
                for i in bytes_per_pixel..cdat.len() {
                    cdat[i] = cdat[i].wrapping_add(
                        (((cdat[i - bytes_per_pixel]) as isize + pdat[i] as isize) / 2) as u8,
                    );
                }
            }
            FT_PAETH => {
                filter_paeth(cdat, pdat, bytes_per_pixel);
            }
            _ => {
                return Err(new_format_error(&format!(
                    "bad filter type {}",
                    filter_type
                )))
            }
        }

        // Convert from bytes to colors.
        match img {
            Img::Paletted(img) => {
                read_line_to_paletted_image(dp, width, cdat, img, y, &mut pix_offset);
            }
            _ => {
                let img: &mut dyn Image = img;
                match dp.cb {
                    CB::G1 => {
                        if dp.use_transparent {
                            let ty = dp.transparent[1];
                            for x in (0..width).step_by(8) {
                                let mut b = cdat[x / 8];
                                let mut x2 = 0;
                                while x2 < 8 && x + x2 < width {
                                    let ycol = (b >> 7) * 0xff;
                                    let mut acol = 0xff_u8;
                                    if ycol == ty {
                                        acol = 0x00
                                    }
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::new_nrgba(ycol, ycol, ycol, acol),
                                    );
                                    b <<= 1;
                                    x2 += 1;
                                }
                            }
                        } else {
                            let mut x = 0;
                            while x < width {
                                let mut b = cdat[x / 8];
                                let mut x2 = 0;
                                while x2 < 8 && x + x2 < width {
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::Gray(Gray::new((b >> 7) * 0xff)),
                                    );
                                    b <<= 1;
                                    x2 += 1;
                                }
                                x += 8;
                            }
                        }
                    }
                    CB::G2 => {
                        if dp.use_transparent {
                            let ty = dp.transparent[1];
                            for x in (0..width).step_by(4) {
                                let mut b = cdat[x / 4];
                                let mut x2 = 0;
                                while x2 < 4 && x + x2 < width {
                                    let ycol = (b >> 6) * 0x55;
                                    let mut acol = 0xff_u8;
                                    if ycol == ty {
                                        acol = 0x00
                                    }
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::new_nrgba(ycol, ycol, ycol, acol),
                                    );
                                    b <<= 2;
                                    x2 += 1;
                                }
                            }
                        } else {
                            let mut x = 0;
                            while x < width {
                                let mut b = cdat[x / 4];
                                let mut x2 = 0;
                                while x2 < 4 && x + x2 < width {
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::Gray(Gray::new((b >> 6) * 0x55)),
                                    );
                                    b <<= 2;
                                    x2 += 1;
                                }
                                x += 4;
                            }
                        }
                    }
                    CB::G4 => {
                        if dp.use_transparent {
                            let ty = dp.transparent[1];
                            for x in (0..width).step_by(2) {
                                let mut b = cdat[x / 2];
                                let mut x2 = 0;
                                while x2 < 2 && x + x2 < width {
                                    let ycol = (b >> 4) * 0x11;
                                    let mut acol = 0xff_u8;
                                    if ycol == ty {
                                        acol = 0x00;
                                    }
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::new_nrgba(ycol, ycol, ycol, acol),
                                    );
                                    b <<= 4;
                                    x2 += 1;
                                }
                            }
                        } else {
                            for x in (0..width).step_by(2) {
                                let mut b = cdat[x / 2];
                                let mut x2 = 0;
                                while x2 < 2 && x + x2 < width {
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::new_gray((b >> 4) * 0x11),
                                    );
                                    b <<= 4;
                                    x2 += 1;
                                }
                            }
                            let mut x = 0;
                            while x < width {
                                let mut b = cdat[x / 2];
                                let mut x2 = 0;
                                while x2 < 2 && x + x2 < width {
                                    img.set(
                                        (x + x2) as isize,
                                        y as isize,
                                        &Color::Gray(Gray::new((b >> 4) * 0x11)),
                                    );
                                    b <<= 4;
                                    x2 += 1;
                                }
                                x += 2;
                            }
                        }
                    }
                    CB::G8 => {
                        if dp.use_transparent {
                            let ty = dp.transparent[1];
                            for x in 0..width {
                                let ycol = cdat[x];
                                let mut acol = 0xff_u8;
                                if ycol == ty {
                                    acol = 0x00;
                                }
                                img.set(
                                    x as isize,
                                    y as isize,
                                    &Color::new_nrgba(ycol, ycol, ycol, acol),
                                );
                            }
                        } else {
                            let pix_mutable = img.get_pix_mutable();
                            compat::copy(&mut pix_mutable[pix_offset..], cdat);
                            pix_offset += img_stride;
                        }
                    }
                    CB::GA8 => {
                        for x in 0..width {
                            let ycol = cdat[2 * x + 0];
                            img.set(
                                x as isize,
                                y as isize,
                                &Color::new_nrgba(ycol, ycol, ycol, cdat[2 * x + 1]),
                            );
                        }
                    }
                    CB::TC8 => {
                        if dp.use_transparent {
                            let pix_mutable = img.get_pix_mutable();
                            let (mut i, mut j) = (pix_offset, 0);
                            let (tr, tg, tb) =
                                (dp.transparent[1], dp.transparent[3], dp.transparent[5]);
                            for _x in 0..width {
                                let r = cdat[j + 0];
                                let g = cdat[j + 1];
                                let b = cdat[j + 2];
                                let mut a = 0xff_u8;
                                if r == tr && g == tg && b == tb {
                                    a = 0x00;
                                }
                                pix_mutable[i + 0] = r;
                                pix_mutable[i + 1] = g;
                                pix_mutable[i + 2] = b;
                                pix_mutable[i + 3] = a;
                                i += 4;
                                j += 3;
                            }
                            pix_offset += img_stride;
                        } else {
                            let pix_mutable = img.get_pix_mutable();
                            let (mut i, mut j) = (pix_offset, 0);
                            for _x in 0..width {
                                pix_mutable[i + 0] = cdat[j + 0];
                                pix_mutable[i + 1] = cdat[j + 1];
                                pix_mutable[i + 2] = cdat[j + 2];
                                pix_mutable[i + 3] = 0xff;
                                i += 4;
                                j += 3;
                            }
                            pix_offset += img_stride;
                        }
                    }
                    CB::TCA8 => {
                        let pix_mutable = img.get_pix_mutable();
                        compat::copy(&mut pix_mutable[pix_offset..], cdat);
                        pix_offset += img_stride;
                    }
                    CB::G16 => {
                        if dp.use_transparent {
                            let ty = ((dp.transparent[0] as u16) << 8) | (dp.transparent[1] as u16);
                            for x in 0..width {
                                let ycol =
                                    ((cdat[2 * x + 0] as u16) << 8) | (cdat[2 * x + 1] as u16);
                                let acol = if ycol == ty { 0x0000 } else { 0xffff };
                                img.set(
                                    x as isize,
                                    y as isize,
                                    &Color::new_nrgba64(ycol, ycol, ycol, acol),
                                );
                            }
                        } else {
                            for x in 0..width {
                                let ycol =
                                    ((cdat[2 * x + 0] as u16) << 8) | (cdat[2 * x + 1] as u16);
                                img.set(x as isize, y as isize, &Color::Gray16(Gray16::new(ycol)));
                            }
                        }
                    }
                    CB::GA16 => {
                        for x in 0..width {
                            let ycol = ((cdat[4 * x + 0] as u16) << 8) | (cdat[4 * x + 1] as u16);
                            let acol = ((cdat[4 * x + 2] as u16) << 8) | (cdat[4 * x + 3] as u16);
                            img.set(
                                x as isize,
                                y as isize,
                                &Color::new_nrgba64(ycol, ycol, ycol, acol),
                            );
                        }
                    }
                    CB::TC16 => {
                        if dp.use_transparent {
                            let tr = ((dp.transparent[0] as u16) << 8) | (dp.transparent[1] as u16);
                            let tg = ((dp.transparent[2] as u16) << 8) | (dp.transparent[3] as u16);
                            let tb = ((dp.transparent[4] as u16) << 8) | (dp.transparent[5] as u16);
                            for x in 0..width {
                                let r = ((cdat[6 * x + 0] as u16) << 8) | (cdat[6 * x + 1] as u16);
                                let g = ((cdat[6 * x + 2] as u16) << 8) | (cdat[6 * x + 3] as u16);
                                let b = ((cdat[6 * x + 4] as u16) << 8) | (cdat[6 * x + 5] as u16);
                                let a = if r == tr && g == tg && b == tb {
                                    0x0000
                                } else {
                                    0xffff
                                };
                                img.set(x as isize, y as isize, &Color::new_nrgba64(r, g, b, a));
                            }
                        } else {
                            for x in 0..width {
                                let r = ((cdat[6 * x + 0] as u16) << 8) | (cdat[6 * x + 1] as u16);
                                let g = ((cdat[6 * x + 2] as u16) << 8) | (cdat[6 * x + 3] as u16);
                                let b = ((cdat[6 * x + 4] as u16) << 8) | (cdat[6 * x + 5] as u16);
                                img.set(
                                    x as isize,
                                    y as isize,
                                    &Color::new_rgba64(r, g, b, 0xffff),
                                );
                            }
                        }
                    }
                    CB::TCA16 => {
                        for x in 0..width {
                            let r = ((cdat[8 * x + 0] as u16) << 8) | (cdat[8 * x + 1] as u16);
                            let g = ((cdat[8 * x + 2] as u16) << 8) | (cdat[8 * x + 3] as u16);
                            let b = ((cdat[8 * x + 4] as u16) << 8) | (cdat[8 * x + 5] as u16);
                            let a = ((cdat[8 * x + 6] as u16) << 8) | (cdat[8 * x + 7] as u16);
                            img.set(x as isize, y as isize, &Color::new_nrgba64(r, g, b, a));
                        }
                    }
                    _ => panic!("unreachable"),
                }
            }
        }

        // The current row for y is the previous row for y+1.
        std::mem::swap(&mut pr, &mut cr);
    }
    Ok(true)
}

fn read_line_to_paletted_image(
    dp: &DecodeParams,
    width: usize,
    cdat: &mut [u8],
    img: &mut image::Paletted,
    y: usize,
    pix_offset: &mut usize,
) {
    match dp.cb {
        CB::P1 => {
            for x in (0..width).step_by(8) {
                let mut b = cdat[x / 8];
                let mut x2 = 0;
                while x2 < 8 && x + x2 < width {
                    let idx = (b >> 7) as usize;
                    if img.palette.colors.len() <= idx {
                        img.palette.colors.resize(idx + 1, color::OPAQUE_BLACK);
                    }
                    img.set_color_index((x + x2) as isize, y as isize, idx as u8);
                    b <<= 1;
                    x2 += 1;
                }
            }
        }
        CB::P2 => {
            for x in (0..width).step_by(4) {
                let mut b = cdat[x / 4];
                let mut x2 = 0;
                while x2 < 4 && x + x2 < width {
                    let idx = (b >> 6) as usize;
                    if img.palette.colors.len() <= idx {
                        img.palette.colors.resize(idx + 1, color::OPAQUE_BLACK);
                    }
                    img.set_color_index((x + x2) as isize, y as isize, idx as u8);
                    b <<= 2;
                    x2 += 1;
                }
            }
        }
        CB::P4 => {
            for x in (0..width).step_by(2) {
                let mut b = cdat[x / 2];
                let mut x2 = 0;
                while x2 < 2 && x + x2 < width {
                    let idx = (b >> 4) as usize;
                    if img.palette.colors.len() <= idx {
                        img.palette.colors.resize(idx + 1, color::OPAQUE_BLACK);
                    }
                    img.set_color_index((x + x2) as isize, y as isize, idx as u8);
                    b <<= 4;
                    x2 += 1;
                }
            }
        }
        CB::P8 => {
            if img.palette.colors.len() != 256 {
                for x in 0..width {
                    if img.palette.colors.len() <= cdat[x] as usize {
                        img.palette
                            .colors
                            .resize((cdat[x] as usize) + 1, color::OPAQUE_BLACK);
                    }
                }
            }
            compat::copy(&mut img.get_pix_mutable()[*pix_offset..], &cdat);
            *pix_offset += img.stride();
        }
        _ => panic!("unreachable"),
    }
}
