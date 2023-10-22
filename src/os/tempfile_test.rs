// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2010 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::path::Path;

use crate::os::temp_dir;

use super::{TempDir, TempFile};

#[test]
fn test_create_temp() {
    let dir = TempDir::new("TestCreateTempBadDir").unwrap();
    let nonexistent_dir = dir.path.join("_not_exists_");
    let res = TempFile::new_in_dir(&nonexistent_dir, "foo");
    if res.is_ok() {
        panic!("create_temp should've failed");
    }
}

#[test]
fn test_create_temp_pattern() {
    struct TestCase {
        pattern: &'static str,
        prefix: &'static str,
        suffix: &'static str,
    }

    impl TestCase {
        fn new(pattern: &'static str, prefix: &'static str, suffix: &'static str) -> Self {
            Self {
                pattern,
                prefix,
                suffix,
            }
        }
    }
    let tests = &[
        TestCase::new("tempfile_test", "tempfile_test", ""),
        TestCase::new("tempfile_test*", "tempfile_test", ""),
        TestCase::new("tempfile_test*xyz", "tempfile_test", "xyz"),
    ];
    for test in tests {
        let tf = TempFile::new(test.pattern).unwrap();
        let base = tf.path.file_name().unwrap().to_str().unwrap();
        assert!(
            base.starts_with(test.prefix) && base.ends_with(test.suffix),
            "create_temp pattern {:?} created bad name {:?}; want prefix {:?} & suffix {:?}",
            test.pattern,
            base,
            test.prefix,
            test.suffix
        );
    }
}

#[test]
fn test_create_temp_bad_pattern() {
    let td = TempDir::new_in_dir(Path::new(""), "test_create_temp_bad_pattern").unwrap();

    struct TestCase(String, bool);
    let sep = std::path::MAIN_SEPARATOR;
    let tests = &[
        TestCase("ioutil*test".to_string(), false),
        TestCase("tempfile_test*foo".to_string(), false),
        TestCase(format!("tempfile_test{}foo", sep), true),
        TestCase(format!("tempfile_test*{}foo", sep), true),
        TestCase(format!("tempfile_test{}*foo", sep), true),
        TestCase(format!("{}tempfile_test{}*foo", sep, sep), true),
        TestCase(format!("tempfile_test*foo{}", sep), true),
    ];
    for tt in tests {
        let (pattern, want_err) = (&tt.0, tt.1);
        let res = TempFile::new_in_dir(&td.path, pattern);
        if want_err {
            assert!(
                res.is_err(),
                "create_temp(..., {:?}) succeeded, expected error",
                pattern
            );
            let err = res.err().unwrap();
            assert!(
                super::is_err_pattern_has_separator(&err),
                "create_temp(..., {:?}): {}, expected ErrPatternHasSeparator",
                pattern,
                err
            );
        } else {
            assert!(res.is_ok(), "create_temp(..., {:?}) want error", pattern);
        }
    }
}

#[test]
fn test_mkdir_temp() {
    let res = TempDir::new_in_dir(Path::new("/_not_exists_"), "foo");
    assert!(
        res.is_err(),
        "mkdir_temp(`/_not_exists_`, `foo`) want error"
    );
    struct TestCase {
        pattern: &'static str,
        _prefix: &'static str,
        _suffix: &'static str,
    }

    impl TestCase {
        fn new(pattern: &'static str, prefix: &'static str, suffix: &'static str) -> Self {
            Self {
                pattern,
                _prefix: prefix,
                _suffix: suffix,
            }
        }
    }

    let tests = &[
        TestCase::new("tempfile_test", "tempfile_test", ""),
        TestCase::new("tempfile_test*", "tempfile_test", ""),
        TestCase::new("tempfile_test*xyz", "tempfile_test", "xyz"),
    ];

    // 	dir := filepath.Clean(temp_dir())
    let dir = temp_dir();

    fn run_test_mkdir_temp(dir: &Path, pattern: &str, _want_re_pat: &str) {
        let _td = TempDir::new_in_dir(dir, pattern).unwrap();
        // 		re := regexp.MustCompile(wantRePat)
        // 		if !re.MatchString(name) {
        // 			t.Errorf("mkdir_temp(%q, %q) created bad name\n\t%q\ndid not match pattern\n\t%q", dir, pattern, name, wantRePat)
        // 		}
    }

    for tt in tests {
        // wantRePat := "^" + regexp.QuoteMeta(filepath.Join(dir, tt.wantPrefix)) + "[0-9]+" + regexp.QuoteMeta(tt.wantSuffix) + "$"
        let want_re_pat = "ggstd TODO: implement when regexp is implemented";
        run_test_mkdir_temp(&dir, tt.pattern, want_re_pat);
    }

    // Separately testing "*xyz" (which has no prefix). That is when constructing the
    // pattern to assert on, as in the previous loop, using filepath.Join for an empty
    // prefix filepath.Join(dir, ""), produces the pattern:
    //     ^<DIR>[0-9]+xyz$
    // yet we just want to match
    //     "^<DIR>/[0-9]+xyz"
    // wantRePat := "^" + regexp.QuoteMeta(filepath.Join(dir)) + regexp.QuoteMeta(string(filepath.Separator)) + "[0-9]+xyz$"
    let want_re_pat = "ggstd TODO: implement when regexp is implemented";
    run_test_mkdir_temp(&dir, "*xyz", want_re_pat);
}

/// test that we return a nice error message if the dir argument to temp_dir doesn't
/// exist (or that it's empty and temp_dir doesn't exist)
#[test]
fn test_mkdir_temp_bad_dir() {
    let dir = TempDir::new_in_dir(Path::new(""), "MkdirTempBadDir").unwrap();
    let bad_dir = dir.path.join("not-exist");
    let res = TempDir::new_in_dir(&bad_dir, "foo");
    assert!(res.is_err());
}

#[test]
fn test_mkdir_temp_bad_pattern() {
    let td = TempDir::new_in_dir(Path::new(""), "test_mkdir_temp_bad_pattern").unwrap();

    struct TestCase(String, bool);
    let sep = std::path::MAIN_SEPARATOR;
    let tests = &[
        TestCase("ioutil*test".to_string(), false),
        TestCase("tempfile_test*foo".to_string(), false),
        TestCase(format!("tempfile_test{}foo", sep), true),
        TestCase(format!("tempfile_test*{}foo", sep), true),
        TestCase(format!("tempfile_test{}*foo", sep), true),
        TestCase(format!("{}tempfile_test{}*foo", sep, sep), true),
        TestCase(format!("tempfile_test*foo{}", sep), true),
    ];
    for tt in tests {
        let (pattern, want_err) = (&tt.0, tt.1);
        let res = TempDir::new_in_dir(&td.path, pattern);
        if want_err {
            assert!(
                res.is_err(),
                "TempDir::new_in_dir(..., {:?}) succeeded, expected error",
                pattern
            );
            let err = res.err().unwrap();
            assert!(
                super::is_err_pattern_has_separator(&err),
                "TempDir::new_in_dir(..., {:?}): {}, expected ErrPatternHasSeparator",
                pattern,
                err
            );
        } else {
            assert!(
                res.is_ok(),
                "TempDir::new_in_dir(..., {:?}) want error",
                pattern
            );
        }
    }
}
