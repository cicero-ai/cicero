
use lazy_static::lazy_static;
pub use self::assets::Assets;
pub use self::account_page::AccountPage;

lazy_static! {
    pub static ref ASSETS: Assets = Assets::new();
}

pub mod account_page;
pub mod assets;
pub mod nav_menu;
pub mod pages;



