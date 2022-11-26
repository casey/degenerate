set positional-arguments

bt := '0'

export RUST_BACKTRACE := bt

ci: check-lockfile test clippy fmt-check forbid

build:
  cargo build

test *args:
  cd tests && npx playwright test --project=chromium --retries=0 "$@"

clippy:
  cargo clippy --target wasm32-unknown-unknown

fmt:
  prettier --write tests features www/{worker.js,index.js}
  cargo fmt --all

fmt-check:
  prettier --check tests
  cargo fmt --all -- --check

check:
  cargo check --target wasm32-unknown-unknown

check-lockfile:
  cargo update --locked --package degenerate

forbid:
  ./bin/forbid

watch +args='test --all --all-targets':
  cargo watch --clear --exec "{{args}}"

build-manual:
  mdbook build man

clean:
  rm -f www/{degenerate,program}{.js,_bg.wasm}
  cargo clean

doc-web:
  cargo doc --open --target wasm32-unknown-unknown

serve:
  cargo run --package serve

build-web:
  cargo build --release --target wasm32-unknown-unknown
  wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/degenerate.wasm --out-dir www

open:
  open http://localhost

update-test-images:
  #!/usr/bin/env bash
  set -euo pipefail
  for image in images/*.actual-memory.png; do
    mv $image ${image%.actual-memory.png}.png
  done

update-glmatrix:
  curl https://raw.githubusercontent.com/toji/gl-matrix/v3.4.1/dist/gl-matrix-min.js > www/gl-matrix-min.js

diff name:
  -compare images/{{name}}.png images/{{name}}.actual-memory.png diff.png
  open diff.png

rustc:
 rustc \
 --edition=2021 \
 --crate-type bin \
 --emit=dep-info,link \
 -C opt-level=3 \
 -C embed-bitcode=no \
 --out-dir tmp \
 --target wasm32-unknown-unknown \
 -L dependency=/Users/rodarmor/src/degenerate/target/wasm32-unknown-unknown/release/deps \
 -L dependency=/Users/rodarmor/src/degenerate/target/release/deps \
 --extern degenerate=/Users/rodarmor/src/degenerate/target/wasm32-unknown-unknown/release/deps/libdegenerate-5151a704cd5b835e.rlib \
 program/src/main.rs
