pub struct Meters(pub u8);

impl Meters {
    pub fn longer(&self, other: &Self) -> bool {
        self.0 > other.0
    }
}
