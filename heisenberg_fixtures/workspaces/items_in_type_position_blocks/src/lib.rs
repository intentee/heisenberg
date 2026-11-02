pub struct Holder {
    pub values: [u8; {
        const SIZE: usize = 3;
        SIZE
    }],
    pub sizes: [u8; {
        extern crate core as base;
        base::mem::size_of::<u16>()
    }],
    pub counts: [u8; {
        struct Count;
        impl Count {
            const VALUE: usize = 3;
        }
        Count::VALUE
    }],
    pub widths: [u8; {
        mod widths {
            pub const fn three() -> usize {
                3
            }
        }
        widths::three()
    }],
    pub totals: [u8; {
        const fn total(extra: usize) -> usize {
            let base = 2;
            base + extra
        }
        total(1)
    }],
    pub bounds: [u8; {
        trait Measured {
            const LOW: usize;
            const HIGH: usize;
        }
        2
    }],
}
