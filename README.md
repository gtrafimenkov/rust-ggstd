# Parts of Go standard library implemented in Rust

Go language has the great [standard library](https://pkg.go.dev/std): extensive and well tested.
It would be great to have something like that in Rust.

## Development process

When a piece of functionality is needed, appropriate Go source code is copied from
https://github.com/golang/go/tree/go1.20.5/src and translated to Rust.  Structures and functions
are renamed to meet Rust naming conventions.

## License

The same license as used by Go project since the code is derived from it.  See [LINCESE](./LICENSE).
