pub struct Marker;

Marker!();

crate::missing!();
crate::absent::missing!();

#[derive(MissingDerive)]
struct Derived;

#[missing_attribute]
fn attributed() {}
