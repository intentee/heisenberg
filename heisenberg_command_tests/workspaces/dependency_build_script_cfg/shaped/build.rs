fn main() {
    println!("cargo::rustc-check-cfg=cfg(shaped_backend)");
    println!("cargo::rustc-cfg=shaped_backend");
}
