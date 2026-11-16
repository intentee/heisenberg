use std::collections::HashSet;

use ra_ap_hir::Crate;
use ra_ap_hir::Field;
use ra_ap_hir::HirFileId;
use ra_ap_hir::Local;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Semantics;
use ra_ap_hir_expand::Lookup;
use ra_ap_hir_expand::MacroCallId;
use ra_ap_hir_expand::MacroDefKind;
use ra_ap_hir_expand::proc_macro::ProcMacroKind;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::NodeOrToken;
use ra_ap_syntax::SyntaxKind;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasAttrs;
use rayon::iter::Either;

use crate::constants::ERROR_DISPLAY_METHOD;
use crate::constants::ERROR_MESSAGE_HELPER;
use crate::constants::ERROR_SOURCE_FIELD;
use crate::constants::ERROR_SOURCE_HELPERS;
use crate::constants::TRANSPARENT_ERROR_MESSAGE;
use crate::derived_sources::DerivedSources;
use crate::error_types::ErrorTypes;
use crate::local_binding::local_binding;
use crate::method_call_resolutions::MethodCallResolutions;
use crate::path_resolutions::PathResolutions;
use crate::substituted_resolution::SubstitutedResolution;

struct VariantField {
    attributes: Vec<ast::Attr>,
    definition: Field,
    node: SyntaxNode,
}

impl VariantField {
    fn new(definition: Field, field: &impl HasAttrs) -> Self {
        Self {
            attributes: field.attrs().collect(),
            definition,
            node: field.syntax().clone(),
        }
    }
}

pub struct ErrorDerive<'derive, 'database> {
    pub derive_crates: &'derive HashSet<Crate>,
    pub error_types: ErrorTypes<'derive, 'database>,
    pub method_calls: &'derive MethodCallResolutions<'derive, 'database>,
    pub paths: &'derive PathResolutions<'derive, 'database>,
    pub runtime_crates: &'derive HashSet<Crate>,
    pub semantics: &'derive Semantics<'database, RootDatabase>,
}

