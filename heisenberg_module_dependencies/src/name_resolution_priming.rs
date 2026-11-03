use std::collections::BTreeMap;
use std::collections::HashMap;
use std::thread::available_parallelism;
use std::thread::scope;

use ra_ap_hir::Crate;
use ra_ap_hir::CrateDependency;
use ra_ap_ide_db::RootDatabase;

use heisenberg_analysis_threads::analysis_threads::AnalysisThreads;
use heisenberg_analysis_threads::panic_payload::PanicPayload;
use heisenberg_error_context::error_context::error_context;

use crate::module_dependencies_error::ModuleDependenciesError;
use crate::priming_thread::PrimingThread;

pub struct NameResolutionPriming<'priming> {
    pub database: &'priming RootDatabase,
    pub members: &'priming [Crate],
}

impl NameResolutionPriming<'_> {
    pub fn prime(&self) -> Result<(), ModuleDependenciesError> {
        available_parallelism()
            .map_err(ModuleDependenciesError::from)
            .and_then(|workers| {
                self.layers().values().try_for_each(|layer| {
                    scope(|scope| {
                        let threads = AnalysisThreads { scope };
                        let failures: Vec<ModuleDependenciesError> = layer
                            .chunks(layer.len().div_ceil(workers.get()))
                            .map(|part| {
                                let crates: Vec<String> = part
                                    .iter()
                                    .filter_map(|krate| krate.display_name(self.database))
                                    .map(|name| name.to_string())
                                    .collect();
                                let database = self.database.clone();

                                threads
                                    .spawn(move || {
                                        for krate in part {
                                            krate.root_module(&database);
                                        }
                                    })
                                    .map_err(error_context(
                                        crates.clone(),
                                        ModuleDependenciesError::NameResolutionThread,
                                    ))
                                    .map(|handle| PrimingThread { crates, handle })
                            })
                            .collect::<Vec<Result<PrimingThread<'_>, ModuleDependenciesError>>>()
                            .into_iter()
                            .map(|spawned| {
                                spawned.and_then(|PrimingThread { crates, handle }| {
                                    handle.join().map_err(PanicPayload).map_err(error_context(
                                        crates,
                                        ModuleDependenciesError::NameResolutionPanicked,
                                    ))
                                })
                            })
                            .collect::<Vec<Result<(), ModuleDependenciesError>>>()
                            .into_iter()
                            .filter_map(Result::err)
                            .collect();
                        let primed = failures.is_empty();

                        primed
                            .then_some(())
                            .ok_or(ModuleDependenciesError::NameResolution(failures))
                    })
                })
            })
    }

    fn depth(
        &self,
        krate: Crate,
        depths: &mut HashMap<Crate, usize>,
        layers: &mut BTreeMap<usize, Vec<Crate>>,
    ) -> usize {
        if let Some(known) = depths.get(&krate) {
            return *known;
        }

        let depth = krate.dependencies(self.database).into_iter().fold(
            0,
            |deepest, CrateDependency { krate, .. }| {
                deepest.max(self.depth(krate, depths, layers) + 1)
            },
        );

        depths.insert(krate, depth);
        layers.entry(depth).or_default().push(krate);

        depth
    }

    fn layers(&self) -> BTreeMap<usize, Vec<Crate>> {
        let mut depths = HashMap::new();
        let mut layers = BTreeMap::new();

        for member in self.members {
            self.depth(*member, &mut depths, &mut layers);
        }

        layers
    }
}
