use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_project_model::configuration::Configuration;

use crate::module_inclusion::ModuleInclusion;
use crate::shared_file::SharedFile;

pub struct SharedFileTree<'files> {
    including: BTreeMap<&'files ModuleInclusion, SharedFile<'files>>,
    roots: RefCell<BTreeMap<&'files str, SharedFile<'files>>>,
}

impl<'files> SharedFileTree<'files> {
    #[must_use]
    pub fn new(
        files: &'files BTreeMap<
            &'files str,
            BTreeMap<ModuleInclusion, BTreeMap<Configuration, ModuleFileAnalysis>>,
        >,
    ) -> Self {
        let mut including = BTreeMap::new();

        for (file, inclusions) in files {
            for inclusion in inclusions.keys() {
                including
                    .entry(inclusion)
                    .or_insert(SharedFile { file, inclusions });
            }
        }

        Self {
            including,
            roots: RefCell::default(),
        }
    }

    #[must_use]
    pub fn parent(
        &self,
        SharedFile { inclusions, .. }: SharedFile<'files>,
    ) -> Option<SharedFile<'files>> {
        let parents: BTreeMap<ModuleInclusion, BTreeSet<Configuration>> = inclusions
            .iter()
            .filter_map(|(inclusion, analyses)| {
                inclusion.module.parent().map(|module| {
                    (
                        ModuleInclusion {
                            crate_target: inclusion.crate_target.clone(),
                            module,
                        },
                        analyses.keys().cloned().collect(),
                    )
                })
            })
            .collect();
        let candidates: BTreeMap<&'files str, SharedFile<'files>> = parents
            .keys()
            .filter_map(|parent| self.including.get(parent))
            .map(|candidate| (candidate.file, *candidate))
            .collect();

        candidates
            .first_key_value()
            .map(|(_, candidate)| *candidate)
            .filter(
                |SharedFile {
                     inclusions: candidate_inclusions,
                     ..
                 }| {
                    candidates.len() == 1
                        && parents.len() == inclusions.len()
                        && candidate_inclusions.len() == parents.len()
                        && inclusions
                            .values()
                            .flat_map(BTreeMap::values)
                            .all(|analysis| *analysis == ModuleFileAnalysis::Analysed)
                        && parents.iter().all(|(parent, configurations)| {
                            candidate_inclusions.get(parent).is_some_and(|analyses| {
                                analyses
                                    .keys()
                                    .cloned()
                                    .collect::<BTreeSet<Configuration>>()
                                    == *configurations
                            })
                        })
                },
            )
    }

    #[must_use]
    pub fn root(&self, shared: SharedFile<'files>) -> SharedFile<'files> {
        let known = self.roots.borrow().get(shared.file).copied();

        known.unwrap_or_else(|| {
            let root = self
                .parent(shared)
                .map_or(shared, |parent| self.root(parent));

            self.roots.borrow_mut().insert(shared.file, root);

            root
        })
    }
}
