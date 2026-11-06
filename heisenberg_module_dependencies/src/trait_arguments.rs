use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasGenericArgs as _;

pub fn trait_arguments<'database>(
    implementation: &ast::Impl,
    semantics: &Semantics<'database, RootDatabase>,
) -> Vec<Type<'database>> {
    implementation
        .trait_()
        .and_then(|written_trait| ast::PathType::cast(written_trait.syntax().clone()))
        .and_then(|path_type| path_type.path())
        .and_then(|path| path.segment())
        .and_then(|segment| segment.generic_arg_list())
        .into_iter()
        .flat_map(|arguments| arguments.generic_args())
        .filter_map(|argument| ast::TypeArg::cast(argument.syntax().clone()))
        .filter_map(|argument| argument.ty())
        .filter_map(|written_type| semantics.resolve_type(&written_type))
        .collect()
}
