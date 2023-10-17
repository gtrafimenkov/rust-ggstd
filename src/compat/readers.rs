use crate::io as ggio;

impl<R: std::io::Read> ggio::ByteReader for std::io::BufReader<R> {
    fn read_byte(&mut self) -> std::io::Result<u8> {
        read_byte(self)
    }
}

/// read_byte reads and returns the next byte from the input or
/// any error encountered. If read_byte returns an error, no input
/// byte was consumed.
///
/// This function can be used to implement crate::io::ByteReader trait
/// for structures that implement std::io::BufRead interface.
pub fn read_byte<T: std::io::BufRead + ?Sized>(r: &mut T) -> std::io::Result<u8> {
    let buf = match r.fill_buf() {
        Ok(buf) => buf,
        Err(err) => return Err(err),
    };
    if buf.is_empty() {
        return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
    }
    let b = buf[0];
    r.consume(1);
    Ok(b)
}
