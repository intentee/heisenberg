#[cfg(target_os = "macos")]
pub mod mac;
pub mod never;
#[cfg(windows)]
pub mod windows_only;
