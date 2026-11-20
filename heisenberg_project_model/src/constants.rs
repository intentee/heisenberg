use std::time::Duration;

use cargo_metadata::semver::Version;

use crate::harness_methods::HarnessMethods;
use crate::lint_tool::LintTool;
use crate::required_clippy_setting::RequiredClippySetting;
use crate::required_lint::RequiredLint;
use crate::supported_target::SupportedTarget;

pub const SUPPORTED_RUST_VERSION: Version = Version::new(1, 100, 0);

pub const VETTED_RUSTFMT_SETTINGS: [&str; 28] = [
    "array_width",
    "attr_fn_like_width",
    "chain_width",
    "edition",
    "fn_args_layout",
    "fn_call_width",
    "fn_params_layout",
    "force_explicit_abi",
    "hard_tabs",
    "hex_literal_case",
    "match_arm_leading_pipes",
    "match_block_trailing_comma",
    "max_width",
    "merge_derives",
    "newline_style",
    "remove_nested_parens",
    "reorder_imports",
    "reorder_modules",
    "short_array_element_width_threshold",
    "single_line_if_else_max_width",
    "single_line_let_else_max_width",
    "struct_lit_width",
    "struct_variant_width",
    "style_edition",
    "tab_spaces",
    "use_field_init_shorthand",
    "use_small_heuristics",
    "use_try_shorthand",
];

pub const SUPPORTED_TARGETS: [SupportedTarget; 3] = [
    SupportedTarget::AArch64AppleDarwin,
    SupportedTarget::X86_64PcWindowsMsvc,
    SupportedTarget::X86_64UnknownLinuxGnu,
];

pub const BENCHMARK_HARNESS_PACKAGE: &str = "criterion";

pub const BENCHMARK_LOOP_OUTPUT_PARAMETER: &str = "O";

pub const BENCHMARK_REGISTRATIONS: [HarnessMethods; 2] = [
    HarnessMethods {
        methods: &["bench_function", "bench_with_input"],
        type_name: "BenchmarkGroup",
    },
    HarnessMethods {
        methods: &["bench_function", "bench_with_input"],
        type_name: "Criterion",
    },
];

pub const CARGO_BUILD_JOBS_VARIABLE: &str = "CARGO_BUILD_JOBS";

pub const CARGO_BUILD_TABLE: &str = "build";

pub const CARGO_ENVIRONMENT_TABLE: &str = "env";

pub const CARGO_MANIFEST_FILE: &str = "Cargo.toml";

pub const CARGO_OUT_DIR_VARIABLE: &str = "OUT_DIR";

pub const CARGO_PATCH_TABLE: &str = "patch";

pub const CARGO_RUSTC: &str = "rustc";

pub const CARGO_RUSTDOC: &str = "rustdoc";

pub const CARGO_RUSTC_WORKSPACE_WRAPPER: &str = "rustc-workspace-wrapper";

pub const CARGO_RUSTC_WRAPPER: &str = "rustc-wrapper";

pub const CARGO_RUSTFLAGS: &str = "rustflags";

pub const CARGO_TARGET_TABLE: &str = "target";

pub const CLIPPY_ARGUMENTS_VARIABLE: &str = "CLIPPY_ARGS";

pub const CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE: &str = "CLIPPY_CONF_DIR";

pub const CLIPPY_CONFIGURATION_FILE: &str = "clippy.toml";

pub const CLIPPY_HIDDEN_CONFIGURATION_FILE: &str = ".clippy.toml";

pub const CLIPPY_RESTRICTION_GROUP: &str = "restriction";

pub const COMPILER_VARIABLES: [&str; 3] =
    [RUSTC_VARIABLE, "RUSTC_WORKSPACE_WRAPPER", "RUSTC_WRAPPER"];

pub const ERROR_DERIVE_PACKAGE: &str = "thiserror-impl";

pub const ERROR_RUNTIME_PACKAGE: &str = "thiserror";

pub const HEISENBERG_SETTINGS_KEY: &str = "heisenberg";

pub const LINT_NAME_SEPARATOR: &str = "_";

pub const LINT_NAME_SEPARATOR_ALIAS: &str = "-";

pub const MAXIMUM_SLOW_TEST_PERIOD: Duration = Duration::from_secs(10);

pub const MAXIMUM_TEST_DURATION: Duration = Duration::from_secs(20);

pub const NEXTEST_CONFIGURATION_DIRECTORY: &str = ".config";

pub const NEXTEST_CONFIGURATION_FILE: &str = "nextest.toml";

pub const NEXTEST_DEFAULT_PROFILE: &str = "default";

pub const NEXTEST_RETRIES_VARIABLE: &str = "NEXTEST_RETRIES";

pub const NIGHTLY_BOOTSTRAP_VARIABLE: &str = "RUSTC_BOOTSTRAP";

