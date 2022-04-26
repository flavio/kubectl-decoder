build:
	cargo build --target wasm32-wasi --release
	cp target/wasm32-wasi/release/kubectl-decoder.wasm decoder.wasm
