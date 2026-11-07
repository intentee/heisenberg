use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use heisenberg_module_graph::crate_target::CrateTarget;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::configuration_list::ConfigurationList;

use crate::analysis_context::AnalysisContext;

fn configurations_by_crate(
    contexts: &BTreeSet<AnalysisContext>,
) -> BTreeMap<&CrateTarget, BTreeSet<Configuration>> {
    let mut configurations_by_crate: BTreeMap<&CrateTarget, BTreeSet<Configuration>> =
        BTreeMap::new();

    for AnalysisContext {
        configuration,
        crate_target,
    } in contexts
    {
        configurations_by_crate
            .entry(crate_target)
            .or_default()
            .insert(configuration.clone());
    }

    configurations_by_crate
}

pub struct Narrowing<'contexts> {
    pub base: &'contexts BTreeSet<AnalysisContext>,
    pub reported: &'contexts BTreeSet<AnalysisContext>,
}

impl Display for Narrowing<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if self.reported == self.base {
            Ok(())
        } else {
            write!(
                formatter,
                " (only in {})",
                configurations_by_crate(self.reported)
                    .into_iter()
                    .map(|(crate_target, configurations)| {
                        let base_configurations: BTreeSet<Configuration> = self
                            .base
                            .iter()
                            .filter(|context| context.crate_target == *crate_target)
                            .map(|context| context.configuration.clone())
                            .collect();

                        if configurations == base_configurations {
                            crate_target.to_string()
                        } else {
                            format!(
                                "{crate_target} on {}",
                                ConfigurationList {
                                    configurations: &configurations
                                }
                            )
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("; ")
            )
        }
    }
}
