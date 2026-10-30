pub struct First;

impl First {
    pub fn limit() -> u8 {
        1
    }

    pub fn print() {
        println!("{}", crate::second::Second::value());
    }
}
