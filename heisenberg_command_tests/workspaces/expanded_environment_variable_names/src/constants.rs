pub const CONCATENATED: &str = env!(concat!("CARGO_PKG_", "NAME"));

pub const FRAGMENT: &str = read_variable!("FRAGMENT_UNDECLARED");

pub const HELD: &str = env!(held_name!());

pub const NESTED: &str = concat!(env!("NESTED_UNDECLARED"), "suffix");

pub const NUMBERED: &str = env!(1);

pub const RAW: &str = env!(r"CARGO_PKG_VERSION");
