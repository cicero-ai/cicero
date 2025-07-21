
use std::collections::HashMap;
use falcon_cli::help::CliHelpScreen;
use falcon_cli::*;
pub use falcon_cli::{indexmap, IndexMap};
use sophia::Sophia;
use sophia::vocab::hashes::id2pos;
use sophia::tokenizer::Token;
use sophia::vocab::VocabDatabase;
use sophia::compile::{database, text2frequency, word2category, word_count, pos_freq2scores, text2noun_pairings};
use std::fs;
use verax::License;
use std::env::args;
use crate::server::CONFIG;
use log::info;

#[derive(Default)]
pub struct SysDevAdminVocab { }

impl SysDevAdminVocab {

    fn compile(&self) {
        cli_send!("Compiling vocab, this may takea  few minutes...\n");
        //let vocab_dir = format!("{}/nlu/{}/vocab", CONFIG.general.libdir, CONFIG.general.language);
        let vocab_dir = format!("{}/vocab/out", CONFIG.general.libdir);
        let mut vocab = database::compile(&vocab_dir.as_str());

        // Save vocab file
        let filename = format!("{}/nlu/{}.dat", CONFIG.general.libdir, CONFIG.general.language);
        vocab.save(&filename.as_str(), &License::load_api());
        cli_send!("Successfully saved vocab file at, {}", filename);
    }

    /// Text 2 frequency
    pub fn text2freq(&self, args: &Vec<String>) {

        // Get dirname
        if args.len() == 1 {
            cli_error!("No --dir flag present, which is required");
            return;
        }
        let parent_dir = args[1].to_string();

        // Create frequence
        let mut freq = text2frequency::process(&parent_dir, &CONFIG.general.libdir, &CONFIG.general.language, License::load_api());

        // Save file
        let filename = format!("{}/nlu/word_freq.dat", CONFIG.general.libdir);
        freq.save(&filename, &License::load_api());
        cli_send!("Saved word frequency file at, {}\n\n", filename);
    }

    /// Save word frequency to category frequency
    fn word2category(&self) {
        let mut res = word2category::compile(&CONFIG.general.libdir, &CONFIG.general.language, License::load_api());
        let freq_dbfile = format!("{}/nlu/cat_freq.dat", CONFIG.general.libdir);
        res.save(&freq_dbfile, &License::load_api());
        cli_send!("Saved category frequence to, {}\n\n", freq_dbfile);
    }

    /// Word count
    fn word_count(&self, args: &Vec<String>) {

        // Get dirname
        if args.len() == 1 {
            cli_error!("No --dir flag present, which is required");
            return;
        }
        let parent_dir = args[1].to_string();

        // Compile word count
        word_count::process(&parent_dir, &CONFIG.general.libdir, &CONFIG.general.language, License::load_api());
    }

    /// Text to POS frequency scores
    fn text2pos_freq(&self, args: &Vec<String>) {
        // Get dirname
        if args.len() == 1 {
            cli_error!("No --dir flag present, which is required");
            return;
        }
        let parent_dir = args[1].to_string();

        let sophia = Sophia::new(&CONFIG.general.libdir, &CONFIG.general.language, License::load_api());
        pos_freq2scores::compile(&parent_dir, &sophia);
    }

    /// Text to noun pairings
    fn text2noun_pairings(&self, args: &Vec<String>) {
        // Get dirname
        if args.len() == 1 {
            cli_error!("No --dir flag present, which is required");
            return;
        }
        let parent_dir = args[1].to_string();

        let sophia = Sophia::new(&CONFIG.general.libdir, &CONFIG.general.language, License::load_api());
        text2noun_pairings::compile(&parent_dir, &sophia);
    }

