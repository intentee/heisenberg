Marker!();

crate::missing!();
crate::absent::missing!();

#[missing_attribute]
fn attributed() {}

#[derive(MissingDerive)]
struct Derived;

pub struct Marker;
