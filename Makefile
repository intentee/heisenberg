.DEFAULT_GOAL := target/release/heisenberg

CARGO_HOME ?= $(HOME)/.cargo
COVERAGE_DIVERGENT_MACRO_FIXTURES := \
	coverage_attribute_failing_in_rust_analyzer \
	coverage_attribute_hidden_from_rust_analyzer \
	coverage_attribute_hidden_in_test_code \
	coverage_attribute_unparsable_in_rust_analyzer \
	coverage_macro_failing_in_rust_analyzer \
	coverage_macro_hidden_from_rust_analyzer \
	coverage_macro_unparsable_in_rust_analyzer
COVERAGE_IDENTITY_MACRO_FIXTURES := \
	coverage_attribute_hidden_from_rust_analyzer \
	coverage_attribute_hidden_in_test_code \
	coverage_attribute_macro_item
COVERAGE_FIXTURES := $(sort $(COVERAGE_DIVERGENT_MACRO_FIXTURES) $(COVERAGE_IDENTITY_MACRO_FIXTURES) \
	coverage_code_outside_functions \
	coverage_fully_covered \
	coverage_inactive_module_file \
	coverage_inactive_region \
	coverage_macro_in_an_inactive_attribute \
	coverage_macro_text_shared_by_functions \
	coverage_macro_written_function \
	coverage_path_module \
	coverage_production_only_attribute \
	coverage_production_only_code \
	coverage_shared_module_file \
	coverage_uncovered_code \
	coverage_unexecuted_example \
	coverage_unexecuted_inline_test_module \
	coverage_unexecuted_integration_test \
	coverage_unexecuted_test_crate \
	coverage_unexecuted_test_module_file \
	coverage_without_coverage_data)
COVERAGE_FIXTURE_DIRECTORY := heisenberg_command_tests/workspaces
COVERAGE_DIVERGENT_MACRO_EXPORTS := $(addprefix $(COVERAGE_FIXTURE_DIRECTORY)/,$(addsuffix /target/llvm-cov.json,$(COVERAGE_DIVERGENT_MACRO_FIXTURES)))
COVERAGE_FIXTURE_EXPORTS := $(addprefix $(COVERAGE_FIXTURE_DIRECTORY)/,$(addsuffix /target/llvm-cov.json,$(COVERAGE_FIXTURES)))
COVERAGE_IDENTITY_MACRO_EXPORTS := $(addprefix $(COVERAGE_FIXTURE_DIRECTORY)/,$(addsuffix /target/llvm-cov.json,$(COVERAGE_IDENTITY_MACRO_FIXTURES)))
HEISENBERG_CRATES := \
	heisenberg \
	heisenberg_absolute_path \
	heisenberg_analysis_threads \
	heisenberg_attached_work \
	heisenberg_command \
	heisenberg_configured_code \
	heisenberg_coverage_export \
	heisenberg_coverage_measurement \
	heisenberg_expanded_code \
	heisenberg_graph_cycles \
	heisenberg_member_crates \
	heisenberg_module_cycles \
	heisenberg_module_dependencies \
	heisenberg_module_graph \
	heisenberg_project_model \
	heisenberg_source_locator \
	heisenberg_task_graph \
	heisenberg_workspace_build
HOST_TUPLE := $(shell rustc --print host-tuple)
IDENTITY_MARKER_MACROS := $(COVERAGE_FIXTURE_DIRECTORY)/identity_marker_macros
RUST_ANALYZER_DIVERGENT_MACROS := $(COVERAGE_FIXTURE_DIRECTORY)/rust_analyzer_divergent_macros
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

HEISENBERG_ARGUMENTS := --cargo-home "$(CARGO_HOME)" --sysroot "$(RUST_SYSROOT)" --configuration $(HOST_TUPLE) --workspace "$(CURDIR)"

export HEISENBERG_CARGO_HOME := $(CARGO_HOME)
export HEISENBERG_SYSROOT := $(RUST_SYSROOT)

.SECONDEXPANSION:

$(COVERAGE_FIXTURE_DIRECTORY)/%/target/llvm-cov.json: $(COVERAGE_FIXTURE_DIRECTORY)/%/Cargo.toml $(COVERAGE_FIXTURE_DIRECTORY)/%/Cargo.lock rust-toolchain.toml $$(wildcard $(COVERAGE_FIXTURE_DIRECTORY)/$$*/*.rs $(COVERAGE_FIXTURE_DIRECTORY)/$$*/*/*.rs $(COVERAGE_FIXTURE_DIRECTORY)/$$*/*/*/*.rs $(COVERAGE_FIXTURE_DIRECTORY)/$$*/*/Cargo.toml)
	cargo llvm-cov clean --manifest-path $< --workspace
	cargo llvm-cov test --manifest-path $< --no-report --workspace --lib --bins --tests --examples
	cargo llvm-cov report --manifest-path $< --json --include-build-script --output-path $@

$(COVERAGE_DIVERGENT_MACRO_EXPORTS): $(RUST_ANALYZER_DIVERGENT_MACROS)/Cargo.toml $(RUST_ANALYZER_DIVERGENT_MACROS)/src/lib.rs

$(COVERAGE_IDENTITY_MACRO_EXPORTS): $(IDENTITY_MARKER_MACROS)/Cargo.toml $(IDENTITY_MARKER_MACROS)/src/lib.rs

heisenberg_project_model/targets/%/crates.cfg: rust-toolchain.toml
	rustc --print cfg --target $* > $@

heisenberg_project_model/targets/%/library.cfg: export RUSTC_BOOTSTRAP := 1
heisenberg_project_model/targets/%/library.cfg: rust-toolchain.toml
	rustc --print cfg -O --target $* > $@

heisenberg_project_model/targets/%/target-spec.json: export RUSTC_BOOTSTRAP := 1
heisenberg_project_model/targets/%/target-spec.json: rust-toolchain.toml
	rustc -Z unstable-options --print target-spec-json --target $* > $@

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
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/benchmark_in_a_package_with_a_library/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/benchmark_target_excluded_from_benchmarks/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/benchmark_target_run_by_tests/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/benchmarks_package_without_libraries/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/dependency_requirement_of_a_root_package/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/dependency_requirements/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/error_messages_and_modules/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/error_types/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/error_types_in_test_code/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/rules_relaxed_in_test_code/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/every_target_kind/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/function_used_only_by_benchmarks/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/must_use_benchmark_output/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/non_benchmark_item_in_benchmark_target/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/tests_and_benchmarks_package_without_libraries/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/undetermined_benchmark_output/Cargo.toml
	cargo fetch --locked --manifest-path $(COVERAGE_FIXTURE_DIRECTORY)/unvetted_criterion_method/Cargo.toml

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: heisenberg.check
heisenberg.check: fetch target/release/heisenberg
	target/release/heisenberg check $(HEISENBERG_ARGUMENTS)

.PHONY: test
test: fetch $(COVERAGE_FIXTURE_EXPORTS) $(TARGET_CFG_TABLES)
	cargo nextest run --workspace

.PHONY: test.coverage
test.coverage: fetch target/release/heisenberg $(COVERAGE_FIXTURE_EXPORTS) $(TARGET_CFG_TABLES)
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest --no-report --workspace --lib --bins --tests --examples
	cargo llvm-cov report --json --include-build-script --output-path target/llvm-cov.json
	cargo llvm-cov report --include-build-script
	target/release/heisenberg coverage-check $(HEISENBERG_ARGUMENTS) --coverage-export "$(CURDIR)/target/llvm-cov.json"
