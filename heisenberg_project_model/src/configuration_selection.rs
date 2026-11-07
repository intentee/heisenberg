use std::collections::BTreeSet;

use ra_ap_paths::AbsPath;

use crate::configuration::Configuration;
use crate::project_model_error::ProjectModelError;

pub struct ConfigurationSelection {
    pub names: Vec<String>,
}

impl ConfigurationSelection {
    /// # Errors
    ///
    /// Returns [`ProjectModelError::SelectedConfigurationNotDeclared`] for a selected name that the
    /// workspace does not declare, and [`ProjectModelError::ConfigurationSelectedTwice`] for one
    /// selected more than once.
    pub fn analysed(
        &self,
        declared: &BTreeSet<Configuration>,
        manifest_path: &AbsPath,
    ) -> Result<BTreeSet<Configuration>, ProjectModelError> {
        let mut analysed = BTreeSet::new();

        for name in &self.names {
            let configuration = declared
                .iter()
                .find(
                    |Configuration {
                         name: declared_name,
                         ..
                     }| declared_name == name,
                )
                .ok_or_else(|| ProjectModelError::SelectedConfigurationNotDeclared {
                    configuration: name.clone(),
                    declared: declared.clone(),
                    manifest_path: manifest_path.to_path_buf(),
                })?;

            if !analysed.insert(configuration.clone()) {
                return Err(ProjectModelError::ConfigurationSelectedTwice {
                    configuration: name.clone(),
                });
            }
        }

        Ok(analysed)
    }
}
