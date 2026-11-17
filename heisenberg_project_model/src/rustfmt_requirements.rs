use std::collections::BTreeSet;
use std::iter;

use ra_ap_paths::Utf8Path;
use toml::Table;

use crate::constants::RUSTFMT_CONFIGURATION_FILE;
use crate::constants::RUSTFMT_HIDDEN_CONFIGURATION_FILE;
use crate::constants::VETTED_RUSTFMT_SETTINGS;
use crate::optional_text::optional_text;
use crate::package_violation::PackageViolation;
use crate::resolved_packages::ResolvedPackages;
use crate::rustfmt_configuration_text::RustfmtConfigurationText;
use crate::rustfmt_violation::RustfmtViolation;
use crate::workspace_path::WorkspacePath;

fn configuration_text(path: &Utf8Path) -> RustfmtConfigurationText {
    match optional_text(path) {
        Ok(Some(text)) => RustfmtConfigurationText::Read { text },
        Ok(None) => RustfmtConfigurationText::Absent,
        Err(error) => RustfmtConfigurationText::Unreadable { kind: error.kind() },
    }
}

fn setting_violations(file: &str, text: &str) -> Vec<RustfmtViolation> {
    text.parse::<Table>().map_or_else(
        |error| {
            vec![RustfmtViolation::ConfigurationInvalid {
                file: file.to_owned(),
                reason: error.message().to_owned(),
            }]
        },
        |settings| {
            settings
                .keys()
                .filter(|setting| !VETTED_RUSTFMT_SETTINGS.contains(&setting.as_str()))
                .map(|setting| RustfmtViolation::UnvettedSetting {
                    file: file.to_owned(),
                    setting: setting.clone(),
                })
                .collect()
        },
    )
}

fn file_violations(
    file: String,
    configuration: &RustfmtConfigurationText,
) -> Vec<RustfmtViolation> {
    match configuration {
        RustfmtConfigurationText::Absent => Vec::new(),
        RustfmtConfigurationText::Read { text } => setting_violations(&file, text),
        RustfmtConfigurationText::Unreadable { kind } => {
            vec![RustfmtViolation::ConfigurationUnreadable { file, kind: *kind }]
        }
    }
}

pub struct RustfmtRequirements<'requirements> {
    pub module_files: &'requirements BTreeSet<&'requirements String>,
    pub resolved: &'requirements ResolvedPackages,
}

impl RustfmtRequirements<'_> {
    #[must_use]
    pub fn violations(&self) -> Vec<PackageViolation> {
        let workspace_path = self.resolved.workspace_path();
        let root = Utf8Path::new("");
        let directories: BTreeSet<&Utf8Path> = self
            .module_files
            .iter()
            .flat_map(|file| Utf8Path::new(file.as_str()).ancestors().skip(1))
            .chain(iter::once(root))
            .collect();

        directories
            .into_iter()
            .flat_map(|directory| {
                Self::directory_violations(&workspace_path, directory, directory == root)
            })
            .map(PackageViolation::Rustfmt)
            .collect()
    }

    fn directory_violations(
        workspace_path: &WorkspacePath<'_>,
        directory: &Utf8Path,
        is_root: bool,
    ) -> Vec<RustfmtViolation> {
        let absolute = workspace_path.root.join(directory);
        let hidden_path = absolute.join(RUSTFMT_HIDDEN_CONFIGURATION_FILE);
        let visible_path = absolute.join(RUSTFMT_CONFIGURATION_FILE);
        let hidden = configuration_text(&hidden_path);
        let visible = configuration_text(&visible_path);

        match hidden {
            RustfmtConfigurationText::Absent => match visible {
                RustfmtConfigurationText::Absent if is_root => {
                    vec![RustfmtViolation::ConfigurationMissing {
                        manifest: workspace_path.manifest(),
                    }]
                }
                _ => file_violations(workspace_path.relative(&visible_path), &visible),
            },
            _ => match visible {
                RustfmtConfigurationText::Absent => {
                    file_violations(workspace_path.relative(&hidden_path), &hidden)
                }
                _ => vec![RustfmtViolation::ConfigurationAmbiguous {
                    file: workspace_path.relative(&visible_path),
                }],
            },
        }
    }
}
