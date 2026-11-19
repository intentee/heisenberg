use std::env;
use std::fs;
use std::io;
use std::path::Path;

use builder::built::built;

fn main() -> Result<(), io::Error> {
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    fs::write(Path::new(&out_dir).join("built.txt"), built().to_string())?;

    Ok(())
}
