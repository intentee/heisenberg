use ra_ap_paths::Utf8Path;
use ra_ap_paths::Utf8PathBuf;
use std::collections::BTreeMap;
use toml::Table;

use crate::constants::CLIPPY_CONFIGURATION_FILE;
use crate::constants::CLIPPY_HIDDEN_CONFIGURATION_FILE;
use crate::optional_text::optional_text;
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
    optional_text(path).map_err(|source| ProjectModelError::ClippyConfigurationUnreadable {
        path: path.as_std_path().to_path_buf(),
        source,
    })
}

#[derive(Clone)]
pub enum ClippyConfigurationFile {
    Ambiguous { file: Utf8PathBuf },
    Missing,
    Present { file: Utf8PathBuf, settings: Table },
}

impl ClippyConfigurationFile {
    pub fn discover(
        manifest_path: &Utf8Path,
        workspace_root: &Utf8Path,
        found: &mut BTreeMap<Utf8PathBuf, Self>,
    ) -> Result<Self, ProjectModelError> {
        let mut visited: Vec<&Utf8Path> = Vec::new();
        let mut discovered = Self::Missing;

        for directory in manifest_path
            .ancestors()
            .skip(1)
            .take_while(|directory| directory.starts_with(workspace_root))
        {
            if let Some(known) = found.get(directory) {
                discovered = known.clone();
                break;
            }

            visited.push(directory);

            if let Some(own) = Self::in_directory(directory)? {
                discovered = own;
                break;
            }
        }

        for directory in visited {
            found.insert(directory.to_owned(), discovered.clone());
        }

        Ok(discovered)
    }

    fn in_directory(directory: &Utf8Path) -> Result<Option<Self>, ProjectModelError> {
        let hidden = directory.join(CLIPPY_HIDDEN_CONFIGURATION_FILE);
        let visible = directory.join(CLIPPY_CONFIGURATION_FILE);

        match read(&hidden)? {
            Some(hidden_text) => match read(&visible)? {
                Some(_) => Ok(Some(Self::Ambiguous { file: visible })),
                None => parse(hidden, &hidden_text).map(Some),
            },
            None => read(&visible)?
                .map(|text| parse(visible, &text))
                .transpose(),
        }
    }
}
