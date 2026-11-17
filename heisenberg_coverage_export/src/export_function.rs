use std::collections::BTreeMap;

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use serde::Deserialize;
use serde::de::IgnoredAny;

use heisenberg_absolute_path::absolute_path::AbsolutePath;

use crate::checked_region::CheckedRegion;
use crate::coverage_export_error::CoverageExportError;
use crate::export_region::ExportRegion;
use crate::exported_path::exported_path;
use crate::exported_region::ExportedRegion;
use crate::region_span::RegionSpan;

fn resolved_path(
    paths: &mut BTreeMap<String, AbsPathBuf>,
    export: &AbsPath,
    function: &str,
    filename: &str,
) -> Result<AbsPathBuf, CoverageExportError> {
    paths.get(filename).cloned().map_or_else(
        || {
            AbsolutePath::parse(filename)
                .map_err(|source| CoverageExportError::RelativeRegionFile {
                    export: export.to_path_buf(),
                    function: function.to_owned(),
                    source,
                })
                .map(exported_path)
                .inspect(|path| {
                    paths.insert(filename.to_owned(), path.clone());
                })
        },
        Ok,
    )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportFunction {
    pub branches: Vec<IgnoredAny>,
    pub count: IgnoredAny,
    pub filenames: Vec<String>,
    pub mcdc_records: Vec<IgnoredAny>,
    pub name: String,
    pub regions: Vec<ExportRegion>,
}

impl ExportFunction {
    pub fn merge_into(
        &self,
        export: &AbsPath,
        paths: &mut BTreeMap<String, AbsPathBuf>,
        files: &mut BTreeMap<AbsPathBuf, BTreeMap<RegionSpan, ExportedRegion>>,
    ) -> Result<(), CoverageExportError> {
        let Self {
            branches,
            count: IgnoredAny,
            filenames,
            mcdc_records,
            name,
            regions,
        } = self;

        if !branches.is_empty() {
            return Err(CoverageExportError::BranchCoverage {
                export: export.to_path_buf(),
                function: name.clone(),
            });
        }

        if !mcdc_records.is_empty() {
            return Err(CoverageExportError::McdcCoverage {
                export: export.to_path_buf(),
                function: name.clone(),
            });
        }

        regions
            .iter()
            .map(|region| CheckedRegion::new(name, export, filenames.len(), region))
            .collect::<Result<Vec<CheckedRegion>, CoverageExportError>>()
            .and_then(|checked| {
                filenames
                    .iter()
                    .enumerate()
                    .try_for_each(|(index, filename)| {
                        resolved_path(paths, export, name, filename).map(|path| {
                            for CheckedRegion {
                                executed,
                                exported,
                                span,
                                ..
                            } in checked
                                .iter()
                                .filter(|CheckedRegion { file_id, .. }| *file_id == index)
                            {
                                files
                                    .entry(path.clone())
                                    .or_default()
                                    .entry(*exported)
                                    .or_insert(ExportedRegion {
                                        executed: false,
                                        exported: *exported,
                                        span: *span,
                                    })
                                    .executed |= executed;
                            }
                        })
                    })
            })
    }
}
