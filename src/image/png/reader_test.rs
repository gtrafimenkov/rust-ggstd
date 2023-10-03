// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::reader::PNG_HEADER;
use crate::image::{
    self,
    color::{Color, ColorTrait, Model, NRGBA64},
    png::{decode, decode_config},
    Image, Img,
};
use crate::io as ggio;
use crate::os;
use crate::strings;
use crate::{bufio, bytes};
use std::collections::HashMap;

pub(super) const FILENAMES: &[&str] = &[
    "basn0g01",
    "basn0g01-30",
    "basn0g02",
    "basn0g02-29",
    "basn0g04",
    "basn0g04-31",
    "basn0g08",
    "basn0g16",
    "basn2c08",
    "basn2c16",
    "basn3p01",
    "basn3p02",
    "basn3p04",
    "basn3p04-31i",
    "basn3p08",
    "basn3p08-trns",
    "basn4a08",
    "basn4a16",
    "basn6a08",
    "basn6a16",
    "ftbbn0g01",
    "ftbbn0g02",
    "ftbbn0g04",
    "ftbbn2c16",
    "ftbbn3p08",
    "ftbgn2c16",
    "ftbgn3p08",
    "ftbrn2c08",
    "ftbwn0g16",
    "ftbwn3p08",
    "ftbyn3p08",
    "ftp0n0g08",
    "ftp0n2c08",
    "ftp0n3p08",
    "ftp1n3p08",
];

const FILENAMES_PALETTED: &[&str] = &[
    "basn3p01",
    "basn3p02",
    "basn3p04",
    "basn3p08",
    "basn3p08-trns",
];

pub(super) fn read_png(filename: &str) -> std::io::Result<Box<Img>> {
    let mut f = std::fs::File::open(filename)?;
    super::decode(&mut f)
}

