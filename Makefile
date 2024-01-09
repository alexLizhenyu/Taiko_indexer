ENDPOINT_URL ?= https://goerli.infura.io/v3/1679a097da9642928f62f5d6ea6a1869
START_BLOCK ?= 10258640
HOST ?= 0.0.0.0
PORT ?= 8010

.PHONY: build
build:
	cargo build --release

.PHONY: run
run:
	cargo run -- all \
		--endpoint-url $(ENDPOINT_URL) \
		--start-block $(START_BLOCK) \
		--host $(HOST) \
		--port $(PORT)

.PHONY: sync
sync:
	cargo run -- sync \
		--endpoint-url $(ENDPOINT_URL) \
		--start-block $(START_BLOCK)

.PHONY: serve
serve:
	cargo run -- serve \
		--host $(HOST) \
		--port $(PORT)
