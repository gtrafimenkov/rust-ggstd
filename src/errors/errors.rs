// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// Package errors implements functions to manipulate errors.
//
// The [New] function creates errors whose only content is a text message.
//
// An error e wraps another error if e's type has one of the methods
//
//	Unwrap() error
//	Unwrap() []error
//
// If e.Unwrap() returns a non-nil error w or a slice containing w,
// then we say that e wraps w. A nil error returned from e.Unwrap()
// indicates that e does not wrap any error. It is invalid for an
// Unwrap method to return an []error containing a nil error value.
//
// An easy way to create wrapped errors is to call [fmt.Errorf] and apply
// the %w verb to the error argument:
//
//	wrapsErr := fmt.Errorf("... %w ...", ..., err, ...)
//
// Successive unwrapping of an error creates a tree. The [Is] and [As]
// functions inspect an error's tree by examining first the error
// itself followed by the tree of each of its children in turn
// (pre-order, depth-first traversal).
//
// Is examines the tree of its first argument looking for an error that
// matches the second. It reports whether it finds a match. It should be
// used in preference to simple equality checks:
//
//	if errors.Is(err, fs.ErrExist)
//
// is preferable to
//
//	if err == fs.ErrExist
//
// because the former will succeed if err wraps [io/fs.ErrExist].
//
// As examines the tree of its first argument looking for an error that can be
// assigned to its second argument, which must be a pointer. If it succeeds, it
// performs the assignment and returns true. Otherwise, it returns false. The form
//
//	var perr *fs.PathError
//	if errors.As(err, &perr) {
//		fmt.Println(perr.Path)
//	}
//
// is preferable to
//
//	if perr, ok := err.(*fs.PathError); ok {
//		fmt.Println(perr.Path)
//	}
//
// because the former will succeed if err wraps an [*io/fs.PathError].

use crate::builtin;

/// new returns an error that formats as the given text.
pub fn new_str(text: &str) -> ErrorString {
    ErrorString::new(text)
}

/// new_static returns an error that formats as the given static text.
pub const fn new_static(text: &'static str) -> ErrorStaticString {
    ErrorStaticString { s: text }
}

/// ErrorString is a trivial implementation of error.
#[derive(Debug)]
pub struct ErrorString {
    s: String,
}

impl ErrorString {
    pub fn new(text: &str) -> Self {
        Self { s: text.to_owned() }
    }
}

impl std::fmt::Display for ErrorString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl std::error::Error for ErrorString {
    // fn description(&self) -> &str {
    //     &self.s
    // }
}

impl builtin::Error for ErrorString {
    fn error(&self) -> String {
        self.s.clone()
    }
}

/// ErrorStaticString is a trivial implementation of error.
#[derive(Debug, Clone, Copy)]
pub struct ErrorStaticString {
    s: &'static str,
}

impl ErrorStaticString {
    pub fn new(text: &'static str) -> Self {
        Self { s: text }
    }
}

impl std::fmt::Display for ErrorStaticString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl std::error::Error for ErrorStaticString {}

impl builtin::Error for ErrorStaticString {
    fn error(&self) -> String {
        self.s.to_string()
    }
}

// // ErrUnsupported indicates that a requested operation cannot be performed,
// // because it is unsupported. For example, a call to [os.Link] when using a
// // file system that does not support hard links.
// //
// // Functions and methods should not return this error but should instead
// // return an error including appropriate context that satisfies
// //
// //	errors.Is(err, errors.ErrUnsupported)
// //
// // either by directly wrapping ErrUnsupported or by implementing an Is method.
// //
// // Functions and methods should document the cases in which an error
// // wrapping this will be returned.
// var ErrUnsupported = New("unsupported operation")

pub fn new_stdio_other_error(message: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, message)
}

pub fn new_unexpected_eof() -> std::io::Error {
    std::io::Error::from(std::io::ErrorKind::UnexpectedEof)
}

pub fn copy_stdio_error(e: &std::io::Error) -> std::io::Error {
    std::io::Error::new(e.kind(), e.to_string())
}

pub fn copy_stdio_option_error(e: &Option<std::io::Error>) -> Option<std::io::Error> {
    e.as_ref().map(copy_stdio_error)
}

/// RUST_EOF is Rust representation  of EOF
pub const RUST_EOF: std::io::Result<usize> = Ok(0);

pub fn is_rust_eof(res: &std::io::Result<usize>) -> bool {
    if let Ok(value) = res {
        *value == 0
    } else {
        false
    }
}

/// iores_to_result converts Go IO error to Rust IO error.
pub fn iores_to_result(iores: crate::io::IoRes) -> std::io::Result<usize> {
    if iores.0 > 0 {
        Ok(iores.0)
    } else if let Some(err) = iores.1 {
        Err(err)
    } else {
        Ok(0)
    }
}
