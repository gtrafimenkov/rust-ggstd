// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::deflate::{
    bulk_hash4, hash4, Compressor, Writer, DEFAULT_COMPRESSION, MAX_MATCH_LENGTH, MIN_MATCH_LENGTH,
};
use super::deflatefast::{DeflateFast, TableEntry, BUFFER_RESET, TABLE_SIZE};
use super::huffman_code::reverse_bits;
use super::inflate::new_reader;
use super::token::Token;
use crate::bytes;
use crate::io as ggio;
use std::io::Write;

struct DeflateTest<'a> {
    input: &'a [u8],
    level: isize,
    out: &'a [u8],
}

struct DeflateInflateTest<'a> {
    input: &'a [u8],
}

struct ReverseBitsTest {
    input: u16,
    bit_count: u8,
    out: u16,
}

const DEFLATE_TESTS: &[DeflateTest] = &[
    DeflateTest {
        input: &[],
        level: 0,
        out: &[1, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11],
        level: -1,
        out: &[18, 4, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11],
        level: DEFAULT_COMPRESSION,
        out: &[18, 4, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11],
        level: 4,
        out: &[18, 4, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11],
        level: 0,
        out: &[0, 1, 0, 254, 255, 17, 1, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11, 0x12],
        level: 0,
        out: &[0, 2, 0, 253, 255, 17, 18, 1, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11],
        level: 0,
        out: &[
            0, 8, 0, 247, 255, 17, 17, 17, 17, 17, 17, 17, 17, 1, 0, 0, 255, 255,
        ],
    },
    DeflateTest {
        input: &[],
        level: 2,
        out: &[1, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11],
        level: 2,
        out: &[18, 4, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11, 0x12],
        level: 2,
        out: &[18, 20, 2, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11],
        level: 2,
        out: &[18, 132, 2, 64, 0, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[],
        level: 9,
        out: &[1, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11],
        level: 9,
        out: &[18, 4, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11, 0x12],
        level: 9,
        out: &[18, 20, 2, 4, 0, 0, 255, 255],
    },
    DeflateTest {
        input: &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11],
        level: 9,
        out: &[18, 132, 2, 64, 0, 0, 0, 255, 255],
    },
];

const DEFLATE_INFLATE_TESTS: &[DeflateInflateTest] = &[
    DeflateInflateTest { input: &[] },
    DeflateInflateTest { input: &[0x11] },
    DeflateInflateTest {
        input: &[0x11, 0x12],
    },
    DeflateInflateTest {
        input: &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11],
    },
    DeflateInflateTest {
        input: &[
            0x11, 0x10, 0x13, 0x41, 0x21, 0x21, 0x41, 0x13, 0x87, 0x78, 0x13,
        ],
    },
];

const REVERSE_BITS_TESTS: &[ReverseBitsTest] = &[
    ReverseBitsTest {
        input: 1,
        bit_count: 1,
        out: 1,
    },
    ReverseBitsTest {
        input: 1,
        bit_count: 2,
        out: 2,
    },
    ReverseBitsTest {
        input: 1,
        bit_count: 3,
        out: 4,
    },
    ReverseBitsTest {
        input: 1,
        bit_count: 4,
        out: 8,
    },
    ReverseBitsTest {
        input: 1,
        bit_count: 5,
        out: 16,
    },
    ReverseBitsTest {
        input: 17,
        bit_count: 5,
        out: 17,
    },
    ReverseBitsTest {
        input: 257,
        bit_count: 9,
        out: 257,
    },
    ReverseBitsTest {
        input: 29,
        bit_count: 5,
        out: 23,
    },
];

fn large_data_chunk() -> Vec<u8> {
    let mut result = Vec::with_capacity(100000);
    for i in 0..result.len() {
        result[i] = (i * i & 0xFF) as u8;
    }
    return result;
}

#[test]
fn test_bulk_hash4() {
    for x in DEFLATE_TESTS {
        let mut y = Vec::new();
        y.extend_from_slice(x.out);
        if y.len() < MIN_MATCH_LENGTH {
            continue;
        }
        y.extend_from_slice(x.out);
        let mut j = 4;
        while j < y.len() {
            let y = &y[..j];
            let mut dst = vec![0; y.len() - MIN_MATCH_LENGTH + 1];
            for i in 0..dst.len() {
                dst[i] = (i + 100) as u32;
            }
            bulk_hash4(y, &mut dst);
            for (i, got) in dst.iter().enumerate() {
                let want = hash4(&y[i..]);
                if *got != want && *got == (i as u32) + 100 {
                    assert!(
                        false,
                        "Len:{} Index:{}, want 0x{:08x} but not modified",
                        y.len(),
                        i,
                        want
                    );
                } else if *got != want {
                    assert!(
                        false,
                        "Len:{} Index:{}, got 0x{:08x} want:0x{:08x}",
                        y.len(),
                        i,
                        *got,
                        want
                    );
                }
            }
            j += 1;
        }
    }
}

