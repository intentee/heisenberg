pub const WIDTH: u8 = 2;

pub const fn widened() -> u8 {
    WIDTH.saturating_add(1)
}
