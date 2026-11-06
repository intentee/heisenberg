pub const CARGO_HOME_VARIABLE: &str = "HEISENBERG_CARGO_HOME";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const HOST_TRIPLE: &str = "x86_64-unknown-linux-gnu";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
pub const HOST_TRIPLE: &str = "x86_64-pc-windows-msvc";

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub const LEFT_OUT_TRIPLE: &str = "x86_64-pc-windows-msvc";

#[cfg(all(target_arch = "x86_64", target_os = "windows", target_env = "msvc"))]
pub const LEFT_OUT_TRIPLE: &str = "x86_64-unknown-linux-gnu";

pub const LLVM_PROFILE_FILE_VARIABLE: &str = "LLVM_PROFILE_FILE";

pub const PACKAGE_DIRECTORY: &str = env!("CARGO_MANIFEST_DIR");

pub const PATH_VARIABLE: &str = "PATH";

pub const RUSTUP_TOOLCHAIN_VARIABLE: &str = "RUSTUP_TOOLCHAIN";

pub const SYSROOT_VARIABLE: &str = "HEISENBERG_SYSROOT";
