default:
	cargo build
	cargo build --release

test:
	cargo test --release
	cargo test

test-flate-speed:
	@echo
	@echo "== Go =="
	time go run goexamples/flate-speed/main.go
	@echo
	@echo "== Rust =="
	time cargo run --example flate-speed --release

print-partially-implemented-modules:
	find src -name '*.rs' -exec dirname '{}' \; | sort | uniq | sed 's@src/@- @g' | sed 's@/@::@' | grep -v compat
