const OFFSET: usize = 1;

pub mod widths {
    pub const fn three() -> usize {
        crate::OFFSET + 2
    }
}

pub fn total() -> usize {
    mod widths {
        pub fn call() -> usize {
            crate::widths::three() + crate::OFFSET
        }
    }

    widths::call()
}
