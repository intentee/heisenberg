use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::configuration::Configuration;
use crate::configuration_reach::ConfigurationReach;
use crate::confined_reach::ConfinedReach;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfinementBreach {
    pub configurations: BTreeSet<Configuration>,
    pub path: Vec<String>,
    pub reach: ConfinedReach,
}

impl ConfinementBreach {
    #[must_use]
    pub fn grouped<'reach>(
        reaches: impl IntoIterator<Item = ConfigurationReach<'reach>>,
    ) -> Vec<Self> {
        let mut breaches: BTreeMap<&ConfinedReach, Self> = BTreeMap::new();

        for ConfigurationReach {
            configuration,
            reach,
        } in reaches
        {
            for (confined, path) in &reach.breaches {
                breaches
                    .entry(confined)
                    .or_insert_with(|| Self {
                        configurations: BTreeSet::new(),
                        path: path.clone(),
                        reach: confined.clone(),
                    })
                    .configurations
                    .insert(configuration.clone());
            }
        }

        breaches.into_values().collect()
    }
}
