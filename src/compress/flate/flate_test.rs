// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! This test tests some internals of the flate package.
//! The tests in package compress/gzip serve as the
//! end-to-end test of the decompressor.

use super::inflate::{DecoderToUse, HuffmanDecoder, Reader};
use crate::bytes;
use crate::encoding::hex;
use crate::errors;
use crate::io as ggio;

/// The following test should not panic.
#[test]
fn test_issue5915() {
    let bits = [
        4, 0, 0, 6, 4, 3, 2, 3, 3, 4, 4, 5, 0, 0, 0, 0, 5, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 6, 0, 11, 0, 8, 0, 6,
        6, 10, 8,
    ];
    let mut h = HuffmanDecoder::new();
    assert!(
        !h.init(&bits),
        "Given sequence of bits is bad, and should not succeed."
    );
}

/// The following test should not panic.
#[test]
fn test_issue5962() {
    let bits = [
        4, 0, 0, 6, 4, 3, 2, 3, 3, 4, 4, 5, 0, 0, 0, 0, 5, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11,
    ];
    let mut h = HuffmanDecoder::new();
    assert!(
        !h.init(&bits),
        "Given sequence of bits is bad, and should not succeed."
    );
}

/// The following test should not panic.
#[test]
fn test_issue6255() {
    let bits1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11];
    let bits2 = [11, 13];
    let mut h = HuffmanDecoder::new();
    assert!(
        h.init(&bits1),
        "Given sequence of bits is good and should succeed."
    );
    assert!(
        !h.init(&bits2),
        "Given sequence of bits is bad, and should not succeed."
    );
}

#[test]
fn test_invalid_encoding() {
    // Initialize Huffman decoder to recognize "0".
    let mut h = HuffmanDecoder::new();
    assert!(h.init(&[1]), "Failed to initialize Huffman decoder");

    // Initialize decompressor with invalid Huffman coding.
    let mut r = bytes::Reader::new(&[0xff]);
    let mut f = Reader::new(&mut r);
    f.h1 = h;

    assert!(
        f.huff_sym(DecoderToUse::H1).is_err(),
        "Should have rejected invalid bit sequence"
    );
}

#[test]
fn test_invalid_bits() {
    let oversubscribed = &[1, 2, 3, 4, 4, 5];
    let incomplete = &[1, 2, 4, 4];
    let mut h = HuffmanDecoder::new();
    assert!(
        !h.init(oversubscribed),
        "Should reject oversubscribed bit-length set"
    );
    assert!(
        !h.init(incomplete),
        "Should reject incomplete bit-length set"
    );
}

