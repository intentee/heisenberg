#[macro_export]
macro_rules! wrapping {
    () => {
        macro_rules! unparsable {
            () => {
                struct
            };
        }

        unparsable!();
    };
}
