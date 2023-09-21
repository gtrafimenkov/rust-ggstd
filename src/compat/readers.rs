use crate::io as ggio;
use std::io::BufRead;

impl<R: std::io::Read> ggio::ByteReader for std::io::BufReader<R> {
    fn read_byte(&mut self) -> std::io::Result<u8> {
        let buf = match self.fill_buf() {
            Ok(buf) => buf,
            Err(err) => return Err(err),
        };
        if buf.is_empty() {
            return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
        }
        let b = buf[0];
        self.consume(1);
        Ok(b)
    }
}
