// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use crate::bytes;
use crate::encoding::base64::base64::ENCODE_STD;

use super::{
    get_raw_std_encoding, get_raw_url_encoding, get_std_encoding, get_url_encoding, Decoder,
    Encoder, Encoding,
};
use crate::strings;
use std::io::{Read, Write};

struct TestPair<'a> {
    decoded: &'a [u8],
    encoded: &'a str,
}

const PAIRS: &[TestPair] = &[
    // RFC 3548 examples
    TestPair {
        decoded: b"\x14\xfb\x9c\x03\xd9\x7e",
        encoded: "FPucA9l+",
    },
    TestPair {
        decoded: b"\x14\xfb\x9c\x03\xd9",
        encoded: "FPucA9k=",
    },
    TestPair {
        decoded: b"\x14\xfb\x9c\x03",
        encoded: "FPucAw==",
    },
    // RFC 4648 examples
    TestPair {
        decoded: b"",
        encoded: "",
    },
    TestPair {
        decoded: b"f",
        encoded: "Zg==",
    },
    TestPair {
        decoded: b"fo",
        encoded: "Zm8=",
    },
    TestPair {
        decoded: b"foo",
        encoded: "Zm9v",
    },
    TestPair {
        decoded: b"foob",
        encoded: "Zm9vYg==",
    },
    TestPair {
        decoded: b"fooba",
        encoded: "Zm9vYmE=",
    },
    TestPair {
        decoded: b"foobar",
        encoded: "Zm9vYmFy",
    },
    // Wikipedia examples
    TestPair {
        decoded: b"sure.",
        encoded: "c3VyZS4=",
    },
    TestPair {
        decoded: b"sure",
        encoded: "c3VyZQ==",
    },
    TestPair {
        decoded: b"sur",
        encoded: "c3Vy",
    },
    TestPair {
        decoded: b"su",
        encoded: "c3U=",
    },
    TestPair {
        decoded: b"leasure.",
        encoded: "bGVhc3VyZS4=",
    },
    TestPair {
        decoded: b"easure.",
        encoded: "ZWFzdXJlLg==",
    },
    TestPair {
        decoded: b"asure.",
        encoded: "YXN1cmUu",
    },
    TestPair {
        decoded: b"sure.",
        encoded: "c3VyZS4=",
    },
];

/// Do nothing to a reference base64 string (leave in standard format)
fn std_ref(reference: String) -> String {
    reference
}

/// Convert a reference string to URL-encoding
fn url_ref(reference: String) -> String {
    reference.replace('+', "-").replace('/', "_")
}

/// Convert a reference string to raw, unpadded format
fn raw_ref(reference: String) -> String {
    reference.trim_end_matches('=').to_string()
}

/// Both URL and unpadding conversions
fn raw_url_ref(reference: String) -> String {
    raw_ref(url_ref(reference))
}

/// A nonstandard encoding with a funny padding character, for testing
fn get_funny_encoding() -> &'static super::Encoding {
    static ENC: std::sync::OnceLock<super::Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| super::Encoding::new_with_options(ENCODE_STD, Some(b'@'), false))
}

fn get_funny_encoding_strict() -> &'static super::Encoding {
    static ENC: std::sync::OnceLock<super::Encoding> = std::sync::OnceLock::new();
    ENC.get_or_init(|| super::Encoding::new_with_options(ENCODE_STD, Some(b'@'), true))
}

fn funny_ref(reference: String) -> String {
    reference.replace('=', "@")
}

struct EncodingTest<'a> {
    enc: &'a super::Encoding,   // Encoding to test
    conv: fn(String) -> String, // Reference string converter
}

