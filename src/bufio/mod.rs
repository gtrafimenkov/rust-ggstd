mod bufio;
mod scan;

pub use bufio::{new_reader, new_writer, new_writer_size, Reader, Writer};
pub use scan::{scan_bytes, scan_lines, Error, Scanner};

#[cfg(test)]
mod bufio_test;
#[cfg(test)]
mod scan_test;
