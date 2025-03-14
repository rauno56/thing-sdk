.SHELLFLAGS += -e -o pipefail

build-run: build run

build:
	cargo build --target wasm32-unknown-unknown
	cp ${CARGO_TARGET_DIR}/wasm32-wasip1/debug/thing_go.wasm ./sdk/src/module.wasm
	ls -lah ${CARGO_TARGET_DIR}/*/*/thing_go.wasm

build-release:
	cargo build --target wasm32-unknown-unknown --release
	cp ${CARGO_TARGET_DIR}/wasm32-wasip1/release/thing_go.wasm ./sdk/src/module.wasm
	ls -lah ${CARGO_TARGET_DIR}/*/*/thing_go.wasm

run:
	cd sdk && go run src/hello.go