fn get_encoding_tests<'a>() -> Vec<EncodingTest<'a>> {
    vec![
        EncodingTest {
            enc: get_std_encoding(),
            conv: std_ref,
        },
        EncodingTest {
            enc: super::get_url_encoding(),
            conv: url_ref,
        },
        EncodingTest {
            enc: super::get_raw_std_encoding(),
            conv: raw_ref,
        },
        EncodingTest {
            enc: super::get_raw_url_encoding(),
            conv: raw_url_ref,
        },
        EncodingTest {
            enc: get_funny_encoding(),
            conv: funny_ref,
        },
        EncodingTest {
            enc: super::get_std_encoding_strict(),
            conv: std_ref,
        },
        EncodingTest {
            enc: super::get_url_encoding_strict(),
            conv: url_ref,
        },
        EncodingTest {
            enc: super::get_raw_std_encoding_strict(),
            conv: raw_ref,
        },
        EncodingTest {
            enc: super::get_raw_url_encoding_strict(),
            conv: raw_url_ref,
        },
        EncodingTest {
            enc: get_funny_encoding_strict(),
            conv: funny_ref,
        },
    ]
}

static BIGTEST: TestPair = TestPair {
    decoded: b"Twas brillig, and the slithy toves",
    encoded: "VHdhcyBicmlsbGlnLCBhbmQgdGhlIHNsaXRoeSB0b3Zlcw==",
};

#[test]
fn test_encode() {
    for p in PAIRS {
        for tt in get_encoding_tests() {
            let got = tt.enc.encode_to_string(p.decoded);
            let want = (tt.conv)(p.encoded.to_string());
            assert_eq!(
                got, want,
                "Encode({:?}) = {:?}, want {:?}",
                p.decoded, got, want
            );
        }
    }
}

#[test]
fn test_encoder() {
    for p in PAIRS {
        let mut bb = strings::Builder::new();
        let mut encoder = Encoder::new(get_std_encoding(), &mut bb);
        encoder.write_all(p.decoded).unwrap();
        encoder.close().unwrap();
        assert_eq!(
            p.encoded,
            bb.string(),
            "Encode({:?}) = {:?}, want {:?}",
            p.decoded,
            bb.string(),
            p.encoded
        );
    }
}

#[test]
fn test_encoder_buffering() {
    let input = BIGTEST.decoded;
    for bs in 1..=12 {
        let mut bb = crate::bytes::Buffer::new();
        let mut encoder = Encoder::new(get_std_encoding(), &mut bb);
        let mut pos = 0;
        while pos < input.len() {
            let end = (pos + bs).min(input.len());
            let n = encoder.write(&input[pos..end]).unwrap();
            assert_eq!(
                n,
                end - pos,
                "Write({:?}) gave length {}, want {}",
                &input[pos..end],
                n,
                end - pos
            );
            pos += bs;
        }
        encoder.close().unwrap();
        assert_eq!(
            bb.string(),
            BIGTEST.encoded,
            "Encoding/{} of {:?} = {:?}, want {:?}",
            bs,
            BIGTEST.decoded,
            bb.string(),
            BIGTEST.encoded
        )
    }
}

#[test]
fn test_decode() {
    for p in PAIRS {
        for tt in get_encoding_tests() {
            let encoded = (tt.conv)(p.encoded.to_string());
            let encoded_bytes = encoded.as_bytes();
            let mut dbuf = vec![0; tt.enc.decoded_len(encoded_bytes.len())];
            let (count, err) = tt.enc.decode(&mut dbuf, encoded_bytes);
            assert!(err.is_none(), "Decode({:?}) = error {:?}", encoded, err);
            assert_eq!(
                count,
                p.decoded.len(),
                "Decode({:?}) = length {}, want {}",
                encoded,
                count,
                p.decoded.len()
            );
            let decoded = &dbuf[0..count];
            assert_eq!(
                decoded, p.decoded,
                "Decode({:?}) = {:?}, want {:?}",
                encoded, decoded, p.decoded
            );

            let (dbuf, err) = tt.enc.decode_string(&encoded);
            assert!(
                err.is_none(),
                "decode_string({:?}) = error {:?}",
                encoded,
                err
            );
            assert_eq!(
                &dbuf, p.decoded,
                "decode_string({:?}) = {:?}, want {:?}",
                encoded, &dbuf, p.decoded
            );
        }
    }
}

