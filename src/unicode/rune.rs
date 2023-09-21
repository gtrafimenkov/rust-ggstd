// Summary of Go Unicode and strings support:
//   > https://go.dev/blog/strings
//   > Go source code is always UTF-8.
//   > A string holds arbitrary bytes.
//   > A string literal, absent byte-level escapes, always holds valid UTF-8 sequences.
//   > Those sequences represent Unicode code points, called runes.
//   > No guarantee is made in Go that characters in strings are normalized.
//
// Difference with Rust:
//   - in Rust a string cannot contain arbitrary binary data - only utf-8 encoded text
//   - in Rust "char" is similar to Go "Rune", but char can contain only valid Unicode
//     code point, while in Go it can contain any i32 number

/// Rune is an alias for i32. It is added for compatibility with Go `rune` type, which is i32.
///
/// In Go `rune` is used to store a Unicode code-point, but the validity is not checked,
/// so it can contain any i32 number.  In Rust `char` is used to store a Unicode code-point,
/// but it is validated, so only valid Unicode code-points are allowed.
pub type Rune = i32;
