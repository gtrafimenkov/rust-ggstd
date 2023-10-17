// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::image::{self, color, png, Image};
use ggstd::os;

fn main() {
    example_encode();
}

fn example_encode() {
    let (width, height) = (256, 256);

    // Create a colored image of the given width and height.
    let mut img = image::Img::new_nrgba(&image::rect(0, 0, width, height));

    for y in 0..height {
        for x in 0..width {
            let c = color::Color::new_rgba(
                ((x + y) & 255) as u8,
                (((x + y) << 1) & 255) as u8,
                (((x + y) << 2) & 255) as u8,
                255,
            );
            img.set(x, y, &c);
        }
    }

    let mut f = os::create("image.png").unwrap();
    png::encode(&mut f, &img).unwrap();
    f.sync_all().unwrap();

    // decoding just the header
    {
        let mut f = std::fs::File::open("image.png").unwrap();
        let mut br = std::io::BufReader::new(&mut f);
        let config = png::decode_config(&mut br).unwrap();
        println!("size:        {}x{}", config.width, config.height);
        println!("color model: {:?}", config.color_model);
    }

    // decoding everything
    {
        let mut f = std::fs::File::open("image.png").unwrap();
        let mut br = std::io::BufReader::new(&mut f);
        let img = png::decode(&mut br).unwrap();
        let width = img.bounds().dx();
        let height = img.bounds().dy();
        println!("size:        {}x{}", width, height);
        println!("color model: {:?}", img.color_model());
        for y in 0..height {
            for x in 0..width {
                let c = img.at(x as isize, y as isize);
                let expect = color::Color::new_rgba(
                    ((x + y) & 255) as u8,
                    (((x + y) << 1) & 255) as u8,
                    (((x + y) << 2) & 255) as u8,
                    255,
                );
                if c != expect {
                    println!(
                        "unexpected color at ({}, {}): expecting {:?}, got {:?}",
                        x, y, expect, c
                    );
                }
            }
        }
    }
}
