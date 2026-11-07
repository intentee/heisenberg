use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::configuration_list::ConfigurationList;

pub struct ConfigurationNarrowing<'configurations> {
    pub all: &'configurations BTreeSet<Configuration>,
    pub configurations: &'configurations BTreeSet<Configuration>,
}

impl Display for ConfigurationNarrowing<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if self.configurations == self.all {
            Ok(())
        } else {
            write!(
                formatter,
                " (only on {})",
                ConfigurationList {
                    configurations: self.configurations
                }
            )
        }
    }
}
