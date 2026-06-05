.PHONY: test-fast lean-build

test-fast:
	cargo test --workspace --test discriminating_benchmarks

lean-build:
	cd GPD/formal && lake build
