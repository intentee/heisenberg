use std::collections::BTreeMap;

use ra_ap_paths::AbsPath;
use serde::Deserialize;
use toml::Value;

use crate::constants::HEISENBERG_SETTINGS_KEY;
use crate::declared_configuration::DeclaredConfiguration;
use crate::declared_out_dir::DeclaredOutDir;
use crate::project_model_error::ProjectModelError;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeisenbergSettings {
    pub configurations: Vec<DeclaredConfiguration>,
    #[serde(default)]
    pub confined_packages: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub out_dirs: BTreeMap<String, DeclaredOutDir>,
    #[serde(default)]
    pub test_crates: Vec<String>,
}

impl HeisenbergSettings {
    pub fn of_workspace(
        metadata: &Value,
        manifest_path: &AbsPath,
    ) -> Result<Self, ProjectModelError> {
        metadata
            .get(HEISENBERG_SETTINGS_KEY)
            .ok_or_else(|| ProjectModelError::HeisenbergSettingsMissing {
                manifest_path: manifest_path.to_path_buf(),
            })?
            .clone()
            .try_into()
            .map_err(|source| ProjectModelError::HeisenbergSettingsInvalid {
                manifest_path: manifest_path.to_path_buf(),
                source: Box::new(source),
            })
    }
}
