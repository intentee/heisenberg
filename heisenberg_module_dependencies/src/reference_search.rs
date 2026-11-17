use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use ra_ap_hir::Adt;
use ra_ap_hir::InFile;
use ra_ap_hir::Local;
use ra_ap_hir::Module;
use ra_ap_hir::Name;
use ra_ap_hir::Semantics;
use ra_ap_hir::Visibility;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::base_db::EditionedFileId;
use ra_ap_ide_db::defs::Definition;
use ra_ap_ide_db::defs::NameClass;
use ra_ap_ide_db::defs::NameRefClass;
use ra_ap_ide_db::search::FileReferenceNode;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::AstToken;
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax::SyntaxToken;
use ra_ap_syntax::TextSize;
use ra_ap_syntax::ast;
use ra_ap_vfs::FileId;

use crate::found_reference::FoundReference;
use crate::found_references::FoundReferences;
use crate::reference_scope::ReferenceScope;
use crate::searched_tokens::SearchedTokens;
use crate::string_site::StringSite;
use crate::template_captures::TemplateCaptures;
use crate::token_site::TokenSite;

const SELF_TYPE: &str = "Self";

struct AssignmentSide {
    expression: ast::BinExpr,
    operator: ast::BinaryOp,
}

pub struct ReferenceSearch<'search, 'database> {
    pub file_modules: &'search HashMap<FileId, Vec<Module>>,
    pub semantics: &'search Semantics<'database, RootDatabase>,
    pub template_captures: &'search TemplateCaptures<'search, 'database>,
    pub tokens: &'search SearchedTokens,
    pub trees: RefCell<HashMap<FileId, SyntaxNode>>,
}

