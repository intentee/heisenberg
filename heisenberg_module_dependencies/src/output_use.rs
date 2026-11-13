#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputUse {
    Free,
    MustUse,
    Undetermined,
}
