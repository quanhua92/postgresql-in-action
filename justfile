set dotenv-required

set dotenv-load := true

install:
    cargo install cargo-watch

build:
    cargo build

run:
    cargo run

dev:
    RUST_LOG=debug cargo watch -w src -w tests -w queries -s "cargo run"

test:
    cargo watch -d 1 -w src -w tests -w queries -x test

test-one NAME:
    RUST_LOG=debug cargo watch -d 1 -w src -w tests -w queries -s "cargo test {{NAME}}"
