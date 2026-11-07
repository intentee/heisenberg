use ra_ap_hir::Crate;
use ra_ap_hir::Function;
use ra_ap_hir::GenericDef;
use ra_ap_hir::HasVisibility;
use ra_ap_hir::ModuleId;
use ra_ap_hir::Semantics;
use ra_ap_hir::Visibility;
use ra_ap_ide_db::RootDatabase;

use heisenberg_module_graph::method_group::MethodGroup;
use heisenberg_module_graph::method_visibility::MethodVisibility;

use crate::constructor::Constructor;
use crate::method_key::MethodKey;
use crate::method_owner::MethodOwner;

pub struct MethodGrouping<'grouping, 'database> {
    pub krate: Crate,
    pub owner: MethodOwner,
    pub semantics: &'grouping Semantics<'database, RootDatabase>,
}

impl MethodGrouping<'_, '_> {
    pub fn key(&self, method: Function) -> MethodKey {
        let database = self.semantics.db;
        let visibility = match self.owner {
            MethodOwner::InherentImpl(_) => match method.visibility(database) {
                Visibility::Public => MethodVisibility::Public,
                Visibility::PubCrate(_) => MethodVisibility::Crate,
                Visibility::Module(owner, _)
                    if owner == ModuleId::from(method.module(database)) =>
                {
                    MethodVisibility::Private
                }
                Visibility::Module(..) => MethodVisibility::Restricted,
            },
            MethodOwner::TraitDeclaration(_) => MethodVisibility::Public,
        };

        MethodKey {
            group: if self.constructs(method) {
                MethodGroup::Constructor(visibility)
            } else {
                MethodGroup::Method(visibility)
            },
            name: method.name(database).as_str().to_owned(),
        }
    }

    fn constructs(&self, method: Function) -> bool {
        let database = self.semantics.db;
        let constructor = Constructor {
            function: method,
            krate: self.krate,
            semantics: self.semantics,
        };

        match self.owner {
            MethodOwner::InherentImpl(implementation) => {
                constructor.constructs(&implementation.self_ty(database))
            }
            MethodOwner::TraitDeclaration(declaring) => GenericDef::Trait(declaring)
                .type_or_const_params(database)
                .into_iter()
                .filter_map(|parameter| parameter.as_type_param(database))
                .find(|parameter| parameter.is_implicit(database))
                .is_some_and(|self_type| constructor.constructs(&self_type.ty(database))),
        }
    }
}
