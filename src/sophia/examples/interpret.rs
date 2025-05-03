use cicero_sophia::{Error, Sophia};
use std::env;
use std::process;

/// Interprets a sample sentence using the Sophia NLP library, demonstrating phrase, token, and score analysis.
///
/// This example reads the vocabulary data directory from the first command-line argument.
/// If no argument is provided, it exits with an error message.
///
/// # Usage
///
/// ```bash
/// cargo run --example interpret -- ./vocab_data
/// ```
///
/// The example processes the sentence "The quick brown fox jumps over the lazy dog" and prints:
/// - Phrases with their token contents (debug format).
/// - Individual tokens with their indices, words, and part-of-speech (POS) tags.
/// - Multi-word entities (MWEs) with their indices, words, and POS tags.
/// - Classification scores with their labels and floating-point values.
fn main() {
    // Retrieve the data directory from the first command-line argument
    let datadir = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: No data directory provided.");
        eprintln!("Usage: cargo run --example interpret -- <data_directory>");
        process::exit(1);
    });

    // Run the interpretation example, handling errors
    if let Err(e) = run(&datadir) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Runs the interpretation example with the provided data directory.
fn run(datadir: &str) -> Result<(), Error> {
    // Initialize Sophia with the vocabulary directory and language
    let sophia = Sophia::new(datadir, "en")?;

    // Interpret the input text
    let input = "The quick brown fox jumps over the lazy dog";
    let output = sophia.interpret(input)?;

    // Print phrases
    println!("\nPhrases:");
    println!("{:-<50}", "");
    for (i, phrase) in output.phrases.iter().enumerate() {
        println!("Phrase {}: {:?}", i + 1, phrase);
    }

    // Print individual tokens
    println!("\nIndividual Tokens:");
    println!("{:-<50}", "");
    println!("{:>6}  {:<15}  {}", "Index", "Word", "POS");
    println!("{:-<50}", "");
    for token in output.tokens.iter() {
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

    // Print classification scores
    println!("\nClassification Scores:");
    println!("{:-<50}", "");
    println!("{:>6}  {:<15}", "Label", "Score");
    println!("{:-<50}", "");
    for (label, score) in output.scores.iter() {
        println!("{:>6}  {:<15.4}", label, score.to_f32());
    }

    Ok(())
}
