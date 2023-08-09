use ggstd::bytes;
use ggstd::compress::flate;
use ggstd::encoding::hex;
use std::io::Write;

fn main() {
    let msg = "Hello World! Hello Rust! Hello World! Hello Rust!";
    let mut buffer = bytes::Buffer::new();
    let mut w = flate::Writer::new(&mut buffer, flate::BEST_COMPRESSION).unwrap();
    w.write(msg.as_bytes()).unwrap();
    w.close().unwrap();

    let result_str = hex::encode_to_string(buffer.bytes());
    println!(
        "input string    ({:02} bytes): {}",
        msg.as_bytes().len(),
        msg,
    );
    println!(
        "deflated string ({:02} bytes): {}",
        result_str.len() / 2,
        result_str
    );
}

// The output can be decompressed using Python:
// python3 -c 'import zlib; print(zlib.decompress(bytes.fromhex("f248cdc9c95708cf2fca4951548070824a8b4b606c4c0940000000ffff"), -15))'
