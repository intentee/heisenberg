use ra_ap_hir::db::HirDatabase;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_syntax::SyntaxError;

#[salsa::tracked(returns(ref))]
pub fn expansion_syntax_errors(database: &dyn HirDatabase, call: MacroCallId) -> Vec<SyntaxError> {
    call.parse_macro_expansion(database).value.0.errors()
}
