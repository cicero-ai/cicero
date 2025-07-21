
use falcon_cli::*;
use falcon_cli::router::CliRouter;
pub use self::generate::CliGenerate;
pub use self::sophia::CliSophia;

mod generate;
mod sophia;

pub fn boot() -> CliRouter {

    // Add route
    let mut router = CliRouter::new();
    router.add::<CliGenerate>("generate", vec!["gen"], vec![]); 
    router.add::<CliSophia>("sophia", vec!["gen"], vec![]); 
    router
}




