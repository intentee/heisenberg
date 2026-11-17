use ra_ap_ide_db::defs::Definition;

use heisenberg_module_graph::used_item::UsedItem;

pub struct UsedDefinition<'database> {
    pub definition: Definition<'database>,
    pub item: UsedItem,
}
