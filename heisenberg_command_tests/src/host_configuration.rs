use toml::Table;

pub struct HostConfiguration {
    pub name: String,
    pub table: Table,
}
