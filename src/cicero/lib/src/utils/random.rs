
use rand::Rng;
use rand::seq::SliceRandom;
use std::iter;

/// Generate a random password of specified length.
/// Uses a cryptographically secure RNG and a fixed character set.
/// Panics if `length` exceeds the number of available unique characters (72).
pub fn generate_password(length: usize) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                   abcdefghijklmnopqrstuvwxyz\
                   0123456789!@#$%^&*()_+";
    if length > charset.len() {
        panic!("Length exceeds available unique characters ({})", charset.len());
    }
    
    let mut rng = rand::thread_rng();

    let mut chars: Vec<char> = charset.chars().collect();
    chars.shuffle(&mut rng);
    chars[..length].iter().collect()
}


/// Generate API key
pub fn generate_api_key(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let charset_len = charset.len();
    let mut rng = rand::thread_rng();
    
    iter::repeat(())
        .map(|()| charset[rng.gen_range(0..charset_len)] as char)
        .take(length)
        .collect()
}

