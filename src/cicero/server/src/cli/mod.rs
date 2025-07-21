
use falcon_cli::*;
use falcon_cli::router::CliRouter;
//use self::sys::server::{SysServerEcho, SysServerApollo};
//use self::sys::dev::{SysDisableDevMode, SysEnableDevMode};

pub mod cli_test;
pub use self::cli_test::CliTest;

//mod sys;

/// Boot the CLI router and all available CLI commands.
pub fn boot() -> CliRouter {

    // Add route
    let mut router = CliRouter::new();
    router.add::<CliTest>("test", vec![], vec![]); 

    // Add categories
    //router.add_category("sys", "System Commands", "Various system commands to manage servers, credentials, API keys, etc.");

    // Sys server commands
    //router.add::<SysServerApollo>("sys server apollo", vec!["apollod"], vec!["num-threads", "port"]); 

    // Sys developer commands
    //router.add::<SysDisableDevMode>("sys dev disable-dev-mode", vec!["disable-dev-mode"], vec![]); 
    //router.add::<SysEnableDevMode>("sys dev enable-dev-mode", vec!["enable-dev-mode"], vec![]); 

    router
}



