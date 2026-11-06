const OFFSET: usize = 1;

pub fn total() -> usize {
    mod widths {
        pub fn widths() -> usize {
            crate::widths::widths() + crate::OFFSET
        }
    }

    widths::widths()
}

pub mod widths {
    pub const fn widths() -> usize {
        crate::OFFSET + 2
    }
}