#[test]
fn test_deflate() {
    for h in DEFLATE_TESTS {
        let mut buf = bytes::Buffer::new();
        let mut w = Writer::new(&mut buf, h.level).unwrap();
        w.write(h.input).unwrap();
        w.close().unwrap();
        assert_eq!(h.out, buf.bytes());
    }
}

#[test]
fn test_writer_close() {
    let mut b = bytes::Buffer::new();
    let mut zw = Writer::new(&mut b, 6).unwrap();

    let c = zw.write("Test".as_bytes()).unwrap();
    assert_eq!(4, c);

    zw.close().unwrap();

    let res = zw.write("Test".as_bytes());
    assert!(res.is_err(), "Write to closed writer");

    let res = zw.flush();
    assert!(res.is_err(), "Flush to closed writer");

    let res = zw.close();
    assert!(!res.is_err(), "Close on a closed writer is not an error");
}

/// A SparseReader returns a stream consisting of 0s followed by 1<<16 1s.
/// This tests missing hash references in a very large input.
struct SparseReader {
    l: u64,
    cur: u64,
}

impl ggio::Reader for SparseReader {
    fn read(&mut self, p: &mut [u8]) -> (usize, Option<ggio::Error>) {
        if self.cur >= self.l {
            return (0, Some(ggio::Error::EOF));
        }
        let mut n = p.len();
        let mut cur = self.cur + n as u64;
        if cur > self.l {
            n -= (cur - self.l) as usize;
            cur = self.l;
        }
        for i in 0..p[0..n].len() {
            p[i] = if self.cur + i as u64 >= self.l - (1 << 16) {
                1
            } else {
                0
            };
        }
        self.cur = cur;
        // println!("{}, {}", n, self.cur);
        return (n, None);
    }
}

// This test is slow, skipping it in debug mode
#[test]
#[cfg(not(debug_assertions))]
fn test_very_long_sparse_chunk() {
    // 	if testing.Short() {
    // 		t.Skip("skipping sparse chunk during short test")
    // 	}
    let mut x = ggio::Discard::new();
    let mut w = Writer::new(&mut x, 1).unwrap();
    let size = 2_300_000_000;
    let mut sr = SparseReader { l: size, cur: 0 };
    let (n, err) = ggio::copy(&mut w, &mut sr);
    assert!(err.is_none(), "Compress failed: {}", err.unwrap());
    assert_eq!(size, n);
}

// type syncBuffer struct {
// 	buf    bytes::Buffer::new()
// 	mu     sync.RWMutex
// 	closed bool
// 	ready  chan bool
// }

// fn newSyncBuffer() *syncBuffer {
// 	return &syncBuffer{ready: make(chan bool, 1)}
// }

// fn (b *syncBuffer) Read(p [u8]) (n isize, err error) {
// 	for {
// 		b.mu.RLock()
// 		n, err = b.buf.Read(p)
// 		b.mu.RUnlock()
// 		if n > 0 || b.closed {
// 			return
// 		}
// 		<-b.ready
// 	}
// }

// fn (b *syncBuffer) signal() {
// 	select {
// 	case b.ready <- true:
// 	default:
// 	}
// }

// fn (b *syncBuffer) Write(p [u8]) (n isize, err error) {
// 	n, err = b.buf.Write(p)
// 	b.signal()
// 	return
// }

// fn (b *syncBuffer) WriteMode() {
// 	b.mu.Lock()
// }

// fn (b *syncBuffer) ReadMode() {
// 	b.mu.Unlock()
// 	b.signal()
// }

// fn (b *syncBuffer) Close() error {
// 	b.closed = true
// 	b.signal()
// 	return nil
// }

// fn test_sync(level: isize, input: &[u8], name: &str) {
//     if input.len() == 0 {
//         return;
//     }

//     // 	t.Logf("--test_sync {}, {}, %s", level, input.len(), name)
//     // 	buf := newSyncBuffer()
//     // 	buf1 := new(bytes::Buffer::new())
//     // 	buf.WriteMode()
//     // 	let mut w = Writer::new(io.MultiWriter(buf, buf1), level).unwrap();
//     // 	if err != nil {
//     // 		t.Errorf("NewWriter: {}", err)
//     // 		return
//     // 	}
//     // 	r := new_reader(buf)

