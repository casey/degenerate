bt := '0'

export RUST_BACKTRACE := bt

ci: check-lockfile (test '--include-ignored') (test-all-features '--include-ignored') clippy fmt-check forbid

build:
	cargo build --release

test *args:
	cargo test -- {{args}}

test-all-features *args:
	cargo test --all-features -- {{args}}

clippy:
  cargo clippy --all-targets --all-features

run *args:
	cargo run --release -- {{args}}

fmt:
	yapf --in-place --recursive .
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

build-pages:
	cargo run --package build-pages

rename-image-tests:
	#!/usr/bin/env bash
	set -euo pipefail

	cd images

	for ((i=48; i<=61; i+=1)); do
		echo $i:
		open $i.png
		cat $i.degen
		read -p "name: " name
		if [[ -e $name.degen ]]; then
			echo $name.degen already exists
			break
		fi
		mv $i.degen $name.degen
		mv $i.png $name.png
	done
