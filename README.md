# Partial implementation of Go standard library in Rust

This is a humble attempt to port some of the Go standard library to Rust.

Why:
- it can make easier for Go programmers to transition to Rust
- while Rust ecosistem has a lot of libraries covering every need (see https://crates.io/),
  it takes time to choose what libary to use from many possible options.
  It would be great to have something like the Go standard library - simple, extensive
  and well tested.

If you are considering using this library, take following into account:
- the quality of the code is lower that in Go library
- the code is less tested
- some optimizations are missing, so the performance may be lower than expected

## Usage examples

- flate: [examples/flate.rs](examples/flate.rs), [goexamples/flate/main.go](goexamples/flate/main.go)

You can run these using following commands:

```
cargo run --example flate
go run goexamples/flate/main.go

cargo run --example user
go run goexamples/user/main.go
```

See all Rust examples in [examples](./examples) folder and
corresponding Go examples in [goexamples](./goexamples) folder.

## Partially implemented modules

- bufio
- builtin
- bytes
- compress
- compress::flate
- compress::zlib
- crypto
- crypto::md5
- crypto::sha256
- encoding
- encoding::base64
- encoding::binary
- encoding::hex
- errors
- hash
- hash::adler32
- hash::crc32
- image
- image::color
- image::color/palette
- image::draw
- image::png
- internal
- internal::bytealg
- io
- math
- math::bits
- os
- os::user
- strconv
- strings
- syscall
- time
  - limitations:
    - no time zone support
    - no duration support
    - no formatting support
    - no monothonic time support
- unicode
- unicode::utf8

## Minimal supported Rust version

1.70.0

## Development process

When a piece of functionality is needed, appropriate Go source code is copied from
[go1.20.8](https://github.com/golang/go/tree/go1.20.8/src) and translated to Rust.

### Changes during convertion from Go to Rust

- structures and functions are renamed to meet Rust naming conventions
- usage of Go Reader and Writer interfaces replaced with Rust `std::io::Read` and
  `std::io::Write`.  That makes the code more idiomatic and more compatible with
  the rest of Rust ecosystem
- functions for creating objects are replaced with associated functions.  For example,
  Go `NewObject()` is changed to Rust `Object::new()`
- functions that return Go interfaces are usually changed to return specific types,
  because in Go it is easy to convert an interface to the underlying type, but in Rust
  it is not always possible

### Gotchas

#### Different priority of '<<' operator

Go: ` 1 << 1 + 1 == 3`
Rust: `1 << 1 + 1 == 4`

## License

The same license as used by Go project.  See [LICENSE](./LICENSE).
