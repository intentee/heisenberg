use library::report::Report;

pub fn rendered(report: &Report) -> u8 {
    report.lines * 2
}
