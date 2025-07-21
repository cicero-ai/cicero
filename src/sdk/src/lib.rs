#![allow(warnings)]//#![allow(warnings)]
use serde_derive::{Serialize, Deserialize};
pub use self::task::CiceroTask;

pub mod api;
pub mod chat;
pub mod task;

pub trait CiceroPlugin {
    fn get_meta(&self) -> CiceroPluginMeta;
    fn get_nav_menus(&self) -> Vec<NavMenu>;
}

pub trait DashboardPage {
    fn render(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct CiceroPluginMeta {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub homepage: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum NavMenuType {
    Separator,
    Parent,
    Internal,
    External
}

#[derive(Serialize, Deserialize)]
pub enum NavMenuPosition {
    Top,
    Bottom,
    Before(String),
    After(String)
}

#[derive(Serialize, Deserialize)]
pub struct NavMenu {
    pub slug: String,
    pub name: String,
    pub parent: Option<String>,
    pub menu_type: NavMenuType,
    pub position: NavMenuPosition,
    pub icon: Option<String>,
    pub submenus: Vec<(String, String)>
}

impl NavMenu {

    pub fn separator(slug: &str, name: &str, position: NavMenuPosition) -> NavMenu {

    NavMenu {
            slug: slug.to_string(),
            name: name.to_string(),
            parent: None,
            menu_type: NavMenuType::Separator,
            position,
            icon: None,
            submenus: Vec::new()
        }
    }

    /// Define new parent dropdown menu with sub-menus
    pub fn parent(slug: &str, name: &str, icon: &str, position: NavMenuPosition) -> NavMenu {
    NavMenu {
            slug: slug.to_string(),
            name: name.to_string(),
            parent: None,
            menu_type: NavMenuType::Parent,
            position,
            icon: if icon.is_empty() { None } else { Some(icon.to_string()) },
            submenus: Vec::new()
        }
    }

    /// Add sub-menu to parent menu
    pub fn submenu(mut self, slug: &str, name: &str) -> Self {
        self.submenus.push((slug.to_string(), name.to_string()));
        self
    }

}


