use ra_ap_hir::Access;
use ra_ap_hir::Adt;
use ra_ap_hir::Crate;
use ra_ap_hir::Function;
use ra_ap_hir::Semantics;
use ra_ap_hir::Type;
use ra_ap_ide_db::RootDatabase;
use ra_ap_ide_db::famous_defs::FamousDefs;

pub struct Constructor<'constructor, 'database> {
    pub function: Function,
    pub krate: Crate,
    pub semantics: &'constructor Semantics<'database, RootDatabase>,
}

impl Constructor<'_, '_> {
    pub fn constructs(&self, constructed: &Type<'_>) -> bool {
        let database = self.semantics.db;
        let famous_definitions = FamousDefs(self.semantics, self.krate);

        self.function
            .self_param(database)
            .is_none_or(|receiver| receiver.access(database) == Access::Owned)
            && if self.function.is_async(database) {
                self.function.async_ret_type(database)
            } else {
                Some(self.function.ret_type(database))
            }
            .is_some_and(|returned| {
                returned == *constructed
                    || [
                        famous_definitions.core_option_Option(),
                        famous_definitions.core_result_Result(),
                    ]
                    .into_iter()
                    .flatten()
                    .any(|wrapper| {
                        returned.as_adt() == Some(Adt::Enum(wrapper))
                            && returned
                                .type_arguments()
                                .next()
                                .is_some_and(|wrapped| wrapped == *constructed)
                    })
            })
    }
}
