// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::adler32;

struct Test {
    out: u32,
    input: &'static str,
    // 	halfState string // marshaled hash state after first half of in written, used by TestGoldenMarshal
}

impl Test {
    const fn new(out: u32, input: &'static str) -> Self {
        Self { out, input }
    }
}

const GOLDEN: &[Test] = &[
Test::new(0x00000001, "" /*, "adl\x01\x00\x00\x00\x01"*/),
Test::new(0x00620062, "a" /*, "adl\x01\x00\x00\x00\x01"*/),
Test::new(0x012600c4, "ab" /*, "adl\x01\x00b\x00b"*/),
Test::new(0x024d0127, "abc" /*, "adl\x01\x00b\x00b"*/),
Test::new(0x03d8018b, "abcd" /*, "adl\x01\x01&\x00\xc4"*/),
Test::new(0x05c801f0, "abcde" /*, "adl\x01\x01&\x00\xc4"*/),
Test::new(0x081e0256, "abcdef" /*, "adl\x01\x02M\x01'"*/),
Test::new(0x0adb02bd, "abcdefg" /*, "adl\x01\x02M\x01'"*/),
Test::new(0x0e000325, "abcdefgh" /*, "adl\x01\x03\xd8\x01\x8b"*/),
Test::new(0x118e038e, "abcdefghi" /*, "adl\x01\x03\xd8\x01\x8b"*/),
Test::new(0x158603f8, "abcdefghij" /*, "adl\x01\x05\xc8\x01\xf0"*/),
Test::new(0x3f090f02, "Discard medicine more than two years old." /*, "adl\x01NU\a\x87"*/),
Test::new(0x46d81477, "He who has a shady past knows that nice guys finish last." /*, "adl\x01\x89\x8e\t\xe9"*/),
Test::new(0x40ee0ee1, "I wouldn't marry him with a ten foot pole." /*, "adl\x01R\t\ag"*/),
Test::new(0x16661315, "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave" /*, "adl\x01\u007f\xbb\t\x10"*/),
Test::new(0x5b2e1480, "The days of the digital watch are numbered.  -Tom Stoppard" /*, "adl\x01\x99:\n~"*/),
Test::new(0x8c3c09ea, "Nepal premier won't resign." /*, "adl\x01\"\x05\x05\x05"*/),
Test::new(0x45ac18fd, "For every action there is an equal and opposite government program." /*, "adl\x01\xcc\xfa\f\x00"*/),
Test::new(0x53c61462, "His money is twice tainted: 'taint yours and 'taint mine." /*, "adl\x01\x93\xa9\n\b"*/),
Test::new(0x7e511e63, "There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977" /*, "adl\x01e\xf5\x10\x14"*/),
Test::new(0xe4801a6a, "It's a tiny change to the code and not completely disgusting. - Bob Manchek" /*, "adl\x01\xee\x00\f\xb2"*/),
Test::new(0x61b507df, "size:  a.out:  bad magic" /*, "adl\x01\x1a\xfc\x04\x1d"*/),
Test::new(0xb8631171, "The major problem is with sendmail.  -Mark Horton" /*, "adl\x01mi\b\xdc"*/),
Test::new(0x8b5e1904, "Give me a rock, paper and scissors and I will move the world.  CCFestoon" /*, "adl\x01\xe3\n\f\x9f"*/),
Test::new(0x7cc6102b, "If the enemy is within range, then so are you." /*, "adl\x01_\xe0\b\x1e"*/),
Test::new(0x700318e7, "It's well we cannot hear the screams/That we create in others' dreams." /*, "adl\x01ۘ\f\x87"*/),
Test::new(0x1e601747, "You remind me of a TV show, but that's all right: I watch it anyway." /*, "adl\x01\xcc}\v\x83"*/),
Test::new(0xb55b0b09, "C is as portable as Stonehedge!!" /*, "adl\x01,^\x05\xad"*/),
Test::new(0x39111dd0, "Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley" /*, "adl\x01M\xd1\x0e\xc8"*/),
Test::new(0x91dd304f, "The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule" /*, "adl\x01#\xd8\x17\xd7"*/),
Test::new(0x2e5d1316, "How can you write a big system without C++?  -Paul Glick" /*, "adl\x01\x8fU\n\x0f"*/),
Test::new(0xd0201df6, "'Invariant assertions' is the most elegant programming technique!  -Tom Szymanski" /*, "adl\x01/\x98\x0e\xc4"*/),
// Test::new(0x211297c8, strings.Repeat("\xff", 5548) + "8" /*, "adl\x01\x9a\xa6\xcb\xc1"*/),
// Test::new(0xbaa198c8, strings.Repeat("\xff", 5549) + "9" /*, "adl\x01gu\xcc\xc0"*/),
// Test::new(0x553499be, strings.Repeat("\xff", 5550) + "0" /*, "adl\x01gu\xcc\xc0"*/),
// Test::new(0xf0c19abe, strings.Repeat("\xff", 5551) + "1" /*, "adl\x015CͿ"*/),
// Test::new(0x8d5c9bbe, strings.Repeat("\xff", 5552) + "2" /*, "adl\x015CͿ"*/),
// Test::new(0x2af69cbe, strings.Repeat("\xff", 5553) + "3" /*, "adl\x01\x04\x10ξ"*/),
// Test::new(0xc9809dbe, strings.Repeat("\xff", 5554) + "4" /*, "adl\x01\x04\x10ξ"*/),
// Test::new(0x69189ebe, strings.Repeat("\xff", 5555) + "5" /*, "adl\x01\xd3\xcdϽ"*/),
// Test::new(0x86af0001, strings.Repeat("\x00", 1e5)/*, "adl\x01\xc3P\x00\x01"*/),
// Test::new(0x79660b4d, strings.Repeat("a", 1e5)/*, "adl\x01\x81k\x05\xa7"*/),
// Test::new(0x110588ee, strings.Repeat("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 1e4)/*, "adl\x01e\xd2\xc4p"*/),
];

