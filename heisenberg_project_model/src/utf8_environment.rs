use std::collections::BTreeMap;
use std::ffi::OsString;

use cargo::util::data_structures::HashMap;

use crate::project_model_error::ProjectModelError;

pub fn utf8_environment(
    environment: &BTreeMap<OsString, OsString>,
) -> Result<HashMap<String, String>, ProjectModelError> {
    environment
        .iter()
        .map(|(name, value)| {
            name.clone()
                .into_string()
                .map_err(ProjectModelError::NonUtf8EnvironmentVariableName)
                .and_then(|name| {
                    value
                        .clone()
                        .into_string()
                        .map_err(|value| ProjectModelError::NonUtf8EnvironmentVariableValue {
                            name: name.clone(),
                            value,
                        })
                        .map(|value| (name, value))
                })
        })
        .collect()
}
