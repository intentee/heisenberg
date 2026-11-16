use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    println!("cargo::rerun-if-changed=build.rs");

    if env::var_os("CARGO_MANIFEST_DIR")
        .is_some_and(|directory| Path::new(&directory).join("undeclared_input").exists())
    {
        env::var_os("OUT_DIR").map_or(Ok(()), |out_dir| {
            fs::write(Path::new(&out_dir).join("extra.rs"), "pub struct Extra;\n")
        })
    } else {
        Ok(())
    }
}
