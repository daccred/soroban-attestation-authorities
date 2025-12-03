# Makefile for airdrop resolver contract
CRATE := airdrop
TARGET ?= wasm32v1-none
PROFILE ?= release
DIST := dist
# Workspace-level target directory (crate name uses underscores)
WASM := ../target/$(TARGET)/$(PROFILE)/$(CRATE).wasm

.PHONY: help test clean build deploy bindings

help:
	@echo "Available targets:"
	@echo "  test                 - Run tests"
	@echo "  clean                - Clean build artifacts"
	@echo "  build                - Build WASM contract"
	@echo "  deploy               - Deploy contract (requires IDENTITY, NETWORK)"
	@echo "  bindings             - Generate TypeScript bindings (requires CONTRACT_ID)"

test:
	cargo test -q

clean:
	cargo clean
	@rm -rf $(DIST)

$(DIST):
	@mkdir -p $(DIST)

build: $(DIST)
	cargo build --target $(TARGET) --release
	cp "$(WASM)" "$(DIST)/$(CRATE).wasm"
	@echo "Built: $(DIST)/$(CRATE).wasm"

deploy: build
	@if [ -z "$(IDENTITY)" ]; then echo "Set IDENTITY=<your_identity>"; exit 1; fi
	@if [ -z "$(NETWORK)" ]; then echo "Set NETWORK=<testnet|mainnet>"; exit 1; fi
	stellar contract deploy \
		--wasm "$(DIST)/$(CRATE).wasm" \
		--source "$(IDENTITY)" \
		--network "$(NETWORK)"

bindings:
	@if [ -z "$(CONTRACT_ID)" ]; then echo "Set CONTRACT_ID=<deployed_contract_id>"; exit 1; fi
	@if [ -z "$(NETWORK)" ]; then echo "Set NETWORK=<testnet|mainnet>"; exit 1; fi
	@mkdir -p bindings
	stellar contract bindings typescript \
		--contract-id "$(CONTRACT_ID)" \
		--network "$(NETWORK)" \
		--output-dir bindings
	@echo "Generated TypeScript bindings in bindings/"
