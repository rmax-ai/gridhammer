# GridHammer Top-Level Makefile

.PHONY: test test-core test-shell lint check clean

# Default target
test: test-core test-shell

# Rust Core
test-core:
	@echo "=== Testing Rust Core ==="
	cd rust-core && cargo fmt -- --check
	cd rust-core && cargo clippy -- -D warnings
	cd rust-core && cargo test

# TypeScript Shell
test-shell:
	@echo "=== Testing TypeScript Shell ==="
	cd ts-shell && npm test
	cd ts-shell && npm run lint

# Lint all
lint:
	cd rust-core && cargo fmt -- --check && cargo clippy -- -D warnings
	cd ts-shell && npm run lint

# Clean all
clean:
	cd rust-core && cargo clean
	cd ts-shell && rm -rf dist node_modules
