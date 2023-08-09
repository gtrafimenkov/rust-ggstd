// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2012 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[cfg(not(debug_assertions))]
use super::deflate::MAX_STORE_BLOCK_SIZE;
use crate::bytes;
use crate::compress::flate;
use crate::io as ggio;
#[cfg(not(debug_assertions))]
use crate::time;
use std::io::Write;

// fn BenchmarkEncode(b *testing.B) {
// 	doBench(b, fn(b *testing.B, buf0 []byte, level, n int) {
// 		b.StopTimer()
// 		b.SetBytes(int64(n))

// 		buf1 := make([]byte, n)
// 		for i := 0; i < n; i += len(buf0) {
// 			if len(buf0) > n-i {
// 				buf0 = buf0[..n-i]
// 			}
// 			copy(buf1[i:], buf0)
// 		}
// 		buf0 = nil
// 		w, err := flate::Writer::new(ggio::Discard::new(), level)
// 		if err != nil {
// 			b.Fatal(err)
// 		}
// 		runtime.GC()
// 		b.StartTimer()
// 		for i := 0; i < b.N; i += 1 {
// 			w.reset(ggio::Discard::new())
// 			w.write(buf1)
// 			w.close()
// 		}
// 	})
// }

#[derive(Debug)]
// errorWriter is a writer that fails after N writes.
struct ErrorWriter {
    n: usize,
}

impl std::io::Write for ErrorWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "intentional error",
            ));
        }
        self.n -= 1;
        return Ok(buf.len());
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
// Test if errors from the underlying writer is passed upwards.
fn test_write_error() {
    // 	t.Parallel()
    let mut buf = bytes::Buffer::new();
    let n = 65536;
    // 	if !testing.Short() {
    // 		n *= 4
    // 	}
    for i in 0..n {
        let str = format!("asdasfasf{}{}fghfgujyut{}yutyu\n", i, i, i);
        buf.write_string(&str).unwrap();
    }
    let input = buf.bytes();
    // We create our own buffer to control number of writes.
    let mut copy_buffer = vec![0; 128];
    for l in 0..10 {
        let mut fail = 1;
        while fail <= 32 {
            // Fail after 'fail' writes
            let mut ew = ErrorWriter { n: fail };
            let mut w = flate::Writer::new(&mut ew, l).unwrap();
            let mut reader = bytes::new_reader(input);
            let (_n, err) = ggio::copy_buffer(&mut w, &mut reader, Some(&mut copy_buffer));
            assert!(
                err.is_some(),
                "Level {}: Expected an error, writer was {:?}",
                l,
                ew
            );
            let res = w.write(&[1, 2, 2, 3, 4, 5]);
            assert!(res.is_err(), "Level {}, Expected an error", l);
            let res = w.flush();
            assert!(res.is_err(), "Level {}, Expected an error on flush", l);
            let res = w.close();
            assert!(res.is_err(), "Level {}, Expected an error on close", l);

            let mut discard_writer = ggio::Discard::new();
            w.reset(&mut discard_writer);
            let res = w.write(&[1, 2, 3, 4, 5, 6]);
            assert!(
                res.as_ref().is_ok() && *res.as_ref().unwrap() == 6,
                "Level {}, Got unexpected error after reset: {:?}",
                l,
                res
            );
            // 			if testing.Short() {
            // 				return
            // 			}
            fail *= 2;
        }
    }
}

// Test if two runs produce identical results
// even when writing different sizes to the Writer.
// This test is slow, skipping it in debug mode
#[test]
#[cfg(not(debug_assertions))]
fn test_deterministic() {
    // 	t.Parallel()
    for level in 0..=9 {
        test_deterministic_int(level);
    }
    test_deterministic_int(-2);
}

