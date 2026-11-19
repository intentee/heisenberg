use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    println!("cargo::rustc-env=DECLARED_STAMP=declared");

    if let Some(out_dir) = env::var_os("OUT_DIR") {
        fs::write(Path::new(&out_dir).join("stamp.rs"), "pub struct Stamp;\n")?;
    }

    Ok(())
}
