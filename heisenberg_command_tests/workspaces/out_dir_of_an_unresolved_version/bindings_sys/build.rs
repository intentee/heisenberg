use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    println!("cargo::rustc-check-cfg=cfg(declared_backend)");
    println!("cargo::rustc-cfg=declared_backend");
    println!("cargo::rustc-env=DECLARED_VALUE=declared");

    if let Some(out_dir) = env::var_os("OUT_DIR") {
        fs::write(Path::new(&out_dir).join("bindings.rs"), "pub struct Binding;\n")?;
    }

    Ok(())
}