#[test]
fn test_decoder() {
    let std_encoding = get_std_encoding();
    for p in PAIRS {
        let mut reader = strings::Reader::new(p.encoded);
        let mut dbuf = vec![0; reader.len() as usize];
        let mut decoder = Decoder::new(std_encoding, &mut reader);
        let count = decoder.read(&mut dbuf).unwrap();
        assert_eq!(
            count,
            p.decoded.len(),
            "Read from {:?} = length {}, want {}",
            p.encoded,
            count,
            p.decoded.len()
        );
        assert_eq!(
            &dbuf[..count],
            p.decoded,
            "Decoding of {:?} = {:?}, want {:?}",
            p.encoded,
            &dbuf[..count],
            p.decoded
        );
        let n = decoder.read(&mut dbuf).unwrap();
        assert_eq!(0, n, "Read from {:?} = {}, want EOF", p.encoded, n);
    }
}

#[test]
fn test_decoder_buffering() {
    for bs in 1..=12 {
        let mut data_reader = strings::Reader::new(BIGTEST.encoded);
        let mut decoder = Decoder::new(get_std_encoding(), &mut data_reader);
        let mut buf = vec![0; BIGTEST.decoded.len() + 12];
        let mut total: usize = 0;
        while total < BIGTEST.decoded.len() {
            let n = decoder.read(&mut buf[total..total + bs]).unwrap();
            total += n;
        }
        assert_eq!(
            BIGTEST.decoded,
            &buf[..total],
            "Decoding/{} of {:?} = {:?}, want {:?}",
            bs,
            BIGTEST.encoded,
            &buf[..total],
            BIGTEST.decoded
        );
    }
}

#[test]
fn test_decode_corrupt() {
    #[derive(Debug)]
    struct TestCase {
        input: &'static str,
        offset: isize, // -1 means no corruption.
    }
    let test_cases = &[
        TestCase {
            input: "",
            offset: -1,
        },
        TestCase {
            input: "\n",
            offset: -1,
        },
        TestCase {
            input: "AAA=\n",
            offset: -1,
        },
        TestCase {
            input: "AAAA\n",
            offset: -1,
        },
        TestCase {
            input: "!!!!",
            offset: 0,
        },
        TestCase {
            input: "====",
            offset: 0,
        },
        TestCase {
            input: "x===",
            offset: 1,
        },
        TestCase {
            input: "=AAA",
            offset: 0,
        },
        TestCase {
            input: "A=AA",
            offset: 1,
        },
        TestCase {
            input: "AA=A",
            offset: 2,
        },
        TestCase {
            input: "AA==A",
            offset: 4,
        },
        TestCase {
            input: "AAA=AAAA",
            offset: 4,
        },
        TestCase {
            input: "AAAAA",
            offset: 4,
        },
        TestCase {
            input: "AAAAAA",
            offset: 4,
        },
        TestCase {
            input: "A=",
            offset: 1,
        },
        TestCase {
            input: "A==",
            offset: 1,
        },
        TestCase {
            input: "AA=",
            offset: 3,
        },
        TestCase {
            input: "AA==",
            offset: -1,
        },
        TestCase {
            input: "AAA=",
            offset: -1,
        },
        TestCase {
            input: "AAAA",
            offset: -1,
        },
        TestCase {
            input: "AAAAAA=",
            offset: 7,
        },
        TestCase {
            input: "YWJjZA=====",
            offset: 8,
        },
        TestCase {
            input: "A!\n",
            offset: 1,
        },
        TestCase {
            input: "A=\n",
            offset: 1,
        },
    ];
    for tc in test_cases {
        let mut dbuf = vec![0; get_std_encoding().decoded_len(tc.input.len())];
        let (_, err) = get_std_encoding().decode(&mut dbuf, tc.input.as_bytes());
        if tc.offset == -1 {
            assert!(
                err.is_none(),
                "Decoder wrongly detected corruption in {:?}",
                tc.input
            );
            continue;
        }
        if let Some(err) = err {
            match err {
                crate::encoding::base64::Error::CorruptInputError(offset) => {
                    let offset = offset as isize;
                    assert_eq!(
                        offset, tc.offset,
                        "Corruption in {:?} at offset {}, want {}",
                        tc.input, offset, tc.offset
                    );
                }
            }
        } else {
            assert!(false, "Decoder failed to detect corruption in {:?}", tc);
        }
    }
}

