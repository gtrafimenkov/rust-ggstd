mod bufio;

pub use bufio::{new_reader, new_writer, new_writer_size, Reader, Writer};

#[cfg(test)]
mod bufio_test;
