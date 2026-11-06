use std::collections::BTreeSet;

use ra_ap_paths::AbsPath;

use crate::constants::HOST_TARGET;
use crate::project_model_error::ProjectModelError;
use crate::supported_target::SupportedTarget;

pub enum TargetSelection {
    Host,
    Selected { triples: Vec<String> },
}

impl TargetSelection {
    /// # Errors
    ///
    /// Returns [`ProjectModelError::HostTargetNotDeclared`] when no target is selected and the
    /// workspace does not declare the host's target,
    /// [`ProjectModelError::SelectedTargetNotDeclared`] for a selected triple that the workspace
    /// does not declare, and [`ProjectModelError::TargetSelectedTwice`] for one selected more
    /// than once.
    pub fn analysed(
        &self,
        declared: &BTreeSet<SupportedTarget>,
        manifest_path: &AbsPath,
    ) -> Result<BTreeSet<SupportedTarget>, ProjectModelError> {
        match self {
            Self::Host if declared.contains(&HOST_TARGET) => Ok(BTreeSet::from([HOST_TARGET])),
            Self::Host => Err(ProjectModelError::HostTargetNotDeclared {
                declared: declared
                    .iter()
                    .map(|target| target.triple().to_owned())
                    .collect(),
                host: HOST_TARGET.triple(),
                manifest_path: manifest_path.to_path_buf(),
            }),
            Self::Selected { triples } => {
                let mut selected = BTreeSet::new();

                for triple in triples {
                    let target = declared
                        .iter()
                        .copied()
                        .find(|target| target.triple() == triple)
                        .ok_or_else(|| ProjectModelError::SelectedTargetNotDeclared {
                            declared: declared
                                .iter()
                                .map(|target| target.triple().to_owned())
                                .collect(),
                            manifest_path: manifest_path.to_path_buf(),
                            target: triple.clone(),
                        })?;

                    if !selected.insert(target) {
                        return Err(ProjectModelError::TargetSelectedTwice {
                            target: triple.clone(),
                        });
                    }
                }

                Ok(selected)
            }
        }
    }
}