// checksum is a slow but simple implementation of the Adler-32 checksum.
// It is a straight port of the sample code in RFC 1950 section 9.
fn slow_checksum(p: &[u8]) -> u32 {
    let mut s1: u32 = 1;
    let mut s2: u32 = 0;
    for x in p {
        s1 = (s1 + (*x as u32)) % adler32::MODULO;
        s2 = (s2 + s1) % adler32::MODULO;
    }
    s2 << 16 | s1
}

#[test]
fn test_golden() {
    for test in GOLDEN {
        let p = test.input.as_bytes();
        let mut input_str = String::from_utf8_lossy(p).to_string();
        if p.len() > 200 {
            // 			in = in[..100] + "..." + in[len(in)-100:]
            let mut shorter = p[0..100].to_vec();
            let mut dots = "...".as_bytes().to_vec();
            let mut end = p[p.len().saturating_sub(100)..].to_vec();
            shorter.append(&mut dots);
            shorter.append(&mut end);
            input_str = String::from_utf8_lossy(&shorter).to_string();
        }
        let got = slow_checksum(p);
        assert_eq!(
            test.out, got,
            "simple implementation: slow_checksum({}) = {:#x} want {:#x}",
            input_str, got, test.out
        );
        let got = adler32::checksum(p);
        assert_eq!(
            test.out, got,
            "optimized implementation: Checksum({}) = {:#x} want {:#x}",
            input_str, got, test.out
        );
    }
}

// fn TestGoldenMarshal() {
// 	for g in GOLDEN {
// 		h := New()
// 		h2 := New()

// 		io.write_string(h, g.input[..len(g.input)/2])

// 		state, err := h.(encoding.BinaryMarshaler).MarshalBinary()
// 		if err != nil {
// 			t.Errorf("could not marshal: {}", err)
// 			continue
// 		}

// 		if string(state) != g.halfState {
// 			t.Errorf("checksum(%q) state = %q, want %q", g.input, state, g.halfState)
// 			continue
// 		}

// 		if err := h2.(encoding.BinaryUnmarshaler).UnmarshalBinary(state); err != nil {
// 			t.Errorf("could not unmarshal: {}", err)
// 			continue
// 		}

// 		io.write_string(h, g.input[len(g.input)/2:])
// 		io.write_string(h2, g.input[len(g.input)/2:])

// 		if h.sum32() != h2.sum32() {
// 			t.Errorf("checksum(%q) = 0x%x != marshaled (0x%x)", g.input, h.sum32(), h2.sum32())
// 		}
// 	}
// }

// fn BenchmarkAdler32KB(b *testing.B) {
// 	b.SetBytes(1024)
// 	data := make([u8], 1024)
// 	for i := range data {
// 		data[i] = byte(i)
// 	}
// 	h := New()
// 	let in = make([u8], 0, h.Size())

// 	b.ResetTimer()
// 	for i := 0; i < b.N; i += 1 {
// 		h.Reset()
// 		h.Write(data)
// 		h.Sum(in)
// 	}
// }
