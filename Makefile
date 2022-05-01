.PHONY: all
all: help

.PHONY: build # cargo build
build:
	RUSTFLAGS="-C link-arg=-s" cargo build --release

.PHONY: fmt # cargo fmt --all
fmt:
	cargo fmt --all

.PHONY: check # cargo clippy --all
check:
	cargo clippy --all

.PHONY: test # cargo test
test:
	cargo test

.PHONY: help # Generate list of targets with descriptions
help:
	@grep '^.PHONY: .* #' Makefile | sed 's/\.PHONY: \(.*\) # \(.*\)/\1	\2/' | expand -t20