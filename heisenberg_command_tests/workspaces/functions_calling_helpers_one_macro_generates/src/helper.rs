macro_rules! helper {
    () => {
        fn helper() -> u8 {
            1
        }

        struct Helper;

        impl Helper {
            const fn value() -> u8 {
                2
            }
        }
    };
}
