use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_hir::attach_db;
use ra_ap_ide_db::FilePosition;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::SourceDatabase;
use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::FileId;
use rayon::join;

use heisenberg_attached_work::attached_work::attached_work;
use heisenberg_configured_code::test_module_code::TestModuleCode;
use heisenberg_configured_code::test_modules::TestModules;
use heisenberg_coverage_export::exported_coverage::ExportedCoverage;
use heisenberg_coverage_export::exported_region::ExportedRegion;
use heisenberg_member_crates::member_crates::member_crates;
use heisenberg_member_crates::module_files::module_files;
use heisenberg_member_crates::resolved_member_crate::ResolvedMemberCrate;
use heisenberg_module_graph::code_category::CodeCategory;
use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_project_model::loaded_project::LoadedProject;
use heisenberg_project_model::source_files::SourceFiles;
use heisenberg_source_locator::source_locator::SourceLocator;
use heisenberg_task_graph::once_map::OnceMap;

use crate::active_code::ActiveCode;
use crate::analysis_context::AnalysisContext;
use crate::attributed_region::AttributedRegion;
use crate::coverage_measurement_error::CoverageMeasurementError;
use crate::file_measurement::FileMeasurement;
use crate::function_collection::FunctionCollection;
use crate::function_identity::FunctionIdentity;
use crate::measured_code::MeasuredCode;
use crate::measured_coverage::MeasuredCoverage;
use crate::measured_file::MeasuredFile;
use crate::measured_function::MeasuredFunction;
use crate::measured_functions::MeasuredFunctions;
use crate::measured_item::MeasuredItem;
use crate::measured_region::MeasuredRegion;
use crate::measurement_run::MeasurementRun;
use crate::member_modules::MemberModules;
use crate::region_owner::RegionOwner;
use crate::source_freshness::SourceFreshness;
use crate::unresolved_attributes::UnresolvedAttributes;

fn exported_file<'paths>(
    paths: BTreeSet<&'paths AbsPath>,
    exported_files: &BTreeMap<AbsPathBuf, Vec<ExportedRegion>>,
) -> Option<&'paths AbsPath> {
    paths
        .into_iter()
        .find(|path| exported_files.contains_key(*path))
}

fn in_context<TItem: Sync, TResult: Send>(
    database: &RootDatabase,
    files: &SourceFiles,
    MeasurementRun {
        active_code,
        measured_files: measured_file_ids,
        measured_positions,
        test_modules,
    }: &MeasurementRun,
    items: &[TItem],
    work: impl Fn(&AnalysisContext<'_, '_>, &TItem) -> Result<TResult, CoverageMeasurementError>
    + Send
    + Sync,
) -> Result<Vec<TResult>, CoverageMeasurementError> {
    attached_work(database, items, |item_database, item| {
        let semantics = Semantics::new(item_database);

        work(
            &AnalysisContext {
                active_code,
                identity: &FunctionIdentity {
                    measured_files: measured_file_ids,
                    measured_positions,
                    semantics: &semantics,
                },
                locator: &SourceLocator {
                    database: item_database,
                    files,
                },
                test_modules: &TestModuleCode {
                    modules: test_modules,
                    semantics: &semantics,
                },
            },
            item,
        )
    })
    .into_iter()
    .collect()
}

fn measured_files<'project>(
    locator: &'project SourceLocator<'project>,
    measured: &[&ResolvedMemberCrate],
) -> Result<Vec<MeasuredFile<'project>>, CoverageMeasurementError> {
    let mut seen = BTreeSet::new();

    measured
        .iter()
        .flat_map(
            |ResolvedMemberCrate {
                 files: member_files,
                 krate,
                 ..
             }| {
                member_files.iter().map(|file_id| {
                    locator.path(*file_id).map(|path| MeasuredFile {
                        file_id: *file_id,
                        krate: *krate,
                        path,
                    })
                })
            },
        )
        .filter(|candidate| {
            candidate
                .as_ref()
                .map_or(true, |MeasuredFile { file_id, .. }| seen.insert(*file_id))
        })
        .collect()
}

pub struct CoverageMeasurement<'measurement, 'resolved> {
    pub coverage: &'measurement ExportedCoverage,
    pub project: &'measurement LoadedProject<'resolved>,
}

