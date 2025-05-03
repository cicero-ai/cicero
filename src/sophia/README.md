
# Sophia NLU Engine (cicero-sophia)

High-performance NLU (natural language understanding) engine built in Rust for speed, accuracy, and privacy.

![Crates.io](https://img.shields.io/crates/v/cicero-sophia.svg)
![Docs.rs](https://docs.rs/cicero-sophia/badge.svg)
![License](https://img.shields.io/badge/license-GPLv3-blue.svg) (LICENSE)

## Features

* 

**Core Capabilities**

* Industry-leading vocabulary with 914,000 (full) or 145,000 (lite) words
* Sophisticated categorization system spanning 8,700+ hierarchical categories, allowing for easy word to action mapping
* Advanced language processing including POS tagging, anaphora resolution, and named entity recognition
* Intelligent phrase parsing with automated spelling correction

**Performance**

* Process ~25,000 words per second on a single thread
* Lightweight deployment: Single 79MB (lite) or 177MB (full) data store
* Zero external dependencies or API calls required
* Privacy-focused with all processing done locally

## License

Typical dual license model, free and open source for individual use via the GPLv3 license, but premium license required for commercial use.  For full details including online demo, please visit: [https://cicero.sh/sophia/](https://cicero.sh/sophia/).


## Installation

Add cicero-sophia to your project by including it in your Cargo.toml:

toml

[dependencies]
cicero-sophia = "0.3.0"


## Vocabulary Data Store

To use Sophia, you must obtain the vocabulary data store, which is available free of charge.  Simply visit [https://cicero.sh/](https://ciciero.sh/) register for a free account, and the vocabulary data store is available for download within the member's area.

## Usage

**Example 1: Tokenizing Text**

```rust
use sophia::{Sophia, Error};

fn main() -> Result<(), Error> {
    // Initialize Sophia
    let datadir = "./vocab_data";
    let sophia = Sophia::new(datadir, "en")?;

    // Tokenize the input text
    let output = sophia.tokenize("The quick brown fox jumps over the lazy dog")?;

    // Print individual tokens
    println!("Individual Tokens:");
    for token in output.iter() {
        println!("  Word: {} POS: {}", token.word, token.pos);
    }

    // Print MWEs
    println!("\nMulti-Word Entities (MWEs):");
    for token in output.mwe() {
        println!("  Word: {} POS: {}", token.word, token.pos);
    }

    Ok(())
}
```

**Example 2: Interpreting Text**

```rust

use sophia::{Sophia, Error};

fn main() -> Result<(), Error> {
    // Initialize Sophia
    let datadir = "./vocab_data";
    let sophia = Sophia::new(datadir, "en")?;

    // Interpret the input text
    let output = sophia.interpret("The quick brown fox jumps over the lazy dog")?;

    // Print phrases
    println!("Phrases:");
    for phrase in output.phrases.iter() {
        println!("  {:?}", phrase);
    }

    // Print individual tokens
    println!("\nIndividual Tokens:");
    for token in output.tokens.iter() {
        println!("  Word: {} POS: {}", token.word, token.pos);
    }

    Ok(())
}
```


**Example 3: Retrieve individual word / toekn**

```rust

use sophia::{Sophia, Error};

fn main() -> Result<(), Error> {
    // Initialize Sophia
    let datadir = "./vocab_data";
    let sophia = Sophia::new(datadir, "en")?;

    // Get word
    let token = sophia.get_word("future").unwrap();
    println!("Got word {}, id {}, pos {}", token.word, token.index, token.pos);

    // Get specific token
    let token = sophia.get_token(82251).unwrap();
    println!("Got word {}, id {}, pos {}", token.word, token.index, token.pos);

    Ok(())
}
```

**Example 4: Retrieve Category**

```rust

use sophia::{Sophia, Error};

fn main() -> Result<(), Error> {
    // Initialize Sophia
    let datadir = "./vocab_data";
    let sophia = Sophia::new(datadir, "en")?;

    // Get category
    let cat = sophia.get_category("verbs/action/travel/depart").unwrap();
    println!("name {}", cat.name);
    println!("fqn: {}", cat.fqn);
    println!("word ids: {:?}", cat.words);

    Ok(())
}
```

## Contact

For all inquiries, please complete the contact form at: https://cicero.sh/contact



