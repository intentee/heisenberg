pub struct First;

impl First {
    pub const LIMIT: u8 = 1;

    pub fn print() {
        println!("{}", crate::second::VALUE);
    }
}
