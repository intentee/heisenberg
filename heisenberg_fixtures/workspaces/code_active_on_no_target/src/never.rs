pub struct Never;

impl Never {
    #[cfg(any())]
    pub fn unreachable() {}

    pub fn finish() {
        #[cfg(any())]
        return;
    }
}

macro_rules! never_item {
    () => {
        #[cfg(any())]
        pub fn generated_never() {}
    };
}

never_item!();
