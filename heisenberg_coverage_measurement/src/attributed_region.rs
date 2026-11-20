use crate::measured_region::MeasuredRegion;
use crate::region_owner::RegionOwner;

pub struct AttributedRegion {
    pub owner: RegionOwner,
    pub region: MeasuredRegion,
}