#[test]
fn test_decode_bounds() {
    let mut buf = [0; 32];
    let s = get_std_encoding().encode_to_string(&buf);
    let (n, err) = get_std_encoding().decode(&mut buf, s.as_bytes());
    assert!(
        n == buf.len() && err.is_none(),
        "get_std_encoding().Decode = {}, {:?}, want {}, nil",
        n,
        err,
        buf.len()
    );
}

#[test]
fn test_encoded_len() {
    struct TestCase {
        enc: &'static Encoding,
        n: usize,
        want: usize,
    }
    let test_cases = &[
        TestCase {
            enc: get_raw_std_encoding(),
            n: 0,
            want: 0,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 1,
            want: 2,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 2,
            want: 3,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 3,
            want: 4,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 7,
            want: 10,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 0,
            want: 0,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 1,
            want: 4,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 2,
            want: 4,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 3,
            want: 4,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 4,
            want: 8,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 7,
            want: 12,
        },
    ];
    for tt in test_cases {
        let got = tt.enc.encoded_len(tt.n);
        assert_eq!(
            got, tt.want,
            "encoded_len({}): got {}, want {}",
            tt.n, got, tt.want
        );
    }
}

#[test]
fn test_decoded_len() {
    struct TestCase {
        enc: &'static Encoding,
        n: usize,
        want: usize,
    }
    let test_cases = &[
        TestCase {
            enc: get_raw_std_encoding(),
            n: 0,
            want: 0,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 2,
            want: 1,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 3,
            want: 2,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 4,
            want: 3,
        },
        TestCase {
            enc: get_raw_std_encoding(),
            n: 10,
            want: 7,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 0,
            want: 0,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 4,
            want: 3,
        },
        TestCase {
            enc: get_std_encoding(),
            n: 8,
            want: 6,
        },
    ];
    for tt in test_cases {
        let got = tt.enc.decoded_len(tt.n);
        assert_eq!(
            got, tt.want,
            "decoded_len({}): got {}, want {}",
            tt.n, got, tt.want
        );
    }
}

#[test]
fn test_big() {
    let n = 3 * 1000 + 1;
    let mut raw = vec![0; n];
    let alpha = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for i in 0..n {
        raw[i] = alpha[i % alpha.len()];
    }
    let mut encoded = bytes::Buffer::new();
    let mut w = Encoder::new(get_std_encoding(), &mut encoded);
    let nn = w.write(&raw).unwrap();
    assert_eq!(n, nn, "Encoder.Write(raw) = {} want {}", nn, n);
    w.close().unwrap();
    let mut decoder = Decoder::new(get_std_encoding(), &mut encoded);
    let (decoded, err) = crate::io::read_all(&mut decoder);
    assert!(
        err.is_none(),
        "crate::io::read_all(Decoder::new(...)): {:?}",
        err
    );
    assert_eq!(raw, decoded);
}

#[test]
fn test_new_line_characters() {
    // Each of these should decode to the string "sure", without errors.
    let expected = "sure";
    let examples = &[
        "c3VyZQ==",
        "c3VyZQ==\r",
        "c3VyZQ==\n",
        "c3VyZQ==\r\n",
        "c3VyZ\r\nQ==",
        "c3V\ryZ\nQ==",
        "c3V\nyZ\rQ==",
        "c3VyZ\nQ==",
        "c3VyZQ\n==",
        "c3VyZQ=\n=",
        "c3VyZQ=\r\n\r\n=",
    ];
    for e in examples {
        let (buf, err) = get_std_encoding().decode_string(e);
        assert!(err.is_none(), "Decode({:?}) failed: {:?}", e, err);
        assert_eq!(
            buf,
            expected.as_bytes(),
            "Decode({:?}) = {:?}, want {:?}",
            e,
            buf,
            expected
        );
    }
}

/// FaultInjectReader returns data from source, rate-limited
/// and with the errors as written to nextc.
struct FaultInjectReader<'a> {
    // source: &'a [u8],
    source: strings::Reader<'a>,
    nextc: Vec<Result<usize, std::io::Error>>,
}

