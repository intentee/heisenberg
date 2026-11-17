use clap::Args;

use crate::catalogue_format::CatalogueFormat;

#[derive(Args, Debug)]
pub struct CatalogueCommand {
    #[arg(
        long,
        value_enum,
        default_value = "text",
        help = "Print the catalogue as text or as a JSON document"
    )]
    pub format: CatalogueFormat,
}
