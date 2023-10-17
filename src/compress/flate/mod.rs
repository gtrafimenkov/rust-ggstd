mod deflate;
mod deflatefast;
mod dict_decoder;
mod huffman_bit_writer;
mod huffman_code;
mod inflate;
mod token;

pub use deflate::{
    Writer, BEST_COMPRESSION, BEST_SPEED, DEFAULT_COMPRESSION, HUFFMAN_ONLY, NO_COMPRESSION,
};
pub use inflate::Reader;

#[cfg(test)]
mod deflate_test;
#[cfg(test)]
mod dict_decoder_test;
#[cfg(test)]
mod flate_test;
#[cfg(test)]
mod huffman_bit_writer_test;
#[cfg(test)]
mod inflate_test;
#[cfg(test)]
mod reader_test;
#[cfg(test)]
mod writer_test;
