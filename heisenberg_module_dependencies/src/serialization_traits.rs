use std::collections::BTreeSet;
use std::collections::HashSet;

use ra_ap_hir::Crate;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;

use crate::module_dependencies_error::ModuleDependenciesError;
use crate::serialization_trait::SerializationTrait;
use crate::standard_item::standard_item;

fn found(
    database: &RootDatabase,
    crates: &HashSet<Crate>,
    required: SerializationTrait,
) -> HashSet<Trait> {
    crates
        .iter()
        .filter_map(
            |krate| match standard_item(database, *krate, &required.path()) {
                Some(ModuleDef::Trait(declared)) => Some(declared),
                _ => None,
            },
        )
        .collect()
}

fn missing(declared: &HashSet<Trait>, required: SerializationTrait) -> Option<SerializationTrait> {
    declared.is_empty().then_some(required)
}

pub struct SerializationTraits {
    pub deserialize: HashSet<Trait>,
    pub serialize: HashSet<Trait>,
}

impl SerializationTraits {
    /// # Errors
    ///
    /// Returns [`ModuleDependenciesError::SerializationTraitsMissing`] when the vetted
    /// serialization crates are present and none of them declares one of the serialization
    /// traits.
    pub fn new(
        database: &RootDatabase,
        crates: &HashSet<Crate>,
    ) -> Result<Self, ModuleDependenciesError> {
        let serialize = found(database, crates, SerializationTrait::Serialize);
        let deserialize = found(database, crates, SerializationTrait::Deserialize);
        let deserialize_owned = found(database, crates, SerializationTrait::DeserializeOwned);
        let absent: Vec<SerializationTrait> = if crates.is_empty() {
            Vec::new()
        } else {
            [
                missing(&serialize, SerializationTrait::Serialize),
                missing(&deserialize, SerializationTrait::Deserialize),
                missing(&deserialize_owned, SerializationTrait::DeserializeOwned),
            ]
            .into_iter()
            .flatten()
            .collect()
        };

        if absent.is_empty() {
            Ok(Self {
                deserialize: deserialize.into_iter().chain(deserialize_owned).collect(),
                serialize,
            })
        } else {
            Err(ModuleDependenciesError::SerializationTraitsMissing {
                crates: crates
                    .iter()
                    .filter_map(|krate| krate.display_name(database))
                    .map(|name| name.to_string())
                    .collect::<BTreeSet<String>>(),
                traits: absent,
            })
        }
    }
}
