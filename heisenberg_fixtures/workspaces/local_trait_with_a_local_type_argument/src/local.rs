use crate::convert::Convert;

pub struct Local;

impl Convert<Local> for u32 {
    fn convert(_source: Local) -> Self {
        1
    }
}
