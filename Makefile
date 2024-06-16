all: build

build:
	cargo build

release:
	cargo build --release

dev:
	cargo run -- $(ARGS)

run:
	cargo run --release -- $(ARGS)

lint:
	cargo clippy

clean:
	cargo clean

.PHONY: all clean