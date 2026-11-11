use crate::measured_function::MeasuredFunction;
use crate::measured_region::MeasuredRegion;

pub struct MeasuredCode {
    pub functions: Vec<MeasuredFunction>,
    pub standalone_regions: Vec<MeasuredRegion>,
}

impl MeasuredCode {
    pub(crate) fn sort(&mut self) {
        for MeasuredFunction { regions, .. } in &mut self.functions {
            regions.sort();
        }

        self.functions.sort();
        self.standalone_regions.sort();
    }
}
