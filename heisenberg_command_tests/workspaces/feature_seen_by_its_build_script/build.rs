use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(fast_seen)");

    if env::var_os("CARGO_FEATURE_FAST").is_some() {
        println!("cargo::rustc-cfg=fast_seen");
    }
}
