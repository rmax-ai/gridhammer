.PHONY: test lint clean

test:
	cargo test

lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

clean:
	cargo clean
