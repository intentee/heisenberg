use super::Report;
use super::line_count;

pub struct Summary {
    pub total: u8,
}

impl Summary {
    pub fn new(report: &Report) -> Self {
        Self {
            total: line_count(report),
        }
    }
}