impl CoverageMeasurement<'_, '_> {
    /// # Errors
    ///
    /// Returns [`CoverageMeasurementError`] when name resolution and trait implementations
    /// cannot be scheduled over the crate dependency graph, when the metadata or modification time of a module file of a
    /// workspace member cannot be read, when the export is older than such a file, records
    /// regions in a file of a workspace member that no crate target of the analysed
    /// configuration includes, or records a region that the current text of its file does not
    /// hold or that lies in code the analysed configuration leaves inactive, when rust-analyzer
    /// cannot resolve a macro call outside attributes, an attribute or a derive, or cannot
    /// expand or parse a macro invocation, in the code of a measured crate target outside test
    /// code, when production code or an attribute is active only in the build without
    /// `cfg(test)`, when rust-analyzer parses a function without its name, a function body
    /// without its braces or a macro call without its path, or expands an attribute macro into
    /// no macro file, when the measurement attributes a region to a function it does not
    /// measure, or when rust-analyzer refers to a file or offset that the loaded project does
    /// not hold.
    pub fn measure(&self) -> Result<MeasuredCoverage, CoverageMeasurementError> {
        let LoadedProject {
            database,
            files,
            packages,
            ..
        } = self.project;
        let locator = SourceLocator { database, files };

        member_crates(self.project)
            .map_err(CoverageMeasurementError::MemberCrates)
            .and_then(|members| {
                let module_files = module_files(members.iter());
                let measured: Vec<&ResolvedMemberCrate> = members
                    .iter()
                    .filter(|ResolvedMemberCrate { crate_target, .. }| {
                        crate_target.code_category(&packages.test_crates)
                            == CodeCategory::Production
                    })
                    .collect();

                module_files
                    .iter()
                    .map(|file_id| locator.path(*file_id))
                    .collect::<Result<Vec<&AbsPath>, CoverageMeasurementError>>()
                    .and_then(|module_paths| {
                        SourceFreshness {
                            coverage: self.coverage,
                            files: &module_paths,
                        }
                        .verify()
                    })
                    .and_then(|()| measured_files(&locator, &measured))
                    .and_then(|measured_files| {
                        self.measured_coverage(
                            &MemberModules {
                                members: &members,
                                module_files: &module_files,
                            },
                            &measured,
                            &measured_files,
                        )
                    })
            })
    }

    fn assembled(
        MeasuredFunctions { names, .. }: MeasuredFunctions,
        regions: &[AttributedRegion],
        crate_targets: usize,
    ) -> Result<MeasuredCoverage, CoverageMeasurementError> {
        let mut attached: HashMap<FilePosition, Vec<MeasuredRegion>> = HashMap::new();
        let mut items: BTreeMap<SourceLocation, Vec<MeasuredRegion>> = BTreeMap::new();

        regions
            .iter()
            .try_for_each(|AttributedRegion { owner, region }| match owner {
                RegionOwner::Functions(functions) => functions.iter().try_for_each(|function| {
                    names
                        .get(function)
                        .ok_or(CoverageMeasurementError::RegionOfAnUnmeasuredFunction {
                            file_id: function.file_id,
                            offset: function.offset,
                        })
                        .map(|_| attached.entry(*function).or_default().push(region.clone()))
                }),
                RegionOwner::Item(item) => {
                    items.entry(item.clone()).or_default().push(region.clone());

                    Ok(())
                }
            })
            .map(|()| {
                let mut code = MeasuredCode {
                    functions: names
                        .into_iter()
                        .map(|(function, location)| MeasuredFunction {
                            location,
                            regions: attached.remove(&function).unwrap_or_default(),
                        })
                        .collect(),
                    items: items
                        .into_iter()
                        .map(|(location, regions)| MeasuredItem { location, regions })
                        .collect(),
                };

                code.sort();

                MeasuredCoverage {
                    code,
                    crate_targets,
                    regions: regions.len(),
                }
            })
    }

