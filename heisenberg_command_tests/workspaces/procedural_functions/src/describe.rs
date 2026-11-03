use crate::report::Report;

pub trait Describe {
    fn describe(report: &Report) -> u8;
}
