OUT_PATH=out
STATIC_PATH=static
RUST_PATH=rust
RUST_OUT_PATH=$(RUST_PATH)/out
CARGO_CHANNEL=+nightly
CARGO_MANIFEST_PATH=--manifest-path $(RUST_PATH)/Cargo.toml
CARGO_OUT_DIR=-Z unstable-options --out-dir=$(RUST_OUT_PATH)

all: out-static
	cargo $(CARGO_CHANNEL) build $(CARGO_MANIFEST_PATH) \
		$(CARGO_OUT_DIR) \
		--target wasm32-unknown-unknown
	wasm-bindgen $(RUST_OUT_PATH)/calories_calc.wasm \
		--no-modules --out-dir $(OUT_PATH)

release: out-static
	cargo $(CARGO_CHANNEL) build $(CARGO_MANIFEST_PATH) \
		--release \
		$(CARGO_OUT_DIR) \
		--target wasm32-unknown-unknown
	wasm-bindgen $(RUST_OUT_PATH)/calories_calc.wasm \
		--no-modules --out-dir $(OUT_PATH)

out-static:
	mkdir -p out
	cp $(STATIC_PATH)/* $(OUT_PATH)/

test:
	cargo $(CARGO_CHANNEL) test $(CARGO_MANIFEST_PATH)

fmt:
	cd $(RUST_PATH) && \
		cargo $(CARGO_CHANNEL) fmt
