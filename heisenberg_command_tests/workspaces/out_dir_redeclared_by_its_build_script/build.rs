fn main() {
    println!(
        "cargo:rustc-env=OUT_DIR={}/handwritten",
        env!("CARGO_MANIFEST_DIR")
    );
}
