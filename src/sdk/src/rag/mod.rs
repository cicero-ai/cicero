
Ok, I think this should do it.  Am I missing anything?

pub trait RAGModel {
    fn define_named_entites(&self, vocab_api: &Box<dyn CiceroVocabAPI>);    // Will call methods available to "vocab_api" to add entities into system's vocabulary, so NLU engine treams them as named entities.
    fn pre_process(&self, input: &str) -> HashMap<i32, String>;
    fn post_process(&self, records: &HashMap<i32, Vec<RAGRecord>) -> Vec<String>;
    fn filter_record(&self, record: &RagRecord) -> bool;
    fn score_record(&self, &RAGRecord) -> f64;
    fn get_template(&self(&self, user_input: &str, context: &Vec<String>) -> String;
    fn handle_feedback(&self, feedback: &RAGFeedback);
}

pub struct RAGRecord {
    id: i32,
    embeddings: Vec<f64>,
    orig_text: String
}

TokenizedOutput struct contains lots of information -- original user input, per-word tokens, tokens / units with MWEs included, phrases broken down by NLU engine into base verbs, nouns, related categories, named entities, etc.

Plus will ensure following functionality is available to developers via SDK:
    - Set temperature for LLM response
    - Set content window fo how many previous user messages to look at when searching FAISS index.

    - Set top k for nearest neighbors search
    - Generic text cleaning / pre-processing tools.
    - Generate embeddings
    - Look into offering functionality to fine-tune and train models internally (this will come in the future)



