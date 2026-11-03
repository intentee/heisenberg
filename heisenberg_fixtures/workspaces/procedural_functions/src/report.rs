pub mod summary;

fn line_count(report: &Report) -> u8 {
    report.lines
}

pub struct Report {
    pub lines: u8,
}
