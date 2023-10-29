// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let path = "src/testdata/hello.txt.gz";
    println!("{}", extract_with_ggstd(path).unwrap());
    // println!("{}", extract_with_flate2(path).unwrap());
}

fn extract_with_ggstd(path: &str) -> std::io::Result<String> {
    use ggstd::compress::gzip;

    let mut f = BufReader::new(File::open(path)?);
    let mut gzip = gzip::Reader::new(&mut f)?;
    let mut content = String::new();
    gzip.read_to_string(&mut content)?;
    Ok(content)
}

// fn extract_with_flate2(path: &str) -> std::io::Result<String> {
//     use flate2::read::GzDecoder;

//     let f = BufReader::new(File::open(path)?);
//     let mut gzip = GzDecoder::new(f);
//     let mut content = String::new();
//     gzip.read_to_string(&mut content)?;
//     Ok(content)
// }
