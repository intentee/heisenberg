use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(three_jobs)");

    if env::var_os("NUM_JOBS").is_some_and(|jobs| jobs == "3") {
        println!("cargo::rustc-cfg=three_jobs");
    }
}
