
use crate::server::apollo::user::ServerUser;
use std::sync::{Arc, Mutex};
use crate::llm::text::{PreProcessor, PreProcessorConfig};
//use crate::llm::nlp;
use cicero_sdk::chat::user_input::{ChatUserInput, Phrase, Word};
use crate::Error;
use log::error;

pub struct UserReply {
    user: Arc<Mutex<ServerUser>>,
    input: String
}

impl UserReply {

    pub fn new(user: Option<Arc<Mutex<ServerUser>>>, input: &str) -> Self {
        let processor = PreProcessor::new(Default::default());

        Self {
            user: user.unwrap(),
            input: processor.process(&input)[0].clone()
        }
    }

    /// Process the user reply
    pub fn process(&self) {

        // Get user
        let mut user = self.user.lock().unwrap();

        // Add to faiss index
        self.add_faiss(&mut user);

        // Chunk phrases
        self.chunk_phrases();

        // Check for conversation kit
        if user.conv().kit.is_some() {
            let kit = user.conv().kit.as_ref().unwrap();
        }


    }

    // Add to faiss index
    fn add_faiss(&self, user: &mut ServerUser) {

        // Pre-process and clean input
        let processor = PreProcessor::new(Default::default());
        let chunks = processor.split_chunks_markdown(&self.input.as_str());

        // Generate sentence embeddings
        let embeddings = match nlp::sentence_embeddings(&chunks) {
            Ok(r) => r,
            Err(e) => {
                error!("Unable to generate sentence embeddings of user reply for addition to faiss index, error: {}", e.to_string());
                return;
            }
        };

        // Add input to faiss index
        let mut x = 0;
        for input_str in chunks.iter() {
            user.faiss.add(embeddings[x].as_slice(), &input_str.as_str());
            x += 1;
        }

    }

    /// Chunk user input into phrases
    fn chunk_phrases(&self) -> Result<Vec<Phrase>, Error> {

        // POST tagging
        let pos_output = nlp::pos_tagging(&vec![self.input.to_string()])?;
        let pos_tags = &pos_output[0];

        // Initialize
        let mut words: Vec<Word> = Vec::new();
        let mut cur = Phrase::default();
        let mut verb_index = pos_tags.len() + 10;

        // Go through words
        let mut pos = 0;
        for tag in pos_tags {
            words.push(Word::new(&tag.word.as_str(), &tag.label.as_str()));

            // Skip if not action verb
            if !vec!["VB", "VV", "VBG"].contains(&tag.label.as_str()) {
                pos += 1;
                continue;
            }

            // Check for first verb
            if verb_index > pos_tags.len() + 1 {
                verb_index = pos;
                continue;
            }

            // Determine where to cut off phrase
            let mut stop_index = verb_index;
            for i in verb_index..pos {
                if vec!["WDT", "WRB", "RB", "ST", "RBS", "POS", "PDT"].contains(&pos_tags[i].label.as_str()) {
                    break;
                }
                stop_index += 1;
            }

        }

        Ok(Vec::new())
    }

}









