use std::collections::BTreeSet;
use std::time::Duration;

use crate::constants::MAXIMUM_SLOW_TEST_PERIOD;
use crate::constants::MAXIMUM_TEST_DURATION;
use crate::constants::NEXTEST_DEFAULT_PROFILE;
use crate::constants::VETTED_NEXTEST_PROFILE_SETTINGS;
use crate::constants::VETTED_NEXTEST_SETTINGS;
use crate::nextest_configuration::NextestConfiguration;
use crate::nextest_configuration_file::NextestConfigurationFile;
use crate::nextest_duration::NextestDuration;
use crate::nextest_period_setting::NextestPeriodSetting;
use crate::nextest_presence::NextestPresence;
use crate::nextest_profile::NextestProfile;
use crate::nextest_review::NextestReview;
use crate::nextest_slow_timeout::NextestSlowTimeout;
use crate::nextest_termination::NextestTermination;
use crate::nextest_timeout_limits::NextestTimeoutLimits;
use crate::nextest_timeout_result::NextestTimeoutResult;
use crate::package_violation::PackageViolation;
use crate::project_model_error::ProjectModelError;
use crate::resolved_packages::ResolvedPackages;

fn configuration_violations(
    file: &str,
    NextestConfiguration { profile, settings }: &NextestConfiguration,
) -> BTreeSet<PackageViolation> {
    settings
        .keys()
        .filter(|setting| !VETTED_NEXTEST_SETTINGS.contains(&setting.as_str()))
        .map(|setting| PackageViolation::UnvettedNextestSetting {
            file: file.to_owned(),
            setting: setting.clone(),
        })
        .chain(
            profile
                .get(NEXTEST_DEFAULT_PROFILE)
                .is_none_or(|NextestProfile { slow_timeout, .. }| {
                    matches!(
                        slow_timeout,
                        NextestSlowTimeout::Text(NextestPeriodSetting::Inherited)
                    )
                })
                .then(|| PackageViolation::NextestSlowTimeoutUnset {
                    file: file.to_owned(),
                }),
        )
        .chain(
            profile
                .iter()
                .flat_map(|(name, profile)| profile_violations(file, name, profile)),
        )
        .collect()
}

fn period_violation(file: &str, profile: &str, period: Duration) -> Option<PackageViolation> {
    (period > MAXIMUM_SLOW_TEST_PERIOD).then(|| PackageViolation::NextestSlowPeriodTooLong {
        file: file.to_owned(),
        period,
        profile: profile.to_owned(),
    })
}

fn profile_violations(
    file: &str,
    name: &str,
    NextestProfile {
        overrides,
        settings,
        slow_timeout,
    }: &NextestProfile,
) -> Vec<PackageViolation> {
    (!overrides.is_empty())
        .then(|| PackageViolation::NextestOverride {
            file: file.to_owned(),
            profile: name.to_owned(),
        })
        .into_iter()
        .chain(
            settings
                .keys()
                .filter(|setting| !VETTED_NEXTEST_PROFILE_SETTINGS.contains(&setting.as_str()))
                .map(|setting| PackageViolation::UnvettedNextestSetting {
                    file: file.to_owned(),
                    setting: format!("profile.{name}.{setting}"),
                }),
        )
        .chain(slow_timeout_violations(file, name, slow_timeout))
        .collect()
}

fn slow_timeout_violations(
    file: &str,
    profile: &str,
    slow_timeout: &NextestSlowTimeout,
) -> Vec<PackageViolation> {
    let never_terminated = || PackageViolation::NextestTestNeverTerminated {
        file: file.to_owned(),
        profile: profile.to_owned(),
    };

    match slow_timeout {
        NextestSlowTimeout::Text(NextestPeriodSetting::Inherited) => Vec::new(),
        NextestSlowTimeout::Text(NextestPeriodSetting::Period { period }) => {
            period_violation(file, profile, *period)
                .into_iter()
                .chain([never_terminated()])
                .collect()
        }
        NextestSlowTimeout::Limits(NextestTimeoutLimits {
            on_timeout,
            period: NextestDuration { duration: period },
            settings,
            terminate_after,
        }) => period_violation(file, profile, *period)
            .into_iter()
            .chain(match terminate_after {
                NextestTermination::AfterPeriods { periods } => period
                    .as_nanos()
                    .checked_mul(u128::from(periods.get()))
                    .is_none_or(|total| total > MAXIMUM_TEST_DURATION.as_nanos())
                    .then(|| PackageViolation::NextestTestTerminatedTooLate {
                        file: file.to_owned(),
                        period: *period,
                        periods: *periods,
                        profile: profile.to_owned(),
                    }),
                NextestTermination::Never => Some(never_terminated()),
            })
            .chain((*on_timeout == NextestTimeoutResult::Pass).then(|| {
                PackageViolation::NextestTimeoutPasses {
                    file: file.to_owned(),
                    profile: profile.to_owned(),
                }
            }))
            .chain(
                settings
                    .keys()
                    .map(|setting| PackageViolation::UnvettedNextestSetting {
                        file: file.to_owned(),
                        setting: format!("profile.{profile}.slow-timeout.{setting}"),
                    }),
            )
            .collect(),
    }
}

pub struct NextestRequirements<'requirements> {
    pub resolved: &'requirements ResolvedPackages,
}

impl NextestRequirements<'_> {
    /// # Errors
    ///
    /// Returns [`ProjectModelError`] when the nextest configuration of the workspace
    /// cannot be read or is not a valid nextest configuration.
    pub fn review(&self) -> Result<NextestReview, ProjectModelError> {
        let workspace_path = self.resolved.workspace_path();

        NextestConfigurationFile::read(workspace_path.root).map(|configuration_file| {
            match configuration_file {
                NextestConfigurationFile::Missing => NextestReview {
                    presence: NextestPresence::Missing {
                        manifest: workspace_path.manifest(),
                    },
                    violations: Vec::new(),
                },
                NextestConfigurationFile::Present {
                    configuration,
                    file,
                } => {
                    let relative = workspace_path.relative(&file);

                    NextestReview {
                        violations: configuration_violations(&relative, &configuration)
                            .into_iter()
                            .collect(),
                        presence: NextestPresence::Present { file: relative },
                    }
                }
            }
        })
    }
}
