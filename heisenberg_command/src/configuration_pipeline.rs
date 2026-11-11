use std::sync::Arc;
use std::thread::scope;

use heisenberg_analysis_threads::analysis_thread::AnalysisThread;
use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::joined::joined;
use heisenberg_analysis_threads::worker_pool::WorkerPool;
use heisenberg_error_context::error_context::error_context;
use heisenberg_project_model::configuration::Configuration;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::loaded_workspace::LoadedWorkspace;
use heisenberg_project_model::project_model::ProjectModel;
use heisenberg_project_model::project_model_error::ProjectModelError;
use heisenberg_project_model::resolved_project::ResolvedProject;
use heisenberg_project_model::target_model::TargetModel;
use heisenberg_workspace_build::build_outputs::BuildOutputs;

use crate::heisenberg_command_error::HeisenbergCommandError;

fn configuration_failure(
    configuration: &Configuration,
) -> impl FnOnce(ProjectModelError) -> HeisenbergCommandError {
    let failure = error_context(
        configuration.clone(),
        HeisenbergCommandError::ConfigurationProjectModel,
    );

    |error| failure(Box::new(error))
}

fn project_model_failure(error: ProjectModelError) -> Vec<HeisenbergCommandError> {
    vec![HeisenbergCommandError::ProjectModel(error)]
}

fn loaded<'resolved>(
    project: &'resolved ResolvedProject<'_>,
    configuration: &Configuration,
    target: &Arc<TargetModel>,
) -> Result<LoadedWorkspace<'resolved>, HeisenbergCommandError> {
    project
        .load(configuration, target)
        .map_err(configuration_failure(configuration))
}

fn analysed<TPipeline: ConfigurationPipeline + ?Sized>(
    pipeline: &TPipeline,
    configuration: &Configuration,
    workspace: LoadedWorkspace<'_>,
    outputs: &BuildOutputs,
    prerequisite: &TPipeline::Prerequisite,
) -> Result<TPipeline::Analysis, HeisenbergCommandError> {
    workspace
        .built(outputs)
        .map_err(configuration_failure(configuration))
        .and_then(|mut project| {
            let analysis = pipeline.analysis(configuration, &mut project, prerequisite);
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
    threads.spawn(work).map_err(error_context(
        configuration.clone(),
        HeisenbergCommandError::AnalysisThread,
    ))
}

fn every_analysis<TAnalysis>(
    outcomes: Vec<Result<TAnalysis, HeisenbergCommandError>>,
) -> Result<Vec<TAnalysis>, Vec<HeisenbergCommandError>> {
    if outcomes.iter().all(Result::is_ok) {
        Ok(outcomes.into_iter().flatten().collect())
    } else {
        Err(outcomes.into_iter().filter_map(Result::err).collect())
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

    fn run(&self, model: &ProjectModel) -> Result<Self::Report, Vec<HeisenbergCommandError>> {
        WorkerPool::start(model.jobs)
            .map_err(|error| vec![HeisenbergCommandError::WorkerPool(error)])?;

        let project = model.resolve().map_err(project_model_failure)?;
        let project = &project;
        let targets = project.target_models();

        scope(|scope| {
            let threads = AnalysisThreads { scope };
            let loading = targets
                .iter()
                .flatten()
                .map(|(configuration, target)| {
                    let owned_configuration = configuration.clone();
                    let target = Arc::clone(target);

                    configuration_thread(&threads, configuration, move || {
                        loaded(project, &owned_configuration, &target)
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
                .and_then(|outputs| targets.map(|_| outputs))
                .map_err(project_model_failure)
                .and_then(|outputs| {
                    prerequisite
                        .map_err(|error| vec![error])
                        .and_then(|prerequisite| {
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
