
use atlas_http::HttpResponse;
use cicero_sdk::NavMenu;
use crate::server::apollo::user::ServerUser;
use std::sync::{Arc, Mutex};
use crate::server::api;
use super::PluginManager;
use log::debug;

pub struct EchoAssistant {

}

impl EchoAssistant {
    pub fn new() -> Self {
        Self { }
    }

    /// Get navigation menus
    pub fn get_menus(&self, plugin_mgr: &PluginManager, auth_user: Option<Arc<Mutex<ServerUser>>>) -> HttpResponse {
    debug!("Getting navigation menus via echo assistant");

        // Check for guest
        if auth_user.is_none() {
            return api::response(200, "", String::new());
        }
        let binding_user = auth_user.unwrap();
        let user = binding_user.lock().unwrap();

        // Go through all plugins
        let mut menus: Vec<NavMenu> = Vec::new();
        for (alias, plugin) in plugin_mgr.plugins.iter() {
            if !user.installed_plugins.contains(&alias) {
                continue;
            }

            let plugin_menus = plugin.get_nav_menus();
            menus.extend(plugin_menus);
        }
        debug!("Obtained total of {} menus, returning response to client.", menus.len());

        api::response(200, "", menus)
    }

}

