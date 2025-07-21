
use falcon_cli::*;
use falcon_cli::router::CliRouter;

#[cfg(feature="echo")]
use crate::cli::profile::{ProfileCreate, ProfileLogin, ProfileLogout};


#[cfg(feature="apollo")]
use crate::cli::sys::server::{SysServerEcho, SysServerApollo};

#[cfg(feature="apollo")]
use crate::cli::sys::dev::{SysDisableDevMode, SysEnableDevMode};

#[cfg(feature="devadmin")]
use crate::cli::sys::devadmin::{SysDevAdminVocab, SysDevAdminRender};


pub mod cli_test;
pub use self::cli_test::CliTest;

pub mod profile;
pub mod sys;

pub fn boot() -> CliRouter {

    // Add route
    let mut router = CliRouter::new();
    router.add::<CliTest>("test", vec![], vec![]); 

    // Add categories
    router.add_category("sys", "System Commands", "Various system commands to manage servers, credentials, API keys, etc.");

    // profile commands
    router.add_category("profile", "Profiles", "Manage and create local profiles.");
    router.add::<ProfileCreate>("profile create", vec!["prf cr"], vec![]); 
    router.add::<ProfileLogin>("profile login", vec!["login"], vec!["password-file"]); 
    router.add::<ProfileLogout>("profile logout", vec!["logout"], vec![]); 

    // Sys server commands
    router.add::<SysServerApollo>("sys server apollo", vec!["apollod"], vec!["num-threads", "port"]); 
    router.add::<SysServerEcho>("sys server echo", vec!["echod"], vec!["port"]); 

    // Sys developer commands
    router.add::<SysDisableDevMode>("sys dev disable-dev-mode", vec!["disable-dev-mode"], vec![]); 
    router.add::<SysEnableDevMode>("sys dev enable-dev-mode", vec!["enable-dev-mode"], vec![]); 

    // Dev admin commands
    #[cfg(feature="devadmin")] {
        router.add::<SysDevAdminVocab>("sys devadmin vocab", vec!["vocab"], vec![]);
        router.add::<SysDevAdminRender>("sys devadmin render", vec!["render-vocab"], vec![]);
    }


    router
}


