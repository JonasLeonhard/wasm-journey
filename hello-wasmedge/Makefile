build:
	cargo build --target wasm32-wasi

wasmedge:
	wasmedge ./target/wasm32-wasi/debug/hello-wasmedge.wasm --test --another-cli-arg
