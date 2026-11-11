use ra_ap_ide_db::FilePosition;

use crate::measured_region::MeasuredRegion;

pub struct AttributedRegion {
    pub functions: Vec<FilePosition>,
    pub region: MeasuredRegion,
}
