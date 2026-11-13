#[cfg(windows)]
mod gated;
mod platform;
mod support;

#[test]
fn first() { assert!(!"first".is_empty()); }
