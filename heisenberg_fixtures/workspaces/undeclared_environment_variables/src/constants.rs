pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const DECLARED: &str = env!("DECLARED");
pub const SET_BY_THE_BUILD_SCRIPT: &str = env!("SET_BY_THE_BUILD_SCRIPT");
pub const UNDECLARED: Option<&str> = option_env!("UNDECLARED_ANYWHERE");
