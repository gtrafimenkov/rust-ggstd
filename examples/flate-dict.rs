// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::bytes;
use ggstd::compress::flate;
use ggstd::io as ggio;

fn main() {
    example_dictionary();
}

/// A preset dictionary can be used to improve the compression ratio.
/// The downside to using a dictionary is that the compressor and decompressor
/// must agree in advance what dictionary to use.
fn example_dictionary() {
    // The dictionary is a string of bytes. When compressing some input data,
    // the compressor will attempt to substitute substrings with matches found
    // in the dictionary. As such, the dictionary should only contain substrings
    // that are expected to be found in the actual data stream.
    let dict = b"<?xml version=\"1.0\"?><book><data><meta name=\"\" content=\"";

    // The data to compress should (but is not required to) contain frequent
    // substrings that match those in the dictionary.
    let data = r#"<?xml version="1.0"?>
<book>
    <meta name="title" content="The Go Programming Language"/>
    <meta name="authors" content="Alan Donovan and Brian Kernighan"/>
    <meta name="published" content="2015-10-26"/>
    <meta name="isbn" content="978-0134190440"/>
    <data>...</data>
</book>
"#;

    let mut b = bytes::Buffer::new();

    // Compress the data using the specially crafted dictionary.
    {
        let mut zw = flate::Writer::new_dict(&mut b, flate::DEFAULT_COMPRESSION, dict).unwrap();
        let mut data_reader = bytes::new_buffer_string(data);
        let (_, err) = ggio::copy(&mut zw, &mut data_reader);
        if err.is_some() {
            panic!("unexpected error: {}", err.unwrap());
        }
        zw.close().unwrap();
    }

    // The decompressor must use the same dictionary as the compressor.
    // Otherwise, the input may appear as corrupted.
    println!("Decompressed output using the dictionary:");
    {
        let mut data_reader = b.bytes().clone();
        let mut zr = flate::Decompressor::new_dict(&mut data_reader, dict);
        let mut decompressed = bytes::Buffer::new();
        let (_, err) = ggio::copy(&mut decompressed, &mut zr);
        if err.is_some() {
            panic!("unexpected error: {}", err.unwrap());
        }
        zr.close().unwrap();
        println!("{}", String::from_utf8_lossy(decompressed.bytes()));
        println!();
    }

    // Substitute all of the bytes in the dictionary with a '#' to visually
    // demonstrate the approximate effectiveness of using a preset dictionary.
    println!("Substrings matched by the dictionary are marked with #:");
    {
        let hash_dict = vec![b'#'; dict.len()];
        let mut zr = flate::Decompressor::new_dict(&mut b, &hash_dict);
        let mut decompressed = bytes::Buffer::new();
        let (_, err) = ggio::copy(&mut decompressed, &mut zr);
        if err.is_some() {
            panic!("unexpected error: {}", err.unwrap());
        }
        zr.close().unwrap();
        println!("{}", String::from_utf8_lossy(decompressed.bytes()));
    }

    // Output:
    // Decompressed output using the dictionary:
    // <?xml version="1.0"?>
    // <book>
    // 	<meta name="title" content="The Go Programming Language"/>
    // 	<meta name="authors" content="Alan Donovan and Brian Kernighan"/>
    // 	<meta name="published" content="2015-10-26"/>
    // 	<meta name="isbn" content="978-0134190440"/>
    // 	<data>...</data>
    // </book>
    //
    // Substrings matched by the dictionary are marked with #:
    // #####################
    // ######
    // 	############title###########The Go Programming Language"/#
    // 	############authors###########Alan Donovan and Brian Kernighan"/#
    // 	############published###########2015-10-26"/#
    // 	############isbn###########978-0134190440"/#
    // 	######...</#####
    // </#####
}
