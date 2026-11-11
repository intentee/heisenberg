#[path = "rounding.rs"]
mod rounding;

fn main() {
    println!("{}", rounding::to_ten(42));
}
