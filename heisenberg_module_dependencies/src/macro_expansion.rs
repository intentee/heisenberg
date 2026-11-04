use ra_ap_hir::MacroCallId;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::SyntaxError;

pub struct MacroExpansion<'expansion> {
    pub call: MacroCallId,
    pub database: &'expansion RootDatabase,
}

impl MacroExpansion<'_> {
    pub fn syntax_errors(&self) -> Vec<SyntaxError> {
        let (parse, _) = &self.call.parse_macro_expansion(self.database).value;

        parse.errors()
    }
}
