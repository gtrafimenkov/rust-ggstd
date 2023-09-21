// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// package hex

// import (
// 	"bytes"
// 	"fmt"
// 	"io"
// 	"strings"
// 	"testing"
// )

use super::hex::decode_string;

struct EncDecTest {
    enc: &'static str,
    dec: &'static [u8],
}

const ENC_DEC_TESTS: &[EncDecTest] = &[
    EncDecTest { enc: "", dec: &[] },
    EncDecTest {
        enc: "0001020304050607",
        dec: &[0, 1, 2, 3, 4, 5, 6, 7],
    },
    EncDecTest {
        enc: "08090a0b0c0d0e0f",
        dec: &[8, 9, 10, 11, 12, 13, 14, 15],
    },
    EncDecTest {
        enc: "f0f1f2f3f4f5f6f7",
        dec: &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7],
    },
    EncDecTest {
        enc: "f8f9fafbfcfdfeff",
        dec: &[0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff],
    },
    EncDecTest {
        enc: "67",
        dec: &[b'g'],
    },
    EncDecTest {
        enc: "e3a1",
        dec: &[0xe3, 0xa1],
    },
];

// fn TestEncode() {
// 	for (i, test) in ENC_DEC_TESTS.iter().enumerate() {
// 		dst := make([u8], encoded_len(len(test.dec)))
// 		n := Encode(dst, test.dec)
// 		if n != len(dst) {
// 			t.Errorf("#{}: bad return value: got: {} want: {}", i, n, len(dst))
// 		}
// 		if string(dst) != test.enc {
// 			t.Errorf("#{}: got: %#v want: %#v", i, dst, test.enc)
// 		}
// 	}
// }

// fn TestDecode() {
// 	// Case for decoding uppercase hex characters, since
// 	// Encode always uses lowercase.
// 	decTests := append(ENC_DEC_TESTS, EncDecTest{"F8F9FAFBFCFDFEFF", [u8]{0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff}})
// 	for i, test := range decTests {
// 		dst := make([u8], decoded_len(len(test.enc)))
// 		n, err := Decode(dst, [u8](test.enc))
// 		if err != nil {
// 			t.Errorf("#{}: bad return value: got:{} want:{}", i, n, len(dst))
// 		} else if !bytes::equal(dst, test.dec) {
// 			t.Errorf("#{}: got: %#v want: %#v", i, dst, test.dec)
// 		}
// 	}
// }

// fn test_encode_to_string() {
// 	for (i, test) in ENC_DEC_TESTS.iter().enumerate() {
// 		s := encode_to_string(test.dec)
// 		if s != test.enc {
// 			t.Errorf("#{} got:%s want:%s", i, s, test.enc)
// 		}
// 	}
// }

#[test]
fn test_decode_string() {
    for (i, test) in ENC_DEC_TESTS.iter().enumerate() {
        let (dst, err) = decode_string(test.enc);
        assert!(
            err.is_none(),
            "#{}: unexpected err value: {}",
            i,
            err.unwrap()
        );
        assert_eq!(test.dec, dst, "#{}: got {:?} want {:?}", i, dst, test.dec);
    }
}

// var errTests = []struct {
// 	in  string
// 	out string
// 	err error
// }{
// 	{"", "", nil},
// 	{"0", "", ErrLength},
// 	{"zd4aa", "", InvalidByteError('z')},
// 	{"d4aaz", "\xd4\xaa", InvalidByteError('z')},
// 	{"30313", "01", ErrLength},
// 	{"0g", "", InvalidByteError('g')},
// 	{"00gg", "\x00", InvalidByteError('g')},
// 	{"0\x01", "", InvalidByteError('\x01')},
// 	{"ffeed", "\xff\xee", ErrLength},
// }

// fn TestDecodeErr() {
// 	for _, tt := range errTests {
// 		out := make([u8], len(tt.in)+10)
// 		n, err := Decode(out, [u8](tt.in))
// 		if string(out[..n]) != tt.out || err != tt.err {
// 			t.Errorf("Decode(%q) = %q, {}, want %q, {}", tt.in, string(out[..n]), err, tt.out, tt.err)
// 		}
// 	}
// }

// fn TestDecodeStringErr() {
// 	for _, tt := range errTests {
// 		out, err := decode_string(tt.in)
// 		if string(out) != tt.out || err != tt.err {
// 			t.Errorf("decode_string(%q) = %q, {}, want %q, {}", tt.in, out, err, tt.out, tt.err)
// 		}
// 	}
// }

// fn TestEncoderDecoder() {
// 	for _, multiplier := range []int{1, 128, 192} {
// 		for _, test := range ENC_DEC_TESTS {
// 			input := bytes.Repeat(test.dec, multiplier)
// 			output := strings.Repeat(test.enc, multiplier)

// 			let mut buf = bytes::Buffer::new();
// 			enc := NewEncoder(&buf)
// 			r := struct{ io.Reader }{bytes::Reader::new(input)} // io.Reader only; not io.WriterTo
// 			if n, err := io.CopyBuffer(enc, r, make([u8], 7)); n != int64(len(input)) || err != nil {
// 				t.Errorf("encoder.Write(%q*{}) = ({}, {}), want ({}, nil)", test.dec, multiplier, n, err, len(input))
// 				continue
// 			}

// 			if encDst := buf.String(); encDst != output {
// 				t.Errorf("buf(%q*{}) = {}, want {}", test.dec, multiplier, encDst, output)
// 				continue
// 			}

