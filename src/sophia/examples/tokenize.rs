use sophia::{Error, Sophia};
use std::env;
use std::process;

/// Tokenizes a sample sentence using the Sophia NLP library, demonstrating token and MWE iteration.
///
/// This example reads the vocabulary data directory from the first command-line argument.
/// If no argument is provided, it exits with an error message.
///
/// # Usage
///
/// ```bash
/// cargo run --example tokenize -- ./vocab_data
/// ```
///
/// The example processes the sentence "The quick brown fox jumps over the lazy dog" and prints:
/// - Individual tokens with their indices, words, and part-of-speech (POS) tags.
/// - Multi-word entities (MWEs) with their indices, words, and POS tags.
/// - Tokens with stopwords removed, showing only content-bearing words.
fn main() {
    // Retrieve the data directory from the first command-line argument
    let datadir = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No data directory provided.");
        eprintln!("Usage: cargo run --example tokenize -- <data_directory>");
        process::exit(1);
    });

    // Run the tokenization example, handling errors
    if let Err(e) = run(&datadir) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Runs the tokenization example with the provided data directory.
fn run(datadir: &str) -> Result<(), Error> {
    // Initialize Sophia with the vocabulary directory and language
    let sophia = Sophia::new(datadir, "en")?;

    // Tokenize the input text
    let input = "The quick brown fox jumps over the lazy dog";
    let output = sophia.tokenize(input)?;

    // Print individual tokens
    println!("\nIndividual Tokens:");
    println!("{:-<50}", "");
    println!("{:>6}  {:<15}  {}", "Index", "Word", "POS");
    println!("{:-<50}", "");
    for token in output.iter() {
        println!("{:>6}  {:<15}  {}", token.index, token.word, token.pos);
    }

    // Print multi-word entities (MWEs)
    println!("\nMulti-Word Entities (MWEs):");
    println!("{:-<50}", "");
    println!("{:>6}  {:<15}  {}", "Index", "Word", "POS");
    println!("{:-<50}", "");
    for token in output.mwe() {
        println!("{:>6}  {:<15}  {}", token.index, token.word, token.pos);
    }

    // Print tokens with stopwords removed
    println!("\nTokens with Stopwords Removed:");
    println!("{:-<50}", "");
    println!("{:>6}  {:<15}  {}", "Index", "Word", "POS");
    println!("{:-<50}", "");
    for token in output.remove_stop_words().iter() {
        println!("{:>6}  {:<15}  {}", token.index, token.word, token.pos);
    }

    Ok(())
}
