
use rand::Rng;
use std::collections::HashSet;
use std::iter;

/// Generate random password
pub fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789!@#$%^&*()_+";
    let mut password = String::with_capacity(length);
    let mut unique_chars = HashSet::new();

    while unique_chars.len() < length {
        let idx = rng.gen_range(0..charset.len());
        let c = charset[idx] as char;
        unique_chars.insert(c);
    }

    for c in unique_chars {
        password.push(c);
    }

    //password.as_mut_slice().shuffle(&mut rng);
    password
}

/// Generate API key
pub fn generate_api_key(length: usize) -> String {

        let charset: &[u8] =  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Uniform::from(0..charset.len())))
        .map(|idx| charset[idx] as char)
        .take(length)
        .collect()
}

