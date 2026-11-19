fn main() {
    println!("cargo::rustc-env=CLIPPY_ARGS=-Aclippy::unwrap_used");
    println!("cargo::rustc-env=GENERATED_STAMP=generated");
}
