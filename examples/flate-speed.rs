use ggstd::bytes;
use ggstd::compress::flate;
use ggstd::crypto::sha256;
use ggstd::encoding::hex;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut file = File::open("src/testdata/Isaac.Newton-Opticks.txt").unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();

    let mut buffer = bytes::Buffer::new();
    let mut w = flate::Writer::new(&mut buffer, flate::BEST_COMPRESSION).unwrap();

    let repeats = 200;
    for _i in 0..repeats {
        w.write(&content).unwrap();
    }

    w.close().unwrap();

    let compressed_data = bytes::Buffer::bytes(&buffer);
    let hash = sha256::sum256(compressed_data);
    let hash_str = hex::encode_to_string(&hash);
    println!("compressed data hash:");
    println!("{}", hash_str);
}
