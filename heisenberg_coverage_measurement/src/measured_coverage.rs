use crate::measured_code::MeasuredCode;

pub struct MeasuredCoverage {
    pub code: MeasuredCode,
    pub crate_targets: usize,
    pub regions: usize,
}
