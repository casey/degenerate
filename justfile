bt := '0'

export RUST_BACKTRACE := bt

ci: check-lockfile test clippy fmt-check forbid

build:
	cargo build --release

test *args:
	cargo test --all --all-targets -- {{args}}

clippy:
	cargo clippy --package integration --tests
	cargo clippy --target wasm32-unknown-unknown

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

check:
	cargo check --target wasm32-unknown-unknown
	cargo check --package integration --tests

check-lockfile:
	cargo update --locked --package degenerate

forbid:
	./bin/forbid

watch +args='test --all --all-targets':
	cargo watch --clear --exec "{{args}}"

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
	rm -f integration/www/degenerate.js
	rm -f integration/www/degenerate_bg.wasm

doc-web:
	cargo doc --open --target wasm32-unknown-unknown

serve:
	cargo run --package serve

build-web:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/debug/degenerate.wasm --out-dir www

build-web-release:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/degenerate.wasm --out-dir www

open:
	open http://localhost:8000

fix:
	#!/usr/bin/env bash
	set -euxo pipefail
	for image in images/*.cpu.actual-memory.png; do
		mv $image ${image%.cpu.actual-memory.png}.png
	done
