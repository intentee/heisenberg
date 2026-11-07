pub const CARGO_BUILD_TABLE: &str = "build";

pub const CARGO_CONFIGURATION_DIRECTORY: &str = ".cargo";

pub const CARGO_CONFIGURATION_FILE: &str = "config.toml";

pub const CARGO_HOME_VARIABLE: &str = "HEISENBERG_CARGO_HOME";

pub const CARGO_TARGET_DIRECTORY_KEY: &str = "target-dir";

pub const FULL_RUN_FOOTER_END: &str =
    "Precondition: rustc 1.99.1 compiles the workspace in every declared configuration\n";

pub const HOST_CONFIGURATION: &str = HOST_TRIPLE;

pub const HOST_ONLY_BUILDS_DIRECTORY: &str = "host_only_fixture_builds";

pub const HOST_SELECTION: [&str; 2] = ["--configuration", HOST_CONFIGURATION];

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const HOST_TRIPLE: &str = "x86_64-unknown-linux-gnu";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
pub const HOST_TRIPLE: &str = "x86_64-pc-windows-msvc";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const LEFT_OUT_CONFIGURATION: &str = "x86_64-pc-windows-msvc";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
pub const LEFT_OUT_CONFIGURATION: &str = "x86_64-unknown-linux-gnu";

pub const LLVM_PROFILE_FILE_VARIABLE: &str = "LLVM_PROFILE_FILE";

pub const PARTIAL_RUN_FOOTER_END: &str = "Not evaluated in a partial run: CodeActiveInNoConfiguration, the empty-library check of TestsOnlyPackageWithoutTestLibraries\n\
    Precondition: rustc 1.99.1 compiles the workspace in every analysed configuration\n";

pub const PACKAGE_DIRECTORY: &str = env!("CARGO_MANIFEST_DIR");

pub const PATH_VARIABLE: &str = "PATH";

pub const RUSTUP_TOOLCHAIN_VARIABLE: &str = "RUSTUP_TOOLCHAIN";

pub const SYSROOT_VARIABLE: &str = "HEISENBERG_SYSROOT";