pub const NIGHTLY_FEATURE_VARIABLES: [&str; 2] = [
    NIGHTLY_BOOTSTRAP_VARIABLE,
    "__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS",
];

pub const OUTPUT_DROPPING_BENCHMARK_LOOPS: [HarnessMethods; 2] = [
    HarnessMethods {
        methods: &[
            "iter",
            "iter_batched",
            "iter_batched_ref",
            "iter_with_large_drop",
        ],
        type_name: "AsyncBencher",
    },
    HarnessMethods {
        methods: &[
            "iter",
            "iter_batched",
            "iter_batched_ref",
            "iter_with_large_drop",
        ],
        type_name: "Bencher",
    },
];

pub const PANICKING_LINTS: [RequiredLint; 10] = [
    RequiredLint {
        former_names: &["integer_arithmetic"],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "arithmetic_side_effects",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &["option_expect_used", "result_expect_used"],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "expect_used",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "indexing_slicing",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "panic",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "panic_in_result_fn",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "string_slice",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "todo",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "unimplemented",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "unreachable",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &["option_unwrap_used", "result_unwrap_used"],
        groups: &[CLIPPY_RESTRICTION_GROUP],
        name: "unwrap_used",
        tool: LintTool::Clippy,
    },
];

pub const PARENT_DIRECTORY: &str = "..";

pub const REQUIRED_CLIPPY_SETTINGS: [RequiredClippySetting; 3] = [
    RequiredClippySetting {
        name: "allow-expect-in-tests",
        value: true,
    },
    RequiredClippySetting {
        name: "allow-unwrap-in-tests",
        value: true,
    },
    RequiredClippySetting {
        name: "avoid-breaking-exported-api",
        value: false,
    },
];

pub const REQUIRED_LINT_GROUPS: [RequiredLint; 4] = [
    RequiredLint {
        former_names: &[],
        groups: &[],
        name: "all",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[],
        name: "nursery",
        tool: LintTool::Clippy,
    },
    RequiredLint {
        former_names: &[],
        groups: &[],
        name: "pedantic",
        tool: LintTool::Clippy,
    },
    RUST_WARNINGS_LINT,
];

pub const RESTRICTING_CLIPPY_SETTINGS: [&str; 5] = [
    "absolute-paths-allowed-crates",
    "absolute-paths-max-segments",
    "disallowed-macros",
    "disallowed-methods",
    "disallowed-types",
];

pub const RUSTC_VARIABLE: &str = "RUSTC";

pub const RUST_SOURCE_EXTENSION: &str = "rs";

pub const RUSTFMT_CONFIGURATION_FILE: &str = "rustfmt.toml";

pub const RUSTFMT_HIDDEN_CONFIGURATION_FILE: &str = ".rustfmt.toml";

pub const RUSTDOC_VARIABLE: &str = "RUSTDOC";

pub const RUSTFLAGS_VARIABLES: [&str; 2] = ["CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS"];

pub const RUST_WARNINGS_LINT: RequiredLint = RequiredLint {
    former_names: &[],
    groups: &[],
    name: "warnings",
    tool: LintTool::Rust,
};

pub const SYSROOT_BINARY_DIRECTORY: &str = "bin";

pub const VETTED_BENCHMARK_HARNESS_METHODS: [HarnessMethods; 4] = [
    HarnessMethods {
        methods: &[
            "iter",
            "iter_batched",
            "iter_batched_ref",
            "iter_custom",
            "iter_with_large_drop",
        ],
        type_name: "AsyncBencher",
    },
    HarnessMethods {
        methods: &[
            "iter",
            "iter_batched",
            "iter_batched_ref",
            "iter_custom",
            "iter_with_large_drop",
            "to_async",
        ],
        type_name: "Bencher",
    },
    HarnessMethods {
        methods: &[
            "bench_function",
            "bench_with_input",
            "finish",
            "plot_config",
            "throughput",
        ],
        type_name: "BenchmarkGroup",
    },
    HarnessMethods {
        methods: &[
            "bench_function",
            "bench_with_input",
            "benchmark_group",
            "configure_from_args",
            "final_summary",
            "output_directory",
            "plotting_backend",
            "retain_baseline",
            "save_baseline",
            "with_measurement",
            "with_output_color",
            "with_plots",
            "with_profiler",
            "without_plots",
        ],
        type_name: "Criterion",
    },
];

pub const VETTED_NEXTEST_PROFILE_SETTINGS: [&str; 10] = [
    "fail-fast",
    "failure-output",
    "final-status-level",
    "global-timeout",
    "inherits",
    "junit",
    "slow-timeout",
    "status-level",
    "success-output",
    "test-threads",
];

pub const VETTED_NEXTEST_SETTINGS: [&str; 1] = ["nextest-version"];

pub const VETTED_NEXTEST_SLOW_TIMEOUT_SETTINGS: [&str; 3] =
    ["on-timeout", "period", "terminate-after"];
