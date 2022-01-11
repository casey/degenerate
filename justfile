bt := '0'

export RUST_BACKTRACE := bt

all: check-lockfile test clippy fmt-check forbid

build:
	cargo build --release

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

generate: build
	#!/usr/bin/env bash
	set -eou pipefail

	rm -rf generate
	mkdir generate
	for i in {0..9}; do
		echo Generating image $i...
		target/release/degenerate resize:512 seed:$i generate save:generate/$i.png
	done

gallery: build
	#!/usr/bin/env bash
	set -eou pipefail

	rm -rf gallery
	mkdir gallery
	for IMAGE in images/*; do
		FILENAME=`basename -- "$IMAGE"`
		PROGRAM=${FILENAME%.*}
		PROGRAM=`echo $PROGRAM | sed 's/resize:[^ ]*/resize:4096/'`
		if [[ $PROGRAM != *autosave* ]]; then
			PROGRAM="autosave $PROGRAM"
		fi
		echo Generating $PROGRAM...
		target/release/degenerate $PROGRAM
		mv output.png "gallery/$PROGRAM.png"
		mkdir -p "gallery/$PROGRAM"
		mv *.png "gallery/$PROGRAM"
	done