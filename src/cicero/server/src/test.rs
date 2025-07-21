
use cicero::security::forge;
use std::fs;


pub fn test() {

    let data = fs::read("/home/boxer/devel/cicero/cicero/src/cicero/server/Cargo.toml").unwrap();

    let enc_data = forge::encrypt_with_str(&data, "white4882").unwrap();

    let dec = forge::decrypt_with_str(&enc_data, "white4882").unwrap();

    if dec == data { println!("Yep, all good"); }
    else { println!("Nope, fucked up"); }

    println!("Yes, testing");

}


