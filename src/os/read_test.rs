// // Copyright 2009 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package os_test

// import (
// 	"bytes"
// 	. "os"
// 	"path/filepath"
// 	"testing"
// )

// func checkNamedSize(t *testing.T, path string, size int64) {
// 	dir, err := Stat(path)
// 	if err != nil {
// 		t.Fatalf("Stat %q (looking for size %d): %s", path, size, err)
// 	}
// 	if dir.size() != size {
// 		t.Errorf("Stat %q: size %d want %d", path, dir.size(), size)
// 	}
// }

// func TestReadFile(t *testing.T) {
// 	filename := "rumpelstilzchen"
// 	contents, err := read_file(filename)
// 	if err == nil {
// 		t.Fatalf("read_file %s: error expected, none found", filename)
// 	}

// 	filename = "read_test.go"
// 	contents, err = read_file(filename)
// 	if err != nil {
// 		t.Fatalf("read_file %s: %v", filename, err)
// 	}

// 	checkNamedSize(t, filename, int64(len(contents)))
// }

// func TestWriteFile(t *testing.T) {
// 	f, err := create_temp("", "ioutil-test")
// 	if err != nil {
// 		t.Fatal(err)
// 	}
// 	defer f.Close()
// 	defer Remove(f.Name())

// 	msg := "Programming today is a race between software engineers striving to " +
// 		"build bigger and better idiot-proof programs, and the Universe trying " +
// 		"to produce bigger and better idiots. So far, the Universe is winning."

// 	if err := WriteFile(f.Name(), []byte(msg), 0644); err != nil {
// 		t.Fatalf("WriteFile %s: %v", f.Name(), err)
// 	}

// 	data, err := read_file(f.Name())
// 	if err != nil {
// 		t.Fatalf("read_file %s: %v", f.Name(), err)
// 	}

// 	if string(data) != msg {
// 		t.Fatalf("read_file: wrong data:\nhave %q\nwant %q", string(data), msg)
// 	}
// }

// func TestReadOnlyWriteFile(t *testing.T) {
// 	if get_uid() == 0 {
// 		t.Skipf("Root can write to read-only files anyway, so skip the read-only test.")
// 	}

// 	// We don't want to use create_temp directly, since that opens a file for us as 0600.
// 	temp_dir_int, err := mkdir_temp("", t.Name())
// 	if err != nil {
// 		t.Fatal(err)
// 	}
// 	defer RemoveAll(temp_dir_int)
// 	filename := filepath.Join(temp_dir_int, "blurp.txt")

// 	shmorp := []byte("shmorp")
// 	florp := []byte("florp")
// 	err = WriteFile(filename, shmorp, 0444)
// 	if err != nil {
// 		t.Fatalf("WriteFile %s: %v", filename, err)
// 	}
// 	err = WriteFile(filename, florp, 0444)
// 	if err == nil {
// 		t.Fatalf("Expected an error when writing to read-only file %s", filename)
// 	}
// 	got, err := read_file(filename)
// 	if err != nil {
// 		t.Fatalf("read_file %s: %v", filename, err)
// 	}
// 	if !bytes.Equal(got, shmorp) {
// 		t.Fatalf("want %s, got %s", shmorp, got)
// 	}
// }

// func TestReadDir(t *testing.T) {
// 	dirname := "rumpelstilzchen"
// 	_, err := ReadDir(dirname)
// 	if err == nil {
// 		t.Fatalf("ReadDir %s: error expected, none found", dirname)
// 	}

// 	dirname = "."
// 	list, err := ReadDir(dirname)
// 	if err != nil {
// 		t.Fatalf("ReadDir %s: %v", dirname, err)
// 	}

// 	foundFile := false
// 	foundSubDir := false
// 	for _, dir := range list {
// 		switch {
// 		case !dir.IsDir() && dir.Name() == "read_test.go":
// 			foundFile = true
// 		case dir.IsDir() && dir.Name() == "exec":
// 			foundSubDir = true
// 		}
// 	}
// 	if !foundFile {
// 		t.Fatalf("ReadDir %s: read_test.go file not found", dirname)
// 	}
// 	if !foundSubDir {
// 		t.Fatalf("ReadDir %s: exec directory not found", dirname)
// 	}
// }
