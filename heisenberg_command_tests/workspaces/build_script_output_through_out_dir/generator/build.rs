use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    fs::write(
        Path::new(&out_dir).join("stamp.rs"),
        "pub const STAMP: &str = \"generated\";\n",
    )?;

    Ok(())
}
