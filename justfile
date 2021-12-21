build:
	cargo build

test *args:
	cargo test -- {{args}}

clippy:
  cargo clippy --all-targets --all-features

run *args:
	cargo run -- {{args}}

image *args:
	cargo run -- --output output.png {{args}}
	open output.png

fmt:
	cargo fmt

check:
 cargo check

forbid:
	./bin/forbid

watch +COMMAND='ltest':
	cargo watch --clear --exec "{{COMMAND}}"
