fn main() {
    println!("cargo::rustc-check-cfg=cfg(generated)");
    println!("cargo::rustc-cfg=generated");
}
