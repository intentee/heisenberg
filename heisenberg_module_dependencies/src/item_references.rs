use ra_ap_hir::Adt;
use ra_ap_hir::Semantics;
use ra_ap_hir::Variant;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use heisenberg_module_graph::used_item::UsedItem;

use crate::definition_usages::DefinitionUsages;
use crate::found_reference::FoundReference;

pub struct ItemReferences<'references, 'database> {
    pub definition: Definition<'database>,
    pub item: UsedItem,
    pub node: &'references SyntaxNode,
    pub semantics: &'references Semantics<'database, RootDatabase>,
    pub usages: &'references DefinitionUsages<'references, 'database>,
}

impl<'database> ItemReferences<'_, 'database> {
    pub fn exporting_uses(&self) -> Vec<SyntaxNode> {
        self.usages
            .of(self.definition)
            .by_file
            .values()
            .flatten()
            .filter_map(|FoundReference { name, .. }| name.syntax().parent())
            .filter(|user| !self.is_inside_its_item(user))
            .collect()
    }

    pub fn using_nodes(&self) -> Option<Vec<SyntaxNode>> {
        self.usages
            .of(self.definition)
            .by_file
            .values()
            .flatten()
            .filter(|reference| self.counts(reference))
            .map(|FoundReference { name, .. }| name.syntax().parent())
            .collect::<Option<Vec<SyntaxNode>>>()
            .map(|using| using.into_iter().chain(self.spreads()).collect())
    }

    fn construction(name: &SyntaxNode) -> Option<SyntaxNode> {
        name.ancestors()
            .find_map(ast::Path::cast)
            .and_then(|path| path.syntax().parent())
            .filter(|parent| {
                ast::PathExpr::can_cast(parent.kind()) || ast::RecordExpr::can_cast(parent.kind())
            })
    }

    fn counts(&self, FoundReference { name, writes }: &FoundReference) -> bool {
        match self.item {
            UsedItem::Constant | UsedItem::Function | UsedItem::Static | UsedItem::TypeAlias => {
                true
            }
            UsedItem::Enum | UsedItem::Struct | UsedItem::Union => name
                .syntax()
                .parent()
                .is_none_or(|parent| !self.is_inside_its_item(&parent)),
            UsedItem::Field => {
                !writes
                    && name
                        .syntax()
                        .parent()
                        .is_none_or(|parent| !ast::RecordExprField::can_cast(parent.kind()))
            }
            UsedItem::Variant => name
                .syntax()
                .parent()
                .is_some_and(|parent| Self::construction(&parent).is_some()),
        }
    }

    fn is_inside_its_item(&self, name: &SyntaxNode) -> bool {
        let database = self.semantics.db;

        self.semantics
            .ancestors_with_macros(name.clone())
            .any(|ancestor| {
                ancestor == *self.node
                    || ast::Impl::cast(ancestor)
                        .and_then(|implementation| self.semantics.to_def(&implementation))
                        .and_then(|implementation| implementation.self_ty(database).as_adt())
                        .is_some_and(|adt| Definition::Adt(adt) == self.definition)
            })
    }

    fn owner(&self) -> Option<Definition<'database>> {
        match self.definition {
            Definition::Field(field) => Some(match field.parent_def(self.semantics.db) {
                Variant::Struct(owner) => Definition::Adt(Adt::Struct(owner)),
                Variant::Union(owner) => Definition::Adt(Adt::Union(owner)),
                Variant::EnumVariant(owner) => Definition::EnumVariant(owner),
            }),
            _ => None,
        }
    }

    fn spreads(&self) -> Vec<SyntaxNode> {
        self.owner().map_or_else(Vec::new, |owner| {
            self.usages
                .with_self_references(owner)
                .by_file
                .values()
                .flatten()
                .filter_map(|FoundReference { name, .. }| {
                    name.syntax()
                        .ancestors()
                        .find_map(ast::Path::cast)
                        .and_then(|path| path.syntax().parent())
                        .and_then(ast::RecordExpr::cast)
                })
                .filter(|literal| {
                    literal
                        .record_expr_field_list()
                        .is_some_and(|fields| fields.spread().is_some())
                })
                .map(|literal| literal.syntax().clone())
                .collect()
        })
    }
}