//     // 	// Write half the input and read back.
//     // 	for i := 0; i < 2; i += 1 {
//     // 		var lo, hi isize
//     // 		if i == 0 {
//     // 			lo, hi = 0, (input.len()+1)/2
//     // 		} else {
//     // 			lo, hi = (input.len()+1)/2, input.len()
//     // 		}
//     // 		t.Logf("#{}: write {}-{}", i, lo, hi)
//     // 		if _, err := w.write(input[lo:hi]); err != nil {
//     // 			t.Errorf("test_sync: write: {}", err)
//     // 			return
//     // 		}
//     // 		if i == 0 {
//     // 			if err := w.flush(); err != nil {
//     // 				t.Errorf("test_sync: flush: {}", err)
//     // 				return
//     // 			}
//     // 		} else {
//     // 			if err := w.close(); err != nil {
//     // 				t.Errorf("test_sync: close: {}", err)
//     // 			}
//     // 		}
//     // 		buf.ReadMode()
//     // 		out := make([u8], hi-lo+1)
//     // 		m, err := io.read_at_least(r, out, hi-lo)
//     // 		t.Logf("#{}: read {}", i, m)
//     // 		if m != hi-lo || err != nil {
//     // 			t.Errorf("test_sync/{} ({}, {}, %s): read {}: {}, {} ({} left)", i, level, input.len(), name, hi-lo, m, err, buf.buf.len())
//     // 			return
//     // 		}
//     // 		if !bytes::equal(input[lo:hi], out[..hi-lo]) {
//     // 			t.Errorf("test_sync/{}: read wrong bytes: %x vs %x", i, input[lo:hi], out[..hi-lo])
//     // 			return
//     // 		}
//     // 		// This test originally checked that after reading
//     // 		// the first half of the input, there was nothing left
//     // 		// in the read buffer (buf.buf.len() != 0) but that is
//     // 		// not necessarily the case: the write Flush may emit
//     // 		// some extra framing bits that are not necessary
//     // 		// to process to obtain the first half of the uncompressed
//     // 		// data. The test ran correctly most of the time, because
//     // 		// the background goroutine had usually read even
//     // 		// those extra bits by now, but it's not a useful thing to
//     // 		// check.
//     // 		buf.WriteMode()
//     // 	}
//     // 	buf.ReadMode()
//     // 	out := make([u8], 10)
//     // 	if n, err := r.Read(out); n > 0 || err != io.EOF {
//     // 		t.Errorf("test_sync ({}, {}, %s): final Read: {}, {} (hex: %x)", level, input.len(), name, n, err, out[0:n])
//     // 	}
//     // 	if buf.buf.len() != 0 {
//     // 		t.Errorf("test_sync ({}, {}, %s): extra data at end", level, input.len(), name)
//     // 	}
//     // 	r.Close()

//     // 	// stream should work for ordinary reader too
//     // 	r = new_reader(buf1)
//     // 	out, err = ggio::read_all(r)
//     // 	if err != nil {
//     // 		t.Errorf("test_sync: read: %s", err)
//     // 		return
//     // 	}
//     // 	r.Close()
//     // 	if !bytes::equal(input, out) {
//     // 		t.Errorf("test_sync: decompress(compress(data)) != data: level={} input=%s", level, name)
//     // 	}
// }

fn test_to_from_with_level_and_limit(level: isize, input: &[u8], name: &str, limit: usize) {
    let mut buffer = bytes::Buffer::new();
    let mut w = Writer::new(&mut buffer, level).unwrap();
    w.write(input).unwrap();
    w.close().unwrap();

    if limit > 0 && buffer.len() > limit {
        assert!(
            false,
            "level: {}, len(compress(data)) = {} > limit = {}",
            level,
            buffer.len(),
            limit
        );
    }
    let mut r = new_reader(&mut buffer);
    let (data, err) = ggio::read_all(&mut r);
    assert!(err.is_none(), "read: {}", err.unwrap());
    r.close().unwrap();
    assert_eq!(
        input,
        data.as_slice(),
        "decompress(compress(data)) != data: level={} name={}",
        level,
        name
    );
    // test_sync(level, input, name);
}

fn test_to_from_with_limit(input: &[u8], name: &str, limit: &[usize; 11]) {
    for i in 0..10 {
        test_to_from_with_level_and_limit(i, input, name, limit[i as usize]);
    }
    // Test HuffmanCompression
    test_to_from_with_level_and_limit(-2, input, name, limit[10]);
}

#[test]
fn test_deflate_inflate() {
    // 	t.Parallel()
    let zero_limits = [0; 11];
    for (i, h) in DEFLATE_INFLATE_TESTS.iter().enumerate() {
        // 		if testing.Short() && len(h.input) > 10000 {
        // 			continue
        // 		}
        let name = format!("#{}", i);
        test_to_from_with_limit(h.input, &name, &zero_limits);
    }
    let large_chunk = large_data_chunk();
    test_to_from_with_limit(&large_chunk, "large chunk", &zero_limits);
}

#[test]
fn test_reverse_bits() {
    for h in REVERSE_BITS_TESTS {
        let v = reverse_bits(h.input, h.bit_count);
        assert_eq!(
            h.out, v,
            "reverseBits({},{}) = {}, want {}",
            h.input, h.bit_count, v, h.out
        );
    }
}

struct DeflateInflateStringTest {
    filename: &'static str,
    label: &'static str,
    limit: [usize; 11],
}

