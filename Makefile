build:
	cargo build --release

build-wasm:
	cargo build --target wasm32-unknown-unknown --release

cp-wasm:
	cp target/wasm32-unknown-unknown/release/conways_gol.wasm www/

server:
	basic-http-server www

all: build

wasm: build-wasm cp-wasm server

test: check-deps
	cargo test --workspace --all-targets --all-features
