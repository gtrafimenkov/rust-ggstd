use crate::io::ByteReader;
use std::io::BufReader;

#[test]
fn test_bufreader_read_byte() {
    let data = b"hello";
    let mut reader = BufReader::new(data.as_slice());
    assert_eq!(b'h', reader.read_byte().unwrap());
    assert_eq!(b'e', reader.read_byte().unwrap());
    assert_eq!(b'l', reader.read_byte().unwrap());
    assert_eq!(b'l', reader.read_byte().unwrap());
    assert_eq!(b'o', reader.read_byte().unwrap());
    assert!(reader.read_byte().err().unwrap().is_eof());
}

// #[test]
// fn test_ggreader_adapter() {
//     const TEST_VALUE: &str = "hello world!";
//     let mut r = bytes::new_buffer_string(TEST_VALUE);
//     let mut ra = readers::GGReaderAdapter::new(&mut r);
//     let mut buf = Vec::new();
//     assert_eq!(12, ra.read_to_end(&mut buf).unwrap());
//     assert_eq!(TEST_VALUE.as_bytes(), &buf);
// }
