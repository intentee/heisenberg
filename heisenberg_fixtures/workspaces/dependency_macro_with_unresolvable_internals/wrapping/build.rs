fn main() {
    println!("cargo::rustc-check-cfg=cfg(wrapping_generated)");
    println!("cargo::rustc-cfg=wrapping_generated");
}
