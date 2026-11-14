#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultMethodEffect {
    DropsATemporaryError,
    DropsTheError,
    KeepsTheError,
    LendsTheError,
    NotAResultMethod,
}
