use ra_ap_hir::HirFileId;
use ra_ap_ide_db::RootDatabase;

pub struct MacroExpansion<'expansion> {
    pub database: &'expansion RootDatabase,
    pub file: HirFileId,
}

impl MacroExpansion<'_> {
    pub fn is_parsable(&self) -> bool {
        self.file.macro_file().is_some_and(|call| {
            let (parse, _) = &call.parse_macro_expansion(self.database).value;

            parse.errors().is_empty()
        })
    }
}
