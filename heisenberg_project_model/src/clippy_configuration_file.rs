use std::fs;
use std::io;

use ra_ap_paths::Utf8Path;
use ra_ap_paths::Utf8PathBuf;
use toml::Table;

use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use crate::project_model_error::ProjectModelError;

fn parse(file: Utf8PathBuf, text: &str) -> Result<ClippyConfigurationFile, ProjectModelError> {
    match text.parse::<Table>() {
        Ok(settings) => Ok(ClippyConfigurationFile::Present { file, settings }),
        Err(source) => Err(ProjectModelError::ClippyConfigurationInvalid {
            path: file.into_std_path_buf(),
            source: Box::new(source),
        }),
    }
}

fn read(path: &Utf8Path) -> Result<Option<String>, ProjectModelError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(ProjectModelError::ClippyConfigurationUnreadable {
            path: path.as_std_path().to_path_buf(),
            source,
        }),
    }
}

pub enum ClippyConfigurationFile {
    Ambiguous { file: Utf8PathBuf },
    Missing,
    Present { file: Utf8PathBuf, settings: Table },
}

impl ClippyConfigurationFile {
    pub fn discover(
        manifest_path: &Utf8Path,
        workspace_root: &Utf8Path,
    ) -> Result<Self, ProjectModelError> {
        for directory in manifest_path
            .ancestors()
            .skip(1)
            .take_while(|directory| directory.starts_with(workspace_root))
        {
            let hidden = directory.join(CLIPPY_HIDDEN_CONFIGURATION_FILE);
            let visible = directory.join(CLIPPY_CONFIGURATION_FILE);

            match read(&hidden)? {
                Some(hidden_text) => {
                    return match read(&visible)? {
                        Some(_) => Ok(Self::Ambiguous { file: visible }),
                        None => parse(hidden, &hidden_text),
                    };
                }
                None => {
                    if let Some(text) = read(&visible)? {
                        return parse(visible, &text);
                    }
                }
            }
        }

        Ok(Self::Missing)
    }
}
