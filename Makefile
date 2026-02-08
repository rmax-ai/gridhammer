# GridHammer Top-Level Makefile

.PHONY: test test-core test-shell lint check clean

# Default target
test: test-core test-shell

# Rust Core
test-core:
	@echo "=== Testing Rust Workspace ==="
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cargo test --workspace

# TypeScript Shell
test-shell:
	@echo "=== Testing TypeScript Shell ==="
	cd ts-shell && npm test
	cd ts-shell && npm run lint

# Lint all
lint:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cd ts-shell && npm run lint

# Clean all
clean:
	cargo clean
	cd ts-shell && rm -rf dist node_modules
