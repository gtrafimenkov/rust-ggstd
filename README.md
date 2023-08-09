# Partial implementation of Go standard library in Rust

Go language has a very good [standard library](https://pkg.go.dev/std): simple,
extensive and well tested.  It would be great to have something like that in Rust.

## Usage examples

- flate: [examples/flate.rs](examples/flate.rs), [goexamples/flate/main.go](goexamples/flate/main.go)

You can run these using following commands:

```
cargo run --example flate
go run goexamples/flate/main.go
```

See all Rust examples in [examples](./examples) folder and
corresponding Go examples in [goexamples](./goexamples) folder.

## Partially implemented modules

- bufio
- builtin
- bytes
- compat
- compress::flate
- crypto::sha256
- encoding::binary
- encoding::hex
- errors
- hash::adler32
- hash::crc32
- io
- math::bits
- time
  - limitations:
    - no time zone support
    - no duration support
    - no formatting support
    - no monothonic time support

## Development process

When a piece of functionality is needed, appropriate Go source code is copied from
[go1.20.6](https://github.com/golang/go/tree/go1.20.6/src) and translated to Rust.
Structures and functions are renamed to meet Rust naming conventions.

## Gotchas

### Different priority of '<<' operator

Go: ` 1 << 1 + 1 == 3`
Rust: `1 << 1 + 1 == 4`

## License

The same license as used by Go project.  See [LICENSE](./LICENSE).
