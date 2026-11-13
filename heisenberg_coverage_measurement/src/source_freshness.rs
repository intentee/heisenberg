use std::fs::metadata;
use std::time::SystemTime;

use filetime::FileTime;
use ra_ap_paths::AbsPath;

use heisenberg_coverage_export::exported_coverage::ExportedCoverage;

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
                .map_err(
                    |source| CoverageMeasurementError::SourceMetadataUnreadable {
                        path: file.to_path_buf(),
                        source,
                    },
                )
                .and_then(|file_metadata| {
                    let file_modified =
                        SystemTime::from(FileTime::from_last_modification_time(&file_metadata));

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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::error::Error;
    use std::fs;
    use std::time::SystemTime;

    use ra_ap_paths::AbsPathBuf;

    use heisenberg_coverage_export::exported_coverage::ExportedCoverage;

    use super::SourceFreshness;

    #[test]
    fn missing_source_file_fails_the_freshness_check() {
        let directory = AbsPathBuf::assert_utf8(env!("CARGO_MANIFEST_DIR").into());
        let missing = directory.join("missing.rs");
        let reason = fs::metadata(&missing).err().map(|error| error.to_string());
        let failure = SourceFreshness {
            coverage: &ExportedCoverage {
                files: BTreeMap::new(),
                modified: SystemTime::UNIX_EPOCH,
                path: directory.join("llvm-cov.json"),
            },
            files: &[missing.as_path()],
        }
        .verify()
        .err()
        .map(|error| {
            [
                error.to_string(),
                error.source().map_or_else(String::new, ToString::to_string),
            ]
        });

        assert_eq!(
            failure,
            reason.map(|reason| [
                format!("the metadata of `{missing}` cannot be read"),
                reason
            ])
        );
    }
}
