pub trait Measure {
    fn measure(&self, bounds: (u8, u8)) -> u8;
}
