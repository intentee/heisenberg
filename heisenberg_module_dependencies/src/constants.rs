use heisenberg_project_model::constants::CARGO_OUT_DIR_VARIABLE;

pub const CARGO_BINARY_PATH_VARIABLE_PREFIX: &str = "CARGO_BIN_EXE_";

pub const CARGO_BUILD_VARIABLES: [&str; 4] = [
    "CARGO_BIN_NAME",
    "CARGO_PRIMARY_PACKAGE",
    "CARGO_TARGET_TMPDIR",
    CARGO_OUT_DIR_VARIABLE,
];
