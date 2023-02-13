use crate::interface::types::{MenuConfig, MenuItem};

pub fn exec(menu_config: &MenuConfig) {
    match menu_config.active_item {
        MenuItem::Home => {}
        _ => {}
    }
}
