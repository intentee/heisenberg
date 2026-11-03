use std::collections::BTreeMap;
use std::collections::BTreeSet;

use heisenberg_module_graph::module_file_analysis::ModuleFileAnalysis;
use heisenberg_project_model::supported_target::SupportedTarget;

use crate::module_inclusion::ModuleInclusion;
use crate::shared_file::SharedFile;

pub struct SharedFileTree<'files> {
    pub files: &'files BTreeMap<
        &'files str,
        BTreeMap<ModuleInclusion, BTreeMap<SupportedTarget, ModuleFileAnalysis>>,
    >,
}

impl<'files> SharedFileTree<'files> {
    #[must_use]
    pub fn parent(
        &self,
        SharedFile { inclusions, .. }: SharedFile<'files>,
    ) -> Option<SharedFile<'files>> {
        let parents: BTreeMap<ModuleInclusion, BTreeSet<SupportedTarget>> = inclusions
            .iter()
            .filter_map(|(inclusion, analyses)| {
                inclusion.module.parent().map(|module| {
                    (
                        ModuleInclusion {
                            crate_target: inclusion.crate_target.clone(),
                            module,
                        },
                        analyses.keys().copied().collect(),
                    )
                })
            })
            .collect();
        let candidates: BTreeMap<&'files str, SharedFile<'files>> = parents
            .keys()
            .filter_map(|parent| {
                self.files
                    .iter()
                    .find(|(_, candidate)| candidate.contains_key(parent))
                    .map(|(file, inclusions)| (*file, SharedFile { file, inclusions }))
            })
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
                        && parents.iter().all(|(parent, targets)| {
                            candidate_inclusions.get(parent).is_some_and(|analyses| {
                                analyses
                                    .keys()
                                    .copied()
                                    .collect::<BTreeSet<SupportedTarget>>()
                                    == *targets
                            })
                        })
                },
            )
    }

    #[must_use]
    pub fn root(&self, shared: SharedFile<'files>) -> SharedFile<'files> {
        self.parent(shared)
            .map_or(shared, |parent| self.root(parent))
    }
}
