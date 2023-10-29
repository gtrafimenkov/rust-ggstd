// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! Package binary implements simple translation between numbers and byte
//! sequences and encoding and decoding of varints.
//!
//! Numbers are translated by reading and writing fixed-size values.
//! A fixed-size value is either a fixed-size arithmetic
//! type (bool, int8, uint8, int16, float32, complex64, ...)
//! or an array or struct containing only fixed-size values.
//!
//! The varint functions encode and decode single integer values using
//! a variable-length encoding; smaller values require fewer bytes.
//! For a specification, see
//! <https://developers.google.com/protocol-buffers/docs/encoding>.
//!
//! This package favors simplicity over efficiency. Clients that require
//! high-performance serialization, especially for large data structures,
//! should look at more advanced solutions such as the encoding/gob
//! package or protocol buffers.

mod binary;

pub use binary::{BigEndian, ByteOrder, LittleEndian, BIG_ENDIAN, LITTLE_ENDIAN};

#[cfg(test)]
mod binary_test;
