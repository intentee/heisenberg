use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;

use cargo::GlobalContext;
use cargo::context::CargoBuildConfig;
use cargo::context::ConfigRelativePath;
use cargo::context::Definition;
use cargo::context::EnvConfigValue;
use cargo::context::OptValue;
use cargo::context::StringList;
use cargo::context::TargetCfgConfig;
use cargo::context::Value;

use crate::constants::CARGO_BUILD_TABLE;
use crate::constants::CARGO_ENVIRONMENT_TABLE;
use crate::constants::CARGO_RUSTC;
use crate::constants::CARGO_RUSTC_WORKSPACE_WRAPPER;
use crate::constants::CARGO_RUSTC_WRAPPER;
use crate::constants::CARGO_RUSTFLAGS;
use crate::constants::CARGO_TARGET_TABLE;
use crate::constants::CLIPPY_ARGUMENTS_VARIABLE;
use crate::constants::CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE;
use crate::constants::COMPILER_VARIABLES;
use crate::constants::NIGHTLY_BOOTSTRAP_VARIABLE;
use crate::constants::NIGHTLY_FEATURE_VARIABLES;
use crate::constants::RUSTFLAGS_VARIABLES;
use crate::constants::SUPPORTED_TARGETS;
use crate::project_model_error::ProjectModelError;

fn target_rustflags_key(target: &str) -> String {
    format!("{CARGO_TARGET_TABLE}.{target}.{CARGO_RUSTFLAGS}")
}

fn rejected<TValue>(
    configured: OptValue<TValue>,
    error: impl FnOnce(Definition) -> ProjectModelError,
) -> Result<(), ProjectModelError> {
    configured.map_or(Ok(()), |Value { definition, .. }| Err(error(definition)))
}

fn reject_compiler(
    key: &str,
    compiler: Option<&ConfigRelativePath>,
) -> Result<(), ProjectModelError> {
    rejected(
        compiler.map(ConfigRelativePath::value).cloned(),
        |definition| ProjectModelError::ConfiguredCompiler {
            definition,
            key: format!("{CARGO_BUILD_TABLE}.{key}"),
        },
    )
}

fn reject_rustflags(key: String, rustflags: OptValue<StringList>) -> Result<(), ProjectModelError> {
    rejected(rustflags, |definition| {
        ProjectModelError::ConfiguredRustflags { definition, key }
    })
}

pub struct CargoConfigurationOverrides<'configuration> {
    pub context: &'configuration GlobalContext,
    pub manifest_path: &'configuration Path,
}

