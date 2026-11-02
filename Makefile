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
	heisenberg_project_model
RUST_SYSROOT := $(shell rustc --print sysroot)
SUPPORTED_TARGETS := \
	aarch64-apple-darwin \
	x86_64-pc-windows-msvc \
	x86_64-unknown-linux-gnu
TARGET_CFG_TABLES := \
	$(addprefix heisenberg_project_model/targets/,$(addsuffix /crates.cfg,$(SUPPORTED_TARGETS))) \
	$(addprefix heisenberg_project_model/targets/,$(addsuffix /library.cfg,$(SUPPORTED_TARGETS)))
HEISENBERG_SOURCES := \
	$(wildcard $(addsuffix /src/*.rs,$(HEISENBERG_CRATES))) \
	$(addsuffix /Cargo.toml,$(HEISENBERG_CRATES)) \
	$(TARGET_CFG_TABLES) \
	Cargo.toml \
	Cargo.lock

HEISENBERG_CHECK := target/release/heisenberg check --cargo-home $(subst \,/,$(CARGO_HOME)) --sysroot $(subst \,/,$(RUST_SYSROOT)) --workspace

export HEISENBERG_CARGO_HOME := $(CARGO_HOME)
export HEISENBERG_SYSROOT := $(RUST_SYSROOT)

heisenberg_project_model/targets/%/crates.cfg: rust-toolchain.toml
	rustc --print cfg --target $* > $@

heisenberg_project_model/targets/%/library.cfg: export RUSTC_BOOTSTRAP := 1
heisenberg_project_model/targets/%/library.cfg: rust-toolchain.toml
	rustc --print cfg -O --target $* > $@

node_modules/.package-lock.json: package.json package-lock.json
	npm ci

target/release/heisenberg: $(HEISENBERG_SOURCES)
	cargo build --release -p heisenberg

.PHONY: benchmark
benchmark: fetch target/release/heisenberg
	hyperfine --shell=none --warmup 1 --ignore-failure=1 --export-markdown target/benchmark.md \
		--command-name acyclic_modules "$(HEISENBERG_CHECK) $(CURDIR)/heisenberg_fixtures/workspaces/acyclic_modules" \
		--command-name cycles_in_a_dependency_diamond "$(HEISENBERG_CHECK) $(CURDIR)/heisenberg_fixtures/workspaces/cycles_in_a_dependency_diamond" \
		--command-name heisenberg "$(HEISENBERG_CHECK) $(CURDIR)"

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
	$(HEISENBERG_CHECK) $(CURDIR)

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
		--workspace-root $(CURDIR) \
		--gated heisenberg=100 \
		--gated heisenberg_absolute_path=100 \
		--gated heisenberg_analysis_threads=100 \
		--gated heisenberg_command=100 \
		--gated heisenberg_command_tests=100 \
		--gated heisenberg_error_context=100 \
		--gated heisenberg_fixtures=100 \
		--gated heisenberg_graph_cycles=100 \
		--gated heisenberg_module_cycles=100 \
		--gated heisenberg_module_dependencies=100 \
		--gated heisenberg_module_graph=100 \
		--gated heisenberg_project_model=100 \
		--gated heisenberg_test_harness=100
