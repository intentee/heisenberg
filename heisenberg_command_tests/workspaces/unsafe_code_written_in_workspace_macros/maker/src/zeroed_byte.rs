#[macro_export]
macro_rules! zeroed_byte {
    () => {
        unsafe { ::core::mem::zeroed::<u8>() }
    };
}
