use ra_ap_hir::FileRangeWrapper;
use ra_ap_syntax::TextRange;
use ra_ap_syntax::TextSize;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::source_location::SourceLocation;
use heisenberg_source_locator::source_locator::SourceLocator;

use crate::module_dependencies_error::ModuleDependenciesError;

pub fn file_start(
    locator: &SourceLocator<'_>,
    file_id: FileId,
) -> Result<SourceLocation, ModuleDependenciesError> {
    locator.locate(FileRangeWrapper {
        file_id,
        range: TextRange::empty(TextSize::from(0)),
    })
}