const DEFLATE_INFLATE_STRING_TESTS: &[DeflateInflateStringTest] = &[
    DeflateInflateStringTest {
        filename: "src/compress/testdata/e.txt",
        label: "2.718281828...",
        limit: [
            100018, 50650, 50960, 51150, 50930, 50790, 50790, 50790, 50790, 50790, 43683,
        ],
    },
    DeflateInflateStringTest {
        filename: "src/testdata/Isaac.Newton-Opticks.txt",
        label: "Isaac.Newton-Opticks",
        limit: [
            567248, 218338, 198211, 193152, 181100, 175427, 175427, 173597, 173422, 173422, 325240,
        ],
    },
];

#[test]
fn test_deflate_inflate_string() {
    // 	t.Parallel()
    // 	if testing.Short() && testenv.Builder() == "" {
    // 		t.Skip("skipping in short mode")
    // 	}
    for test in DEFLATE_INFLATE_STRING_TESTS {
        let gold = std::fs::read(test.filename).unwrap();
        test_to_from_with_limit(&gold, test.label, &test.limit);
        // 		if testing.Short() {
        // 			break
        // 		}
    }
}

// #[test]
// fn test_reader_dict() {
//     // ggrust: this test doesn't work because we can't do `b.reset` while mutable reference to b
//     // is held by the writer.
//     let dict = "hello world";
//     let text = "hello again world";
//     let mut b = bytes::Buffer::new();
//     let mut w = Writer::new(&mut b, 5).unwrap();
//     w.write(dict.as_bytes()).unwrap();
//     w.flush().unwrap();
//     b.reset();
//     let _ = w.write(text.as_bytes());
//     w.close().unwrap();

//     // 	r := new_reader_dict(&b, [u8](dict))
//     // 	data, err := ggio::read_all(r)
//     // 	if err != nil {
//     // 		t.Fatal(err)
//     // 	}
//     // 	if string(data) != "hello again world" {
//     // 		t.Fatalf("read returned %q want %q", string(data), text)
//     // 	}
// }

#[test]
fn test_writer_dict() {
    let dict = "hello world".as_bytes();
    let text = "hello again world".as_bytes();

    let mut b = bytes::Buffer::new();
    {
        let mut w = Writer::new(&mut b, 5).unwrap();
        w.write(dict).unwrap();
        w.flush().unwrap();
        // data written since this moment should be the same as in b1
        w.write(text).unwrap();
        w.close().unwrap();
    }

    let mut b1 = bytes::Buffer::new();

    {
        let mut w = Writer::new_dict(&mut b1, 5, dict).unwrap();
        w.write(text).unwrap();
        w.close().unwrap();
    }

    assert!(bytes::has_suffix(b.bytes(), b1.bytes()));
}

// See https://golang.org/issue/2508
#[test]
fn test_regression2508() {
    // 	if testing.Short() {
    // 		t.Logf("test disabled with -short")
    // 		return
    // 	}
    let mut discard = ggio::Discard::new();
    let mut w = Writer::new(&mut discard, 1).unwrap();
    let mut buf = vec![0; 1024];
    for i in 0..131072 {
        let res = w.write(&mut buf);
        assert!(res.is_ok(), "writer failed: {}, {:?}", i, res.err());
    }
    w.close().unwrap();
}

#[test]
fn test_writer_reset() {
    // 	t.Parallel()
    for level in 0..=9 {
        // 		if testing.Short() && level > 1 {
        // 			break
        // 		}
        let mut x = ggio::Discard::new();
        let mut w = Writer::new(&mut x, level).unwrap();
        let buf = b"hello world";
        let n = 1024;
        // 		if testing.Short() {
        // 			n = 10
        // 		}
        for _ in 0..n {
            w.write(buf).unwrap();
        }
        let mut x = ggio::Discard::new();
        w.reset(&mut x);

        // ggrust: I am not sure it is worth testing the internals as below

        // 		wref, err := Writer::new(ggio::Discard::new(), level)
        // 		if err != nil {
        // 			t.Fatalf("NewWriter: {}", err)
        // 		}

        // 		// DeepEqual doesn't compare functions.
        // 		w.d.fill, wref.d.fill = nil, nil
        // 		w.d.step, wref.d.step = nil, nil
        // 		w.d.bulkHasher, wref.d.bulkHasher = nil, nil
        // 		w.d.bestSpeed, wref.d.bestSpeed = nil, nil
        // 		// hashMatch is always overwritten when used.
        // 		copy(w.d.hashMatch[..], wref.d.hashMatch[..])
        // 		if len(w.d.tokens) != 0 {
        // 			t.Errorf("level {} Writer not reset after Reset. {} tokens were present", level, len(w.d.tokens))
        // 		}
        // 		// As long as the length is 0, we don't care about the content.
        // 		w.d.tokens = wref.d.tokens

        // 		// We don't care if there are values in the window, as long as it is at d.index is 0
        // 		w.d.window = wref.d.window
        // 		if !reflect.DeepEqual(w, wref) {
        // 			t.Errorf("level {} Writer not reset after Reset", level)
        // 		}
    }

    // 	levels := []isize{0, 1, 2, 5, 9}
    // 	for _, level := range levels {
    // 		t.Run(fmt.Sprint(level), fn() {
    // 			testResetOutput(t, level, nil)
    // 		})
    // 	}

    // 	t.Run("dict", fn() {
    // 		for _, level := range levels {
    // 			t.Run(fmt.Sprint(level), fn() {
    // 				testResetOutput(t, level, nil)
    // 			})
    // 		}
    // 	})
}

