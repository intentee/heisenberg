use generator::generated_zeroed_byte;
use maker::zeroed_byte;

#[must_use]
pub fn bytes() -> u8 {
    let written: u8 = zeroed_byte!();
    let generated: u8 = generated_zeroed_byte!();

    written.wrapping_add(generated)
}
