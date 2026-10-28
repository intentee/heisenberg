use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(generated)");
    println!("cargo::rustc-cfg=generated");

    let out_directory = env::var("OUT_DIR").expect("cargo sets OUT_DIR for build scripts");

    fs::write(
        Path::new(&out_directory).join("included.rs"),
        "pub struct Included;\n",
    )
    .expect("the build script writes into OUT_DIR");
}
