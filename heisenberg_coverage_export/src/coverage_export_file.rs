use std::fs;
use std::time::SystemTime;

use filetime::FileTime;
use ra_ap_paths::AbsPath;
use serde::Deserialize;
use serde_json::Value;
use serde_json::from_slice;

use crate::coverage_export::CoverageExport;
use crate::coverage_export_error::CoverageExportError;
use crate::export_header::ExportHeader;
use crate::exported_coverage::ExportedCoverage;

pub struct CoverageExportFile<'export> {
    pub path: &'export AbsPath,
    pub workspace_manifest: &'export AbsPath,
}

impl CoverageExportFile<'_> {
    /// # Errors
    ///
    /// Returns [`CoverageExportError`] when the export cannot be read, is not a
    /// `cargo llvm-cov report --json` export in the llvm-cov JSON format 3.1.0, covers another
    /// workspace, or records a region that Heisenberg cannot attribute to a code region of an
    /// absolute source file.
    pub fn read(&self) -> Result<ExportedCoverage, CoverageExportError> {
        let path = self.path;

        fs::metadata(path)
            .map_err(|source| CoverageExportError::MetadataUnreadable {
                path: path.to_path_buf(),
                source,
            })
            .and_then(|metadata| {
                let modified = SystemTime::from(FileTime::from_last_modification_time(&metadata));

                fs::read(path)
                    .map_err(|source| CoverageExportError::Unreadable {
                        path: path.to_path_buf(),
                        source,
                    })
                    .and_then(|contents| {
                        from_slice::<Value>(&contents).map_err(|source| {
                            CoverageExportError::MalformedJson {
                                path: path.to_path_buf(),
                                source,
                            }
                        })
                    })
                    .and_then(|document| {
                        ExportHeader::deserialize(&document)
                            .map_err(|source| CoverageExportError::NotAnExport {
                                path: path.to_path_buf(),
                                source,
                            })
                            .and_then(|header| header.supported(path))
                            .and_then(|()| {
                                CoverageExport::deserialize(&document).map_err(|source| {
                                    CoverageExportError::SchemaMismatch {
                                        path: path.to_path_buf(),
                                        source,
                                    }
                                })
                            })
                    })
                    .and_then(|export| export.coverage(path, modified, self.workspace_manifest))
            })
    }
}
