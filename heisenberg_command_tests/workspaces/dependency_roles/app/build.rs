use builder::built::built;

fn main() {
    println!("cargo::rustc-env=BUILT_COUNT={}", built());
}
