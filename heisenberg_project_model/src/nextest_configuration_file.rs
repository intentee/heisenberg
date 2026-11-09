use std::fs;
use std::io;

use ra_ap_paths::Utf8Path;
use ra_ap_paths::Utf8PathBuf;
use toml::from_str;

use crate::constants::NEXTEST_CONFIGURATION_DIRECTORY;
use crate::constants::NEXTEST_CONFIGURATION_FILE;
use crate::nextest_configuration::NextestConfiguration;
use crate::project_model_error::ProjectModelError;

pub enum NextestConfigurationFile {
    Missing,
    Present {
        configuration: NextestConfiguration,
        file: Utf8PathBuf,
    },
}

impl NextestConfigurationFile {
    pub fn read(workspace_root: &Utf8Path) -> Result<Self, ProjectModelError> {
        let file = workspace_root
            .join(NEXTEST_CONFIGURATION_DIRECTORY)
            .join(NEXTEST_CONFIGURATION_FILE);

        match fs::read_to_string(&file) {
            Ok(text) => match from_str(&text) {
                Ok(configuration) => Ok(Self::Present {
                    configuration,
                    file,
                }),
                Err(source) => Err(ProjectModelError::NextestConfigurationInvalid {
                    path: file.into_std_path_buf(),
                    source: Box::new(source),
                }),
            },
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::Missing),
            Err(source) => Err(ProjectModelError::NextestConfigurationUnreadable {
                path: file.into_std_path_buf(),
                source,
            }),
        }
    }
}
