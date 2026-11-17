use ra_ap_hir::ModuleDef;
use ra_ap_hir::PathResolution;
use ra_ap_hir::Type;
use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;
use ra_ap_syntax::ast::HasGenericArgs;

use heisenberg_module_graph::error_issuance::ErrorIssuance;
use heisenberg_module_graph::error_violation::ErrorViolation;
use heisenberg_module_graph::untyped_error_name::UntypedErrorName;

use crate::error_boundary::ErrorBoundary;
use crate::module_dependencies_error::ModuleDependenciesError;
use crate::path_resolutions::PathResolutions;
use crate::qualified_names::QualifiedNames;
use crate::signature_scope::SignatureScope;

pub struct WrittenResultError<'written, 'database> {
    pub boundary: &'written ErrorBoundary<'written, 'database>,
    pub names: &'written QualifiedNames<'written, 'database>,
    pub paths: &'written PathResolutions<'written, 'database>,
    pub written: &'written ast::PathType,
}

impl<'database> WrittenResultError<'_, 'database> {
    pub fn violation(&self) -> Result<Option<ErrorViolation>, ModuleDependenciesError> {
        let semantics = self.names.semantics;
        let database = semantics.db;
        let error_types = &self.boundary.error_types;
        let scope = SignatureScope {
            node: self.written.syntax(),
        };

        scope
            .is_in_a_signature()
            .then(|| semantics.resolve_type(&ast::Type::PathType(self.written.clone())))
            .flatten()
            .and_then(|resolved| error_types.result_error(&resolved))
            .filter(|error| {
                error.as_type_param(database).is_none()
                    && error.as_associated_type_parent_trait(database).is_none()
            })
            .map_or(Ok(None), |error| {
                if !error_types.is_error_type(&error) {
                    self.name(&error)
                        .map(|name| Some(self.boundary.untyped_violation(&error, name)))
                } else if scope.is_in_a_return_type()
                    && let Some(owner) = self.boundary.outside_owner(&error)
                {
                    self.names.crate_name(owner).and_then(|owner| {
                        self.name(&error).map(|name| {
                            Some(ErrorViolation::IssuedOutsideItsCrate {
                                issuance: ErrorIssuance::Returned { error: name },
                                owner,
                            })
                        })
                    })
                } else {
                    Ok(None)
                }
            })
    }

    fn name(&self, error: &Type<'database>) -> Result<UntypedErrorName, ModuleDependenciesError> {
        let path = self.written.path();
        let result = self.written.syntax().to_string();

        match path.as_ref().and_then(|path| self.paths.resolve(path)) {
            Some(PathResolution::Def(ModuleDef::TypeAlias(alias))) => {
                self.names.type_alias(alias).and_then(|alias| {
                    self.names
                        .error_type(error)
                        .map(|error_type| UntypedErrorName::NamedByAlias { alias, error_type })
                })
            }
            _ => path
                .and_then(|path| path.segment())
                .and_then(|segment| segment.generic_arg_list())
                .and_then(|arguments| {
                    arguments
                        .generic_args()
                        .filter_map(|argument| ast::TypeArg::cast(argument.syntax().clone()))
                        .nth(1)
                })
                .and_then(|argument| argument.ty())
                .ok_or(ModuleDependenciesError::ResultErrorNotWritten { written: result })
                .map(|written_error| UntypedErrorName::Named {
                    error_type: written_error.syntax().to_string(),
                }),
        }
    }
}
