pub struct First;

#[cfg(feature = "cyclic")]
pub fn second() -> crate::second::Second {
    crate::second::Second
}
