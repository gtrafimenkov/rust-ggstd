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
    assert!(reader.read_byte().err().unwrap().kind() == std::io::ErrorKind::UnexpectedEof);
}
