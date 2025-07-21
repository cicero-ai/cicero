#![allow(warnings)]//#![allow(warnings)]
use cicero_sdk::{CiceroPlugin, CiceroPluginMeta, NavMenu, NavMenuType, NavMenuPosition};

pub mod chat;

pub struct CiceroCore { }

impl CiceroCore {

    pub fn new() -> Self {
        Self { }
    }

}

impl CiceroPlugin for CiceroCore {

    fn get_meta(&self) -> CiceroPluginMeta {
        CiceroPluginMeta {
            slug: "core".to_string(),
            name: "Cicero Core".to_string(),
            description: "Provides core functionality such as plugin and model management via web based panel.".to_string(),
            homepage: "https://cicero.sh/".to_string()
        }
    }

    fn get_nav_menus(&self) -> Vec<NavMenu> {

        let mut menus = vec![
            NavMenu::separator("hdr_setup", "Setup", NavMenuPosition::Top),
            NavMenu::parent("settings", "Settings", "fa fa-fw fa-wrench", NavMenuPosition::After("hdr_setup".to_string()))
                .submenu("general", "General")
                .submenu("device", "Devices")
        ];

        menus
    }

}

/// Initialize the CiceroPlugin
#[no_mangle]
pub extern "C" fn init_plugin() -> Box<dyn CiceroPlugin> {
    Box::new(CiceroCore::new())
}

