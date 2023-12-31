// // Copyright 2017 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.
// //
// //go:build linux

// package bytes_test

// import (
// 	. "bytes"
// 	"syscall"
// 	"testing"
// )

// // This file tests the situation where byte operations are checking
// // data very near to a page boundary. We want to make sure those
// // operations do not read across the boundary and cause a page
// // fault where they shouldn't.

// // These tests run only on linux. The code being tested is
// // not OS-specific, so it does not need to be tested on all
// // operating systems.

// // dangerousSlice returns a slice which is immediately
// // preceded and followed by a faulting page.
// func dangerousSlice(t *testing.T) [u8] {
// 	pagesize := syscall.Getpagesize()
// 	b, err := syscall.Mmap(0, 0, 3*pagesize, syscall.PROT_READ|syscall.PROT_WRITE, syscall.MAP_ANONYMOUS|syscall.MAP_PRIVATE)
// 	if err != nil {
// 		t.Fatalf("mmap failed %s", err)
// 	}
// 	err = syscall.Mprotect(b[..pagesize], syscall.PROT_NONE)
// 	if err != nil {
// 		t.Fatalf("mprotect low failed %s\n", err)
// 	}
// 	err = syscall.Mprotect(b[2*pagesize:], syscall.PROT_NONE)
// 	if err != nil {
// 		t.Fatalf("mprotect high failed %s\n", err)
// 	}
// 	return b[pagesize : 2*pagesize]
// }

// func TestEqualNearPageBoundary(t *testing.T) {
// 	t.Parallel()
// 	b := dangerousSlice(t)
// 	for i := range b {
// 		b[i] = 'A'
// 	}
// 	for i := 0; i <= b.len(); i += 1 {
// 		Equal(b[..i], b[b.len()-i:])
// 		Equal(b[b.len()-i:], b[..i])
// 	}
// }

// func TestIndexByteNearPageBoundary(t *testing.T) {
// 	t.Parallel()
// 	b := dangerousSlice(t)
// 	for i := range b {
// 		idx := index_byte(b[i:], 1)
// 		if idx != -1 {
// 			t.Fatalf("index_byte(b[{}:])={}, want -1\n", i, idx)
// 		}
// 	}
// }

// func TestIndexNearPageBoundary(t *testing.T) {
// 	t.Parallel()
// 	q := dangerousSlice(t)
// 	if len(q) > 64 {
// 		// Only worry about when we're near the end of a page.
// 		q = q[len(q)-64:]
// 	}
// 	b := dangerousSlice(t)
// 	if b.len() > 256 {
// 		// Only worry about when we're near the end of a page.
// 		b = b[b.len()-256:]
// 	}
// 	for j := 1; j < len(q); j++ {
// 		q[j-1] = 1 // difference is only found on the last byte
// 		for i := range b {
// 			idx := Index(b[i:], q[..j])
// 			if idx != -1 {
// 				t.Fatalf("Index(b[{}:], q[..{}])={}, want -1\n", i, j, idx)
// 			}
// 		}
// 		q[j-1] = 0
// 	}

// 	// Test differing alignments and sizes of q which always end on a page boundary.
// 	q[len(q)-1] = 1 // difference is only found on the last byte
// 	for j := 0; j < len(q); j++ {
// 		for i := range b {
// 			idx := Index(b[i:], q[j:])
// 			if idx != -1 {
// 				t.Fatalf("Index(b[{}:], q[{}:])={}, want -1\n", i, j, idx)
// 			}
// 		}
// 	}
// 	q[len(q)-1] = 0
// }
