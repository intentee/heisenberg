.DEFAULT_GOAL := target/release/heisenberg

CARGO_HOME ?= $(HOME)/.cargo
HEISENBERG_CRATES := \
	heisenberg \
	heisenberg_absolute_path \
	heisenberg_analysis_threads \
	heisenberg_command \
	heisenberg_error_context \
	heisenberg_graph_cycles \
	heisenberg_module_cycles \
	heisenberg_module_dependencies \
	heisenberg_module_graph \
	heisenberg_project_model \
	heisenberg_task_graph \
	heisenberg_workspace_build
HOST_TUPLE := $(shell rustc --print host-tuple)
RUST_SYSROOT := $(shell rustc --print sysroot)
SUPPORTED_TARGETS := \
	aarch64-apple-darwin \
	x86_64-pc-windows-msvc \
	x86_64-unknown-linux-gnu
TARGET_CFG_TABLES := \
	$(addprefix heisenberg_project_model/targets/,$(addsuffix /crates.cfg,$(SUPPORTED_TARGETS))) \
	$(addprefix heisenberg_project_model/targets/,$(addsuffix /library.cfg,$(SUPPORTED_TARGETS))) \
	$(addprefix heisenberg_project_model/targets/,$(addsuffix /target-spec.json,$(SUPPORTED_TARGETS)))
HEISENBERG_SOURCES := \
	$(wildcard $(addsuffix /src/*.rs,$(HEISENBERG_CRATES))) \
	$(addsuffix /Cargo.toml,$(HEISENBERG_CRATES)) \
	$(TARGET_CFG_TABLES) \
	Cargo.toml \
	Cargo.lock

HEISENBERG_CHECK := target/release/heisenberg check --cargo-home "$(CARGO_HOME)" --sysroot "$(RUST_SYSROOT)" --configuration $(HOST_TUPLE) --workspace

export HEISENBERG_CARGO_HOME := $(CARGO_HOME)
export HEISENBERG_SYSROOT := $(RUST_SYSROOT)

heisenberg_project_model/targets/%/crates.cfg: rust-toolchain.toml
	rustc --print cfg --target $* > $@

heisenberg_project_model/targets/%/library.cfg: export RUSTC_BOOTSTRAP := 1
heisenberg_project_model/targets/%/library.cfg: rust-toolchain.toml
	rustc --print cfg -O --target $* > $@

heisenberg_project_model/targets/%/target-spec.json: export RUSTC_BOOTSTRAP := 1
heisenberg_project_model/targets/%/target-spec.json: rust-toolchain.toml
	rustc -Z unstable-options --print target-spec-json --target $* > $@

node_modules/.package-lock.json: package.json package-lock.json
	npm ci

target/release/heisenberg: $(HEISENBERG_SOURCES)
	cargo build --release -p heisenberg

.PHONY: clean
clean:
	cargo clean

.PHONY: clippy
clippy: $(TARGET_CFG_TABLES)
	cargo clippy --workspace --all-targets -- --deny warnings

.PHONY: fetch
fetch:
	cargo fetch

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: heisenberg.check
heisenberg.check: fetch target/release/heisenberg
	$(HEISENBERG_CHECK) "$(CURDIR)"

.PHONY: test
test: fetch $(TARGET_CFG_TABLES)
	cargo nextest run --workspace

.PHONY: test.coverage
test.coverage: fetch node_modules/.package-lock.json $(TARGET_CFG_TABLES)
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest --no-report --workspace
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root "$(CURDIR)" \
		--gated heisenberg=100 \
		--gated heisenberg_absolute_path=100 \
		--gated heisenberg_analysis_threads=100 \
		--gated heisenberg_command=100 \
		--gated heisenberg_command_tests=100 \
		--gated heisenberg_error_context=100 \
		--gated heisenberg_graph_cycles=100 \
		--gated heisenberg_module_cycles=100 \
		--gated heisenberg_module_dependencies=100 \
		--gated heisenberg_module_graph=100 \
		--gated heisenberg_project_model=100 \
		--gated heisenberg_task_graph=100 \
		--gated heisenberg_workspace_build=100
