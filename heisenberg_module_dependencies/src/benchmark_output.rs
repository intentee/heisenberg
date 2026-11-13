use ra_ap_hir::HasAttrs as _;
use ra_ap_hir::LangItem;
use ra_ap_hir::Semantics;
use ra_ap_hir::Trait;
use ra_ap_hir::Type;
use ra_ap_hir_def::AdtId;
use ra_ap_hir_def::AttrDefId;
use ra_ap_hir_def::TraitId;
use ra_ap_hir_def::attrs::AttrFlags;
use ra_ap_ide_db::RootDatabase;

use crate::output_use::OutputUse;

fn combined(uses: impl IntoIterator<Item = OutputUse>) -> OutputUse {
    uses.into_iter()
        .fold(OutputUse::Free, |combined, output_use| {
            match (combined, output_use) {
                (OutputUse::MustUse, _) | (_, OutputUse::MustUse) => OutputUse::MustUse,
                (OutputUse::Undetermined, _) | (_, OutputUse::Undetermined) => {
                    OutputUse::Undetermined
                }
                (OutputUse::Free, OutputUse::Free) => OutputUse::Free,
            }
        })
}

pub struct BenchmarkOutput<'output, 'database> {
    pub semantics: &'output Semantics<'database, RootDatabase>,
}

impl<'database> BenchmarkOutput<'_, 'database> {
    pub fn output_use(&self, output: &Type<'database>) -> OutputUse {
        let database = self.semantics.db;

        if output.is_unknown() || output.as_type_param(database).is_some() {
            OutputUse::Undetermined
        } else if output.is_closure() {
            OutputUse::MustUse
        } else if let Some(adt) = output.as_adt() {
            if matches!(
                adt.attrs(database).lang(database),
                Some(LangItem::OwnedBox | LangItem::Pin)
            ) {
                output
                    .type_arguments()
                    .next()
                    .map_or(OutputUse::Undetermined, |inner| self.output_use(&inner))
            } else if AttrFlags::query(database, AttrDefId::from(AdtId::from(adt)))
                .contains(AttrFlags::IS_MUST_USE)
            {
                OutputUse::MustUse
            } else {
                OutputUse::Free
            }
        } else if let Some(traits) = output.as_impl_traits(database) {
            self.traits_use(traits)
        } else if let Some(implemented) = output.as_dyn_trait() {
            self.traits_use([implemented])
        } else if let Some((element, length)) = output.as_array(database) {
            if length == 0 {
                OutputUse::Free
            } else {
                self.output_use(&element)
            }
        } else {
            combined(
                output
                    .tuple_fields(database)
                    .iter()
                    .map(|field| self.output_use(field)),
            )
        }
    }

    fn traits_use(&self, traits: impl IntoIterator<Item = Trait>) -> OutputUse {
        let database = self.semantics.db;

        if traits
            .into_iter()
            .flat_map(|implemented| implemented.all_supertraits(database))
            .any(|implemented| {
                AttrFlags::query(database, AttrDefId::from(TraitId::from(implemented)))
                    .contains(AttrFlags::IS_MUST_USE)
            })
        {
            OutputUse::MustUse
        } else {
            OutputUse::Free
        }
    }
}
