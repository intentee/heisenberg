#[cfg(feature = "web_admin_panel")]
pub fn admin_panel_port() -> u16 {
    8062
}

#[cfg(not(feature = "web_admin_panel"))]
pub fn admin_panel_port() -> u16 {
    0
}
