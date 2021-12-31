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
	cargo watch --clear --exec "{{args}}"

gallery:
	#!/usr/bin/env bash
	set -euxo pipefail
	rm -rf gallery
	mkdir gallery
	cd gallery
	cargo run --release resize:4096:4096 square top save:logo.png
	cargo run --release resize:8192:8192 x save:x.png
	cargo run --release resize:1024:1024 rotate:0.111 for:16 square circle loop save:grain.png
	cargo run --release resize:4096:4096 seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-filter loop rotate-color:b:0.01 rotate:0.01 for:100 random-filter loop save:smear.png
	cargo run --release resize:4096:4096 rotate-color:g:0.07 rotate:0.07 for:10 x loop rotate-color:b:0.09 rotate:0.09 for:10 x loop save:brilliance.png
	cargo run --release resize:4096:4096 seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-filter loop rotate-color:b:0.1 rotate:0.1 for:10 random-filter loop save:starburst.png
	cargo run --release resize:4096:4096 rotate-color:red:0.083333 rotate:0.1 for:12 circle cross x loop save:compass.png
	cargo run --release resize:4096:4096 scale:0.99 for:100 circle loop save:singularity.png
