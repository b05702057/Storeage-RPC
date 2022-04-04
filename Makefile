
build:
	cargo build

test:
	cargo test

lab1_test:
	cargo test --test lab1_test

format:
	cargo fmt --all

# requires clippy linter
# $ rustup component add clippy
lint:
	cargo clippy --all


check:
	cargo check --all
