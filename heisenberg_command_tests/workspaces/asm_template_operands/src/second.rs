use core::arch::asm;

pub fn second(value: u64) -> u64 {
    let copied: u64;

    unsafe {
        asm!("mov {0}, {1}", out(reg) copied, in(reg) value);
    }

    copied
}
