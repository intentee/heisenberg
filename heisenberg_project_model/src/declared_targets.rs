use std::collections::BTreeSet;

use ra_ap_paths::AbsPath;
use toml::Value;

use crate::constants::HEISENBERG_SETTINGS_KEY;
use crate::constants::SUPPORTED_TARGETS;
use crate::heisenberg_settings::HeisenbergSettings;
use crate::project_model_error::ProjectModelError;
use crate::supported_target::SupportedTarget;

pub fn declared_targets(
    metadata: &Value,
    manifest_path: &AbsPath,
) -> Result<BTreeSet<SupportedTarget>, ProjectModelError> {
    let HeisenbergSettings { targets } = metadata
        .get(HEISENBERG_SETTINGS_KEY)
        .ok_or_else(|| ProjectModelError::HeisenbergSettingsMissing {
            manifest_path: manifest_path.to_path_buf(),
        })?
        .clone()
        .try_into()
        .map_err(|source| ProjectModelError::HeisenbergSettingsInvalid {
            manifest_path: manifest_path.to_path_buf(),
            source: Box::new(source),
        })?;

    if targets.is_empty() {
        return Err(ProjectModelError::NoTargets {
            manifest_path: manifest_path.to_path_buf(),
        });
    }

    let mut declared = BTreeSet::new();

    for target in targets {
        let supported = SUPPORTED_TARGETS
            .into_iter()
            .find(|supported| supported.triple() == target)
            .ok_or_else(|| ProjectModelError::UnknownTarget {
                manifest_path: manifest_path.to_path_buf(),
                target: target.clone(),
            })?;

        if !declared.insert(supported) {
            return Err(ProjectModelError::DuplicateTarget {
                manifest_path: manifest_path.to_path_buf(),
                target,
            });
        }
    }

    Ok(declared)
}
