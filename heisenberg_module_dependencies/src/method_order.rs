use ra_ap_cfg::CfgOptions;
use ra_ap_hir::Function;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::defined_items::DefinedItems;
use crate::ordered_entries::OrderedEntries;
use crate::ordered_entry::OrderedEntry;

pub struct MethodOrder<'order, 'database> {
    pub items: &'order ast::AssocItemList,
    pub options: &'order CfgOptions,
    pub semantics: &'order Semantics<'database, RootDatabase>,
}

impl MethodOrder<'_, '_> {
    pub fn entries<TKey>(&self, key: impl Fn(Function) -> Option<TKey>) -> OrderedEntries<TKey> {
        OrderedEntries {
            entries: DefinedItems {
                container: self.items.syntax(),
                options: self.options,
                semantics: self.semantics,
            }
            .items()
            .into_iter()
            .filter_map(|item| match item {
                ast::Item::Fn(function) => Some(function),
                _ => None,
            })
            .filter_map(|function| {
                self.semantics
                    .to_def(&function)
                    .and_then(&key)
                    .map(|key| OrderedEntry {
                        key,
                        node: function.syntax().clone(),
                    })
            })
            .collect(),
        }
    }
}
