use ggstd::bytes::Buffer;

use std::io::Write;

fn main() {
    let mut buf = Buffer::new();
    buf.write("hello world".as_bytes()).unwrap();
    println!("{}", buf.string());

    let mut buf = Buffer::new();
    buf.write_byte(33).unwrap();
    buf.write("hello world".as_bytes()).unwrap();
    println!("{}", buf.read_byte().unwrap());
    println!("{}", buf.string());
}
