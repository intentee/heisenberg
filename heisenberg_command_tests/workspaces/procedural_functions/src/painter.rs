use crate::report::Report;

pub struct Painter;

impl Painter {
    pub fn paint(&self, Report { lines }: &Report) -> u8 {
        lines + 2
    }
}
