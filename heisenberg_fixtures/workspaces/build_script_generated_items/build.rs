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

    let nested = Path::new(&out_directory).join("nested");

    fs::create_dir_all(&nested).expect("the build script creates a nested directory");
    fs::write(nested.join("deeper.rs"), "pub struct Deeper;\n")
        .expect("the build script writes into a nested directory");
    fs::write(Path::new(&out_directory).join("binary.bin"), [0xff, 0xfe])
        .expect("the build script writes a binary file");
}
