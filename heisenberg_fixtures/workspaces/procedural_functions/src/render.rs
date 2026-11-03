use crate::report::Report;

pub fn render(report: &Report) -> u8 {
    report.lines * 2
}
