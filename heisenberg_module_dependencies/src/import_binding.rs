use ra_ap_hir::Crate;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::name::AsName;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasModuleItem;
use ra_ap_syntax::ast::HasName;

pub struct ImportBinding<'binding, 'database> {
    pub krate: Crate,
    pub semantics: &'binding Semantics<'database, RootDatabase>,
}

impl ImportBinding<'_, '_> {
    pub fn imports_from_another_crate(&self, scope: &SyntaxNode, name: &ast::NameRef) -> bool {
        scope
            .ancestors()
            .take_while(|ancestor| !ast::AnyHasModuleItem::can_cast(ancestor.kind()))
            .chain(
                scope
                    .ancestors()
                    .find(|ancestor| ast::AnyHasModuleItem::can_cast(ancestor.kind())),
            )
            .find_map(|ancestor| Self::binding_in(&ancestor, name))
            .is_some_and(|binding| self.is_foreign(&binding))
    }

    fn binding_in(scope: &SyntaxNode, name: &ast::NameRef) -> Option<ast::UseTree> {
        let imports: Vec<ast::Use> = ast::StmtList::cast(scope.clone()).map_or_else(
            || {
                ast::AnyHasModuleItem::cast(scope.clone())
                    .into_iter()
                    .flat_map(|owner| owner.items())
                    .filter_map(|item| ast::Use::cast(item.syntax().clone()))
                    .collect()
            },
            |statements| {
                statements
                    .statements()
                    .filter_map(|statement| ast::Use::cast(statement.syntax().clone()))
                    .collect()
            },
        );

        imports
            .iter()
            .flat_map(|import| import.syntax().descendants().filter_map(ast::UseTree::cast))
            .find(|tree| Self::binds(tree, name))
    }

    fn binds(tree: &ast::UseTree, name: &ast::NameRef) -> bool {
        tree.use_tree_list().is_none()
            && tree
                .rename()
                .and_then(|rename| rename.name())
                .map(|renamed| renamed.as_name())
                .or_else(|| {
                    tree.path()
                        .and_then(|path| path.segment())
                        .and_then(|segment| segment.name_ref())
                        .map(|bound| bound.as_name())
                })
                .is_some_and(|bound| bound == name.as_name())
    }

    fn is_foreign(&self, binding: &ast::UseTree) -> bool {
        binding
            .top_use_tree()
            .path()
            .map(|path| path.first_qualifier_or_self())
            .is_some_and(|root| {
                matches!(
                    self.semantics.resolve_path(&root),
                    Some(PathResolution::Def(ModuleDef::Module(module)))
                        if module.krate(self.semantics.db) != self.krate
                )
            })
    }
}