#[cfg(not(debug_assertions))]
fn test_deterministic_int(level: isize) {
    // 	t.Parallel()
    // Test so much we cross a good number of block boundaries.
    let length = MAX_STORE_BLOCK_SIZE * 30 + 500;
    // 	if testing.Short() {
    // 		length /= 10
    // 	}

    // Create a random, but compressible stream.
    // ggrust TODO: uncomment when rand is implemented
    // 	rng := rand.New(rand.NewSource(1))
    let mut t1 = vec![0; length];
    for i in 0..length {
        // 		t1[i] = byte(rng.Int63() & 7)
        t1[i] = (time::now().nanosecond() / 100) as u8 & 7;
    }

    // Do our first encode.
    let mut b1 = bytes::Buffer::new();
    {
        let mut br = bytes::new_buffer(t1.clone());
        let mut w = flate::Writer::new(&mut b1, level).unwrap();
        // Use a very small prime sized buffer.
        let mut cbuf = vec![0; 787];
        let (_, err) = ggio::copy_buffer(&mut w, &mut br, Some(&mut cbuf));
        assert!(err.is_none());
        w.close().unwrap();
    }

    // We choose a different buffer size,
    // bigger than a maximum block, and also a prime.
    let mut b2 = bytes::Buffer::new();
    {
        let mut br = bytes::new_buffer(t1.clone());
        let mut w = flate::Writer::new(&mut b2, level).unwrap();
        let mut cbuf = vec![0; 81761];
        let (_, err) = ggio::copy_buffer(&mut w, &mut br, Some(&mut cbuf));
        assert!(err.is_none());
        w.close().unwrap();
    }

    let b1b = b1.bytes();
    let b2b = b2.bytes();

    if b1b != b2b {
        assert!(
            false,
            "level {} did not produce deterministic result, result mismatch, len(a) = {}, b.len() = {}",
            level,
            b1b.len(),
        b2b.len()
    );
    }
}

// // TestDeflateFast_Reset will test that encoding is consistent
// // across a warparound of the table offset.
// // See https://github.com/golang/go/issues/34121
// fn TestDeflateFast_Reset() {
// 	let mut buf = bytes::Buffer::new();
// 	n := 65536

// 	for i := 0; i < n; i += 1 {
// 		fmt.Fprintf(buf, "asdfasdfasdfasdf{}%dfghfgujyut%dyutyu\n", i, i, i)
// 	}
// 	// This is specific to level 1.
// 	const level = 1
// 	in := buf.bytes()
// 	offset := 1
// 	if testing.Short() {
// 		offset = 256
// 	}

// 	// We do an encode with a clean buffer to compare.
// 	var want bytes::Buffer::new()
// 	w, err := flate::Writer::new(&want, level)
// 	if err != nil {
// 		t.Fatalf("flate::Writer::new: level {}: {}", level, err)
// 	}

// 	// Output written 3 times.
// 	w.write(in)
// 	w.write(in)
// 	w.write(in)
// 	w.close()

// 	for ; offset <= 256; offset *= 2 {
// 		w, err := flate::Writer::new(ggio::Discard::new(), level)
// 		if err != nil {
// 			t.Fatalf("flate::Writer::new: level {}: {}", level, err)
// 		}

// 		// Reset until we are right before the wraparound.
// 		// Each reset adds maxMatchOffset to the offset.
// 		for i := 0; i < (bufferReset-len(in)-offset-maxMatchOffset)/maxMatchOffset; i += 1 {
// 			// skip ahead to where we are close to wrap around...
// 			w.d.reset(nil)
// 		}
// 		var got bytes::Buffer::new()
// 		w.reset(&got)

// 		// Write 3 times, close.
// 		for i := 0; i < 3; i += 1 {
// 			_, err = w.write(in)
// 			if err != nil {
// 				t.Fatal(err)
// 			}
// 		}
// 		err = w.close()
// 		if err != nil {
// 			t.Fatal(err)
// 		}
// 		if !bytes::equal(got.bytes(), want.bytes()) {
// 			t.Fatalf("output did not match at wraparound, len(want)  = {}, len(got) = {}", want.Len(), got.Len())
// 		}
// 	}
// }
