//! compat module contains functions that ease porting of Go code to Rust.
//! These functions are intended for internal usage only.

/// Copy data from one buffer to another similar to what Go copy function does.
pub fn copy(dest: &mut [u8], src: &[u8]) -> usize {
    let dest_size = dest.len();
    let src_size = src.len();
    if dest_size == src_size {
        dest.copy_from_slice(src);
        return src_size;
    } else {
        let size = src_size.min(dest_size);
        dest[..size].copy_from_slice(&src[..size]);
        return size;
    }
}

/// Copy data from one place of the buffer to another similar to what Go copy function can do.
// pub fn copy_within<R: std::ops::RangeBounds<usize>>(buf: &mut [u8], src: R, dest: usize) -> usize {
pub fn copy_within(buf: &mut [u8], src: std::ops::Range<usize>, dest: usize) -> usize {
    let buf_len = buf.len();
    let dest_size = buf_len - dest;
    let src_size = src.end - src.start;
    if src_size > dest_size {
        buf.copy_within(src.start..src.start + dest_size, dest);
        return dest_size;
    } else {
        buf.copy_within(src, dest);
        return src_size;
    }
}

/// It is not possible to copy std::io::Error directly because it doesn't implement
/// Copy trait.  This function creates a new std::io::Error with the same
/// error kind and a message copied from the original error.
pub fn copy_stdio_error(e: &std::io::Error) -> std::io::Error {
    std::io::Error::new(e.kind(), e.to_string())
}

pub mod readers;

#[cfg(test)]
mod readers_test;
