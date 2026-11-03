const OFFSET: usize = 1;

pub mod widths {
    pub const fn widths() -> usize {
        crate::OFFSET + 2
    }
}

pub fn total() -> usize {
    mod widths {
        pub fn widths() -> usize {
            crate::widths::widths() + crate::OFFSET
        }
    }

    widths::widths()
}
