use crate::clippy_lint::ClippyLint;
use crate::supported_target::SupportedTarget;

pub const SUPPORTED_TARGETS: [SupportedTarget; 3] = [
    SupportedTarget::AArch64AppleDarwin,
    SupportedTarget::X86_64PcWindowsMsvc,
    SupportedTarget::X86_64UnknownLinuxGnu,
];

pub const CARGO_BUILD_TABLE: &str = "build";

pub const CARGO_ENVIRONMENT_TABLE: &str = "env";

pub const CARGO_MANIFEST_FILE: &str = "Cargo.toml";

pub const CARGO_OUT_DIR_VARIABLE: &str = "OUT_DIR";

pub const CARGO_RUSTFLAGS: &str = "rustflags";

pub const CARGO_TARGET_TABLE: &str = "target";

pub const CLIPPY_ARGUMENTS_VARIABLE: &str = "CLIPPY_ARGS";

pub const CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE: &str = "CLIPPY_CONF_DIR";

pub const CLIPPY_CONFIGURATION_FILE: &str = "clippy.toml";

pub const CLIPPY_DENIED_LINTS: [ClippyLint; 3] = [
    ClippyLint {
        former_names: &["option_expect_used", "result_expect_used"],
        name: "expect_used",
    },
    ClippyLint {
        former_names: &[],
        name: "panic",
    },
    ClippyLint {
        former_names: &["option_unwrap_used", "result_unwrap_used"],
        name: "unwrap_used",
    },
];

pub const CLIPPY_DENIED_LINTS_GROUP: &str = "restriction";

pub const CLIPPY_HIDDEN_CONFIGURATION_FILE: &str = ".clippy.toml";

pub const CLIPPY_TEST_ALLOWANCES: [&str; 3] = [
    "allow-expect-in-tests",
    "allow-panic-in-tests",
    "allow-unwrap-in-tests",
];

pub const CLIPPY_TOOL: &str = "clippy";

pub const HEISENBERG_SETTINGS_KEY: &str = "heisenberg";

pub const LINT_NAME_SEPARATOR: &str = "_";

pub const LINT_NAME_SEPARATOR_ALIAS: &str = "-";

pub const NIGHTLY_BOOTSTRAP_VARIABLE: &str = "RUSTC_BOOTSTRAP";

pub const NIGHTLY_FEATURE_VARIABLES: [&str; 2] = [
    NIGHTLY_BOOTSTRAP_VARIABLE,
    "__CARGO_TEST_CHANNEL_OVERRIDE_DO_NOT_USE_THIS",
];

pub const PARENT_DIRECTORY: &str = "..";

pub const RUSTFLAGS_VARIABLES: [&str; 2] = ["CARGO_ENCODED_RUSTFLAGS", "RUSTFLAGS"];

pub const RUST_TOOL: &str = "rust";

pub const RUST_WARNINGS_GROUP: &str = "warnings";
