use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if let Some(out_dir) = env::var_os("OUT_DIR") {
        fs::write(
            Path::new(&out_dir).join("bindings.rs"),
            "pub struct c_handle;\npub struct c_status;\n",
        )
        .ok();
    }
}
