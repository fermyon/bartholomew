SPIN ?= spin
# If PREVIEW_MODE is on then unpublished content will be displayed.
PREVIEW_MODE ?= 0
SHOW_DEBUG ?= 1
BASE_URL ?= http://localhost:3000

.PHONY: build
build:
	cargo build --target wasm32-wasi --release

.PHONY: build-release
build-release: build
	wasm-opt -O target/wasm32-wasi/release/bartholomew.wasm -o target/wasm32-wasi/release/bartholomew.wasm
	# Keep an eye on the binary size. We want it under 5M
	@ls -lah target/wasm32-wasi/release/*.wasm

.PHONY: bart
bart:
	cargo build --release --manifest-path=bart/Cargo.toml

.PHONY: test
test:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

.PHONY: serve
serve: build
serve:
	$(SPIN) up --log-dir ./logs -e PREVIEW_MODE=$(PREVIEW_MODE) -e SHOW_DEBUG=$(SHOW_DEBUG) -e BASE_URL=$(BASE_URL)

.PHONY: run
run: serve
