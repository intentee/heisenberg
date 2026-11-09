#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WrittenOffset {
    pub file: String,
    pub offset: u32,
}
