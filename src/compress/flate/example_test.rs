// // Copyright 2016 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// // package flate_test

// // import (
// // 	"bytes"
// // 	"compress/flate"
// // 	"fmt"
// // 	"io"
// // 	"log"
// // 	"os"
// // 	"strings"
// // 	"sync"
// // )

// /// In performance critical applications, Reset can be used to discard the
// /// current compressor or decompressor state and reinitialize them quickly
// /// by taking advantage of previously allocated memory.
// fn Example_reset() {
//     let proverbs = &[
//         "Don't communicate by sharing memory, share memory by communicating.\n",
//         "Concurrency is not parallelism.\n",
//         "The bigger the interface, the weaker the abstraction.\n",
//         "Documentation is for users.\n",
//     ];

//     // 	var r strings.Reader
//     // 	let mut b = bytes::Buffer::new();
//     let mut buf = vec![0; 32 << 10];

//     // zw, err :=
//     flate::Writer::new(nil, flate.DEFAULT_COMPRESSION);
//     // 	if err != nil {
//     // 		log.Fatal(err)
//     // 	}
//     // 	zr := flate.new_reader(nil)

//     // 	for _, s := range proverbs {
//     // 		r.Reset(s)
//     // 		b.reset()

//     // 		// Reset the compressor and encode from some input stream.
//     // 		zw.Reset(&b)
//     // 		if _, err := ggio::copyBuffer(zw, &r, buf); err != nil {
//     // 			log.Fatal(err)
//     // 		}
//     // 		if err := zw.Close(); err != nil {
//     // 			log.Fatal(err)
//     // 		}

//     // 		// Reset the decompressor and decode to some output stream.
//     // 		if err := zr.(flate.Resetter).Reset(&b, nil); err != nil {
//     // 			log.Fatal(err)
//     // 		}
//     // 		if _, err := ggio::copyBuffer(os.Stdout, zr, buf); err != nil {
//     // 			log.Fatal(err)
//     // 		}
//     // 		if err := zr.Close(); err != nil {
//     // 			log.Fatal(err)
//     // 		}
//     // 	}

//     // Output:
//     // Don't communicate by sharing memory, share memory by communicating.
//     // Concurrency is not parallelism.
//     // The bigger the interface, the weaker the abstraction.
//     // Documentation is for users.
// }

// A preset dictionary can be used to improve the compression ratio.
// The downside to using a dictionary is that the compressor and decompressor
// must agree in advance what dictionary to use.
//
// See examples/flate-dict.rs

// // // DEFLATE is suitable for transmitting compressed data across the network.
// // fn Example_synchronization() {
// // 	var wg sync.WaitGroup
// // 	defer wg.Wait()

// // 	// Use io.Pipe to simulate a network connection.
// // 	// A real network application should take care to properly close the
// // 	// underlying connection.
// // 	rp, wp := io.Pipe()

// // 	// Start a goroutine to act as the transmitter.
// // 	wg.Add(1)
// // 	go fn() {
// // 		defer wg.Done()

// // 		zw, err := flate::Writer::new(wp, flate.BEST_SPEED)
// // 		if err != nil {
// // 			log.Fatal(err)
// // 		}

// // 		b := make([u8], 256)
// // 		for _, m := range strings.Fields("A long time ago in a galaxy far, far away...") {
// // 			// We use a simple framing format where the first byte is the
// // 			// message length, followed the message itself.
// // 			b[0] = uint8(copy(b[1:], m))

// // 			if _, err := zw.Write(b[..1+len(m)]); err != nil {
// // 				log.Fatal(err)
// // 			}

// // 			// Flush ensures that the receiver can read all data sent so far.
// // 			if err := zw.Flush(); err != nil {
// // 				log.Fatal(err)
// // 			}
// // 		}

// // 		if err := zw.Close(); err != nil {
// // 			log.Fatal(err)
// // 		}
// // 	}()

// // 	// Start a goroutine to act as the receiver.
// // 	wg.Add(1)
// // 	go fn() {
// // 		defer wg.Done()

// // 		zr := flate.new_reader(rp)

// // 		b := make([u8], 256)
// // 		for {
// // 			// Read the message length.
// // 			// This is guaranteed to return for every corresponding
// // 			// Flush and Close on the transmitter side.
// // 			if _, err := io.read_full(zr, b[..1]); err != nil {
// // 				if err == io.EOF {
// // 					break // The transmitter closed the stream
// // 				}
// // 				log.Fatal(err)
// // 			}

// // 			// Read the message content.
// // 			n := int(b[0])
// // 			if _, err := io.read_full(zr, b[..n]); err != nil {
// // 				log.Fatal(err)
// // 			}

// // 			fmt.Printf("Received {} bytes: %s\n", n, b[..n])
// // 		}
// // 		fmt.Println()

// // 		if err := zr.Close(); err != nil {
// // 			log.Fatal(err)
// // 		}
// // 	}()

// // 	// Output:
// // 	// Received 1 bytes: A
// // 	// Received 4 bytes: long
// // 	// Received 4 bytes: time
// // 	// Received 3 bytes: ago
// // 	// Received 2 bytes: in
// // 	// Received 1 bytes: a
// // 	// Received 6 bytes: galaxy
// // 	// Received 4 bytes: far,
// // 	// Received 3 bytes: far
// // 	// Received 7 bytes: away...
// // }
