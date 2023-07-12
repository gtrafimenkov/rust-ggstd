# Partial implementation of Go standard library in Rust

Go language has a very good [standard library](https://pkg.go.dev/std): simple,
extensive and well tested.  It would be great to have something like that in Rust.

## Partially implemented modules

- crypto::sha256
- encoding::binary
- encoding::hex
- hash
- math::bits

## Development process

When a piece of functionality is needed, appropriate Go source code is copied from
[go1.20.6](https://github.com/golang/go/tree/go1.20.6/src) and translated to Rust.
Structures and functions are renamed to meet Rust naming conventions.

## License

The same license as used by Go project.  See [LICENSE](./LICENSE).
