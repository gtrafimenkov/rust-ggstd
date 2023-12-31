// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::bytes;
use crate::compress::zlib;
use std::io::Read;

struct ZlibTest {
    _desc: &'static str,
    raw: &'static str,
    compressed: &'static [u8],
    dict: Option<&'static [u8]>,
    expected_err: Option<zlib::Error>,
}

// Compare-to-golden test data was generated by the ZLIB example program at
// https://www.zlib.net/zpipe.c

fn get_tests() -> [ZlibTest; 14] {
    [
        ZlibTest {
            _desc: "truncated empty",
            raw: "",
            compressed: &[],
            dict: None,
            expected_err: Some(zlib::Error::from(std::io::ErrorKind::UnexpectedEof)),
        },
        ZlibTest {
            _desc: "truncated dict",
            raw: "",
            compressed: &[0x78, 0xbb],
            dict: Some(&[0x00]),
            expected_err: Some(zlib::Error::from(std::io::ErrorKind::UnexpectedEof)),
        },
        ZlibTest {
            _desc: "truncated checksum",
            raw: "",
            compressed: &[
                0x78, 0xbb, 0x00, 0x01, 0x00, 0x01, 0xca, 0x48, 0xcd, 0xc9, 0xc9, 0xd7, 0x51, 0x28,
                0xcf, 0x2f, 0xca, 0x49, 0x01, 0x04, 0x00, 0x00, 0xff, 0xff,
            ],
            dict: Some(&[0x00]),
            expected_err: Some(zlib::Error::from(std::io::ErrorKind::UnexpectedEof)),
        },
        ZlibTest {
            _desc: "empty",
            raw: "",
            compressed: &[0x78, 0x9c, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01],
            dict: None,
            expected_err: None,
        },
        ZlibTest {
            _desc: "goodbye",
            raw: "goodbye, world",
            compressed: &[
                0x78, 0x9c, 0x4b, 0xcf, 0xcf, 0x4f, 0x49, 0xaa, 0x4c, 0xd5, 0x51, 0x28, 0xcf, 0x2f,
                0xca, 0x49, 0x01, 0x00, 0x28, 0xa5, 0x05, 0x5e,
            ],
            dict: None,
            expected_err: None,
        },
        ZlibTest {
            _desc: "bad header (CINFO)",
            raw: "",
            compressed: &[0x88, 0x98, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01],
            dict: None,
            expected_err: Some(zlib::Error::ErrHeader),
        },
        ZlibTest {
            _desc: "bad header (FCHECK)",
            raw: "",
            compressed: &[0x78, 0x9f, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01],
            dict: None,
            expected_err: Some(zlib::Error::ErrHeader),
        },
        ZlibTest {
            _desc: "bad checksum",
            raw: "",
            compressed: &[0x78, 0x9c, 0x03, 0x00, 0x00, 0x00, 0x00, 0xff],
            dict: None,
            // expected_err: Some(zlib::Error::ErrChecksum),
            expected_err: Some(zlib::Error::StdIo(std::io::Error::new(
                std::io::ErrorKind::Other,
                "zlib: invalid checksum",
            ))),
        },
        ZlibTest {
            _desc: "not enough data",
            raw: "",
            compressed: &[0x78, 0x9c, 0x03, 0x00, 0x00, 0x00],
            dict: None,
            expected_err: Some(zlib::Error::from(std::io::ErrorKind::UnexpectedEof)),
        },
        ZlibTest {
            _desc: "excess data is silently ignored",
            raw: "",
            compressed: &[
                0x78, 0x9c, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01, 0x78, 0x9c, 0xff,
            ],
            dict: None,
            expected_err: None,
        },
        ZlibTest {
            _desc: "dictionary",
            raw: "Hello, World!\n",
            compressed: &[
                0x78, 0xbb, 0x1c, 0x32, 0x04, 0x27, 0xf3, 0x00, 0xb1, 0x75, 0x20, 0x1c, 0x45, 0x2e,
                0x00, 0x24, 0x12, 0x04, 0x74,
            ],
            dict: Some(&[
                0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x0a,
            ]),
            expected_err: None,
        },
        ZlibTest {
            _desc: "wrong dictionary",
            raw: "",
            compressed: &[
                0x78, 0xbb, 0x1c, 0x32, 0x04, 0x27, 0xf3, 0x00, 0xb1, 0x75, 0x20, 0x1c, 0x45, 0x2e,
                0x00, 0x24, 0x12, 0x04, 0x74,
            ],
            dict: Some(&[0x48, 0x65, 0x6c, 0x6c]),
            expected_err: Some(zlib::Error::ErrDictionary),
        },
        ZlibTest {
            _desc: "truncated zlib stream amid raw-block",
            raw: "hello",
            compressed: &[
                0x78, 0x9c, 0x00, 0x0c, 0x00, 0xf3, 0xff, 0x68, 0x65, 0x6c, 0x6c, 0x6f,
            ],
            dict: None,
            expected_err: Some(zlib::Error::from(std::io::ErrorKind::UnexpectedEof)),
        },
        ZlibTest {
            _desc: "truncated zlib stream amid fixed-block",
            raw: "He",
            compressed: &[0x78, 0x9c, 0xf2, 0x48, 0xcd],
            dict: None,
            expected_err: Some(zlib::Error::from(std::io::ErrorKind::UnexpectedEof)),
        },
    ]
}

#[test]
fn test_decompressor() {
    fn compare_errors_zlib(got: &zlib::Error, want: &Option<zlib::Error>) {
        if want.is_none() {
            panic!("unexpected error {:?}", got);
        }
        assert_eq!(
            got,
            want.as_ref().unwrap(),
            "expecting {:?}, got {:?}",
            want.as_ref().unwrap(),
            got
        );
    }
    fn compare_errors(got: &std::io::Error, want: &Option<zlib::Error>) {
        if want.is_none() {
            panic!("unexpected error {:?}", got);
        }
        if let zlib::Error::StdIo(want) = want.as_ref().unwrap() {
            assert_eq!(got.kind(), want.kind());
            return;
        }
        panic!("expecting {:?}, got {:?}", want.as_ref().unwrap(), got);
    }
    for tt in get_tests() {
        let mut input = bytes::Reader::new(tt.compressed);
        let zr = if tt.dict.is_none() {
            zlib::Reader::new(&mut input)
        } else {
            zlib::Reader::new_dict(&mut input, tt.dict.unwrap())
        };
        let mut zr = match zr {
            Ok(zr) => zr,
            Err(err) => {
                compare_errors_zlib(&err, &tt.expected_err);
                continue;
            }
        };

        // Read and verify correctness of data.
        let mut decoded = String::new();
        let res = zr.read_to_string(&mut decoded);
        match res {
            Ok(_) => {
                assert_eq!(decoded, tt.raw);
            }
            Err(err) => {
                compare_errors(&err, &tt.expected_err);
            }
        }

        let res = zr.close();
        assert!(res.is_ok());
    }
}
