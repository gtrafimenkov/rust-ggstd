// Copyright 2023 The rust-ggstd authors.
// Copyright 2016 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[test]
fn test_fix_long_path() {
    // 	if os.CanUseLongPaths {
    // 		return
    // 	}
    // 248 is long enough to trigger the longer-than-248 checks in
    // fix_long_path, but short enough not to make a path component
    // longer than 255, which is illegal on Windows. (which
    // doesn't really matter anyway, since this is purely a string
    // function we're testing, and it's not actually being used to
    // do a system call)
    let very_long = format!("l{}ng", "o".repeat(248));
    struct TestCase(&'static str, &'static str);
    let test_cases = &[
        // Short; unchanged:
        TestCase(r"C:\short.txt", r"C:\short.txt"),
        TestCase(r"C:\", r"C:\"),
        TestCase(r"C:", r"C:"),
        // The "long" substring is replaced by a looooooong
        // string which triggers the rewriting. Except in the
        // cases below where it doesn't.
        TestCase(r"C:\long\foo.txt", r"\\?\C:\long\foo.txt"),
        TestCase(r"C:/long/foo.txt", r"\\?\C:\long\foo.txt"),
        TestCase(r"C:\long\foo\\bar\.\baz\\", r"\\?\C:\long\foo\bar\baz"),
        TestCase(r"\\unc\path", r"\\unc\path"),
        TestCase(r"long.txt", r"long.txt"),
        TestCase(r"C:long.txt", r"C:long.txt"),
        TestCase(r"c:\long\..\bar\baz", r"c:\long\..\bar\baz"),
        TestCase(r"\\?\c:\long\foo.txt", r"\\?\c:\long\foo.txt"),
        TestCase(r"\\?\c:\long/foo.txt", r"\\?\c:\long/foo.txt"),
    ];
    for (i, test) in test_cases.iter().enumerate() {
        let input = test.0.to_string().replace("long", &very_long);
        let want = test.1.to_string().replace("long", &very_long);
        let got = super::fix_long_path(&input);
        assert_eq!(
            want, got,
            "#{} fix_long_path({:?}) = {:?}; want {:?}",
            i, input, got, want
        );
    }
}

// #[test]
// fn TestMkdirAllLongPath() {
// 	tmpDir := t.TempDir()
// 	path := tmpDir
// 	for i := 0; i < 100; i++ {
// 		path += `\another-path-component`
// 	}
// 	if err := os.MkdirAll(path, 0777); err != nil {
// 		t.Fatalf("MkdirAll(%q) failed; %v", path, err)
// 	}
// 	if err := os.RemoveAll(tmpDir); err != nil {
// 		t.Fatalf("RemoveAll(%q) failed; %v", tmpDir, err)
// 	}
// }

// #[test]
// fn TestMkdirAllExtendedLength() {
// 	tmpDir := t.TempDir()

// 	const prefix = `\\?\`
// 	if len(tmpDir) < 4 || tmpDir[:4] != prefix {
// 		fullPath, err := syscall.FullPath(tmpDir)
// 		if err != nil {
// 			t.Fatalf("FullPath(%q) fails: %v", tmpDir, err)
// 		}
// 		tmpDir = prefix + fullPath
// 	}
// 	path := tmpDir + `\dir\`
// 	if err := os.MkdirAll(path, 0777); err != nil {
// 		t.Fatalf("MkdirAll(%q) failed: %v", path, err)
// 	}

// 	path = path + `.\dir2`
// 	if err := os.MkdirAll(path, 0777); err == nil {
// 		t.Fatalf("MkdirAll(%q) should have failed, but did not", path)
// 	}
// }

// #[test]
// fn TestOpenRootSlash() {
// 	tests := []string{
// 		`/`,
// 		`\`,
// 	}

// 	for _, test := range tests {
// 		dir, err := os.Open(test)
// 		if err != nil {
// 			t.Fatalf("Open(%q) failed: %v", test, err)
// 		}
// 		dir.Close()
// 	}
// }
