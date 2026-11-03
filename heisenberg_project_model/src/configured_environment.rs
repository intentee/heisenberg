use std::collections::BTreeMap;

use cargo::GlobalContext;
use cargo::context::EnvConfig;
use cargo::context::EnvConfigValue;
use cargo::util::data_structures::HashMap;

use heisenberg_error_context::error_context::error_context;

use crate::constants::CARGO_ENVIRONMENT_TABLE;
use crate::project_model_error::ProjectModelError;

pub struct ConfiguredEnvironment<'environment> {
    pub context: &'environment GlobalContext,
    pub environment: &'environment HashMap<String, String>,
}

impl ConfiguredEnvironment<'_> {
    pub fn variables(&self) -> Result<BTreeMap<String, String>, ProjectModelError> {
        self.context
            .get::<EnvConfig>(CARGO_ENVIRONMENT_TABLE)
            .map_err(Into::into)
            .map_err(ProjectModelError::CargoEnvironmentTable)
            .and_then(|declared| {
                declared.into_iter().try_fold(
                    BTreeMap::new(),
                    |mut variables, (name, configured)| {
                        self.value(&name, &configured).map(|value| {
                            variables.insert(name, value);

                            variables
                        })
                    },
                )
            })
    }

    fn value(&self, name: &str, configured: &EnvConfigValue) -> Result<String, ProjectModelError> {
        match self.environment.get(name) {
            Some(inherited) if !configured.is_force() => Ok(inherited.clone()),
            Some(_) | None => configured
                .resolve(self.context.cwd())
                .into_owned()
                .into_string()
                .map_err(error_context(
                    name.to_owned(),
                    ProjectModelError::NonUtf8ConfiguredEnvironmentValue,
                )),
        }
    }
}
