#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TokenSpelling {
    pub kind: u16,
    pub text: String,
}
