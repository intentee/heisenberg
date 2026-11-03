use std::collections::BTreeSet;

use ra_ap_paths::AbsPath;

use crate::constants::SUPPORTED_TARGETS;
use crate::project_model_error::ProjectModelError;
use crate::supported_target::SupportedTarget;

pub fn declared_targets(
    targets: Vec<String>,
    manifest_path: &AbsPath,
) -> Result<BTreeSet<SupportedTarget>, ProjectModelError> {
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
