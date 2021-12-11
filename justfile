build:
	cargo build

test:
	cargo test

clippy:
  cargo clippy --all-targets --all-features

run *args:
	cargo run -- {{args}}

fmt:
	cargo fmt

check:
 cargo check

watch +COMMAND='ltest':
	cargo watch --clear --exec "{{COMMAND}}"
