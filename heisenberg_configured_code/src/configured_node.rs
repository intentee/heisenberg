use std::iter;
use std::ops::ControlFlow;

use ra_ap_cfg::CfgExpr;
use ra_ap_cfg::CfgOptions;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;

use crate::attribute_owner::attribute_owner;
use crate::conditional_attribute::ConditionalAttribute;
use crate::negated_atom::NegatedAtom;
use crate::negated_configuration::NegatedConfiguration;
use crate::test_mentions::TestMentions;
use crate::test_predicate::TestPredicate;
use crate::visit_active_metas::visit_active_metas;

pub struct ConfiguredNode<'node> {
    pub node: &'node SyntaxNode,
    pub options: &'node CfgOptions,
}

impl ConfiguredNode<'_> {
    #[must_use]
    pub fn conditional_attributes(&self) -> Vec<ConditionalAttribute> {
        attribute_owner(self.node)
            .into_iter()
            .flat_map(|owner| ast::attrs_including_inner(&owner).collect::<Vec<ast::Attr>>())
            .filter_map(|attribute| attribute.meta())
            .flat_map(|meta| self.conditional(&meta, true))
            .collect()
    }

    #[must_use]
    pub fn is_disabled(&self) -> bool {
        visit_active_metas(self.node, self.options, |meta| match meta {
            ast::Meta::CfgMeta(cfg)
                if cfg.cfg_predicate().is_some_and(|predicate| {
                    self.options.check(&CfgExpr::parse_from_ast(predicate)) == Some(false)
                }) =>
            {
                ControlFlow::Break(())
            }
            _ => ControlFlow::Continue(()),
        })
        .is_some()
    }

    #[must_use]
    pub fn negated_atoms(&self) -> Vec<NegatedAtom> {
        attribute_owner(self.node)
            .into_iter()
            .flat_map(|owner| ast::attrs_including_inner(&owner).collect::<Vec<ast::Attr>>())
            .flat_map(|attribute| {
                attribute
                    .meta()
                    .map_or_default(|meta| Self::predicates(&meta))
                    .into_iter()
                    .flat_map(|predicate| {
                        NegatedConfiguration {
                            expression: &CfgExpr::parse_from_ast(predicate),
                        }
                        .negated_atoms()
                    })
                    .map(|atom| NegatedAtom {
                        is_set: self.options.check_atom(&atom),
                        atom,
                        attribute: attribute.clone(),
                    })
                    .collect::<Vec<NegatedAtom>>()
            })
            .collect()
    }

    #[must_use]
    pub fn negated_attributes(&self) -> Vec<ast::Attr> {
        attribute_owner(self.node)
            .into_iter()
            .flat_map(|owner| ast::attrs_including_inner(&owner).collect::<Vec<ast::Attr>>())
            .filter(|attribute| attribute.meta().is_some_and(|meta| Self::negates(&meta)))
            .collect()
    }

    #[must_use]
    pub fn test_mentions(&self) -> TestMentions {
        let metas: Vec<ast::Meta> = attribute_owner(self.node)
            .into_iter()
            .flat_map(|owner| ast::attrs_including_inner(&owner).collect::<Vec<ast::Attr>>())
            .filter_map(|attribute| attribute.meta())
            .filter(|meta| {
                Self::predicates(meta).into_iter().any(|predicate| {
                    TestPredicate {
                        expression: &CfgExpr::parse_from_ast(predicate),
                    }
                    .affirms_test()
                })
            })
            .collect();

        TestMentions {
            count: metas.len(),
            is_a_plain_gate: matches!(
                metas.as_slice(),
                [ast::Meta::CfgMeta(cfg)]
                    if cfg.cfg_predicate().is_some_and(|predicate| {
                        TestPredicate {
                            expression: &CfgExpr::parse_from_ast(predicate),
                        }
                        .is_test_alone()
                    })
            ),
        }
    }

    fn conditional(&self, meta: &ast::Meta, enclosing_applies: bool) -> Vec<ConditionalAttribute> {
        match meta {
            ast::Meta::CfgAttrMeta(cfg_attr) => {
                let applies = enclosing_applies
                    && cfg_attr.cfg_predicate().is_some_and(|predicate| {
                        self.options.check(&CfgExpr::parse_from_ast(predicate)) == Some(true)
                    });

                iter::once(ConditionalAttribute {
                    applies,
                    meta: cfg_attr.clone(),
                })
                .chain(
                    cfg_attr
                        .metas()
                        .flat_map(|inner| self.conditional(&inner, applies)),
                )
                .collect()
            }
            _ => Vec::new(),
        }
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

    fn predicates(meta: &ast::Meta) -> Vec<ast::CfgPredicate> {
        match meta {
            ast::Meta::CfgMeta(cfg) => cfg.cfg_predicate().into_iter().collect(),
            ast::Meta::CfgAttrMeta(cfg_attr) => cfg_attr
                .cfg_predicate()
                .into_iter()
                .chain(cfg_attr.metas().flat_map(|inner| Self::predicates(&inner)))
                .collect(),
            _ => Vec::new(),
        }
    }
}
