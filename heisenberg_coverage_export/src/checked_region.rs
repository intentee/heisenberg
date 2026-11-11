use ra_ap_paths::AbsPath;
use serde::de::IgnoredAny;

use crate::constants::CODE_REGION_KIND;
use crate::coverage_export_error::CoverageExportError;
use crate::export_region::ExportRegion;
use crate::region_span::RegionSpan;
use crate::zero_based_span::ZeroBasedSpan;

pub struct CheckedRegion {
    pub executed: bool,
    pub exported: RegionSpan,
    pub file_id: usize,
    pub span: ZeroBasedSpan,
}

impl CheckedRegion {
    pub fn new(
        function: &str,
        export: &AbsPath,
        listed: usize,
        region: &ExportRegion,
    ) -> Result<Self, CoverageExportError> {
        let ExportRegion {
            execution_count,
            expanded_file_id: IgnoredAny,
            file_id,
            kind,
            ..
        } = region;
        let RegionSpan { start, end } = region.span();
        let Some(span) = start.zero_based().and_then(|zero_based_start| {
            end.zero_based().map(|zero_based_end| ZeroBasedSpan {
                start: zero_based_start,
                end: zero_based_end,
            })
        }) else {
            return Err(CoverageExportError::InvalidRegionPosition {
                end,
                export: export.to_path_buf(),
                function: function.to_owned(),
                start,
            });
        };

        if end < start {
            return Err(CoverageExportError::RegionEndBeforeStart {
                end,
                export: export.to_path_buf(),
                function: function.to_owned(),
                start,
            });
        }

        if *kind != CODE_REGION_KIND {
            return Err(CoverageExportError::UnsupportedRegionKind {
                export: export.to_path_buf(),
                function: function.to_owned(),
                kind: *kind,
                start,
            });
        }

        if *file_id >= listed {
            return Err(CoverageExportError::RegionFileOutOfRange {
                export: export.to_path_buf(),
                file_id: *file_id,
                function: function.to_owned(),
                listed,
            });
        }

        Ok(Self {
            executed: *execution_count > 0,
            exported: RegionSpan { start, end },
            file_id: *file_id,
            span,
        })
    }
}