// fn testResetOutput(, level isize, dict [u8]) {
// 	writeData := fn(w *Writer) {
// 		msg := [u8]("now is the time for all good gophers")
// 		w.write(msg)
// 		w.flush()

// 		hello := [u8]("hello world")
// 		for i := 0; i < 1024; i += 1 {
// 			w.write(hello)
// 		}

// 		fill := bytes.Repeat([u8]("x"), 65000)
// 		w.write(fill)
// 	}

// 	let mut buf = bytes::Buffer::new();
// 	var w *Writer
// 	var err error
// 	if dict == nil {
// 		w, err = Writer::new(buf, level)
// 	} else {
// 		w, err = Writer::new_dict(buf, level, dict)
// 	}
// 	if err != nil {
// 		t.Fatalf("NewWriter: {}", err)
// 	}

// 	writeData(w)
// 	w.close()
// 	out1 := buf.bytes()

// 	buf2 := new(bytes::Buffer::new())
// 	w.reset(buf2)
// 	writeData(w)
// 	w.close()
// 	out2 := buf2.bytes()

// 	if len(out1) != len(out2) {
// 		t.Errorf("got {}, expected {} bytes", len(out2), len(out1))
// 		return
// 	}
// 	if !bytes::equal(out1, out2) {
// 		mm := 0
// 		for i, b := range out1[..len(out2)] {
// 			if b != out2[i] {
// 				t.Errorf("mismatch index {}: %#02x, expected %#02x", i, out2[i], b)
// 			}
// 			mm++
// 			if mm == 10 {
// 				t.Fatal("Stopping")
// 			}
// 		}
// 	}
// 	t.Logf("got {} bytes", len(out1))
// }

// // TestBestSpeed tests that round-tripping through deflate and then inflate
// // recovers the original input. The Write sizes are near the thresholds in the
// // compressor.encSpeed method (0, 16, 128), as well as near MAX_STORE_BLOCK_SIZE
// // (65535).
// fn TestBestSpeed() {
// 	t.Parallel()
// 	abc := make([u8], 128)
// 	for i in 0..abc.len() {
// 		abc[i] = byte(i)
// 	}
// 	abcabc := bytes.Repeat(abc, 131072/len(abc))
// 	var want [u8]

// 	testCases := [][]isize{
// 		{65536, 0},
// 		{65536, 1},
// 		{65536, 1, 256},
// 		{65536, 1, 65536},
// 		{65536, 14},
// 		{65536, 15},
// 		{65536, 16},
// 		{65536, 16, 256},
// 		{65536, 16, 65536},
// 		{65536, 127},
// 		{65536, 128},
// 		{65536, 128, 256},
// 		{65536, 128, 65536},
// 		{65536, 129},
// 		{65536, 65536, 256},
// 		{65536, 65536, 65536},
// 	}

// 	for i, tc := range testCases {
// 		if i >= 3 && testing.Short() {
// 			break
// 		}
// 		for _, firstN := range []isize{1, 65534, 65535, 65536, 65537, 131072} {
// 			tc[0] = firstN
// 		outer:
// 			for _, flush := range []bool{false, true} {
// 				let mut buf = bytes::Buffer::new();
// 				want = want[..0]

// 				let mut w = Writer::new(buf, BestSpeed).unwrap();
// 				if err != nil {
// 					t.Errorf("i={}, firstN={}, flush=%t: NewWriter: {}", i, firstN, flush, err)
// 					continue
// 				}
// 				for _, n := range tc {
// 					want = append(want, abcabc[..n]...)
// 					if _, err := w.write(abcabc[..n]); err != nil {
// 						t.Errorf("i={}, firstN={}, flush=%t: Write: {}", i, firstN, flush, err)
// 						continue outer
// 					}
// 					if !flush {
// 						continue
// 					}
// 					if err := w.flush(); err != nil {
// 						t.Errorf("i={}, firstN={}, flush=%t: Flush: {}", i, firstN, flush, err)
// 						continue outer
// 					}
// 				}
// 				if err := w.close(); err != nil {
// 					t.Errorf("i={}, firstN={}, flush=%t: Close: {}", i, firstN, flush, err)
// 					continue
// 				}

// 				r := new_reader(buf)
// 				got, err := ggio::read_all(r)
// 				if err != nil {
// 					t.Errorf("i={}, firstN={}, flush=%t: read_all: {}", i, firstN, flush, err)
// 					continue
// 				}
// 				r.Close()

