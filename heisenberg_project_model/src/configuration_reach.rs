use crate::configuration::Configuration;
use crate::confined_package_reach::ConfinedPackageReach;

pub struct ConfigurationReach<'reach> {
    pub configuration: &'reach Configuration,
    pub reach: &'reach ConfinedPackageReach,
}
