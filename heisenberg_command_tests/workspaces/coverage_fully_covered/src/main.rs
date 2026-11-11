use std::env;

fn main() {
    let value = env::args()
        .nth(1)
        .and_then(|argument| argument.parse().ok())
        .unwrap_or(0);

    println!("{}", coverage_fully_covered::parity(value));
}
