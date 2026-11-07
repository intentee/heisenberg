fn main() {
    println!("cargo::rustc-check-cfg=cfg(accelerated_backend)");
    println!("cargo::rustc-check-cfg=cfg(declared_backend)");
    println!("cargo::rustc-cfg=declared_backend");
    println!("cargo::rustc-env=DECLARED_VALUE=declared");
}
