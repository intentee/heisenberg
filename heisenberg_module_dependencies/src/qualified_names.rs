use std::iter;

use ra_ap_hir::Crate;
use ra_ap_hir::HirDisplay;
use ra_ap_hir::Module;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_hir::TypeAlias;
use ra_ap_ide_db::RootDatabase;

use crate::module_dependencies_error::ModuleDependenciesError;

pub struct QualifiedNames<'names, 'database> {
    pub krate: Crate,
    pub semantics: &'names Semantics<'database, RootDatabase>,
}

impl<'database> QualifiedNames<'_, 'database> {
    pub fn error_type(&self, error: &Type<'database>) -> Result<String, ModuleDependenciesError> {
        let database = self.semantics.db;
        let rendered = error
            .display_truncated(database, None, self.krate.to_display_target(database))
            .to_string();

        match error.as_adt() {
            Some(definition) => self
                .module_path(definition.module(database))
                .map(|path| format!("{path}::{rendered}")),
            None => Ok(rendered),
        }
    }

    pub fn type_alias(&self, alias: TypeAlias) -> Result<String, ModuleDependenciesError> {
        let database = self.semantics.db;

        self.module_path(alias.module(database))
            .map(|path| format!("{path}::{}", alias.name(database).as_str()))
    }

    fn module_path(&self, module: Module) -> Result<String, ModuleDependenciesError> {
        let database = self.semantics.db;
        let krate = module.krate(database);

        krate
            .display_name(database)
            .ok_or(ModuleDependenciesError::ErrorTypeCrateWithoutName(krate))
            .map(|crate_name| {
                iter::once(crate_name.to_string())
                    .chain(
                        module
                            .path_segments(database)
                            .map(|segment| segment.as_str().to_owned()),
                    )
                    .collect::<Vec<String>>()
                    .join("::")
            })
    }
}
