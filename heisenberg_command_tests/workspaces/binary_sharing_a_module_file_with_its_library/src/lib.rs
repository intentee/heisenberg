mod support;

pub struct Marker;

impl Marker {
    pub fn shared() -> Self {
        support::support()
    }
}
