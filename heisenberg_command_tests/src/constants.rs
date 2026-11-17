pub const CARGO_BUILD_TABLE: &str = "build";

pub const CARGO_CONFIGURATION_DIRECTORY: &str = ".cargo";

pub const CARGO_CONFIGURATION_FILE: &str = "config.toml";

pub const CARGO_HOME_ARGUMENT: &str = "--cargo-home";

pub const CARGO_HOME_VARIABLE: &str = "HEISENBERG_CARGO_HOME";

pub const CARGO_TARGET_DIRECTORY_KEY: &str = "target-dir";

pub const CATALOGUE_SUBCOMMAND: &str = "catalogue";

pub const CHECK_SUBCOMMAND: &str = "check";

pub const CONFIGURATION_ARGUMENT: &str = "--configuration";

pub const COVERAGE_CHECK_SUBCOMMAND: &str = "coverage-check";

pub const COVERAGE_EXPORT_ARGUMENT: &str = "--coverage-export";

pub const COVERAGE_EXPORT_FILE: &str = "llvm-cov.json";

pub const COVERAGE_FOOTER_END: &str = "Precondition: rustc 1.99.1 compiles the workspace in the measured configuration, whose tests the coverage export records\nRules: 2 evaluated, 0 evaluated in part, 0 not evaluated\nHeisenberg 0.1.0\n";

pub const EXPLAIN_SUBCOMMAND: &str = "explain";

pub const EXPORTED_DATA_POINTER: &str = "/data/0";

pub const EXPORTED_FUNCTIONS_KEY: &str = "functions";

pub const EXPORTED_MANIFEST_POINTER: &str = "/cargo_llvm_cov/manifest_path";

pub const FIRST_FUNCTION_BRANCHES_POINTER: &str = "/data/0/functions/0/branches";

pub const FIRST_FUNCTION_FILENAMES_POINTER: &str = "/data/0/functions/0/filenames";

pub const FIRST_FUNCTION_FILE_POINTER: &str = "/data/0/functions/0/filenames/0";

pub const FIRST_FUNCTION_MCDC_RECORDS_POINTER: &str = "/data/0/functions/0/mcdc_records";

pub const FIRST_FUNCTION_NAME_POINTER: &str = "/data/0/functions/0/name";

pub const FIRST_REGION_COLUMN_POINTER: &str = "/data/0/functions/0/regions/0/1";

pub const FIRST_REGION_END_COLUMN_POINTER: &str = "/data/0/functions/0/regions/0/3";

pub const FIRST_REGION_END_LINE_POINTER: &str = "/data/0/functions/0/regions/0/2";

pub const FIRST_REGION_FILE_ID_POINTER: &str = "/data/0/functions/0/regions/0/5";

pub const FIRST_REGION_KIND_POINTER: &str = "/data/0/functions/0/regions/0/7";

pub const FIRST_REGION_LINE_POINTER: &str = "/data/0/functions/0/regions/0/0";

pub const FULL_RUN_FOOTER_END: &str = "Precondition: rustc 1.99.1 compiles the workspace in every declared configuration\nRules: 112 evaluated, 0 evaluated in part, 0 not evaluated\nHeisenberg 0.1.0\n";

pub const FUNCTION_FIRST_FILE_POINTER: &str = "/filenames/0";

pub const HELP_ARGUMENT: &str = "--help";

pub const HOST_CONFIGURATION: &str = HOST_TRIPLE;

pub const HOST_ONLY_BUILDS_DIRECTORY: &str = "host_only_fixture_builds";

pub const HOST_SELECTION: [&str; 2] = [CONFIGURATION_ARGUMENT, HOST_CONFIGURATION];

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const HOST_TRIPLE: &str = "x86_64-unknown-linux-gnu";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
pub const HOST_TRIPLE: &str = "x86_64-pc-windows-msvc";

pub const JOBS_ARGUMENT: &str = "--jobs";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const LEFT_OUT_CONFIGURATION: &str = "x86_64-pc-windows-msvc";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
pub const LEFT_OUT_CONFIGURATION: &str = "x86_64-unknown-linux-gnu";

pub const LLVM_PROFILE_FILE_VARIABLE: &str = "LLVM_PROFILE_FILE";

pub const PACKAGE_DIRECTORY: &str = env!("CARGO_MANIFEST_DIR");

pub const PATH_VARIABLE: &str = "PATH";

pub const PROGRAM_NAME: &str = "heisenberg";

pub const RUSTUP_TOOLCHAIN_VARIABLE: &str = "RUSTUP_TOOLCHAIN";

pub const SKIPPED_REGION_KIND: u64 = 2;

pub const SYSROOT_ARGUMENT: &str = "--sysroot";

pub const SYSROOT_VARIABLE: &str = "HEISENBERG_SYSROOT";

pub const WORKSPACE_ARGUMENT: &str = "--workspace";
