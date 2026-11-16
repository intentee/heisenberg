use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    if env::var_os("NUM_JOBS").is_some_and(|jobs| jobs == "3") {
        env::var_os("OUT_DIR").map_or(Ok(()), |out_dir| {
            fs::write(Path::new(&out_dir).join("extra.rs"), "pub struct Extra;\n")
        })
    } else {
        Ok(())
    }
}
