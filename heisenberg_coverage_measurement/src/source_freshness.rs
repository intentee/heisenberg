use std::fs::metadata;

use ra_ap_paths::AbsPath;

use heisenberg_coverage_export::exported_coverage::ExportedCoverage;
use heisenberg_error_context::error_context::error_context;

use crate::coverage_measurement_error::CoverageMeasurementError;

pub struct SourceFreshness<'freshness> {
    pub coverage: &'freshness ExportedCoverage,
    pub files: &'freshness [&'freshness AbsPath],
}

impl SourceFreshness<'_> {
    pub fn verify(&self) -> Result<(), CoverageMeasurementError> {
        let ExportedCoverage { modified, path, .. } = self.coverage;

        self.files.iter().try_for_each(|file| {
            metadata(file)
                .map_err(error_context(
                    file.to_path_buf(),
                    CoverageMeasurementError::SourceMetadataUnreadable,
                ))
                .and_then(|file_metadata| {
                    file_metadata.modified().map_err(error_context(
                        file.to_path_buf(),
                        CoverageMeasurementError::SourceModificationTimeUnavailable,
                    ))
                })
                .and_then(|file_modified| {
                    if file_modified > *modified {
                        Err(CoverageMeasurementError::OutdatedExport {
                            export: path.clone(),
                            export_modified: *modified,
                            file: file.to_path_buf(),
                            file_modified,
                        })
                    } else {
                        Ok(())
                    }
                })
        })
    }
}
