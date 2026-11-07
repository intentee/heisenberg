pub trait Tally {
    fn total(&self) -> u8;

    fn create() -> Self;

    fn average(&self) -> u8;
}