/// An approximation of the sng command-line tool.
fn sng(w: &mut dyn std::io::Write, filename: &str, png: &Img) -> std::io::Result<()> {
    // fakeIHDRUsings maps from filenames to fake IHDR "using" lines for our
    // approximation to the sng command-line tool. The PNG model is that
    // transparency (in the tRNS chunk) is separate to the color/grayscale/palette
    // color model (in the IHDR chunk). The Go model is that the concrete
    // image.Image type returned by png.decode, such as image.RGBA (with all pixels
    // having 100% alpha) or image.NRGBA, encapsulates whether or not the image has
    // transparency. This map is a hack to work around the fact that the Go model
    // can't otherwise discriminate PNG's "IHDR says color (with no alpha) but tRNS
    // says alpha" and "IHDR says color with alpha".
    let mut fake_ihdr_usings = HashMap::new();
    fake_ihdr_usings.insert("ftbbn0g01", "    using grayscale;\n");
    fake_ihdr_usings.insert("ftbbn0g02", "    using grayscale;\n");
    fake_ihdr_usings.insert("ftbbn0g04", "    using grayscale;\n");
    fake_ihdr_usings.insert("ftbbn2c16", "    using color;\n");
    fake_ihdr_usings.insert("ftbgn2c16", "    using color;\n");
    fake_ihdr_usings.insert("ftbrn2c08", "    using color;\n");
    fake_ihdr_usings.insert("ftbwn0g16", "    using grayscale;\n");

    // fakegAMAs maps from filenames to fake gAMA chunks for our approximation to
    // the sng command-line tool. Package png doesn't keep that metadata when
    // png.decode returns an image.Image.
    let mut fake_gamas = HashMap::new();
    fake_gamas.insert("ftbbn0g01", "");
    fake_gamas.insert("ftbbn0g02", "gAMA {0.45455}\n");

    // fake_bKGDs maps from filenames to fake bKGD chunks for our approximation to
    // the sng command-line tool. Package png doesn't keep that metadata when
    // png.decode returns an image.Image.
    let mut fake_bkgds = HashMap::new();
    fake_bkgds.insert("ftbbn0g01", "bKGD {gray: 0;}\n");
    fake_bkgds.insert("ftbbn0g02", "bKGD {gray: 0;}\n");
    fake_bkgds.insert("ftbbn0g04", "bKGD {gray: 0;}\n");
    fake_bkgds.insert("ftbbn2c16", "bKGD {red: 0;  green: 0;  blue: 65535;}\n");
    fake_bkgds.insert("ftbbn3p08", "bKGD {index: 245}\n");
    fake_bkgds.insert("ftbgn2c16", "bKGD {red: 0;  green: 65535;  blue: 0;}\n");
    fake_bkgds.insert("ftbgn3p08", "bKGD {index: 245}\n");
    fake_bkgds.insert("ftbrn2c08", "bKGD {red: 255;  green: 0;  blue: 0;}\n");
    fake_bkgds.insert("ftbwn0g16", "bKGD {gray: 65535;}\n");
    fake_bkgds.insert("ftbwn3p08", "bKGD {index: 0}\n");
    fake_bkgds.insert("ftbyn3p08", "bKGD {index: 245}\n");

    let bounds = png.bounds();
    let cm = png.color_model();
    let bitdepth = cm.bitdepth();

    // Write the filename and IHDR.
    w.write_all(format!("#SNG: from {}.png\nIHDR {{\n", filename).as_bytes())?;
    w.write_all(
        format!(
            "    width: {}; height: {}; bitdepth: {};\n",
            bounds.dx(),
            bounds.dy(),
            bitdepth
        )
        .as_bytes(),
    )?;
    if let Some(s) = fake_ihdr_usings.get(filename) {
        ggio::write_string(w, s)?;
    } else {
        match cm {
            Model::RGBAModel | Model::RGBA64Model => {
                ggio::write_string(w, "    using color;\n")?;
            }
            Model::NRGBAModel | Model::NRGBA64Model => {
                ggio::write_string(w, "    using color alpha;\n")?;
            }
            Model::GrayModel | Model::Gray16Model => {
                ggio::write_string(w, "    using grayscale;\n")?;
            }
            Model::Paletted(_) => {
                ggio::write_string(w, "    using color palette;\n")?;
            }
            _ => {
                ggio::write_string(w, "unknown PNG decoder color model\n")?;
            }
        };
    }
    w.write_all("}\n".as_bytes())?;

    // We fake a gAMA chunk. The test files have a gAMA chunk but the go PNG
    // parser ignores it (the PNG spec section 11.3 says "Ancillary chunks may
    // be ignored by a decoder").
    if let Some(s) = fake_gamas.get(filename) {
        ggio::write_string(w, s)?;
    } else {
        ggio::write_string(w, "gAMA {1.0000}\n")?;
    }

    // Write the PLTE and tRNS (if applicable).
    let mut use_transparent = false;
    match &cm {
        Model::Paletted(pal) => {
            let mut last_alpha = None;
            ggio::write_string(w, "PLTE {\n")?;
            for (i, c) in pal.colors.iter().enumerate() {
                let (r, g, b, a) = match c {
                    Color::RGBA(c) => (c.r, c.g, c.b, 0xff),
                    Color::NRGBA(c) => (c.r, c.g, c.b, c.a),
                    _ => {
                        panic!("unknown palette color type");
                    }
                };
                if a != 0xff {
                    last_alpha = Some(i);
                }
                ggio::write_string(
                    w,
                    &format!(
                        "    ({:3},{:3},{:3})     # rgb = (0x{:02x},0x{:02x},0x{:02x})\n",
                        r, g, b, r, g, b
                    ),
                )?;
            }
            ggio::write_string(w, "}\n")?;
            if let Some(s) = fake_bkgds.get(filename) {
                ggio::write_string(w, s)?;
            }
            if let Some(last_alpha) = last_alpha {
                ggio::write_string(w, "tRNS {\n")?;
                for i in 0..=last_alpha {
                    let (_, _, _, mut a) = pal.colors[i].rgba();
                    a >>= 8;
                    ggio::write_string(w, &format!(" {}", a))?;
                }
                ggio::write_string(w, "}\n")?;
            }
        }
        _ => {
            if filename.starts_with("ft") {
                if let Some(s) = fake_bkgds.get(filename) {
                    ggio::write_string(w, s)?;
                }
                // We fake a tRNS chunk. The test files' grayscale and truecolor
                // transparent images all have their top left corner transparent.
                match png.at(0, 0) {
                    Color::NRGBA(c) => {
                        if c.a == 0 {
                            use_transparent = true;
                            ggio::write_string(w, "tRNS {\n")?;
                            match filename {
                                "ftbbn0g01" | "ftbbn0g02" | "ftbbn0g04" => {
                                    // The standard image package doesn't have a "gray with
                                    // alpha" type. Instead, we use an image.NRGBA.
                                    ggio::write_string(w, &format!("    gray: {};\n", c.r,))?;
                                }
                                _ => {
                                    ggio::write_string(
                                        w,
                                        &format!(
                                            "    red: {}; green: {}; blue: {};\n",
                                            c.r, c.g, c.b,
                                        ),
                                    )?;
                                }
                            }
                            ggio::write_string(w, "}\n")?;
                        }
                    }
                    Color::NRGBA64(c) => {
                        if c.a == 0 {
                            use_transparent = true;
                            ggio::write_string(w, "tRNS {\n")?;
                            match filename {
                                "ftbwn0g16" => {
                                    // The standard image package doesn't have a "gray16 with
                                    // alpha" type. Instead, we use an image.NRGBA64.
                                    ggio::write_string(w, &format!("    gray: {};\n", c.r,))?;
                                }
                                _ => {
                                    ggio::write_string(
                                        w,
                                        &format!(
                                            "    red: {}; green: {}; blue: {};\n",
                                            c.r, c.g, c.b,
                                        ),
                                    )?;
                                }
                            }
                            ggio::write_string(w, "}\n")?;
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    // Write the IMAGE.
    w.write_all(b"IMAGE {\n    pixels hex\n")?;
    for y in bounds.min.y..bounds.max.y {
        match &cm {
            Model::GrayModel => {
                for x in bounds.min.x..bounds.max.x {
                    if let Color::Gray(gray) = png.at(x, y) {
                        ggio::write_string(w, &format!("{:02x}", gray.y))?;
                    }
                }
            }
            Model::Gray16Model => {
                for x in bounds.min.x..bounds.max.x {
                    if let Color::Gray16(gray16) = png.at(x, y) {
                        ggio::write_string(w, &format!("{:04x} ", gray16.y))?;
                    }
                }
            }
            Model::RGBAModel => {
                for x in bounds.min.x..bounds.max.x {
                    if let Color::RGBA(rgba) = png.at(x, y) {
                        ggio::write_string(
                            w,
                            &format!("{:02x}{:02x}{:02x} ", rgba.r, rgba.g, rgba.b),
                        )?;
                    }
                }
            }
            Model::RGBA64Model => {
                for x in bounds.min.x..bounds.max.x {
                    if let Color::RGBA64(rgba64) = png.at(x, y) {
                        ggio::write_string(
                            w,
                            &format!("{:04x}{:04x}{:04x} ", rgba64.r, rgba64.g, rgba64.b),
                        )?;
                    }
                }
            }
            Model::NRGBAModel => {
                for x in bounds.min.x..bounds.max.x {
                    if let Color::NRGBA(nrgba) = png.at(x, y) {
                        match filename {
                            "ftbbn0g01" | "ftbbn0g02" | "ftbbn0g04" => {
                                ggio::write_string(w, &format!("{:02x}", nrgba.r))?;
                            }
                            _ => {
                                if use_transparent {
                                    ggio::write_string(
                                        w,
                                        &format!("{:02x}{:02x}{:02x} ", nrgba.r, nrgba.g, nrgba.b),
                                    )?;
                                } else {
                                    ggio::write_string(
                                        w,
                                        &format!(
                                            "{:02x}{:02x}{:02x}{:02x} ",
                                            nrgba.r, nrgba.g, nrgba.b, nrgba.a
                                        ),
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
            Model::NRGBA64Model => {
                for x in bounds.min.x..bounds.max.x {
                    if let Color::NRGBA64(nrgba64) = png.at(x, y) {
                        match filename {
                            "ftbwn0g16" => {
                                ggio::write_string(w, &format!("{:04x} ", nrgba64.r))?;
                            }
                            _ => {
                                if use_transparent {
                                    ggio::write_string(
                                        w,
                                        &format!(
                                            "{:04x}{:04x}{:04x} ",
                                            nrgba64.r, nrgba64.g, nrgba64.b
                                        ),
                                    )?;
                                } else {
                                    ggio::write_string(
                                        w,
                                        &format!(
                                            "{:04x}{:04x}{:04x}{:04x} ",
                                            nrgba64.r, nrgba64.g, nrgba64.b, nrgba64.a
                                        ),
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
            Model::Paletted(_pal) => {
                if let Img::Paletted(paletted) = png {
                    let mut b: usize = 0;
                    let mut c: usize = 0;
                    for x in bounds.min.x..bounds.max.x {
                        b = (b << bitdepth) | (paletted.color_index_at(x, y) as usize);
                        c += 1;
                        if c == 8 / bitdepth {
                            ggio::write_string(w, &format!("{:02x}", b))?;
                            b = 0;
                            c = 0;
                        }
                    }
                    if c != 0 {
                        while c != 8 / bitdepth {
                            b <<= bitdepth;
                            c += 1;
                        }
                        ggio::write_string(w, &format!("{:02x}", b))?;
                    }
                }
            }
            Model::AlphaModel => todo!(),
            Model::Alpha16Model => todo!(),
        }
        ggio::write_string(w, "\n")?;
    }
    w.write_all(b"}\n")?;
    Ok(())
}

#[test]
fn test_reader() {
    let names = FILENAMES;
    for filename in names {
        // Read the .png file.
        let file_path = format!(
            "{}{}{}",
            "src/image/png/testdata/pngsuite/", filename, ".png"
        );
        let img = read_png(&file_path).unwrap();

        if *filename == "basn4a16" {
            // basn4a16.sng is gray + alpha but sng() will produce true color + alpha
            // so we just check a single random pixel.
            let c = NRGBA64::new_from(&img.at(2, 1)); //.(color.NRGBA64)
            assert!(
                c.r == 0x11a7 && c.g == 0x11a7 && c.b == 0x11a7 && c.a == 0x1085,
                "{}: wrong pixel value at (2, 1): {:x?}",
                filename,
                c
            );
            continue;
        }

        let mut gensng = bytes::Buffer::new();
        sng(&mut gensng, filename, &img).unwrap();

        let mut pb = bufio::Scanner::new(&mut gensng);

        // Read the .sng file.
        let mut sf =
            os::open(&format!("src/image/png/testdata/pngsuite/{}.sng", filename)).unwrap();
        let mut sb = bufio::Scanner::new(&mut sf);

        // Compare the two, in SNG format, line by line.
        loop {
            let pdone = !pb.scan();
            let sdone = !sb.scan();
            if pdone && sdone {
                break;
            }
            assert_eq!(pdone, sdone, "{}: Different sizes", filename);
            let ps = pb.text();
            let mut ss = sb.text();

            // Newer versions of the sng command line tool append an optional
            // color name to the RGB tuple. For example:
            //	# rgb = (0xff,0xff,0xff) grey100
            //	# rgb = (0x00,0x00,0xff) blue1
            // instead of the older version's plainer:
            //	# rgb = (0xff,0xff,0xff)
            //	# rgb = (0x00,0x00,0xff)
            // We strip any such name.
            if strings::contains(&ss, "# rgb = (") && !strings::has_suffix(&ss, ")") {
                if let Some(i) = ss.rfind(") ") {
                    ss.truncate(i + 1);
                }
            }

            assert_eq!(ps, ss, "{}: Mismatch\n{}\nversus\n{}\n", filename, ps, ss);
        }
        assert!(pb.err().is_none(), "pb error: {:?}", pb.err());
        assert!(sb.err().is_none(), "sb error: {:?}", sb.err());
    }
}

struct ReaderErrorTest {
    file: &'static str,
    err: &'static str,
}

static READER_ERRORS: &[ReaderErrorTest] = &[
    ReaderErrorTest {
        file: "invalid-zlib.png",
        err: "zlib: invalid checksum",
    },
    ReaderErrorTest {
        file: "invalid-crc32.png",
        err: "invalid checksum",
    },
    ReaderErrorTest {
        file: "invalid-noend.png",
        err: "failed to fill whole buffer", // "unexpected EOF",
    },
    ReaderErrorTest {
        file: "invalid-trunc.png",
        err: "failed to fill whole buffer", // "unexpected EOF",
    },
];

#[test]
fn test_reader_error() {
    for tt in READER_ERRORS {
        let res = read_png(&format!("src/image/png/testdata/{}", tt.file));
        assert!(res.is_err(), "decoding {}: missing error", tt.file);
        let err = res.err().unwrap();
        assert!(
            tt.err.contains(&err.to_string()),
            "decoding {}: {:?}, want {}",
            tt.file,
            err,
            tt.err
        );
    }
}

#[test]
fn test_paletted_decode_config() {
    for filename in FILENAMES_PALETTED {
        let mut f = os::open(&format!("src/image/png/testdata/pngsuite/{}.png", filename)).unwrap();
        let cfg = decode_config(&mut f).unwrap();
        match cfg.color_model {
            image::color::Model::Paletted(pal) => {
                assert!(
                    !pal.colors.is_empty(),
                    "{}: palette not initialized",
                    filename
                )
            }
            _ => {
                assert!(false, "{}: expected paletted color model", filename);
            }
        }
    }
}

#[test]
fn test_interlaced() {
    let a = read_png("src/image/png/testdata/gray-gradient.png").unwrap();
    let b = read_png("src/image/png/testdata/gray-gradient.interlaced.png").unwrap();
    assert!(
        a.deep_equal(b.as_ref()),
        "decodings differ: non-interlaced and interlaced"
    );
    assert!(
        b.deep_equal(a.as_ref()),
        "decodings differ: non-interlaced and interlaced"
    );
}

#[test]
fn test_incomplete_idaton_row_boundary() {
    // The following is an invalid 1x2 grayscale PNG image. The header is OK,
    // but the zlib-compressed IDAT payload contains two bytes "\x02\x00",
    // which is only one row of data (the leading "\x02" is a row filter).
    let ihdr =
        b"\x00\x00\x00\x0dIHDR\x00\x00\x00\x01\x00\x00\x00\x02\x08\x00\x00\x00\x00\xbc\xea\xe9\xfb";
    let idat = b"\x00\x00\x00\x0eIDAT\x78\x9c\x62\x62\x00\x04\x00\x00\xff\xff\x00\x06\x00\x03\xfa\xd0\x59\xae";
    let iend = b"\x00\x00\x00\x00IEND\xae\x42\x60\x82";
    let mut data = Vec::new();
    data.extend_from_slice(PNG_HEADER);
    data.extend_from_slice(ihdr);
    data.extend_from_slice(idat);
    data.extend_from_slice(iend);
    let res = decode(&mut bytes::Reader::new(&data));
    assert!(res.is_err(), "got nil error, want non-nil");
}

#[test]
fn test_trailing_idatchunks() {
    // The following is a valid 1x1 PNG image containing color.Gray{255} and
    // a trailing zero-length IDAT chunk (see PNG specification section 12.9):
    let ihdr =
        b"\x00\x00\x00\x0dIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x00\x00\x00\x00\x3a\x7e\x9b\x55";
    let idat_white = b"\x00\x00\x00\x0eIDAT\x78\x9c\x62\xfa\x0f\x08\x00\x00\xff\xff\x01\x05\x01\x02\x5a\xdd\x39\xcd";
    let idat_zero = b"\x00\x00\x00\x00IDAT\x35\xaf\x06\x1e";
    let iend = b"\x00\x00\x00\x00IEND\xae\x42\x60\x82";
    let mut data = Vec::new();
    data.extend_from_slice(PNG_HEADER);
    data.extend_from_slice(ihdr);
    data.extend_from_slice(idat_white);
    data.extend_from_slice(idat_zero);
    data.extend_from_slice(iend);
    let res = decode(&mut bytes::Reader::new(&data));
    assert!(res.is_ok(), "decoding valid image: {}", res.err().unwrap());

    // Non-zero-length trailing IDAT chunks should be ignored (recoverable error).
    // The following chunk contains a single pixel with color.Gray{0}.
    let idat_black = b"\x00\x00\x00\x0eIDAT\x78\x9c\x62\x62\x00\x04\x00\x00\xff\xff\x00\x06\x00\x03\xfa\xd0\x59\xae";

    let mut data = Vec::new();
    data.extend_from_slice(PNG_HEADER);
    data.extend_from_slice(ihdr);
    data.extend_from_slice(idat_white);
    data.extend_from_slice(idat_black);
    data.extend_from_slice(iend);
    let res = decode(&mut bytes::Reader::new(&data));
    assert!(
        res.is_ok(),
        "trailing IDAT not ignored: {}",
        res.err().unwrap()
    );
    let img = res.unwrap();
    let c = img.at(0, 0);
    assert_eq!(
        c,
        Color::new_gray(255),
        "decoded image from trailing IDAT chunk"
    );
}

#[test]
fn test_multiplet_rnschunks() {
    /*
        The following is a valid 1x1 paletted PNG image with a 1-element palette
        containing color.NRGBA{0xff, 0x00, 0x00, 0x7f}:
            0000000: 8950 4e47 0d0a 1a0a 0000 000d 4948 4452  .PNG........IHDR
            0000010: 0000 0001 0000 0001 0803 0000 0028 cb34  .............(.4
            0000020: bb00 0000 0350 4c54 45ff 0000 19e2 0937  .....PLTE......7
            0000030: 0000 0001 7452 4e53 7f80 5cb4 cb00 0000  ....tRNS..\.....
            0000040: 0e49 4441 5478 9c62 6200 0400 00ff ff00  .IDATx.bb.......
            0000050: 0600 03fa d059 ae00 0000 0049 454e 44ae  .....y.....IEND.
            0000060: 4260 82                                  B`.
        Dropping the tRNS chunk makes that color's alpha 0xff instead of 0x7f.
    */
    let ihdr =
        b"\x00\x00\x00\x0dIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x03\x00\x00\x00\x28\xcb\x34\xbb";
    let plte = b"\x00\x00\x00\x03PLTE\xff\x00\x00\x19\xe2\x09\x37";
    let trns = b"\x00\x00\x00\x01tRNS\x7f\x80\x5c\xb4\xcb";
    let idat = b"\x00\x00\x00\x0eIDAT\x78\x9c\x62\x62\x00\x04\x00\x00\xff\xff\x00\x06\x00\x03\xfa\xd0\x59\xae";
    let iend = b"\x00\x00\x00\x00IEND\xae\x42\x60\x82";
    for i in 0..4 {
        let mut data = Vec::new();
        data.extend_from_slice(PNG_HEADER);
        data.extend_from_slice(ihdr);
        data.extend_from_slice(plte);
        for _j in 0..i {
            data.extend_from_slice(trns);
        }
        data.extend_from_slice(idat);
        data.extend_from_slice(iend);

        let res = decode(&mut bytes::Reader::new(&data));
        match i {
            0 => {
                assert!(res.is_ok(), "{} tRNS chunks: {}", i, res.err().unwrap());
                let want = Color::new_rgba(0xff, 0x00, 0x00, 0xff);
                let got = res.unwrap().at(0, 0);
                assert_eq!(
                    got, want,
                    "{} tRNS chunks: got {:?}, want {:?}",
                    i, got, want
                )
            }
            1 => {
                assert!(res.is_ok(), "{} tRNS chunks: {}", i, res.err().unwrap());

                let want = Color::new_nrgba(0xff, 0x00, 0x00, 0x7f);
                let got = res.unwrap().at(0, 0);
                assert_eq!(
                    got, want,
                    "{} tRNS chunks: got {:?}, want {:?}",
                    i, got, want
                )
            }
            _ => {
                assert!(
                    res.is_err(),
                    "{} tRNS chunks: got nil error, want non-nil",
                    i
                );
            }
        }
    }
}

#[test]
fn test_unknown_chunk_length_underflow() {
    let data = &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0x06, 0xf4, 0x7c, 0x55, 0x04, 0x1a, 0xd3, 0x11, 0x9a, 0x73, 0x00, 0x00, 0xf8, 0x1e,
        0xf3, 0x2e, 0x00, 0x00, 0x01, 0x00, 0xff, 0xff, 0xff, 0xff, 0x07, 0xf4, 0x7c, 0x55, 0x04,
        0x1a, 0xd3,
    ];
    let res = decode(&mut bytes::Reader::new(data));
    assert!(
        res.is_err(),
        "Didn't fail reading an unknown chunk with length 0xffffffff"
    );
}

#[test]
fn test_paletted8_out_of_range_pixel() {
    // IDAT contains a reference to a palette index that does not exist in the file.
    let res = read_png("src/image/png/testdata/invalid-palette.png");
    assert!(
        res.is_ok(),
        "decoding invalid-palette.png: unexpected error {}",
        res.err().unwrap()
    );

    // Expect that the palette is extended with opaque black.
    let want = Color::new_rgba(0x00, 0x00, 0x00, 0xff);
    let got = res.unwrap().at(15, 15);
    assert_eq!(got, want, "got {:?}, expected {:?}", got, want);
}

#[test]
fn test_gray8_transparent() {
    // These bytes come from https://golang.org/issues/19553
    let m = decode(&mut bytes::Reader::new(&[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x0b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x85,
        0x2c, 0x88, 0x80, 0x00, 0x00, 0x00, 0x02, 0x74, 0x52, 0x4e, 0x53, 0x00, 0xff, 0x5b, 0x91,
        0x22, 0xb5, 0x00, 0x00, 0x00, 0x02, 0x62, 0x4b, 0x47, 0x44, 0x00, 0xff, 0x87, 0x8f, 0xcc,
        0xbf, 0x00, 0x00, 0x00, 0x09, 0x70, 0x48, 0x59, 0x73, 0x00, 0x00, 0x0a, 0xf0, 0x00, 0x00,
        0x0a, 0xf0, 0x01, 0x42, 0xac, 0x34, 0x98, 0x00, 0x00, 0x00, 0x07, 0x74, 0x49, 0x4d, 0x45,
        0x07, 0xd5, 0x04, 0x02, 0x12, 0x11, 0x11, 0xf7, 0x65, 0x3d, 0x8b, 0x00, 0x00, 0x00, 0x4f,
        0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8, 0xff, 0xff, 0xff, 0xb9, 0xbd, 0x70, 0xf0,
        0x8c, 0x01, 0xc8, 0xaf, 0x6e, 0x99, 0x02, 0x05, 0xd9, 0x7b, 0xc1, 0xfc, 0x6b, 0xff, 0xa1,
        0xa0, 0x87, 0x30, 0xff, 0xd9, 0xde, 0xbd, 0xd5, 0x4b, 0xf7, 0xee, 0xfd, 0x0e, 0xe3, 0xef,
        0xcd, 0x06, 0x19, 0x14, 0xf5, 0x1e, 0xce, 0xef, 0x01, 0x31, 0x92, 0xd7, 0x82, 0x41, 0x31,
        0x9c, 0x3f, 0x07, 0x02, 0xee, 0xa1, 0xaa, 0xff, 0xff, 0x9f, 0xe1, 0xd9, 0x56, 0x30, 0xf8,
        0x0e, 0xe5, 0x03, 0x00, 0xa9, 0x42, 0x84, 0x3d, 0xdf, 0x8f, 0xa6, 0x8f, 0x00, 0x00, 0x00,
        0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ]))
    .unwrap();

    let hex = b"0123456789abcdef";
    let mut got = Vec::new();
    let bounds = m.bounds();
    for y in bounds.min.y..bounds.max.y {
        for x in bounds.min.x..bounds.max.x {
            let (r, _, _, a) = m.at(x, y).rgba();
            if a != 0 {
                got.extend_from_slice(&[
                    hex[(0x0f & (r >> 12)) as usize],
                    hex[(0x0f & (r >> 8)) as usize],
                    b' ',
                ]);
            } else {
                got.extend_from_slice(&[b'.', b'.', b' ']);
            }
        }
        got.push(b'\n');
    }
    let got = String::from_utf8_lossy(&got).to_string();
    let want = "\
.. .. .. ce bd bd bd bd bd bd bd bd bd bd e6 \n\
.. .. .. 7b 84 94 94 94 94 94 94 94 94 6b bd \n\
.. .. .. 7b d6 .. .. .. .. .. .. .. .. 8c bd \n\
.. .. .. 7b d6 .. .. .. .. .. .. .. .. 8c bd \n\
.. .. .. 7b d6 .. .. .. .. .. .. .. .. 8c bd \n\
e6 bd bd 7b a5 bd bd f7 .. .. .. .. .. 8c bd \n\
bd 6b 94 94 94 94 5a ef .. .. .. .. .. 8c bd \n\
bd 8c .. .. .. .. 63 ad ad ad ad ad ad 73 bd \n\
bd 8c .. .. .. .. 63 9c 9c 9c 9c 9c 9c 9c de \n\
bd 6b 94 94 94 94 5a ef .. .. .. .. .. .. .. \n\
e6 b5 b5 b5 b5 b5 b5 f7 .. .. .. .. .. .. .. \n";
    assert_eq!(got, want, "got:\n{}want:\n{}", got, want);
}

#[test]
fn test_dimension_overflow() {
    const HAVE32_BIT_INTS: bool = std::mem::size_of::<isize>() == 4;

    struct TestCase {
        src: &'static [u8],
        unsupported_config: bool,
        width: usize,
        height: usize,
    }
    static TEST_CASES: &[TestCase] = &[
        // These bytes come from https://golang.org/issues/22304
        //
        // It encodes a 2147483646 × 2147483646 (i.e. 0x7ffffffe × 0x7ffffffe)
        // NRGBA image. The (width × height) per se doesn't overflow an int64, but
        // (width × height × bytesPerPixel) will.
        TestCase {
            src: &[
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x7f, 0xff, 0xff, 0xfe, 0x7f, 0xff, 0xff, 0xfe, 0x08, 0x06, 0x00, 0x00,
                0x00, 0x30, 0x57, 0xb3, 0xfd, 0x00, 0x00, 0x00, 0x15, 0x49, 0x44, 0x41, 0x54, 0x78,
                0x9c, 0x62, 0x62, 0x20, 0x12, 0x8c, 0x2a, 0xa4, 0xb3, 0x42, 0x40, 0x00, 0x00, 0x00,
                0xff, 0xff, 0x13, 0x38, 0x00, 0x15, 0x2d, 0xef, 0x5f, 0x0f, 0x00, 0x00, 0x00, 0x00,
                0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
            ],
            // It's debatable whether decode_config (which does not allocate a
            // pixel buffer, unlike decode) should fail in this case. The Go
            // standard library has made its choice, and the standard library
            // has compatibility constraints.
            unsupported_config: true,
            width: 0x7ffffffe,
            height: 0x7ffffffe,
        },
        // The next three cases come from https://golang.org/issues/38435
        TestCase {
            src: &[
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0xb5, 0x04, 0x00, 0x00, 0xb5, 0x04, 0x08, 0x06, 0x00, 0x00,
                0x00, 0xf5, 0x60, 0x2c, 0xb8, 0x00, 0x00, 0x00, 0x15, 0x49, 0x44, 0x41, 0x54, 0x78,
                0x9c, 0x62, 0x62, 0x20, 0x12, 0x8c, 0x2a, 0xa4, 0xb3, 0x42, 0x40, 0x00, 0x00, 0x00,
                0xff, 0xff, 0x13, 0x38, 0x00, 0x15, 0x2d, 0xef, 0x5f, 0x0f, 0x00, 0x00, 0x00, 0x00,
                0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
            ],
            // Here, width * height = 0x7ffea810, just under MaxInt32, but at 4
            // bytes per pixel, the number of pixels overflows an int32.
            unsupported_config: HAVE32_BIT_INTS,
            width: 0x0000b504,
            height: 0x0000b504,
        },
        TestCase {
            src: &[
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
                0x00, 0x30, 0x6e, 0xc5, 0x21, 0x00, 0x00, 0x00, 0x15, 0x49, 0x44, 0x41, 0x54, 0x78,
                0x9c, 0x62, 0x62, 0x20, 0x12, 0x8c, 0x2a, 0xa4, 0xb3, 0x42, 0x40, 0x00, 0x00, 0x00,
                0xff, 0xff, 0x13, 0x38, 0x00, 0x15, 0x2d, 0xef, 0x5f, 0x0f, 0x00, 0x00, 0x00, 0x00,
                0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
            ],
            unsupported_config: false,
            width: 0x04000000,
            height: 0x00000001,
        },
        TestCase {
            src: &[
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
                0x00, 0xaa, 0xd4, 0x7c, 0xda, 0x00, 0x00, 0x00, 0x15, 0x49, 0x44, 0x41, 0x54, 0x78,
                0x9c, 0x62, 0x66, 0x20, 0x12, 0x30, 0x8d, 0x2a, 0xa4, 0xaf, 0x42, 0x40, 0x00, 0x00,
                0x00, 0xff, 0xff, 0x14, 0xd2, 0x00, 0x16, 0x00, 0x00, 0x00,
            ],
            unsupported_config: false,
            width: 0x08000000,
            height: 0x00000001,
        },
    ];

    for (i, tc) in TEST_CASES.iter().enumerate() {
        let cfg = decode_config(&mut bytes::Reader::new(tc.src));
        if tc.unsupported_config {
            match cfg {
                Ok(_) => {
                    assert!(false, "i={}: decode_config: got nil error, want non-nil", i);
                }
                Err(err) => {
                    let msg = err.to_string();
                    assert!(
                        msg.starts_with("png: unsupported feature:"),
                        "decode: got {:?}, want non-nil error (of type png.UnsupportedError)",
                        err
                    );
                }
            }
            continue;
        }
        assert!(
            cfg.is_ok(),
            "i={}: decode_config: {:?}",
            i,
            cfg.err().unwrap()
        );
        let cfg = cfg.unwrap();
        assert_eq!(
            cfg.width, tc.width,
            "i={}: width: got {}, want {}",
            i, cfg.width, tc.width
        );
        assert_eq!(
            cfg.height, tc.height,
            "i={}: height: got {}, want {}",
            i, cfg.height, tc.height
        );

        #[cfg(windows)]
        {
            // skip tests that fail on Windows in GitHub Actions

            let n_pixels = (cfg.width as u64) * (cfg.height as u64);
            if n_pixels > 0x7f000000 {
                // In theory, calling decode would succeed, given several gigabytes
                // of memory. In practice, trying to make a []uint8 big enough to
                // hold all of the pixels can often result in OOM (out of memory).
                // OOM is unrecoverable; we can't write a test that passes when OOM
                // happens. Instead we skip the decode call (and its tests).
                continue;
            }
        }

        // 		if testing.Short() {
        // 			// Even for smaller image dimensions, calling decode might allocate
        // 			// 1 GiB or more of memory. This is usually feasible, and we want
        // 			// to check that calling decode doesn't panic if there's enough
        // 			// memory, but we provide a runtime switch (testing.Short) to skip
        // 			// these if it would OOM. See also http://golang.org/issue/5050
        // 			// "decoding... images can cause huge memory allocations".
        // 			continue
        // 		}

        // Even if we don't panic, these aren't valid PNG images.
        let res = decode(&mut bytes::Reader::new(tc.src));
        assert!(res.is_err(), "i={}: decode: got nil error, want non-nil", i);
    }
}

#[test]
fn test_decode_paletted_with_transparency() {
    // These bytes come from https://go.dev/issue/54325
    //
    // Per the PNG spec, a PLTE chunk contains 3 (not 4) bytes per palette
    // entry: RGB (not RGBA). The alpha value comes from the optional tRNS
    // chunk. Here, the PLTE chunk (0x50, 0x4c, 0x54, 0x45, etc) has 16 entries
    // (0x30 = 48 bytes) and the tRNS chunk (0x74, 0x52, 0x4e, 0x53, etc) has 1
    // entry (0x01 = 1 byte) that sets the first palette entry's alpha to zero.
    //
    // Both decode and decode_config should pick up that the first palette
    // entry's alpha is zero.
    let src = &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x04, 0x03, 0x00, 0x00, 0x00, 0x81,
        0x54, 0x67, 0xc7, 0x00, 0x00, 0x00, 0x30, 0x50, 0x4c, 0x54, 0x45, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0x0e, 0x00, 0x23, 0x27, 0x7b, 0xb1, 0x2d, 0x0a, 0x49, 0x3f, 0x19, 0x78, 0x5f,
        0xcd, 0xe4, 0x69, 0x69, 0xe4, 0x71, 0x59, 0x53, 0x80, 0x11, 0x14, 0x8b, 0x00, 0xa9, 0x8d,
        0x95, 0xcb, 0x99, 0x2f, 0x6b, 0xd7, 0x29, 0x91, 0xd7, 0x7b, 0xba, 0xff, 0xe3, 0xd7, 0x13,
        0xc6, 0xd3, 0x58, 0x00, 0x00, 0x00, 0x01, 0x74, 0x52, 0x4e, 0x53, 0x00, 0x40, 0xe6, 0xd8,
        0x66, 0x00, 0x00, 0x00, 0xfd, 0x49, 0x44, 0x41, 0x54, 0x28, 0xcf, 0x63, 0x60, 0x00, 0x83,
        0x55, 0x0c, 0x68, 0x60, 0x9d, 0x02, 0x9a, 0x80, 0xde, 0x23, 0x74, 0x15, 0xef, 0x50, 0x94,
        0x70, 0x2d, 0xd2, 0x7b, 0x87, 0xa2, 0x84, 0xeb, 0xee, 0xbb, 0x77, 0x6f, 0x51, 0x94, 0xe8,
        0xbd, 0x7d, 0xf7, 0xee, 0x12, 0xb2, 0x80, 0xd2, 0x3d, 0x54, 0x01, 0x26, 0x10, 0x1f, 0x59,
        0x40, 0x0f, 0xc8, 0xd7, 0x7e, 0x84, 0x70, 0x1c, 0xd7, 0xba, 0xb7, 0x4a, 0xda, 0xda, 0x77,
        0x11, 0xf6, 0xac, 0x5a, 0xa5, 0xf4, 0xf9, 0xbf, 0xfd, 0x3d, 0x24, 0x6b, 0x98, 0x94, 0xf4,
        0xff, 0x7f, 0x52, 0x42, 0x16, 0x30, 0x0e, 0xd9, 0xed, 0x6a, 0x8c, 0xec, 0x10, 0x65, 0x53,
        0x97, 0x60, 0x23, 0x64, 0x1d, 0x8a, 0x2e, 0xc6, 0x2e, 0x42, 0x08, 0x3d, 0x4c, 0xca, 0x81,
        0xc1, 0x82, 0xa6, 0xa2, 0x46, 0x08, 0x3d, 0x4a, 0xa1, 0x82, 0xc6, 0x82, 0xa1, 0x4a, 0x08,
        0x3d, 0xfa, 0xa6, 0x81, 0xa1, 0xa2, 0xc1, 0x9f, 0x10, 0x66, 0xd4, 0x2b, 0x87, 0x0a, 0x86,
        0x1a, 0x7d, 0x57, 0x80, 0x9b, 0x99, 0xaf, 0x62, 0x1a, 0x1a, 0xec, 0xf0, 0x0d, 0x66, 0x2a,
        0x7b, 0x5a, 0xba, 0xd2, 0x64, 0x63, 0x4b, 0xa6, 0xb2, 0xb4, 0x02, 0xa8, 0x12, 0xb5, 0x24,
        0xa5, 0x99, 0x2e, 0x33, 0x95, 0xd4, 0x92, 0x10, 0xee, 0xd0, 0x59, 0xb9, 0x6a, 0xd6, 0x21,
        0x24, 0xb7, 0x33, 0x9d, 0x01, 0x01, 0x64, 0xbf, 0xac, 0x59, 0xb2, 0xca, 0xeb, 0x14, 0x92,
        0x80, 0xd6, 0x9a, 0x53, 0x4a, 0x6b, 0x4e, 0x2d, 0x42, 0x52, 0xa1, 0x73, 0x28, 0x54, 0xe7,
        0x90, 0x6a, 0x00, 0x92, 0x92, 0x45, 0xa1, 0x40, 0x84, 0x2c, 0xe0, 0xc4, 0xa0, 0xb2, 0x28,
        0x14, 0xc1, 0x67, 0xe9, 0x50, 0x60, 0x60, 0xea, 0x70, 0x40, 0x12, 0x00, 0x79, 0x54, 0x09,
        0x22, 0x00, 0x00, 0x30, 0xf3, 0x52, 0x87, 0xc6, 0xe4, 0xbd, 0x70, 0x00, 0x00, 0x00, 0x00,
        0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ];

    let cfg = decode_config(&mut bytes::Reader::new(src)).unwrap();
    match cfg.color_model {
        image::color::Model::Paletted(pal) => {
            let (_, _, _, alpha) = pal.colors[0].rgba();
            assert_eq!(alpha, 0, "decode_config: got {}, want 0", alpha);
        }
        _ => assert!(false, "expect Paletted model"),
    }

    let img = decode(&mut bytes::Reader::new(src)).unwrap();
    match img.color_model() {
        image::color::Model::Paletted(pal) => {
            let (_, _, _, alpha) = pal.colors[0].rgba();
            assert_eq!(alpha, 0, "decode: got {}, want 0", alpha);
        }
        _ => assert!(false, "expect Paletted model"),
    }
}

// fn benchmarkDecode(b *testing.b, filename string, bytesPerPixel int) {
// 	data, err := os.ReadFile(filename)
// 	if err != nil {
// 		b.Fatal(err)
// 	}
// 	cfg, err := decode_config(bytes::Reader::new(data))
// 	if err != nil {
// 		b.Fatal(err)
// 	}
// 	b.SetBytes(int64(cfg.Width * cfg.Height * bytesPerPixel))
// 	b.ReportAllocs()
// 	b.ResetTimer()
// 	for i := 0; i < b.N; i++ {
// 		decode(&mut bytes::Reader::new(data))
// 	}
// }

// fn BenchmarkDecodeGray(b *testing.b) {
// 	benchmarkDecode(b, "src/image/png/testdata/benchGray.png", 1)
// }

// fn BenchmarkDecodeNRGBAGradient(b *testing.b) {
// 	benchmarkDecode(b, "src/image/png/testdata/benchNRGBA-gradient.png", 4)
// }

// fn BenchmarkDecodeNRGBAOpaque(b *testing.b) {
// 	benchmarkDecode(b, "src/image/png/testdata/benchNRGBA-opaque.png", 4)
// }

// fn BenchmarkDecodePaletted(b *testing.b) {
// 	benchmarkDecode(b, "src/image/png/testdata/benchPaletted.png", 1)
// }

// fn BenchmarkDecodeRGB(b *testing.b) {
// 	benchmarkDecode(b, "src/image/png/testdata/benchRGB.png", 4)
// }

// fn BenchmarkDecodeInterlacing(b *testing.b) {
// 	benchmarkDecode(b, "src/image/png/testdata/benchRGB-interlace.png", 4)
// }
