use ggstd::bytes;
use ggstd::compress::zlib;

fn main() {
    let mut b = bytes::Buffer::new();
    let mut w = zlib::new_writer(&mut b);
    w.write("hello, world\n".as_bytes()).unwrap();
    w.close().unwrap();
    println!("{:?}", b.bytes());
    // Output: [120 156 202 72 205 201 201 215 81 40 207 47 202 73 225 2 4 0 0 255 255 33 231 4 147]
}
