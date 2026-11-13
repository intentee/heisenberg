use std::collections::HashSet;

use ra_ap_hir::Crate;
use ra_ap_hir::ModuleDef;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;

use crate::constants::DESERIALIZE_TRAIT_PATHS;
use crate::constants::SERIALIZE_TRAIT_PATH;
use crate::standard_item::standard_item;

fn traits(database: &RootDatabase, crates: &HashSet<Crate>, paths: &[[&str; 2]]) -> HashSet<Trait> {
    crates
        .iter()
        .flat_map(|krate| {
            paths
                .iter()
                .filter_map(move |path| match standard_item(database, *krate, path) {
                    Some(ModuleDef::Trait(found)) => Some(found),
                    _ => None,
                })
        })
        .collect()
}

pub struct SerializationTraits {
    pub deserialize: HashSet<Trait>,
    pub serialize: HashSet<Trait>,
}

impl SerializationTraits {
    pub fn new(database: &RootDatabase, crates: &HashSet<Crate>) -> Self {
        Self {
            deserialize: traits(database, crates, &DESERIALIZE_TRAIT_PATHS),
            serialize: traits(database, crates, &[SERIALIZE_TRAIT_PATH]),
        }
    }
}
