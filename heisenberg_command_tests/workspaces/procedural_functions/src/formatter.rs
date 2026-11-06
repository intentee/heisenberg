use crate::report::Report;

pub struct Formatter;

impl Formatter {
    pub fn format(Report { lines }: &Report) -> u8 {
        lines + 1
    }
}