// 				if !bytes::equal(got, want) {
// 					t.Errorf("i={}, firstN={}, flush=%t: corruption during deflate-then-inflate", i, firstN, flush)
// 					continue
// 				}
// 			}
// 		}
// 	}
// }

// var errIO = errors.New("IO error")

// // failWriter fails with errIO exactly at the nth call to Write.
// type failWriter struct{ n isize }

// fn (w *failWriter) Write(b [u8]) (isize, error) {
// 	w.n--
// 	if w.n == -1 {
// 		return 0, errIO
// 	}
// 	return b.len(), nil
// }

// fn TestWriterPersistentWriteError() {
// 	t.Parallel()
// 	d, err := os.ReadFile("../../testdata/Isaac.Newton-Opticks.txt")
// 	if err != nil {
// 		t.Fatalf("ReadFile: {}", err)
// 	}
// 	d = d[..10000] // Keep this test short

// 	zlet mut w = Writer::new(nil, DEFAULT_COMPRESSION).unwrap();
// 	if err != nil {
// 		t.Fatalf("NewWriter: {}", err)
// 	}

// 	// Sweep over the threshold at which an error is returned.
// 	// The variable i makes it such that the ith call to failWriter.Write will
// 	// return errIO. Since failWriter errors are not persistent, we must ensure
// 	// that flate.Writer errors are persistent.
// 	for i := 0; i < 1000; i += 1 {
// 		fw := &failWriter{i}
// 		zw.Reset(fw)

// 		_, werr := zw.Write(d)
// 		cerr := zw.Close()
// 		ferr := zw.Flush()
// 		if werr != errIO && werr != nil {
// 			t.Errorf("test {}, mismatching Write error: got {}, want {}", i, werr, errIO)
// 		}
// 		if cerr != errIO && fw.n < 0 {
// 			t.Errorf("test {}, mismatching Close error: got {}, want {}", i, cerr, errIO)
// 		}
// 		if ferr != errIO && fw.n < 0 {
// 			t.Errorf("test {}, mismatching Flush error: got {}, want {}", i, ferr, errIO)
// 		}
// 		if fw.n >= 0 {
// 			// At this point, the failure threshold was sufficiently high enough
// 			// that we wrote the whole stream without any errors.
// 			return
// 		}
// 	}
// }
// fn TestWriterPersistentFlushError() {
// 	zlet mut w = Writer::new(&failWriter{0}, DEFAULT_COMPRESSION).unwrap();
// 	if err != nil {
// 		t.Fatalf("NewWriter: {}", err)
// 	}
// 	flushErr := zw.Flush()
// 	closeErr := zw.Close()
// 	_, writeErr := zw.Write([u8]("Test"))
// 	checkErrors([]error{closeErr, flushErr, writeErr}, errIO, t)
// }

// fn TestWriterPersistentCloseError() {
// 	// If underlying writer return error on closing stream we should persistent this error across all writer calls.
// 	zlet mut w = Writer::new(&failWriter{0}, DEFAULT_COMPRESSION).unwrap();
// 	if err != nil {
// 		t.Fatalf("NewWriter: {}", err)
// 	}
// 	closeErr := zw.Close()
// 	flushErr := zw.Flush()
// 	_, writeErr := zw.Write([u8]("Test"))
// 	checkErrors([]error{closeErr, flushErr, writeErr}, errIO, t)

// 	// After closing writer we should persistent "write after close" error across Flush and Write calls, but return nil
// 	// on next Close calls.
// 	let mut b = bytes::Buffer::new();
// 	zw.Reset(&b)
// 	err = zw.Close()
// 	if err != nil {
// 		t.Fatalf("First call to close returned error: %s", err)
// 	}
// 	err = zw.Close()
// 	if err != nil {
// 		t.Fatalf("Second call to close returned error: %s", err)
// 	}

// 	flushErr = zw.Flush()
// 	_, writeErr = zw.Write([u8]("Test"))
// 	checkErrors([]error{flushErr, writeErr}, errWriterClosed, t)
// }

// fn checkErrors(got []error, want error) {
// 	t.Helper()
// 	for _, err := range got {
// 		if err != want {
// 			t.Errorf("Error doesn't match\nWant: %s\nGot: %s", want, got)
// 		}
// 	}
// }

