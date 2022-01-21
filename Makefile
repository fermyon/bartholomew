WAGI ?= wagi
# If PREVIEW_MODE is on then unpublished content will be displayed.
PREVIEW_MODE ?= 0
SHOW_DEBUG ?= 1
DISABLE_CACHE ?= 1

.PHONY: build
build:
	cargo build --target wasm32-wasi --release
	# Keep an eye on the binary size. We want it under 5M
	@ls -lah target/wasm32-wasi/release/*.wasm

.PHONY: serve
serve: build
serve:
	$(WAGI) -c ./modules.toml --log-dir ./logs -e PREVIEW_MODE=$(PREVIEW_MODE) -e SHOW_DEBUG=$(SHOW_DEBUG) -e DISABLE_CACHE=$(DISABLE_CACHE)

.PHONY: run
run: serve

# Quick ergonomic to create an ISO 8601 date on Mac or on Linux (Gnu date)
.PHONY: date
date:
	date -u +"%Y-%m-%dT%H:%M:%SZ"

.PHONY: clean
clean:
	rm -rf config/_cache.json