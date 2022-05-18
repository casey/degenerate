bt := '0'

export RUST_BACKTRACE := bt

ci: check-lockfile test clippy fmt-check forbid

build:
	cargo build --release

test *args:
	cargo test --package integration --lib -- {{args}}

clippy:
	cargo clippy --package integration --tests
	cargo clippy --target wasm32-unknown-unknown

run *args:
	cargo run --release -- {{args}}

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

watch +args='ltest --release':
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

build-manual:
	mdbook build man

# publish current GitHub master branch
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:casey/degenerate.git tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cd tmp/release
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release

clean:
	cargo clean
	rm -f www/degenerate.js
	rm -f www/degenerate_bg.wasm

doc-web:
	cargo doc --open --target wasm32-unknown-unknown

serve:
	python3 -m http.server --directory www --bind 0.0.0.0

build-web:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/debug/degenerate.wasm --out-dir www

build-web-release:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/degenerate.wasm --out-dir www

open:
	open http://localhost:8000
