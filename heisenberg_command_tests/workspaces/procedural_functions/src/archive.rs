use crate::report::Report;

pub fn archive(report: Box<Report>) -> u8 {
    report.lines
}
