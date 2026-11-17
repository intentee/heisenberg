use serde::Serialize;

use crate::catalogue_entry::CatalogueEntry;

#[derive(Serialize)]
pub struct CatalogueDocument {
    pub category: &'static str,
    pub decidability: &'static str,
    pub rules: Vec<CatalogueEntry>,
    pub version: &'static str,
}
