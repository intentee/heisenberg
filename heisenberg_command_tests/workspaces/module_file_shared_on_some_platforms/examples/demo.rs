#[cfg(windows)]
#[path = "../tests/gated/mod.rs"]
mod gated;
#[cfg(windows)]
#[path = "../tests/platform/mod.rs"]
mod platform;
#[path = "../tests/support/mod.rs"]
mod support;

fn main() {}