#[test]
fn test_best_speed_match() {
    // 	t.Parallel()
    struct Test {
        previous: Vec<u8>,
        current: Vec<u8>,
        t: i32,
        s: usize,
        want: usize,
    }
    let cases = [
        Test {
            previous: vec![0, 0, 0, 1, 2],
            current: vec![3, 4, 5, 0, 1, 2, 3, 4, 5],
            t: -3,
            s: 3,
            want: 6,
        },
        Test {
            previous: vec![0, 0, 0, 1, 2],
            current: vec![2, 4, 5, 0, 1, 2, 3, 4, 5],
            t: -3,
            s: 3,
            want: 3,
        },
        Test {
            previous: vec![0, 0, 0, 1, 1],
            current: vec![3, 4, 5, 0, 1, 2, 3, 4, 5],
            t: -3,
            s: 3,
            want: 2,
        },
        Test {
            previous: vec![0, 0, 0, 1, 2],
            current: vec![2, 2, 2, 2, 1, 2, 3, 4, 5],
            t: -1,
            s: 0,
            want: 4,
        },
        Test {
            previous: vec![0, 0, 0, 1, 2, 3, 4, 5, 2, 2],
            current: vec![2, 2, 2, 2, 1, 2, 3, 4, 5],
            t: -7,
            s: 4,
            want: 5,
        },
        Test {
            previous: vec![9, 9, 9, 9, 9],
            current: vec![2, 2, 2, 2, 1, 2, 3, 4, 5],
            t: -1,
            s: 0,
            want: 0,
        },
        Test {
            previous: vec![9, 9, 9, 9, 9],
            current: vec![9, 2, 2, 2, 1, 2, 3, 4, 5],
            t: 0,
            s: 1,
            want: 0,
        },
        Test {
            previous: vec![],
            current: vec![9, 2, 2, 2, 1, 2, 3, 4, 5],
            t: -5,
            s: 1,
            want: 0,
        },
        Test {
            previous: vec![],
            current: vec![9, 2, 2, 2, 1, 2, 3, 4, 5],
            t: -1,
            s: 1,
            want: 0,
        },
        Test {
            previous: vec![],
            current: vec![2, 2, 2, 2, 1, 2, 3, 4, 5],
            t: 0,
            s: 1,
            want: 3,
        },
        Test {
            previous: vec![3, 4, 5],
            current: vec![3, 4, 5],
            t: -3,
            s: 0,
            want: 3,
        },
        Test {
            previous: vec![0; 1000],
            current: vec![0; 1000],
            t: -1000,
            s: 0,
            want: MAX_MATCH_LENGTH - 4,
        },
        Test {
            previous: vec![0; 200],
            current: vec![0; 500],
            t: -200,
            s: 0,
            want: MAX_MATCH_LENGTH - 4,
        },
        Test {
            previous: vec![0; 200],
            current: vec![0; 500],
            t: 0,
            s: 1,
            want: MAX_MATCH_LENGTH - 4,
        },
        Test {
            previous: vec![0; MAX_MATCH_LENGTH - 4],
            current: vec![0; 500],
            t: -(MAX_MATCH_LENGTH as i32 - 4),
            s: 0,
            want: MAX_MATCH_LENGTH - 4,
        },
        Test {
            previous: vec![0; 200],
            current: vec![0; 500],
            t: -200,
            s: 400,
            want: 100,
        },
        Test {
            previous: vec![0; 10],
            current: vec![0; 500],
            t: 200,
            s: 400,
            want: 100,
        },
    ];
    for c in cases {
        let mut e = DeflateFast {
            prev: c.previous,
            table: vec![TableEntry::default(); TABLE_SIZE as usize],
            cur: 0,
        };
        let got = e.match_len(c.s, c.t, &c.current);
        assert_eq!(c.want, got);
    }
}

// fn TestBestSpeedMaxMatchOffset() {
// 	t.Parallel()
// 	const abc, xyz = "abcdefgh", "stuvwxyz"
// 	for _, matchBefore := range []bool{false, true} {
// 		for _, extra := range []isize{0, inputMargin - 1, inputMargin, inputMargin + 1, 2 * inputMargin} {
// 			for offsetAdj := -5; offsetAdj <= +5; offsetAdj++ {
// 				report := fn(desc string, err error) {
// 					t.Errorf("matchBefore=%t, extra={}, offsetAdj={}: %s{}",
// 						matchBefore, extra, offsetAdj, desc, err)
// 				}

// 				offset := maxMatchOffset + offsetAdj

// 				// Make src to be a [u8] of the form
// 				//	"%s%s%s%s%s" % (abc, zeros0, xyzMaybe, abc, zeros1)
// 				// where:
// 				//	zeros0 is approximately maxMatchOffset zeros.
// 				//	xyzMaybe is either xyz or the empty string.
// 				//	zeros1 is between 0 and 30 zeros.
// 				// The difference between the two abc's will be offset, which
// 				// is maxMatchOffset plus or minus a small adjustment.
// 				src := make([u8], offset+len(abc)+extra)
// 				copy(src, abc)
// 				if !matchBefore {
// 					copy(src[offset-len(xyz):], xyz)
// 				}
// 				copy(src[offset:], abc)

// 				let mut buf = bytes::Buffer::new();
// 				let mut w = Writer::new(buf, BestSpeed).unwrap();
// 				if err != nil {
// 					report("NewWriter: ", err)
// 					continue
// 				}
// 				if _, err := w.write(src); err != nil {
// 					report("Write: ", err)
// 					continue
// 				}
// 				if err := w.close(); err != nil {
// 					report("Writer.Close: ", err)
// 					continue
// 				}

