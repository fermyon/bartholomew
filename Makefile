WAGI ?= wagi

.PHONY: build
build:
	cargo build --target wasm32-wasi --release

.PHONY: serve
serve: build
serve:
	$(WAGI) -c ./modules.toml --log-dir ./logs