use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Crate;
use ra_ap_hir::Function;
use ra_ap_hir::HasVisibility;
use ra_ap_hir::Impl;
use ra_ap_hir::ModuleId;
use ra_ap_hir::Semantics;
use ra_ap_hir::Visibility;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use heisenberg_module_graph::method_group::MethodGroup;

use crate::constructor::Constructor;
use crate::defined_items::DefinedItems;
use crate::method_key::MethodKey;
use crate::ordered_entries::OrderedEntries;
use crate::ordered_entry::OrderedEntry;

pub struct MethodOrder<'order, 'database> {
    pub implementation: &'order ast::Impl,
    pub krate: Crate,
    pub options: &'order CfgOptions,
    pub semantics: &'order Semantics<'database, RootDatabase>,
}

impl MethodOrder<'_, '_> {
    pub fn entries(&self) -> OrderedEntries<MethodKey> {
        OrderedEntries {
            entries: self
                .semantics
                .to_def(self.implementation)
                .into_iter()
                .flat_map(|definition| {
                    self.implementation
                        .assoc_item_list()
                        .into_iter()
                        .flat_map(move |list| {
                            DefinedItems {
                                container: list.syntax(),
                                options: self.options,
                                semantics: self.semantics,
                            }
                            .items()
                        })
                        .filter_map(|item| match item {
                            ast::Item::Fn(function) => Some(function),
                            _ => None,
                        })
                        .filter_map(move |function| {
                            self.semantics.to_def(&function).map(|method| OrderedEntry {
                                key: self.key(method, definition),
                                node: function.syntax().clone(),
                            })
                        })
                })
                .collect(),
        }
    }

    fn group(&self, method: Function, implementation: Impl) -> MethodGroup {
        let database = self.semantics.db;

        if (Constructor {
            function: method,
            krate: self.krate,
            semantics: self.semantics,
        })
        .constructs(&implementation.self_ty(database))
        {
            MethodGroup::Constructor
        } else if implementation.trait_(database).is_some() {
            MethodGroup::Trait
        } else {
            match method.visibility(database) {
                Visibility::Public => MethodGroup::Public,
                Visibility::PubCrate(_) => MethodGroup::Crate,
                Visibility::Module(owner, _)
                    if owner == ModuleId::from(method.module(database)) =>
                {
                    MethodGroup::Private
                }
                Visibility::Module(..) => MethodGroup::Restricted,
            }
        }
    }

    fn key(&self, method: Function, implementation: Impl) -> MethodKey {
        MethodKey {
            group: self.group(method, implementation),
            name: method.name(self.semantics.db).as_str().to_owned(),
        }
    }
}