// 				r := new_reader(buf)
// 				dst, err := ggio::read_all(r)
// 				r.Close()
// 				if err != nil {
// 					report("read_all: ", err)
// 					continue
// 				}

// 				if !bytes::equal(dst, src) {
// 					report("", fmt.Errorf("bytes differ after round-tripping"))
// 					continue
// 				}
// 			}
// 		}
// 	}
// }

#[test]
fn test_best_speed_shift_offsets() {
    // Test if shiftoffsets properly preserves matches and resets out-of-range matches
    // seen in https://github.com/golang/go/issues/4142
    let mut enc = DeflateFast::new();

    // testData may not generate internal matches.
    let mut test_data: Vec<u8> = vec![0; 32];
    // ggrust TODO: use rand when implemented
    // 	let rng = rand.New(rand.NewSource(0));
    for i in 0..test_data.len() {
        // testData[i] = byte(rng.Uint32())
        test_data[i] = i as u8;
    }

    // Encode the testdata with clean state.
    // Second part should pick up matches from the first block.
    let mut dest1: Vec<Token> = Vec::new();
    enc.encode(&mut dest1, &test_data);
    let want_first_tokens = dest1.len();
    let mut dest2: Vec<Token> = Vec::new();
    enc.encode(&mut dest2, &test_data);
    let want_second_tokens = dest2.len();

    println!("{}, {}", want_first_tokens, want_second_tokens);

    assert!(
        want_first_tokens > want_second_tokens,
        "test needs matches between inputs to be generated"
    );
    // Forward the current indicator to before wraparound.
    enc.cur = BUFFER_RESET - test_data.len() as i32;

    // Part 1 before wrap, should match clean state.
    {
        let mut dest: Vec<Token> = Vec::new();
        enc.encode(&mut dest, &test_data);
        let got = dest.len();
        assert_eq!(
            want_first_tokens, got,
            "got {}, want {} tokens",
            got, want_first_tokens
        );

        // Verify we are about to wrap.
        assert_eq!(
            BUFFER_RESET, enc.cur,
            "got {}, want e.cur to be at bufferReset ({})",
            enc.cur, BUFFER_RESET
        );
    }

    // Part 2 should match clean state as well even if wrapped.
    {
        let mut dest1: Vec<Token> = Vec::new();
        enc.encode(&mut dest1, &test_data);
        let got = dest1.len();
        assert_eq!(
            want_second_tokens, got,
            "got {}, want {} token",
            got, want_second_tokens
        );

        // Verify that we wrapped.
        assert!(
            enc.cur < BUFFER_RESET,
            "want e.cur to be < bufferReset ({}), got {}",
            BUFFER_RESET,
            enc.cur
        );

        // Forward the current buffer, leaving the matches at the bottom.
        enc.cur = BUFFER_RESET;
        enc.shift_offsets();

        // Ensure that no matches were picked up.
        let mut dest2: Vec<Token> = Vec::new();
        enc.encode(&mut dest2, &test_data);
        let got = dest2.len();
        assert_eq!(
            want_first_tokens, got,
            "got {}, want {} tokens",
            got, want_first_tokens
        );
    }
}

// #[test]
// fn test_max_stack_size() {
//     // 	// This test must not run in parallel with other tests as debug.SetMaxStack
//     // 	// affects all goroutines.
//     // 	n := debug.SetMaxStack(1 << 16)
//     // 	defer debug.SetMaxStack(n)

//     // 	var wg sync.WaitGroup
//     // 	defer wg.Wait()

//     // 	b := make([u8], 1<<20)
//     // 	for level := HuffmanOnly; level <= BestCompression; level++ {
//     // 		// Run in separate goroutine to increase probability of stack regrowth.
//     // 		wg.Add(1)
//     // 		go fn(level isize) {
//     // 			defer wg.Done()
//     // 			zlet mut w = Writer::new(ggio::Discard::new(), level).unwrap();
//     // 			if err != nil {
//     // 				t.Errorf("level {}, Writer::new() = {}, want nil", level, err)
//     // 			}
//     // 			if n, err := zw.Write(b); n != b.len() || err != nil {
//     // 				t.Errorf("level {}, Write() = ({}, {}), want ({}, nil)", level, n, err, b.len())
//     // 			}
//     // 			if err := zw.Close(); err != nil {
//     // 				t.Errorf("level {}, Close() = {}, want nil", level, err)
//     // 			}
//     // 			zw.Reset(ggio::Discard::new())
//     // 		}(level)
//     // 	}
// }

#[test]
fn test_structure_sizes() {
    let size = std::mem::size_of::<Compressor>();
    assert!(
        500 >= size,
        "consider reducing size of Compressor ({})",
        size
    );
}
