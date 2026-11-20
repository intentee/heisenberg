use crate::measured_function::MeasuredFunction;
use crate::measured_item::MeasuredItem;

pub struct MeasuredCode {
    pub functions: Vec<MeasuredFunction>,
    pub items: Vec<MeasuredItem>,
}

impl MeasuredCode {
    pub(crate) fn sort(&mut self) {
        for MeasuredFunction { regions, .. } in &mut self.functions {
            regions.sort();
        }

        for MeasuredItem { regions, .. } in &mut self.items {
            regions.sort();
        }

        self.functions.sort();
        self.items.sort();
    }
}
