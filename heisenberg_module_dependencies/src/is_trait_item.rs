use ra_ap_hir::AsAssocItem;
use ra_ap_hir::Function;
use ra_ap_hir::Trait;
use ra_ap_ide_db::RootDatabase;

pub fn is_trait_item(database: &RootDatabase, function: Function, owner: Trait) -> bool {
    function
        .as_assoc_item(database)
        .and_then(|item| item.container_or_implemented_trait(database))
        == Some(owner)
}