impl<'database> ReferenceSearch<'_, 'database> {
    pub fn references(
        &self,
        definition: Definition<'database>,
        with_self_references: bool,
    ) -> FoundReferences {
        definition
            .name(self.semantics.db)
            .zip(self.scope(definition))
            .map(|(name, scope)| self.found(definition, &name, &scope, with_self_references))
            .unwrap_or_default()
    }

    fn aliases(&self, referred: Definition<'database>, definition: Definition<'database>) -> bool {
        let database = self.semantics.db;

        matches!(
            (referred, definition),
            (Definition::TypeAlias(alias), Definition::Adt(adt))
                if alias.ty(database) == adt.ty(database)
        )
    }

    fn found(
        &self,
        definition: Definition<'database>,
        name: &Name,
        scope: &ReferenceScope,
        with_self_references: bool,
    ) -> FoundReferences {
        let mut found = FoundReferences::default();
        let written = name.as_str();

        for spelling in [written.to_owned(), format!("r#{written}")] {
            for named in self.nodes_named(&spelling, scope) {
                if let Some(reference) = self.reference(definition, &named, with_self_references) {
                    self.record(&named, reference, &mut found);
                }
            }
        }

        self.record_captures(definition, written, scope, &mut found);

        if with_self_references && let Definition::Adt(adt) = definition {
            for named in self.nodes_named(SELF_TYPE, scope) {
                if let Some(name_ref) = ast::NameRef::cast(named.clone())
                    && self.names_type_of(&name_ref, adt)
                {
                    self.record(
                        &named,
                        FoundReference {
                            name: FileReferenceNode::NameRef(name_ref),
                            writes: false,
                        },
                        &mut found,
                    );
                }
            }
        }

        found
    }

    fn is_in_scope(&self, scope: &ReferenceScope, TokenSite { file, offset }: &TokenSite) -> bool {
        let database = self.semantics.db;

        match scope {
            ReferenceScope::Crates(crates) => self.file_modules.get(file).is_some_and(|modules| {
                modules
                    .iter()
                    .any(|module| crates.contains(&module.krate(database)))
            }),
            ReferenceScope::ModuleTree(root) => {
                root.definition_source_file_id(database)
                    .original_file(database)
                    .file_id(database)
                    == *file
                    || self.file_modules.get(file).is_some_and(|modules| {
                        modules.iter().any(|module| {
                            module
                                .path_to_root(database)
                                .into_iter()
                                .any(|ancestor| ancestor == *root)
                        })
                    })
            }
            ReferenceScope::Range {
                file: searched,
                range,
            } => file == searched && range.contains_inclusive(*offset),
        }
    }

    fn local_scope(&self, local: Local<'database>) -> Option<ReferenceScope> {
        let database = self.semantics.db;

        local
            .primary_source(database)
            .source
            .map(|declared| {
                declared.either(
                    |pattern| pattern.syntax().clone(),
                    |receiver| receiver.syntax().clone(),
                )
            })
            .ancestors_with_macros(database)
            .find(|InFile { value, .. }| ast::Item::can_cast(value.kind()))
            .map(|owner| {
                let searched = owner.original_file_range_with_macro_call_input(database);

                ReferenceScope::Range {
                    file: searched.file_id.file_id(database),
                    range: searched.range,
                }
            })
    }

    fn names_type_of(&self, name_ref: &ast::NameRef, adt: Adt) -> bool {
        matches!(
            NameRefClass::classify(self.semantics, name_ref),
            Some(NameRefClass::Definition(Definition::SelfType(implementation), _))
                if implementation.self_ty(self.semantics.db).as_adt() == Some(adt)
        )
    }

    fn nodes(&self, file: FileId, token: SyntaxToken) -> Vec<SyntaxNode> {
        let editioned: EditionedFileId = self.semantics.attach_first_edition(file);

        if self
            .semantics
            .is_inside_macro_call(InFile::new(editioned.into(), &token))
        {
            self.semantics
                .descend_into_macros_exact(token)
                .into_iter()
                .filter_map(|descended| descended.parent())
                .collect()
        } else {
            token.parent().into_iter().collect()
        }
    }

    fn nodes_named(&self, spelling: &str, scope: &ReferenceScope) -> Vec<SyntaxNode> {
        self.tokens
            .names
            .get(spelling)
            .into_iter()
            .flatten()
            .filter(|site| self.is_in_scope(scope, site))
            .filter_map(|TokenSite { file, offset }| {
                self.token(*file, *offset, spelling)
                    .map(|token| self.nodes(*file, token))
            })
            .flatten()
            .collect()
    }

    fn record(
        &self,
        named: &SyntaxNode,
        reference: FoundReference,
        FoundReferences { by_file }: &mut FoundReferences,
    ) {
        by_file
            .entry(
                self.semantics
                    .original_range(named)
                    .file_id
                    .file_id(self.semantics.db),
            )
            .or_default()
            .push(reference);
    }

    fn record_captures(
        &self,
        definition: Definition<'database>,
        written: &str,
        scope: &ReferenceScope,
        FoundReferences { by_file }: &mut FoundReferences,
    ) {
        for (file, sites) in &self.tokens.strings {
            for StringSite { offset, text } in sites {
                if text.contains(written)
                    && self.is_in_scope(
                        scope,
                        &TokenSite {
                            file: *file,
                            offset: *offset,
                        },
                    )
                    && let Some(string) = self
                        .token(*file, *offset, text)
                        .filter(|token| {
                            self.template_captures
                                .captures(token)
                                .contains(&Some(definition))
                        })
                        .and_then(ast::String::cast)
                {
                    let range = string.syntax().text_range();

                    by_file.entry(*file).or_default().push(FoundReference {
                        name: FileReferenceNode::FormatStringEntry(string, range),
                        writes: false,
                    });
                }
            }
        }
    }

    fn reference(
        &self,
        definition: Definition<'database>,
        named: &SyntaxNode,
        with_self_references: bool,
    ) -> Option<FoundReference> {
        ast::NameRef::cast(named.clone())
            .and_then(|name_ref| {
                NameRefClass::classify(self.semantics, &name_ref)
                    .and_then(|class| match class {
                        NameRefClass::Definition(referred, _) => (referred == definition
                            || (with_self_references && self.aliases(referred, definition)))
                        .then_some(referred),
                        NameRefClass::FieldShorthand {
                            local_ref,
                            field_ref,
                            ..
                        } => [Definition::Field(field_ref), Definition::Local(local_ref)]
                            .into_iter()
                            .find(|referred| *referred == definition),
                        NameRefClass::ExternCrateShorthand { .. } => None,
                    })
                    .map(|referred| FoundReference {
                        writes: Self::writes(&referred, &name_ref),
                        name: FileReferenceNode::NameRef(name_ref),
                    })
            })
            .or_else(|| {
                ast::Name::cast(named.clone()).and_then(|written| {
                    NameClass::classify(self.semantics, &written)
                        .is_some_and(|class| match class {
                            NameClass::PatFieldShorthand { field_ref, .. } => {
                                Definition::Field(field_ref) == definition
                            }
                            NameClass::ConstReference(referred) => referred == definition,
                            NameClass::Definition(_) => false,
                        })
                        .then(|| FoundReference {
                            name: FileReferenceNode::Name(written),
                            writes: false,
                        })
                })
            })
    }

    fn scope(&self, definition: Definition<'database>) -> Option<ReferenceScope> {
        let database = self.semantics.db;

        if let Definition::Local(local) = definition {
            self.local_scope(local)
        } else {
            definition
                .visibility(database)
                .and_then(|visibility| match visibility {
                    Visibility::Module(module, _) => {
                        Some(ReferenceScope::ModuleTree(module.into()))
                    }
                    Visibility::PubCrate(krate) => {
                        Some(ReferenceScope::Crates(HashSet::from([krate.into()])))
                    }
                    Visibility::Public => definition.module(database).map(|module| {
                        ReferenceScope::Crates(
                            module
                                .krate(database)
                                .transitive_reverse_dependencies(database)
                                .collect(),
                        )
                    }),
                })
        }
    }

    fn token(&self, file: FileId, offset: TextSize, spelling: &str) -> Option<SyntaxToken> {
        self.trees
            .borrow_mut()
            .entry(file)
            .or_insert_with(|| {
                self.semantics
                    .parse(self.semantics.attach_first_edition(file))
                    .syntax()
                    .clone()
            })
            .token_at_offset(offset)
            .find(|token| token.text_range().start() == offset && token.text() == spelling)
    }

    fn writes(named: &Definition<'_>, name_ref: &ast::NameRef) -> bool {
        matches!(named, Definition::Local(_) | Definition::Field(_))
            && name_ref
                .syntax()
                .ancestors()
                .find_map(|ancestor| {
                    ast::BinExpr::cast(ancestor).and_then(|expression| {
                        expression.op_kind().map(|operator| AssignmentSide {
                            expression,
                            operator,
                        })
                    })
                })
                .is_some_and(
                    |AssignmentSide {
                         expression,
                         operator,
                     }| {
                        matches!(operator, ast::BinaryOp::Assignment { .. })
                            && expression.lhs().is_some_and(|assignee| {
                                assignee
                                    .syntax()
                                    .text_range()
                                    .contains_range(name_ref.syntax().text_range())
                            })
                    },
                )
    }
}
