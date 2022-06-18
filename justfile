bt := '0'

export RUST_BACKTRACE := bt

ci: check-lockfile test clippy fmt-check forbid

build:
  cargo build

test *args:
  cd tests && npx playwright test {{args}}

clippy:
  cargo clippy --target wasm32-unknown-unknown

fmt:
  prettier --write tests
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
  rm www/degenerate{.js,_bg.wasm}
  cargo clean

doc-web:
  cargo doc --open --target wasm32-unknown-unknown

serve:
  cargo run --package serve

build-web:
  cargo --release build --target wasm32-unknown-unknown
  wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/degenerate.wasm --out-dir www

open:
  open http://localhost:8000

update-test-images:
  #!/usr/bin/env bash
  set -euo pipefail
  for image in images/*.actual-memory.png; do
    mv $image ${image%.actual-memory.png}.png
  done

diff name:
  -compare images/{{name}}.png images/{{name}}.actual-memory.png diff.png
  open diff.png
