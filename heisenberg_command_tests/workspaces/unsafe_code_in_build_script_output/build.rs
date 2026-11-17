use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_directory = env::var("OUT_DIR").expect("cargo sets OUT_DIR for build scripts");

    fs::write(
        Path::new(&out_directory).join("generated.rs"),
        "pub fn generated_zero() -> u8 {\n    unsafe { ::core::mem::zeroed::<u8>() }\n}\n\npub fn generated_absent() {\n    crate::absent::value();\n    0_u8.absent();\n}\n",
    )
    .expect("the build script writes into OUT_DIR");
}
