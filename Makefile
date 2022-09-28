all: build-optimized test

CARGO_TEST_SUBCOMMAND:=$(shell type -p cargo-nextest >/dev/null && echo nextest run || echo test)

test: build
	cargo hack --feature-powerset --exclude-features docs $(CARGO_TEST_SUBCOMMAND)

build:
	cargo hack build --target wasm32-unknown-unknown --release \
		--workspace \
		--exclude tx

build-optimized:
	cargo +nightly hack build --target wasm32-unknown-unknown --release \
		--workspace \
		--exclude tx \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort
	cd target-tiny/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

clean:
	cargo clean