#[test]
fn test_streams() {
    // To verify any of these hexstrings as valid or invalid flate streams
    // according to the C zlib library, you can use the Python wrapper library:
    // >>> hex_string = "010100feff11"
    // >>> import zlib
    // >>> zlib.decompress(hex_string.decode("hex"), -15) # Negative means raw DEFLATE
    // '\x11'

    struct TestCase {
        desc: &'static str,   // Description of the stream
        stream: &'static str, // Hexstring of the input DEFLATE stream
        want: &'static str,   // Expected result. Use "fail" to expect failure
    }

    let test_cases = &[
        TestCase {
            desc: "degenerate HCLenTree",
            stream: "05e0010000000000100000000000000000000000000000000000000000000000\
                00000000000000000004",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, empty HLitTree, empty HDistTree",
            stream: "05e0010400000000000000000000000000000000000000000000000000000000\
                00000000000000000010",
            want: "fail",
        },
        TestCase {
            desc: "empty HCLenTree",
            stream: "05e0010000000000000000000000000000000000000000000000000000000000\
                00000000000000000010",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, empty HDistTree, use missing HDist symbol",
            stream: "000100feff000de0010400000000100000000000000000000000000000000000\
                0000000000000000000000000000002c",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, degenerate HDistTree, use missing HDist symbol",
            stream: "000100feff000de0010000000000000000000000000000000000000000000000\
                00000000000000000610000000004070",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, empty HLitTree, empty HDistTree",
            stream: "05e0010400000000100400000000000000000000000000000000000000000000\
                0000000000000000000000000008",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, empty HLitTree, degenerate HDistTree",
            stream: "05e0010400000000100400000000000000000000000000000000000000000000\
                0000000000000000000800000008",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, degenerate HLitTree, degenerate HDistTree, use missing HLit symbol",
            stream: "05e0010400000000100000000000000000000000000000000000000000000000\
                0000000000000000001c",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, too large HDistTree",
            stream: "edff870500000000200400000000000000000000000000000000000000000000\
                000000000000000000080000000000000004",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, empty HDistTree, excessive repeater code",
            stream: "edfd870500000000200400000000000000000000000000000000000000000000\
                000000000000000000e8b100",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, empty HDistTree of normal length 30",
            stream: "05fd01240000000000f8ffffffffffffffffffffffffffffffffffffffffffff\
                ffffffffffffffffff07000000fe01",
            want: 	"",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, empty HDistTree of excessive length 31",
            stream: "05fe01240000000000f8ffffffffffffffffffffffffffffffffffffffffffff\
                ffffffffffffffffff07000000fc03",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, over-subscribed HLitTree, empty HDistTree",
            stream: "05e001240000000000fcffffffffffffffffffffffffffffffffffffffffffff\
                ffffffffffffffffff07f00f",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, under-subscribed HLitTree, empty HDistTree",
            stream: "05e001240000000000fcffffffffffffffffffffffffffffffffffffffffffff\
                fffffffffcffffffff07f00f",
            want: "fail",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree with single code, empty HDistTree",
            stream: "05e001240000000000f8ffffffffffffffffffffffffffffffffffffffffffff\
                ffffffffffffffffff07f00f",
            want: "01",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree with multiple codes, empty HDistTree",
            stream: "05e301240000000000f8ffffffffffffffffffffffffffffffffffffffffffff\
                ffffffffffffffffff07807f",
            want: "01",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, degenerate HDistTree, use valid HDist symbol",
            stream: "000100feff000de0010400000000100000000000000000000000000000000000\
                0000000000000000000000000000003c",
            want: "00000000",
        },
        TestCase {
            desc: "complete HCLenTree, degenerate HLitTree, degenerate HDistTree",
            stream: "05e0010400000000100000000000000000000000000000000000000000000000\
                0000000000000000000c",
            want: "",
        },
        TestCase {
            desc: "complete HCLenTree, degenerate HLitTree, empty HDistTree",
            stream: "05e0010400000000100000000000000000000000000000000000000000000000\
                00000000000000000004",
            want: "",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, empty HDistTree, spanning repeater code",
            stream: "edfd870500000000200400000000000000000000000000000000000000000000\
                000000000000000000e8b000",
            want: "",
        },
        TestCase {
            desc: "complete HCLenTree with length codes, complete HLitTree, empty HDistTree",
            stream: "ede0010400000000100000000000000000000000000000000000000000000000\
                0000000000000000000400004000",
            want: "",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, degenerate HDistTree, use valid HLit symbol 284 with count 31",
            stream: "000100feff00ede0010400000000100000000000000000000000000000000000\
                000000000000000000000000000000040000407f00",
            want: "0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                0000000000000000000000000000000000000000000000000000000000000000\
                000000",
        },
        TestCase {
            desc: "complete HCLenTree, complete HLitTree, degenerate HDistTree, use valid HLit and HDist symbols",
            stream: "0cc2010d00000082b0ac4aff0eb07d27060000ffff",
            want:"616263616263",
        },
        TestCase {
            desc: "fixed block, use reserved symbol 287",
            stream: "33180700",
            want:"fail",
        },
        TestCase {
            desc: "raw block",
            stream: "010100feff11",
            want:"11",
        },
        TestCase {
            desc: "issue 10426 - over-subscribed HCLenTree causes a hang",
            stream: "344c4a4e494d4b070000ff2e2eff2e2e2e2e2eff",
            want:"fail",
        },
        TestCase {
            desc: "issue 11030 - empty HDistTree unexpectedly leads to error",
            stream: "05c0070600000080400fff37a0ca",
            want: "",
        },
        TestCase {
            desc: "issue 11033 - empty HDistTree unexpectedly leads to error",
            stream: "050fb109c020cca5d017dcbca044881ee1034ec149c8980bbc413c2ab35be9dc\
                b1473449922449922411202306ee97b0383a521b4ffdcf3217f9f7d3adb701",
            want: "3130303634342068652e706870005d05355f7ed957ff084a90925d19e3ebc6d0\
                c6d7",
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        let (data, err) = hex::decode_string(tc.stream);
        assert!(err.is_none());
        let (data, err) = ggio::read_all(&mut Reader::new(&mut bytes::Reader::new(&data)));
        if tc.want == "fail" {
            assert!(
                err.is_some(),
                "#{} ({}): got nil error, want non-nil",
                i,
                tc.desc
            );
        } else {
            assert!(err.is_none(), "#{} ({}): {}", i, tc.desc, err.unwrap());
            let got = hex::encode_to_string(&data);
            assert_eq!(
                got, tc.want,
                "#{} ({}): got {:?}, want {:?}",
                i, tc.desc, got, tc.want
            );
        }
    }
}

#[test]
fn test_truncated_streams() {
    let data = b"\x00\x0c\x00\xf3\xffhello, world\x01\x00\x00\xff\xff";

    // all data should be decompressed correctly
    {
        let mut reader = bytes::Reader::new(data.as_slice());
        let mut r = Reader::new(&mut reader);
        std::io::copy(&mut r, &mut ggio::Discard::new()).unwrap();
    }

    // truncated streams shoud report error
    for i in 0..data.len() - 1 {
        let mut reader = bytes::Reader::new(&data[..i]);
        let mut r = Reader::new(&mut reader);
        let (_n, err) = ggio::copy(&mut ggio::Discard::new(), &mut r);
        assert!(
            err.is_some() && err.as_ref().unwrap().kind() == std::io::ErrorKind::UnexpectedEof,
            "ggio::copy({}) on truncated stream: got {}, want {}",
            i,
            err.unwrap(),
            errors::new_unexpected_eof()
        );
    }
}

// This test doesn't work after switching to useage of std::io::Read instead of ggio::Reader
// /// Verify that flate.Reader.Read returns (n, io.EOF) instead
// /// of (n, nil) + (0, io.EOF) when possible.
// ///
// /// This helps net/http.Transport reuse HTTP/1 connections more
// /// aggressively.
// ///
// /// See https://github.com/google/go-github/pull/317 for background.
// #[test]
// fn test_reader_early_eof() {
//     // 	t.Parallel()
//     let test_sizes = [
//         1,
//         2,
//         3,
//         4,
//         5,
//         6,
//         7,
//         8,
//         100,
//         1000,
//         10000,
//         100000,
//         128,
//         1024,
//         16384,
//         131072,
//         // Testing multiples of WINDOW_SIZE triggers the case
//         // where Read will fail to return an early io.EOF.
//         deflate::WINDOW_SIZE * 1,
//         deflate::WINDOW_SIZE * 2,
//         deflate::WINDOW_SIZE * 3,
//     ];

//     let max_size = test_sizes.iter().max().unwrap();

//     let mut read_buf = vec![0; 40];
//     let mut data = vec![0; *max_size];
//     for i in 0..data.len() {
//         data[i] = i as u8;
//     }

//     for sz in test_sizes {
//         // 		if testing.Short() && sz > WINDOW_SIZE {
//         // 			continue
//         // 		}
//         for flush in [true, false] {
//             let mut early_eof = true; // Do we expect early io.EOF?

//             let mut buf = bytes::Buffer::new();
//             {
//                 let mut w = deflate::Writer::new(&mut buf, 5).unwrap();
//                 let n = w.write(&data[..sz]).unwrap();
//                 assert_eq!(sz, n);
//                 if flush {
//                     // If a Flush occurs after all the actual data, the flushing
//                     // semantics dictate that we will observe a (0, io.EOF) since
//                     // Read must return data before it knows that the stream ended.
//                     w.flush().unwrap();
//                     early_eof = false;
//                 }
//                 w.close().unwrap();
//             }
//             let mut r = Decompressor::new(&mut buf);
//             loop {
//                 let (n, err) = r.read(&mut read_buf);
//                 if err.as_ref().is_some_and(|err| err.is_eof()) {
//                     // If the availWrite == WINDOW_SIZE, then that means that the
//                     // previous Read returned because the write buffer was full
//                     // and it just so happened that the stream had no more data.
//                     // This situation is rare, but unavoidable.
//                     if r.td.dict.avail_write() == deflate::WINDOW_SIZE {
//                         early_eof = false;
//                     }
//                     assert!(
//                         n != 0 || !early_eof,
//                         "On size:{} flush:{}, Read() = (0, io.EOF), want (n, io.EOF)",
//                         sz,
//                         flush
//                     );
//                     assert!(
//                         n == 0 || early_eof,
//                         "On size:{} flush:{}, Read() = ({}, io.EOF), want (0, io.EOF)",
//                         sz,
//                         flush,
//                         n
//                     );
//                     break;
//                 }
//                 assert!(err.is_none());
//             }
//         }
//     }
// }
