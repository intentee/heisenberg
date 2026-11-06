use std::collections::BTreeSet;

use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Crate;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use heisenberg_module_graph::item_group::ItemGroup;

use crate::defined_items::DefinedItems;
use crate::impl_home::ImplHome;
use crate::implemented_type::ImplementedType;
use crate::item_visibility::ItemVisibility;
use crate::ordered_entries::OrderedEntries;
use crate::ordered_entry::OrderedEntry;
use crate::outside_visibility::OutsideVisibility;
use crate::trait_arguments::trait_arguments;

pub struct ItemOrder<'order, 'database> {
    pub container: &'order SyntaxNode,
    pub krate: Crate,
    pub options: &'order CfgOptions,
    pub semantics: &'order Semantics<'database, RootDatabase>,
    pub workspace_files: &'order BTreeSet<FileId>,
}

impl ItemOrder<'_, '_> {
    pub fn entries(&self) -> OrderedEntries<ItemGroup> {
        OrderedEntries {
            entries: self
                .defined_in(self.container)
                .iter()
                .flat_map(|item| self.item_entries(item))
                .collect(),
        }
    }

    fn crate_group(&self, krate: Crate) -> ItemGroup {
        if krate == self.krate {
            ItemGroup::LocalImport
        } else if self
            .workspace_files
            .contains(&krate.root_file(self.semantics.db))
        {
            ItemGroup::WorkspaceImport
        } else {
            ItemGroup::VendorImport
        }
    }

    fn defined_in(&self, container: &SyntaxNode) -> Vec<ast::Item> {
        DefinedItems {
            container,
            options: self.options,
            semantics: self.semantics,
        }
        .items()
    }

    fn impl_group(&self, implementation: &ast::Impl) -> Option<ItemGroup> {
        let database = self.semantics.db;

        self.semantics.to_def(implementation).map(|definition| {
            match (ImplementedType {
                database,
                implementation: definition,
                krate: self.krate,
            })
            .home(&trait_arguments(implementation, self.semantics))
            {
                ImplHome::Local { home }
                    if !OutsideVisibility { database }.definition_reaches_outside(home.owner()) =>
                {
                    ItemGroup::PrivateType
                }
                ImplHome::Foreign | ImplHome::Local { .. } => ItemGroup::PublicItem,
            }
        })
    }

    fn import_entries(&self, tree: &ast::UseTree) -> Vec<OrderedEntry<ItemGroup>> {
        tree.path().map_or_else(
            || {
                tree.use_tree_list()
                    .into_iter()
                    .flat_map(|list| list.use_trees())
                    .flat_map(|nested| self.import_entries(&nested))
                    .collect()
            },
            |path| {
                self.path_crate(&path)
                    .map(|krate| OrderedEntry {
                        key: self.crate_group(krate),
                        node: tree.syntax().clone(),
                    })
                    .into_iter()
                    .collect()
            },
        )
    }

    fn item_entries(&self, item: &ast::Item) -> Vec<OrderedEntry<ItemGroup>> {
        let group = match item {
            ast::Item::ExternBlock(block) => {
                return block.extern_item_list().map_or_else(Vec::new, |list| {
                    self.defined_in(list.syntax())
                        .iter()
                        .flat_map(|inner| self.item_entries(inner))
                        .collect()
                });
            }
            ast::Item::Use(import) => {
                return import
                    .use_tree()
                    .map_or_else(Vec::new, |tree| self.import_entries(&tree));
            }
            ast::Item::ExternCrate(declaration) => self
                .semantics
                .to_def(declaration)
                .and_then(|definition| definition.resolved_crate(self.semantics.db))
                .map(|krate| self.crate_group(krate)),
            ast::Item::Impl(implementation) => self.impl_group(implementation),
            ast::Item::AsmExpr(_) | ast::Item::MacroCall(_) => Some(ItemGroup::PrivateHelper),
            ast::Item::Module(module) => Some(if module.item_list().is_some() {
                ItemGroup::InlineModule
            } else {
                ItemGroup::ModuleDeclaration
            }),
            ast::Item::Const(_)
            | ast::Item::Fn(_)
            | ast::Item::MacroDef(_)
            | ast::Item::MacroRules(_)
            | ast::Item::Static(_) => Some(self.visible_group(item, ItemGroup::PrivateHelper)),
            ast::Item::Enum(_)
            | ast::Item::Struct(_)
            | ast::Item::Trait(_)
            | ast::Item::TypeAlias(_)
            | ast::Item::Union(_) => Some(self.visible_group(item, ItemGroup::PrivateType)),
        };

        group
            .map(|key| OrderedEntry {
                key,
                node: item.syntax().clone(),
            })
            .into_iter()
            .collect()
    }

    fn path_crate(&self, path: &ast::Path) -> Option<Crate> {
        let database = self.semantics.db;

        match self.semantics.resolve_path(&path.first_qualifier_or_self()) {
            Some(PathResolution::Def(ModuleDef::Module(module))) => Some(module.krate(database)),
            Some(PathResolution::Def(definition)) => definition
                .module(database)
                .map(|module| module.krate(database)),
            _ => None,
        }
    }

    fn visible_group(&self, item: &ast::Item, private: ItemGroup) -> ItemGroup {
        if (ItemVisibility {
            item,
            semantics: self.semantics,
        })
        .is_public()
        {
            ItemGroup::PublicItem
        } else {
            private
        }
    }
}