impl std::io::Read for FaultInjectReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.nextc.is_empty() {
            return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
        }
        match self.nextc.remove(0) {
            Ok(n) => {
                self.source.read(&mut buf[..n])
            }
            Err(err) => Err(err),
        }
    }
}

/// tests that we don't ignore errors from our underlying reader
#[test]
fn test_decoder_issue3577() {
    let mut reader = FaultInjectReader {
        source: strings::Reader::new("VHdhcyBicmlsbGlnLCBhbmQgdGhlIHNsaXRoeSB0b3Zlcw=="), // twas brillig...,
        nextc: vec![
            Ok(5),
            Ok(10),
            Err(std::io::Error::new(std::io::ErrorKind::Other, "my error")),
            Ok(0),
            Err(std::io::Error::new(std::io::ErrorKind::Other, "my error")),
        ],
    };
    let mut d = Decoder::new(get_std_encoding(), &mut reader);
    let (_, err) = crate::io::read_all(&mut d);
    assert!(err.is_some());
    assert_eq!("my error", err.unwrap().to_string());
}

#[test]
fn test_decoder_issue4779() {
    let encoded = r#"CP/EAT8AAAEF
AQEBAQEBAAAAAAAAAAMAAQIEBQYHCAkKCwEAAQUBAQEBAQEAAAAAAAAAAQACAwQFBgcICQoLEAAB
BAEDAgQCBQcGCAUDDDMBAAIRAwQhEjEFQVFhEyJxgTIGFJGhsUIjJBVSwWIzNHKC0UMHJZJT8OHx
Y3M1FqKygyZEk1RkRcKjdDYX0lXiZfKzhMPTdePzRieUpIW0lcTU5PSltcXV5fVWZnaGlqa2xtbm
9jdHV2d3h5ent8fX5/cRAAICAQIEBAMEBQYHBwYFNQEAAhEDITESBEFRYXEiEwUygZEUobFCI8FS
0fAzJGLhcoKSQ1MVY3M08SUGFqKygwcmNcLSRJNUoxdkRVU2dGXi8rOEw9N14/NGlKSFtJXE1OT0
pbXF1eX1VmZ2hpamtsbW5vYnN0dXZ3eHl6e3x//aAAwDAQACEQMRAD8A9VSSSSUpJJJJSkkkJ+Tj
1kiy1jCJJDnAcCTykpKkuQ6p/jN6FgmxlNduXawwAzaGH+V6jn/R/wCt71zdn+N/qL3kVYFNYB4N
ji6PDVjWpKp9TSXnvTf8bFNjg3qOEa2n6VlLpj/rT/pf567DpX1i6L1hs9Py67X8mqdtg/rUWbbf
+gkp0kkkklKSSSSUpJJJJT//0PVUkkklKVLq3WMDpGI7KzrNjADtYNXvI/Mqr/Pd/q9W3vaxjnvM
NaCXE9gNSvGPrf8AWS3qmba5jjsJhoB0DAf0NDf6sevf+/lf8Hj0JJATfWT6/dV6oXU1uOLQeKKn
EQP+Hubtfe/+R7Mf/g7f5xcocp++Z11JMCJPgFBxOg7/AOuqDx8I/ikpkXkmSdU8mJIJA/O8EMAy
j+mSARB/17pKVXYWHXjsj7yIex0PadzXMO1zT5KHoNA3HT8ietoGhgjsfA+CSnvvqh/jJtqsrwOv
2b6NGNzXfTYexzJ+nU7/ALkf4P8Awv6P9KvTQQ4AgyDqCF85Pho3CTB7eHwXoH+LT65uZbX9X+o2
bqbPb06551Y4
"#;
    let encoded_short = strings::replace_all(encoded, "\n", "");

    let mut data_reader = strings::Reader::new(encoded);
    let mut dec = Decoder::new(get_std_encoding(), &mut data_reader);
    let (res1, err) = crate::io::read_all(&mut dec);
    assert!(err.is_none(), "ReadAll failed: {:?}", err);

    let mut data_reader = strings::Reader::new(&encoded_short);
    let mut dec = Decoder::new(get_std_encoding(), &mut data_reader);
    let (res2, err) = crate::io::read_all(&mut dec);
    assert!(err.is_none(), "ReadAll failed: {:?}", err);

    assert_eq!(res1, res2);
}

