use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::configuration::Configuration;

pub struct ConfigurationList<'configurations> {
    pub configurations: &'configurations BTreeSet<Configuration>,
}

impl Display for ConfigurationList<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            &self
                .configurations
                .iter()
                .map(|Configuration { name, .. }| name.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
        )
    }
}
