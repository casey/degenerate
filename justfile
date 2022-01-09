bt := '0'

export RUST_BACKTRACE := bt

all: check-lockfile test clippy fmt-check forbid

build:
	cargo build

test *args:
	cargo test -- {{args}}

clippy:
  cargo clippy --all-targets --all-features

run *args:
	cargo run --release -- {{args}}

image *args:
	cargo run -- --output output.png {{args}}
	open output.png

fmt:
	cargo fmt

fmt-check:
	cargo fmt --all -- --check

check:
 cargo check

check-lockfile:
	cargo update --locked --package degenerate

forbid:
	./bin/forbid

watch +args='ltest':
	cargo watch --ignore README.md --clear --exec "{{args}}"

generate:
	#!/usr/bin/env bash
	set -eou pipefail

	cargo build --release

	rm -rf generate
	mkdir generate
	for i in {0..9}; do
		echo Generating image $i...
		target/release/degenerate resize:512 seed:$i generate save:generate/$i.png
	done
