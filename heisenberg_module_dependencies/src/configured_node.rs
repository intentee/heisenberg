use std::ops::ControlFlow;

use ra_ap_cfg::CfgExpr;
use ra_ap_cfg::CfgOptions;
use ra_ap_hir_expand::attrs::expand_cfg_attr;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::negated_configuration::NegatedConfiguration;

pub struct ConfiguredNode<'node> {
    pub node: &'node SyntaxNode,
    pub options: &'node CfgOptions,
}

impl ConfiguredNode<'_> {
    pub fn enabled_items(&self) -> Vec<ast::Item> {
        self.node
            .children()
            .filter_map(ast::Item::cast)
            .filter(|item| {
                !ConfiguredNode {
                    node: item.syntax(),
                    options: self.options,
                }
                .is_disabled()
            })
            .collect()
    }

    pub fn is_disabled(&self) -> bool {
        ast::AnyHasAttrs::cast(self.node.clone()).is_some_and(|owner| {
            expand_cfg_attr(
                ast::attrs_including_inner(&owner),
                || self.options,
                |meta, _| match meta {
                    ast::Meta::CfgMeta(cfg)
                        if cfg.cfg_predicate().is_some_and(|predicate| {
                            self.options.check(&CfgExpr::parse_from_ast(predicate)) == Some(false)
                        }) =>
                    {
                        ControlFlow::Break(())
                    }
                    _ => ControlFlow::Continue(()),
                },
            )
            .is_some()
        })
    }

    pub fn negated_attributes(&self) -> Vec<ast::Attr> {
        ast::AnyHasAttrs::cast(self.node.clone())
            .into_iter()
            .flat_map(|owner| ast::attrs_including_inner(&owner).collect::<Vec<ast::Attr>>())
            .filter(|attribute| attribute.meta().is_some_and(|meta| Self::negates(&meta)))
            .collect()
    }

    fn negates(meta: &ast::Meta) -> bool {
        match meta {
            ast::Meta::CfgMeta(cfg) => Self::negates_predicate(cfg.cfg_predicate()),
            ast::Meta::CfgAttrMeta(cfg_attr) => {
                Self::negates_predicate(cfg_attr.cfg_predicate())
                    || cfg_attr.metas().any(|inner| Self::negates(&inner))
            }
            _ => false,
        }
    }

    fn negates_predicate(predicate: Option<ast::CfgPredicate>) -> bool {
        predicate.is_some_and(|predicate| {
            NegatedConfiguration {
                expression: &CfgExpr::parse_from_ast(predicate),
            }
            .is_negated()
        })
    }
}
