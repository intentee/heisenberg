use crate::report::Report;

pub struct Formatter;

impl Formatter {
    pub fn format(report: &Report) -> u8 {
        report.lines + 1
    }
}