    fn measured_coverage(
        &self,
        member_modules: &MemberModules<'_>,
        measured: &[&ResolvedMemberCrate],
        measured_files: &[MeasuredFile<'_>],
    ) -> Result<MeasuredCoverage, CoverageMeasurementError> {
        let LoadedProject {
            database, files, ..
        } = self.project;
        let ExportedCoverage {
            files: exported_files,
            path: export,
            ..
        } = self.coverage;
        let run = MeasurementRun {
            active_code: ActiveCode::default(),
            measured_files: measured_files
                .iter()
                .map(|MeasuredFile { file_id, .. }| *file_id)
                .collect(),
            measured_positions: OnceMap::default(),
            test_modules: TestModules::default(),
        };
        let modules: Vec<Module> = attach_db(database, || {
            measured
                .iter()
                .flat_map(|ResolvedMemberCrate { krate, .. }| krate.modules(database))
                .filter(|module| !module.is_inline(database))
                .collect()
        });

        let measurement_run = &run;
        let diagnosis_database = database.clone();
        let collection_database = database.clone();
        let (verified, collected) = join(
            move || {
                in_context(
                    &diagnosis_database,
                    files,
                    measurement_run,
                    &modules,
                    |context, module| UnresolvedAttributes { context }.verify(*module),
                )
            },
            move || {
                in_context(
                    &collection_database,
                    files,
                    measurement_run,
                    measured_files,
                    |context, file| FunctionCollection { context, file }.collect(),
                )
            },
        );

        verified
            .and(collected)
            .map(|collected| {
                collected
                    .into_iter()
                    .fold(MeasuredFunctions::default(), |mut merged, functions| {
                        merged.merge(functions);

                        merged
                    })
            })
            .and_then(|functions| {
                self.verify_attribution(&SourceLocator { database, files }, member_modules)
                    .map(|()| functions)
            })
            .and_then(|functions| {
                in_context(database, files, &run, measured_files, |context, file| {
                    FileMeasurement {
                        context,
                        export,
                        file,
                        measured: &functions,
                        regions: exported_files.get(file.path).map_or(&[], Vec::as_slice),
                    }
                    .measure()
                })
                .and_then(|regions| {
                    Self::assembled(
                        functions,
                        &regions
                            .into_iter()
                            .flatten()
                            .collect::<Vec<AttributedRegion>>(),
                        measured.len(),
                    )
                })
            })
    }

    fn verify_attribution(
        &self,
        locator: &SourceLocator<'_>,
        MemberModules {
            members,
            module_files: member_module_files,
        }: &MemberModules<'_>,
    ) -> Result<(), CoverageMeasurementError> {
        let LoadedProject {
            build_script_output_files,
            database,
            packages,
            ..
        } = self.project;
        let ExportedCoverage {
            files: exported_files,
            path: export,
            ..
        } = self.coverage;

        attach_db(database, || {
            members
                .iter()
                .map(|ResolvedMemberCrate { krate, .. }| {
                    database
                        .file_source_root(krate.root_file(database))
                        .source_root_id(database)
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .flat_map(|source_root| {
                    database
                        .source_root(source_root)
                        .source_root(database)
                        .iter()
                        .collect::<Vec<FileId>>()
                })
                .filter(|file_id| {
                    !member_module_files.contains(file_id)
                        && !build_script_output_files.contains(file_id)
                })
                .map(|file_id| locator.path(file_id))
                .collect::<Result<BTreeSet<&AbsPath>, CoverageMeasurementError>>()
        })
        .and_then(|unattributed| {
            exported_file(unattributed, exported_files).map_or(Ok(()), |file| {
                Err(CoverageMeasurementError::RegionInFileOutsideModuleTrees {
                    export: export.clone(),
                    file: file.to_path_buf(),
                })
            })
        })
        .and_then(|()| {
            let benchmark = |ResolvedMemberCrate { crate_target, .. }: &&ResolvedMemberCrate| {
                crate_target.code_category(&packages.test_crates) == CodeCategory::Benchmark
            };
            let other_files = module_files(members.iter().filter(|member| !benchmark(member)));

            module_files(members.iter().filter(benchmark))
                .difference(&other_files)
                .map(|file_id| locator.path(*file_id))
                .collect::<Result<BTreeSet<&AbsPath>, CoverageMeasurementError>>()
        })
        .and_then(|benchmark_only| {
            exported_file(benchmark_only, exported_files).map_or(Ok(()), |file| {
                Err(CoverageMeasurementError::RegionInBenchmarkTarget {
                    export: export.clone(),
                    file: file.to_path_buf(),
                })
            })
        })
    }
}
