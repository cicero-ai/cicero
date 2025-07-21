
use std::time::Instant;
use verax::License;
use cicero_interfaces::sophia::SophiaInterface;
use crate::server::CONFIG;
use sophia::Sophia;

pub fn test() {

    let datadir = format!("{}/nlu", CONFIG.general.libdir);
    let nlu = Sophia::new(&datadir, "en", License::load_api());





    let start_time = Instant::now();
    //let input = "she had to go swimming with her friends, but he couldn't have gone even if he wanted";
    let input = "I'm looking for a hotel room in downtown Los Angeles with free Wi-Fi and a gym, for a 3-night stay starting on March 12th.";
    let output = nlu.interpret(&input);

    for token in output.mwe {
        println!("{} -- {}", token.word, token.label);
    }

    println!("\nDone in {}ms", start_time.elapsed().as_millis());
}


