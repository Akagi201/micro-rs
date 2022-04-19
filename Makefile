.PHONY: all
all: help

.PHONY: build # cargo build
build:
	cargo build --release

.PHONY: test # cargo test
test:
	cargo test

.PHONY: help # Generate list of targets with descriptions
help:
	@grep '^.PHONY: .* #' Makefile | sed 's/\.PHONY: \(.*\) # \(.*\)/\1	\2/' | expand -t20