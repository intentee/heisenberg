use std::sync::Arc;
use std::thread::scope;

use heisenberg_analysis_threads::analysis_thread::AnalysisThread;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::joined::joined;
use heisenberg_analysis_threads::worker_pool::WorkerPool;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::loaded_workspace::LoadedWorkspace;
use heisenberg_project_model::project_model::ProjectModel;
use heisenberg_project_model::project_model_error::ProjectModelError;
use heisenberg_project_model::resolved_project::ResolvedProject;
use heisenberg_project_model::run_scope::RunScope;
use heisenberg_workspace_build::build_outputs::BuildOutputs;

use crate::heisenberg_command_error::HeisenbergCommandError;

fn configuration_failure(
    configuration: &Configuration,
) -> impl FnOnce(ProjectModelError) -> HeisenbergCommandError {
    let configuration = configuration.clone();

    move |source| HeisenbergCommandError::ConfigurationProjectModel {
        configuration,
        source: Box::new(source),
    }
}

fn loaded<'resolved>(
    project: &'resolved ResolvedProject<'_>,
    configuration: &Configuration,
) -> Result<LoadedWorkspace<'resolved>, HeisenbergCommandError> {
    project
        .load(configuration)
        .map_err(configuration_failure(configuration))
}

fn analysed<TPipeline: ConfigurationPipeline + ?Sized>(
    pipeline: &TPipeline,
    configuration: &Configuration,
    workspace: LoadedWorkspace<'_>,
    outputs: &BuildOutputs,
    scope: &RunScope,
    prerequisite: &TPipeline::Prerequisite,
) -> Result<TPipeline::Analysis, HeisenbergCommandError> {
    workspace
        .built(outputs)
        .map_err(configuration_failure(configuration))
        .and_then(|mut project| {
            let analysis = pipeline.analysis(configuration, &mut project, scope, prerequisite);
            let LoadedProject { database, .. } = project;

            WorkerPool::release(database);

            analysis
        })
}

fn configuration_thread<'scope, TResult: Send + 'scope>(
    threads: &AnalysisThreads<'scope, '_>,
    configuration: &Configuration,
    work: impl FnOnce() -> Result<TResult, HeisenbergCommandError> + Send + 'scope,
) -> Result<AnalysisThread<'scope, Result<TResult, HeisenbergCommandError>>, HeisenbergCommandError>
{
    threads
        .spawn(work)
        .map_err(|source| HeisenbergCommandError::AnalysisThread {
            configuration: configuration.clone(),
            source,
        })
}

fn every_analysis<TAnalysis>(
    outcomes: Vec<Result<TAnalysis, HeisenbergCommandError>>,
) -> Result<Vec<TAnalysis>, HeisenbergCommandError> {
    let mut analyses = Vec::new();
    let mut failures = Vec::new();

    for outcome in outcomes {
        match outcome {
            Ok(completed) => analyses.push(completed),
            Err(failure) => failures.push(failure),
        }
    }

    if failures.is_empty() {
        Ok(analyses)
    } else {
        Err(HeisenbergCommandError::Failures(failures))
    }
}

pub trait ConfigurationPipeline: Sync {
    type Analysis: Send;
    type Prerequisite: Send + Sync;
    type Report;

    fn analysis(
        &self,
        configuration: &Configuration,
        project: &mut LoadedProject<'_>,
        scope: &RunScope,
        prerequisite: &Self::Prerequisite,
    ) -> Result<Self::Analysis, HeisenbergCommandError>;

    fn prerequisite(
        &self,
        project: &ResolvedProject<'_>,
    ) -> Result<Self::Prerequisite, HeisenbergCommandError>;

    fn report(
        &self,
        project: &ResolvedProject<'_>,
        analyses: Vec<Self::Analysis>,
        prerequisite: &Self::Prerequisite,
    ) -> Self::Report;

    fn run(&self, model: &ProjectModel) -> Result<Self::Report, HeisenbergCommandError> {
        WorkerPool::start(model.jobs).map_err(HeisenbergCommandError::WorkerPool)?;

        let project = model
            .resolve()
            .map_err(HeisenbergCommandError::ProjectModel)?;
        let project = &project;

        scope(|scope| {
            let threads = AnalysisThreads::new(scope);
            let loading = project
                .configurations
                .keys()
                .map(|configuration| {
                    let owned_configuration = configuration.clone();

                    configuration_thread(&threads, configuration, move || {
                        loaded(project, &owned_configuration)
                    })
                })
                .collect();
            let reading = threads
                .spawn(|| self.prerequisite(project))
                .map_err(HeisenbergCommandError::PrerequisiteThread);
            let builds = project.build_outputs();
            let prerequisite = reading.and_then(AnalysisThread::join);
            let loads = joined(loading);

            builds
                .and_then(|outputs| project.resolve_standard_libraries().map(|()| outputs))
                .map_err(HeisenbergCommandError::ProjectModel)
                .and_then(|outputs| {
                    prerequisite.and_then(|prerequisite| {
                        let shared = Arc::new(prerequisite);

                        every_analysis(joined(
                            outputs
                                .into_iter()
                                .zip(loads)
                                .map(|((configuration, build), load)| {
                                    let shared = Arc::clone(&shared);

                                    load.and_then(|workspace| {
                                        configuration_thread(
                                            &threads,
                                            &configuration.clone(),
                                            move || {
                                                analysed(
                                                    self,
                                                    &configuration,
                                                    workspace,
                                                    &build,
                                                    &project.scope,
                                                    &shared,
                                                )
                                            },
                                        )
                                    })
                                })
                                .collect(),
                        ))
                        .map(|analyses| self.report(project, analyses, &shared))
                    })
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::error::Error;
    use std::iter;
    use std::thread;
    use std::thread::Builder;

    use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
    use heisenberg_analysis_threads::constants::UNRESERVABLE_STACK_SIZE;
    use heisenberg_project_model::configuration::Configuration;
    use heisenberg_project_model::feature_selection::FeatureSelection;
    use heisenberg_project_model::supported_target::SupportedTarget;

    use super::configuration_thread;
    use crate::heisenberg_command_error::HeisenbergCommandError;

    #[test]
    fn configuration_whose_thread_cannot_start_fails_with_its_name() {
        let configuration = Configuration {
            name: "unstartable".to_owned(),
            target: SupportedTarget::X86_64UnknownLinuxGnu,
            features: FeatureSelection {
                default_features: true,
                features: BTreeSet::new(),
            },
        };

        thread::scope(|scope| {
            let refused = Builder::new()
                .stack_size(UNRESERVABLE_STACK_SIZE)
                .spawn_scoped(scope, || ())
                .err()
                .map(|error| error.to_string())
                .expect("the operating system must refuse a stack it cannot reserve");
            let failure = configuration_thread(
                &AnalysisThreads {
                    scope,
                    stack_size: UNRESERVABLE_STACK_SIZE,
                },
                &configuration,
                || Ok::<(), HeisenbergCommandError>(()),
            )
            .err()
            .map(|error| {
                iter::successors(Some(&error as &dyn Error), |cause| (*cause).source())
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
            });

            assert_eq!(
                failure,
                Some(vec![
                    "the analysis thread for the configuration `unstartable` cannot be started"
                        .to_owned(),
                    "the operating system cannot start a thread".to_owned(),
                    refused,
                ])
            );
        });
    }
}
