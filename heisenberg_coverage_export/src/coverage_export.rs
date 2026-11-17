use std::collections::BTreeMap;
use std::time::SystemTime;

use ra_ap_paths::AbsPath;
use serde::Deserialize;
use serde::de::IgnoredAny;

use heisenberg_absolute_path::absolute_path::AbsolutePath;

use crate::cargo_llvm_cov_record::CargoLlvmCovRecord;
use crate::coverage_export_error::CoverageExportError;
use crate::export_data::ExportData;
use crate::exported_coverage::ExportedCoverage;
use crate::exported_path::exported_path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageExport {
    pub cargo_llvm_cov: CargoLlvmCovRecord,
    pub data: [ExportData; 1],
    #[serde(rename = "type")]
    pub export_type: IgnoredAny,
    pub version: IgnoredAny,
}

impl CoverageExport {
    pub fn coverage(
        self,
        path: &AbsPath,
        modified: SystemTime,
        workspace_manifest: &AbsPath,
    ) -> Result<ExportedCoverage, CoverageExportError> {
        let Self {
            cargo_llvm_cov:
                CargoLlvmCovRecord {
                    manifest_path,
                    version: IgnoredAny,
                },
            data:
                [
                    ExportData {
                        files: IgnoredAny,
                        functions,
                        totals: IgnoredAny,
                    },
                ],
            export_type: IgnoredAny,
            version: IgnoredAny,
        } = self;

        let mut files = BTreeMap::new();
        let mut paths = BTreeMap::new();

        AbsolutePath::parse(&manifest_path)
            .map_err(|source| CoverageExportError::RelativeManifestPath {
                export: path.to_path_buf(),
                source,
            })
            .map(exported_path)
            .and_then(|exported| {
                if exported == workspace_manifest {
                    Ok(())
                } else {
                    Err(CoverageExportError::OtherWorkspace {
                        export: path.to_path_buf(),
                        manifest_path: exported,
                        workspace_manifest: workspace_manifest.to_path_buf(),
                    })
                }
            })
            .and_then(|()| {
                functions
                    .iter()
                    .try_for_each(|function| function.merge_into(path, &mut paths, &mut files))
            })
            .map(|()| ExportedCoverage {
                files: files
                    .into_iter()
                    .map(|(file, regions)| (file, regions.into_values().collect()))
                    .collect(),
                modified,
                path: path.to_path_buf(),
            })
    }
}
