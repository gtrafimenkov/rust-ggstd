use std::io::Read;

use ggstd::bytes;
use ggstd::compress::zlib;

fn main() {
    let buff: &[u8] = &[
        120, 156, 202, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 225, 2, 4, 0, 0, 255, 255,
        33, 231, 4, 147,
    ];
    let mut b = bytes::Reader::new(buff);
    let mut r = zlib::Reader::new(&mut b).unwrap();

    // reading using Reader::read
    // loop {
    //     let mut buf = [0; 10];
    //     let res = r.read(&mut buf);
    //     if res.is_err() {
    //         break;
    //     }
    //     let n = res.unwrap();
    //     if n == 0 {
    //         break;
    //     }
    // }

    // reading using std::io::Read::read_to_string
    let mut output = String::new();
    r.read_to_string(&mut output).unwrap();
    println!("{}", output);

    r.close().unwrap();
}