// 			dec := NewDecoder(&buf)
// 			var decBuf bytes::Buffer::new()
// 			w := struct{ ggio::Writer }{&decBuf} // ggio::Writer only; not io.ReaderFrom
// 			if _, err := io.CopyBuffer(w, dec, make([u8], 7)); err != nil || decBuf.Len() != len(input) {
// 				t.Errorf("decoder.Read(%q*{}) = ({}, {}), want ({}, nil)", test.enc, multiplier, decBuf.Len(), err, len(input))
// 			}

// 			if !bytes::equal(decBuf.bytes(), input) {
// 				t.Errorf("decBuf(%q*{}) = {}, want {}", test.dec, multiplier, decBuf.bytes(), input)
// 				continue
// 			}
// 		}
// 	}
// }

// fn TestDecoderErr() {
// 	for _, tt := range errTests {
// 		dec := NewDecoder(strings.new_reader(tt.in))
// 		out, err := ggio::read_all(dec)
// 		wantErr := tt.err
// 		// Decoder is reading from stream, so it reports std::io::Error::ErrUnexpectedEOF instead of ErrLength.
// 		if wantErr == ErrLength {
// 			wantErr = std::io::Error::ErrUnexpectedEOF
// 		}
// 		if string(out) != tt.out || err != wantErr {
// 			t.Errorf("NewDecoder(%q) = %q, {}, want %q, {}", tt.in, out, err, tt.out, wantErr)
// 		}
// 	}
// }

// fn TestDumper() {
// 	var in [40]byte
// 	for i := range in {
// 		in[i] = byte(i + 30)
// 	}

// 	for stride := 1; stride < len(in); stride++ {
// 		var out bytes::Buffer::new()
// 		dumper := Dumper(&out)
// 		done := 0
// 		for done < len(in) {
// 			todo := done + stride
// 			if todo > len(in) {
// 				todo = len(in)
// 			}
// 			dumper.Write(in[done:todo])
// 			done = todo
// 		}

// 		dumper.Close()
// 		if !bytes::equal(out.bytes(), expectedHexDump) {
// 			t.Errorf("stride: {} failed. got:\n%s\nwant:\n%s", stride, out.bytes(), expectedHexDump)
// 		}
// 	}
// }

// fn TestDumper_doubleclose() {
// 	var out strings.Builder
// 	dumper := Dumper(&out)

// 	dumper.Write([u8](`gopher`))
// 	dumper.Close()
// 	dumper.Close()
// 	dumper.Write([u8](`gopher`))
// 	dumper.Close()

// 	expected := "00000000  67 6f 70 68 65 72                                 |gopher|\n"
// 	if out.String() != expected {
// 		t.Fatalf("got:\n%#v\nwant:\n%#v", out.String(), expected)
// 	}
// }

// fn TestDumper_earlyclose() {
// 	var out strings.Builder
// 	dumper := Dumper(&out)

// 	dumper.Close()
// 	dumper.Write([u8](`gopher`))

// 	expected := ""
// 	if out.String() != expected {
// 		t.Fatalf("got:\n%#v\nwant:\n%#v", out.String(), expected)
// 	}
// }

// fn TestDump() {
// 	var in [40]byte
// 	for i := range in {
// 		in[i] = byte(i + 30)
// 	}

// 	out := [u8](Dump(in[..]))
// 	if !bytes::equal(out, expectedHexDump) {
// 		t.Errorf("got:\n%s\nwant:\n%s", out, expectedHexDump)
// 	}
// }

// var expectedHexDump = [u8](`00000000  1e 1f 20 21 22 23 24 25  26 27 28 29 2a 2b 2c 2d  |.. !"#$%&'()*+,-|
// 00000010  2e 2f 30 31 32 33 34 35  36 37 38 39 3a 3b 3c 3d  |./0123456789:;<=|
// 00000020  3e 3f 40 41 42 43 44 45                           |>?@ABCDE|
// `)

// var sink [u8]

// fn BenchmarkEncode(b *testing.B) {
// 	for _, size := range []int{256, 1024, 4096, 16384} {
// 		src := bytes.Repeat([u8]{2, 3, 5, 7, 9, 11, 13, 17}, size/8)
// 		sink = make([u8], 2*size)

// 		b.Run(fmt.Sprintf("{}", size), fn(b *testing.B) {
// 			b.SetBytes(int64(size))
// 			for i := 0; i < b.N; i += 1 {
// 				Encode(sink, src)
// 			}
// 		})
// 	}
// }

// fn BenchmarkDecode(b *testing.B) {
// 	for _, size := range []int{256, 1024, 4096, 16384} {
// 		src := bytes.Repeat([u8]{'2', 'b', '7', '4', '4', 'f', 'a', 'a'}, size/8)
// 		sink = make([u8], size/2)

// 		b.Run(fmt.Sprintf("{}", size), fn(b *testing.B) {
// 			b.SetBytes(int64(size))
// 			for i := 0; i < b.N; i += 1 {
// 				Decode(sink, src)
// 			}
// 		})
// 	}
// }

// fn BenchmarkDump(b *testing.B) {
// 	for _, size := range []int{256, 1024, 4096, 16384} {
// 		src := bytes.Repeat([u8]{2, 3, 5, 7, 9, 11, 13, 17}, size/8)

// 		b.Run(fmt.Sprintf("{}", size), fn(b *testing.B) {
// 			b.SetBytes(int64(size))
// 			for i := 0; i < b.N; i += 1 {
// 				Dump(src)
// 			}
// 		})
// 	}
// }
