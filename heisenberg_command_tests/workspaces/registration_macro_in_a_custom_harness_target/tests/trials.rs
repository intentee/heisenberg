use registry_macros::registered;

macro_rules! anonymous_constant {
    () => {
        const _: () = ();
    };
}

anonymous_constant!();

#[registered]
fn loads() {}

fn main() {}
