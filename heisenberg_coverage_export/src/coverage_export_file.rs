use std::fs::File;
use std::io::Read;

use ra_ap_paths::AbsPath;
use serde::Deserialize;
use serde_json::Value;
use serde_json::from_slice;

use heisenberg_error_context::error_context::error_context;

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

        File::open(path)
            .map_err(error_context(
                path.to_path_buf(),
                CoverageExportError::Unopenable,
            ))
            .and_then(|mut file| {
                file.metadata()
                    .map_err(error_context(
                        path.to_path_buf(),
                        CoverageExportError::MetadataUnreadable,
                    ))
                    .and_then(|metadata| {
                        metadata.modified().map_err(error_context(
                            path.to_path_buf(),
                            CoverageExportError::ModificationTimeUnavailable,
                        ))
                    })
                    .and_then(|modified| {
                        let mut contents = Vec::new();

                        file.read_to_end(&mut contents)
                            .map_err(error_context(
                                path.to_path_buf(),
                                CoverageExportError::Unreadable,
                            ))
                            .and_then(|_| {
                                from_slice::<Value>(&contents).map_err(error_context(
                                    path.to_path_buf(),
                                    CoverageExportError::MalformedJson,
                                ))
                            })
                            .and_then(|document| {
                                ExportHeader::deserialize(&document)
                                    .map_err(error_context(
                                        path.to_path_buf(),
                                        CoverageExportError::NotAnExport,
                                    ))
                                    .and_then(|header| header.supported(path))
                                    .and_then(|()| {
                                        CoverageExport::deserialize(&document).map_err(
                                            error_context(
                                                path.to_path_buf(),
                                                CoverageExportError::SchemaMismatch,
                                            ),
                                        )
                                    })
                            })
                            .and_then(|export| {
                                export.coverage(path, modified, self.workspace_manifest)
                            })
                    })
            })
    }
}
