use std::env;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(undeclared_input)");

    if env::var_os("CARGO_MANIFEST_DIR")
        .is_some_and(|directory| Path::new(&directory).join("undeclared_input").exists())
    {
        println!("cargo::rustc-cfg=undeclared_input");
    }
}