#[test]
fn test_decoder_issue7733() {
    let (s, err) = get_std_encoding().decode_string("YWJjZA=====");
    let want = Some(super::Error::CorruptInputError(8));
    assert_eq!(err, want, "Error = {:?}; want CorruptInputError(8)", err);
    assert_eq!(s, b"abcd", "decode_string = {:?}; want abcd", s);
}

#[test]
fn test_decoder_issue15656() {
    let (_, err) = get_std_encoding()
        .strict()
        .decode_string("WvLTlMrX9NpYDQlEIFlnDB==");
    let want = Some(super::Error::CorruptInputError(22));
    assert_eq!(err, want, "Error = {:?}; want CorruptInputError(22)", err);
    let (_, err) = get_std_encoding()
        .strict()
        .decode_string("WvLTlMrX9NpYDQlEIFlnDA==");
    assert!(err.is_none(), "Error = {:?}; want nil", err);
    let (_, err) = get_std_encoding().decode_string("WvLTlMrX9NpYDQlEIFlnDB==");
    assert!(err.is_none(), "Error = {:?}; want nil", err);
}

// fn Benchmarkencode_to_string(b *testing.B) {
// 	data := make([]byte, 8192)
// 	b.SetBytes(int64(len(data)))
// 	for i := 0; i < b.N; i++ {
// 		get_std_encoding().encode_to_string(data)
// 	}
// }

// fn BenchmarkDecodeString(b *testing.B) {
// 	sizes := []isize{2, 4, 8, 64, 8192}
// 	benchFunc := fn(b *testing.B, benchSize isize) {
// 		data := get_std_encoding().encode_to_string(make([]byte, benchSize))
// 		b.SetBytes(int64(len(data)))
// 		b.ResetTimer()
// 		for i := 0; i < b.N; i++ {
// 			get_std_encoding().decode_string(data)
// 		}
// 	}
// 	for _, size := range sizes {
// 		b.Run(fmt.Sprintf("{}", size), fn(b *testing.B) {
// 			benchFunc(b, size)
// 		})
// 	}
// }

// fn BenchmarkNewEncoding(b *testing.B) {
// 	b.SetBytes(int64(len(Encoding{}.decodeMap)))
// 	for i := 0; i < b.N; i++ {
// 		e := NewEncoding(encodeStd)
// 		for _, v := range e.decodeMap {
// 			_ = v
// 		}
// 	}
// }

#[test]
fn test_decoder_raw() {
    let source = "AAAAAA";
    let want = [0; 4];

    // Direct.
    let (dec1, err) = get_raw_url_encoding().decode_string(source);
    assert!(
        err.is_none() && dec1 == want,
        "get_raw_url_encoding().decode_string({:?}) = {:?}, {:?}, want {:?}, nil",
        source,
        dec1,
        err,
        want
    );

    // Through reader. Used to fail.
    let mut reader = bytes::Reader::new(source.as_bytes());
    let mut r = Decoder::new(get_raw_url_encoding(), &mut reader);
    let (dec2, err) = crate::io::read_all(&mut r);
    assert!(
        err.is_none() && dec2 == want,
        "reading Decoder::new(get_raw_url_encoding(), {:?}) = {:?}, {:?}, want {:?}, nil",
        source,
        dec2,
        err,
        want
    );

    // Should work with padding.
    let input = source.to_string() + "==";
    let mut reader = bytes::Reader::new(input.as_bytes());
    let mut r = Decoder::new(get_url_encoding(), &mut reader);
    let (dec3, err) = crate::io::read_all(&mut r);
    assert!(
        err.is_none() && dec3 == want,
        "reading Decoder::new(get_url_encoding(), {:?}) = {:?}, {:?}, want {:?}, nil",
        input,
        dec3,
        err,
        want
    );
}