impl<'database> ErrorDerive<'_, 'database> {
    pub fn generates(&self, expansion: MacroCallId) -> bool {
        let definition = expansion.lookup(self.semantics.db).def;

        matches!(
            definition.kind,
            MacroDefKind::ProcMacro(_, _, ProcMacroKind::CustomDerive)
        ) && self.derive_crates.contains(&Crate::from(definition.krate))
    }

    pub fn sources(&self, written: &ast::Enum, expansion: MacroCallId) -> DerivedSources {
        let database = self.semantics.db;
        let transparent = self.is_transparent(written);
        let mut derived = DerivedSources::default();

        for variant in Self::variants(written) {
            let fields = self.fields(&variant);
            let source = self.source(&variant, &fields, transparent);

            derived.outside_chain.extend(
                fields
                    .iter()
                    .filter(|VariantField { definition, .. }| {
                        source != Some(*definition)
                            && self.error_types.implements_error(&definition.ty(database))
                            && !(source.is_some()
                                && self.formats_in_its_message(expansion, *definition))
                    })
                    .map(|VariantField { node, .. }| node.clone()),
            );
            derived.sources.extend(source);
        }

        derived
    }

    pub fn sources_in_messages(
        &self,
        expansion: MacroCallId,
        sources: &HashSet<Field>,
    ) -> Vec<SyntaxNode> {
        let root = self.semantics.parse_or_expand(HirFileId::from(expansion));

        self.formats(&root)
            .into_iter()
            .flat_map(|format| {
                format
                    .args()
                    .filter_map(|argument| argument.expr())
                    .filter(|argument| self.displays_source(&root, argument.clone(), sources))
                    .map(|argument| argument.syntax().clone())
                    .chain(
                        format
                            .template()
                            .filter(|template| self.captures_source(&root, template, sources))
                            .map(|template| template.syntax().clone()),
                    )
                    .collect::<Vec<SyntaxNode>>()
            })
            .collect()
    }

    fn binds_source(
        &self,
        root: &SyntaxNode,
        local: Local<'database>,
        sources: &HashSet<Field>,
    ) -> bool {
        local_binding(self.semantics, root, local).is_some_and(|bound| {
            bound.syntax().parent().is_some_and(|parent| {
                ast::RecordPatField::cast(parent.clone())
                    .and_then(|field| self.semantics.resolve_record_pat_field(&field))
                    .or_else(|| {
                        ast::TupleStructPat::cast(parent.clone()).and_then(|pattern| {
                            pattern
                                .fields()
                                .position(|field| field.syntax() == bound.syntax())
                                .and_then(|position| {
                                    self.semantics
                                        .resolve_tuple_struct_pat_fields(&pattern)
                                        .and_then(|fields| fields.into_iter().nth(position))
                                })
                        })
                    })
                    .map_or_else(
                        || {
                            ast::TuplePat::cast(parent).is_some_and(|pattern| {
                                pattern
                                    .fields()
                                    .position(|field| field.syntax() == bound.syntax())
                                    .and_then(|position| {
                                        pattern
                                            .syntax()
                                            .ancestors()
                                            .find_map(ast::MatchExpr::cast)
                                            .and_then(|matched| matched.expr())
                                            .and_then(|scrutinee| {
                                                ast::TupleExpr::cast(scrutinee.syntax().clone())
                                            })
                                            .and_then(|values| values.fields().nth(position))
                                    })
                                    .is_some_and(|value| self.displays_source(root, value, sources))
                            })
                        },
                        |(field, _)| sources.contains(&field),
                    )
            })
        })
    }

    fn captures_source(
        &self,
        root: &SyntaxNode,
        template: &ast::Expr,
        sources: &HashSet<Field>,
    ) -> bool {
        ast::Literal::cast(template.syntax().clone())
            .and_then(|literal| ast::String::cast(literal.token()))
            .and_then(|string| self.semantics.as_format_args_parts(&string))
            .into_iter()
            .flatten()
            .filter_map(|(_, resolution)| resolution.and_then(Either::left))
            .any(|resolution| match resolution {
                PathResolution::Local(local) => self.binds_source(root, local, sources),
                _ => false,
            })
    }

    fn displayed_binding(&self, argument: ast::Expr) -> Option<ast::Path> {
        let database = self.semantics.db;

        match argument {
            ast::Expr::MethodCallExpr(call) => self.method_calls.resolve(&call).and_then(
                |SubstitutedResolution {
                     resolution: method, ..
                 }| {
                    (method.name(database).as_str() == ERROR_DISPLAY_METHOD
                        && self
                            .runtime_crates
                            .contains(&method.module(database).krate(database)))
                    .then(|| call.receiver())
                    .flatten()
                    .and_then(|receiver| self.displayed_binding(receiver))
                },
            ),
            ast::Expr::ParenExpr(inner) => inner
                .expr()
                .and_then(|expression| self.displayed_binding(expression)),
            ast::Expr::PathExpr(path) => path.path(),
            ast::Expr::RefExpr(reference) => reference
                .expr()
                .and_then(|expression| self.displayed_binding(expression)),
            _ => None,
        }
    }

    fn displays_source(
        &self,
        root: &SyntaxNode,
        argument: ast::Expr,
        sources: &HashSet<Field>,
    ) -> bool {
        self.displayed_binding(argument)
            .and_then(|path| match self.paths.resolve(&path) {
                Some(PathResolution::Local(local)) => Some(local),
                _ => None,
            })
            .is_some_and(|local| self.binds_source(root, local, sources))
    }

    fn fields(&self, variant: &ast::Variant) -> Vec<VariantField> {
        match variant.field_list() {
            Some(ast::FieldList::RecordFieldList(list)) => list
                .fields()
                .filter_map(|field| {
                    self.semantics
                        .to_def(&field)
                        .map(|definition| VariantField::new(definition, &field))
                })
                .collect(),
            Some(ast::FieldList::TupleFieldList(list)) => list
                .fields()
                .filter_map(|field| {
                    self.semantics
                        .to_def(&field)
                        .map(|definition| VariantField::new(definition, &field))
                })
                .collect(),
            None => Vec::new(),
        }
    }

    fn formats(&self, root: &SyntaxNode) -> Vec<ast::FormatArgsExpr> {
        root.descendants()
            .flat_map(|node| {
                ast::FormatArgsExpr::cast(node.clone())
                    .map(|format| vec![format])
                    .or_else(|| {
                        ast::MacroCall::cast(node)
                            .and_then(|call| self.semantics.to_def(&call))
                            .map(|expansion| {
                                self.formats(
                                    &self.semantics.parse_or_expand(HirFileId::from(expansion)),
                                )
                            })
                    })
                    .unwrap_or_default()
            })
            .collect()
    }

    fn formats_in_its_message(&self, expansion: MacroCallId, field: Field) -> bool {
        !self
            .sources_in_messages(expansion, &HashSet::from([field]))
            .is_empty()
    }

    fn has_helper(&self, attributes: &[ast::Attr], names: &[&str]) -> bool {
        attributes
            .iter()
            .any(|attribute| self.is_helper(attribute, names))
    }

    fn is_helper(&self, attribute: &ast::Attr, names: &[&str]) -> bool {
        let database = self.semantics.db;

        attribute
            .path()
            .is_some_and(|path| match self.paths.resolve(&path) {
                Some(PathResolution::DeriveHelper(helper)) => {
                    self.derive_crates
                        .contains(&helper.derive().module(database).krate(database))
                        && names.contains(&helper.name(database).as_str())
                }
                _ => false,
            })
    }

    fn is_transparent(&self, owner: &impl HasAttrs) -> bool {
        owner.attrs().any(|attribute| {
            self.is_helper(&attribute, &[ERROR_MESSAGE_HELPER])
                && attribute
                    .meta()
                    .and_then(|meta| ast::TokenTreeMeta::cast(meta.syntax().clone()))
                    .and_then(|meta| meta.token_tree())
                    .is_some_and(|arguments| {
                        let delimiters = [
                            arguments.left_delimiter_token(),
                            arguments.right_delimiter_token(),
                        ];
                        let tokens: Vec<_> = arguments
                            .token_trees_and_tokens()
                            .filter_map(NodeOrToken::into_token)
                            .filter(|token| {
                                !token.kind().is_trivia()
                                    && !delimiters.contains(&Some(token.clone()))
                            })
                            .collect();

                        matches!(
                            tokens.as_slice(),
                            [only] if only.kind() == SyntaxKind::IDENT
                                && only.text() == TRANSPARENT_ERROR_MESSAGE
                        )
                    })
        })
    }

    fn source(
        &self,
        variant: &ast::Variant,
        fields: &[VariantField],
        enum_is_transparent: bool,
    ) -> Option<Field> {
        let database = self.semantics.db;
        let transparent = self.is_transparent(variant)
            || (enum_is_transparent
                && !self.has_helper(
                    &variant.attrs().collect::<Vec<ast::Attr>>(),
                    &[ERROR_MESSAGE_HELPER],
                ));

        if transparent {
            fields
                .first()
                .map(|VariantField { definition, .. }| *definition)
        } else {
            fields
                .iter()
                .find(|VariantField { attributes, .. }| {
                    self.has_helper(attributes, &ERROR_SOURCE_HELPERS)
                })
                .or_else(|| {
                    fields.iter().find(|VariantField { definition, .. }| {
                        definition.name(database).as_str() == ERROR_SOURCE_FIELD
                    })
                })
                .map(|VariantField { definition, .. }| *definition)
        }
    }

    fn variants(written: &ast::Enum) -> impl Iterator<Item = ast::Variant> {
        written
            .variant_list()
            .into_iter()
            .flat_map(|list| list.variants())
    }
}