    /// Create blacklist
    fn blacklist(&self) {

        // Create blacklist
        let (missing, unknown) = word_count::create_blacklist(&CONFIG.general.libdir, &CONFIG.general.language, License::load_api());

        // Save missing
        let missing_file = format!("{}/nlu/missing", CONFIG.general.libdir);
        fs::write(&missing_file, &missing.join("\n").to_string());

        // Save unknown.json
        let unknown_file = format!("{}/nlu/unknown.json", CONFIG.general.libdir);
        fs::write(&unknown_file, &serde_json::to_string(&unknown).unwrap()).unwrap();

        println!("Saved /lib/nlu/missing file with {} entries", missing.len());
        println!("Saved /lib/nlu/unknown.json file with {} entries", unknown.len());
    }



    /// Lookup words
    fn lookup(&self) {

        // GEt nlu
        let datadir = format!("{}/nlu", CONFIG.general.libdir);
        let sophia = Sophia::new(&datadir, &CONFIG.general.language, License::load_api());
        let rev_wordlist: HashMap<i32, String> = serde_json::from_str(&fs::read_to_string(&format!("{}/vocab/out/wordlist_rev.json", CONFIG.general.libdir)).unwrap()).unwrap();
        //let action_names: HashMap<i8, String> = serde_json::from_str(&fs::read_to_string(&format!("{}/vocab/out/actions/names.json", CONFIG.general.libdir)).unwrap()).unwrap();

        // Lookup words  
        loop {
            let input = cli_get_input("Query Word: ", "").trim().to_string();
            if input == "q".to_string() || input == "quit".to_string() {
                cli_send!("Ok, quitting.\n\n");
                break;
            }

            // Get token
            let token = Token::new(&input, &sophia.vocab);
            if token.index == 0 {
                cli_send!("Word does not exist, {}\n\n", input);
                continue;
            }


            // Display word
            let mut map: IndexMap<String, String> = IndexMap::new();
            map.insert("Word: ".to_string(), input.to_string());
            map.insert("Token ID: ".to_string(), token.index.to_string());
            map.insert("POS Tags: ".to_string(), token.potential_pos.join(", ").to_string());

            let mut x = 1;
            for cat_id in token.categories.iter() {
println!("Category: {}", cat_id);
                let category = sophia.vocab.categories.nodes.get(&cat_id.clone()).unwrap();
                let cat_label = format!("Category {}: ", x);
                map.insert(cat_label.clone(), format!("{:?}", category)); 
                x += 1;
            }

            for cat_id in token.ner.iter() {

            x = 1;
                let category = sophia.vocab.categories.nodes.get(&cat_id.clone()).unwrap();
                let cat_label = format!("NER {}: ", x);
                map.insert(cat_label.clone(), format!("{:?}", category)); 
                x += 1;
            }


            // Display basic word info
            cli_display_array(&map);
        }

    }

}

impl CliCommand for SysDevAdminVocab {

    fn process(&self, args: Vec<String>, flags: Vec<String>, value_flags: HashMap<String, String>) {

        // Lookup
        if args.len() == 0 {
            self.lookup();

        // Compile
        } else if args[0] == "compile".to_string() {
            self.compile();

        // Create frequence
        } else if args[0] == "text2freq".to_string() {
            self.text2freq(&args);

        } else if args[0] == "word2category".to_string() {
            self.word2category();

        // Word count
        } else if (args[0] == "word_count".to_string()) {
            self.word_count(&args);

        } else if args[0] == "text2pos_freq".to_string() {
            self.text2pos_freq(&args);

        } else if args[0] == "text2noun_pairings".to_string() {
            self.text2noun_pairings(&args);

        } else if args[0] == "blacklist".to_string() {
            self.blacklist();
        }

    }

    fn help(&self) -> CliHelpScreen {

        let mut help = CliHelpScreen::new(
            "Compile Vocabulary", 
            "cicero compile-vocab",
            "Compiles the vocabulary file from plain text vocab lists -- for maintainers of Cicero only."
        );

        help.add_example("cicero compile-vocab");
        help
    }

}



