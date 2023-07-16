# Partial implementation of Go standard library in Rust

Go language has a very good [standard library](https://pkg.go.dev/std): simple,
extensive and well tested.  It would be great to have something like that in Rust.

## Usage examples

Usage examples can be found in [src/bin](./src/bin) folder.
They can be run with `cargo run --bin NAME` command, for example:

```
cargo run --bin time
```

## Partially implemented modules

- bytes
- crypto::sha256
- encoding::binary
- encoding::hex
- hash
- hash::adler32
- hash::crc32
- math::bits
- time::time
  - limitations:
    - no time zone support
    - no duration support
    - no formatting support
    - no monothonic time support

## Development process

When a piece of functionality is needed, appropriate Go source code is copied from
[go1.20.6](https://github.com/golang/go/tree/go1.20.6/src) and translated to Rust.
Structures and functions are renamed to meet Rust naming conventions.

## License

The same license as used by Go project.  See [LICENSE](./LICENSE).