impl CargoConfigurationOverrides<'_> {
    pub fn reject(&self) -> Result<(), ProjectModelError> {
        RUSTFLAGS_VARIABLES
            .into_iter()
            .try_for_each(|variable| {
                self.reject_environment_variable(
                    variable,
                    ProjectModelError::EnvironmentRustflags { variable },
                )
            })
            .and_then(|()| {
                COMPILER_VARIABLES.into_iter().try_for_each(|variable| {
                    self.reject_environment_variable(
                        variable,
                        ProjectModelError::EnvironmentCompiler { variable },
                    )
                })
            })
            .and_then(|()| {
                self.reject_environment_variable(
                    CLIPPY_ARGUMENTS_VARIABLE,
                    ProjectModelError::EnvironmentClippyArguments,
                )
            })
            .and_then(|()| {
                self.reject_environment_variable(
                    CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE,
                    ProjectModelError::EnvironmentClippyConfigurationDirectory,
                )
            })
            .and_then(|()| {
                NIGHTLY_FEATURE_VARIABLES
                    .into_iter()
                    .try_for_each(|variable| {
                        self.reject_environment_variable(
                            variable,
                            ProjectModelError::EnvironmentNightlyFeatures { variable },
                        )
                    })
            })
            .and_then(|()| {
                self.reject_configured_rustflags(format!("{CARGO_BUILD_TABLE}.{CARGO_RUSTFLAGS}"))
            })
            .and_then(|()| {
                self.context
                    .get::<BTreeMap<String, TargetCfgConfig>>(CARGO_TARGET_TABLE)
                    .map_err(self.unreadable_value(CARGO_TARGET_TABLE))
            })
            .and_then(|targets| {
                targets.into_iter().try_for_each(|(target, configuration)| {
                    reject_rustflags(target_rustflags_key(&target), configuration.rustflags)
                })
            })
            .and_then(|()| {
                SUPPORTED_TARGETS.into_iter().try_for_each(|target| {
                    self.reject_configured_rustflags(target_rustflags_key(target.triple()))
                })
            })
            .and_then(|()| {
                self.context
                    .build_config()
                    .map_err(self.unreadable_value(CARGO_BUILD_TABLE))
            })
            .and_then(
                |CargoBuildConfig {
                     rustc,
                     rustc_workspace_wrapper,
                     rustc_wrapper,
                     ..
                 }| {
                    reject_compiler(CARGO_RUSTC, rustc.as_ref())
                        .and_then(|()| {
                            reject_compiler(
                                CARGO_RUSTC_WORKSPACE_WRAPPER,
                                rustc_workspace_wrapper.as_ref(),
                            )
                        })
                        .and_then(|()| reject_compiler(CARGO_RUSTC_WRAPPER, rustc_wrapper.as_ref()))
                },
            )
            .and_then(|()| {
                self.reject_configured_environment(CLIPPY_ARGUMENTS_VARIABLE, |definition| {
                    ProjectModelError::ConfiguredClippyArguments { definition }
                })
            })
            .and_then(|()| {
                self.reject_configured_environment(
                    CLIPPY_CONFIGURATION_DIRECTORY_VARIABLE,
                    |definition| ProjectModelError::ConfiguredClippyConfigurationDirectory {
                        definition,
                    },
                )
            })
            .and_then(|()| {
                self.reject_configured_environment(NIGHTLY_BOOTSTRAP_VARIABLE, |definition| {
                    ProjectModelError::ConfiguredNightlyBootstrap { definition }
                })
            })
    }

    fn reject_configured_environment(
        &self,
        variable: &str,
        error: impl FnOnce(Definition) -> ProjectModelError,
    ) -> Result<(), ProjectModelError> {
        self.context
            .env_config()
            .map_err(self.unreadable_value(CARGO_ENVIRONMENT_TABLE))
            .map(|configured| {
                configured
                    .keys()
                    .find(|name| name.eq_ignore_ascii_case(variable))
                    .map(|name| format!("{CARGO_ENVIRONMENT_TABLE}.{name}"))
            })
            .and_then(|key| {
                key.map_or(Ok(()), |key| {
                    self.context
                        .get::<OptValue<EnvConfigValue>>(&key)
                        .map_err(self.unreadable_value(&key))
                        .and_then(|configured| rejected(configured, error))
                })
            })
    }

    fn reject_configured_rustflags(&self, key: String) -> Result<(), ProjectModelError> {
        self.context
            .get::<OptValue<StringList>>(&key)
            .map_err(self.unreadable_value(&key))
            .and_then(|rustflags| reject_rustflags(key, rustflags))
    }

    fn reject_environment_variable(
        &self,
        variable: &str,
        error: ProjectModelError,
    ) -> Result<(), ProjectModelError> {
        self.context
            .get_env_os(variable)
            .map_or(Ok(()), |_| Err(error))
    }

    fn unreadable_value<TSource: Into<Box<dyn Error + Send + Sync>>>(
        &self,
        key: &str,
    ) -> impl FnOnce(TSource) -> ProjectModelError {
        let key = key.to_owned();
        let manifest_path = self.manifest_path.to_path_buf();

        move |source| ProjectModelError::CargoConfigurationValue {
            key,
            manifest_path,
            source: source.into(),
        }
    }
}
