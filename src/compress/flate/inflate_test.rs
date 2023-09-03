// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2014 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::deflate::{Writer, DEFAULT_COMPRESSION};
use super::inflate::{new_reader, new_reader_dict};
use crate::bytes;
use crate::io as ggio;
use std::io::Write;

#[test]
fn test_reset() {
    let ss = &[
        "lorem ipsum izzle fo rizzle",
        "the quick brown fox jumped over",
    ];

    let mut deflated0 = bytes::Buffer::new();
    let mut deflated1 = bytes::Buffer::new();
    {
        let mut w = Writer::new(&mut deflated0, 1).unwrap();
        w.write(ss[0].as_bytes()).unwrap();
        w.close().unwrap();

        let mut w = Writer::new(&mut deflated1, 1).unwrap();
        w.write(ss[1].as_bytes()).unwrap();
        w.close().unwrap();
    }

    let inflated = &mut [bytes::Buffer::new(), bytes::Buffer::new()];

    let mut f = new_reader(&mut deflated0);
    let (n, err) = ggio::copy(&mut inflated[0], &mut f);
    assert!(err.is_none(), "not expecting error '{}'", err.unwrap());
    assert!(n > 0);

    f.reset(&mut deflated1, &[]);
    let (n, err) = ggio::copy(&mut inflated[1], &mut f);
    assert!(err.is_none(), "not expecting error '{}'", err.unwrap());
    assert!(n > 0);
    f.close().unwrap();

    for (i, s) in ss.iter().enumerate() {
        assert_eq!(
            s.to_string(),
            inflated[i].string(),
            "inflated[{}]: got '{}', want '{}'",
            i,
            inflated[i].string(),
            s
        );
    }
}

#[test]
fn test_reader_truncated() {
    struct Test<'a> {
        input: &'a [u8],
        output: &'a [u8],
    }
    let vectors: &[Test] = &[
        // Test {
        //     input: b"\x00",
        //     output: b"",
        // },
        // Test {
        //     input: b"\x00\x0c",
        //     output: b"",
        // },
        // Test {
        //     input: b"\x00\x0c\x00",
        //     output: b"",
        // },
        // Test {
        //     input: b"\x00\x0c\x00\xf3\xff",
        //     output: b"",
        // },
        Test {
            input: b"\x00\x0c\x00\xf3\xffhello",
            output: b"hello",
        },
        // Test {
        //     input: b"\x00\x0c\x00\xf3\xffhello, world",
        //     output: b"hello, world",
        // },
        // Test {
        //     input: b"\x02",
        //     output: b"",
        // },
        // Test {
        //     input: b"\xf2H\xcd",
        //     output: b"He",
        // },
        // Test {
        //     input: &[242, 72, 205, 153, 48, 97, 194, 132, 9], // "\xf2H͙0a\u0084\t"
        //     output: b"Hel\x90\x90\x90\x90\x90",
        // },
        // Test {
        //     input: &[242, 72, 205, 153, 48, 97, 194, 132, 9, 0], // "\xf2H͙0a\u0084\t\x00",
        //     output: b"Hel\x90\x90\x90\x90\x90",
        // },
    ];

    for (i, v) in vectors.iter().enumerate() {
        let mut input = std::io::Cursor::new(v.input);
        let mut zr = new_reader(&mut input);
        let (data, err) = ggio::read_all(&mut zr);
        println!("{:?}, {:?}", data, err);
        assert!(
            err.as_ref()
                .is_some_and(|e| e.kind() == std::io::ErrorKind::UnexpectedEof),
            "test {}, error mismatch: got {:?}, want std::io::Error::ErrUnexpectedEOF",
            i,
            err
        );
        assert_eq!(
            data.as_slice(),
            v.output,
            "test {}, output mismatch: got {:?}, want {:?}",
            i,
            data,
            v.output
        );
    }
}

#[test]
fn test_reset_dict() {
    let dict = "the lorem fox".as_bytes();
    let ss = &[
        "lorem ipsum izzle fo rizzle",
        "the quick brown fox jumped over",
    ];

    let mut deflated0 = bytes::Buffer::new();
    let mut deflated1 = bytes::Buffer::new();
    {
        let mut w = Writer::new_dict(&mut deflated0, DEFAULT_COMPRESSION, dict).unwrap();
        w.write(ss[0].as_bytes()).unwrap();
        w.close().unwrap();

        let mut w = Writer::new_dict(&mut deflated1, DEFAULT_COMPRESSION, dict).unwrap();
        w.write(ss[1].as_bytes()).unwrap();
        w.close().unwrap();
    }

    let inflated = &mut [bytes::Buffer::new(), bytes::Buffer::new()];

    let mut f = new_reader_dict(&mut deflated0, dict);
    let (n, err) = ggio::copy(&mut inflated[0], &mut f);
    assert!(err.is_none(), "not expecting error '{}'", err.unwrap());
    assert!(n > 0);

    f.reset(&mut deflated1, dict);
    let (n, err) = ggio::copy(&mut inflated[1], &mut f);
    assert!(err.is_none(), "not expecting error '{}'", err.unwrap());
    assert!(n > 0);
    f.close().unwrap();

    for (i, s) in ss.iter().enumerate() {
        assert_eq!(
            s.to_string(),
            inflated[i].string(),
            "inflated[{}]: got '{}', want '{}'",
            i,
            inflated[i].string(),
            s
        );
    }
}
