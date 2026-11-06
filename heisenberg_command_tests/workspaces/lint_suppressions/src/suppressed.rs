#[cfg_attr(unix, allow(unused_variables))]
fn platform() {}

#[deny(unsafe_code)]
#[warn(missing_docs)]
fn raised() {}

#[cfg(unix)]
#[allow(dead_code)]
fn unix_only() {}

#[warn(clippy::unwrap_used)]
fn lowered() {}

#[warn(clippy::restriction)]
fn lowered_group() {}

generator::generated!();

fn suppressed_statement() {
    #[allow(unused)]
    println!("suppressed");
}

#[allow(dead_code)]
struct Unused;

#[expect(clippy::expect_used, reason = "the fixture expects")]
pub fn suppressed() {}
