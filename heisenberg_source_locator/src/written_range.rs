use ra_ap_hir::FileRangeWrapper;
use ra_ap_hir::HirFileId;
use ra_ap_hir_expand::span_for_offset;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::TextRange;
use ra_ap_vfs::FileId;

use crate::original_file_id::original_file_id;

#[must_use]
pub fn written_range(
    database: &RootDatabase,
    file_id: HirFileId,
    range: TextRange,
) -> FileRangeWrapper<FileId> {
    file_id.macro_file().map_or_else(
        || FileRangeWrapper {
            file_id: original_file_id(database, file_id),
            range,
        },
        |call| {
            let (written, _) =
                span_for_offset(database, call.expansion_span_map(database), range.start());

            FileRangeWrapper {
                file_id: written.file_id.file_id(database),
                range: written.range,
            }
        },
    )
}
