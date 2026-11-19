use std::env;
use std::fs;
use std::io;
use std::path::Path;

use builder::built::built;

fn main() -> Result<(), io::Error> {
    if let Some(out_dir) = env::var_os("OUT_DIR") {
        fs::write(Path::new(&out_dir).join("built.txt"), built().to_string())?;
    }

    Ok(())
}
