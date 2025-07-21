
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatUserInput {
    //phrases: Vec<ChatPipelineUserInputPhrase>,
    //requests: Vec<ChatPipelineUserInputRequest>,
    //collected_info: Vec<RequestedItem>,
    //matched_triggers: Vec<ChatPipelineTrigger>,
    //sentiment_analysis: Sentiment
    named_entities: Vec<String>,
    //tone: Tone,
    //urgency: Urgency,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub words: Vec<Word>,
    pub action_verb: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    word: String,
    label: String,
    is_plural: Option<bool>,
    tense: Option<Tense>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tense {
    Past,
    Present,
    Future,
    FutureLongTerm
}

impl Word {

    pub fn new(word_str: &str, label: &str) -> Self {

        // Define word
        let mut word = Self {
            word: word_str.to_string(),
            label: label.to_string(),
            is_plural: None,
            tense: None
        };

        // Check label type
        if vec!["VBG", "VBP", "VBZ", "VVG", "VVP", "VVZ"].contains(&label) {
            word.tense = Some(Tense::Present);
        } else if label == "VBD" || label == "VBN" {
            word.tense = Some(Tense::Past);
        } else if label == "NN" || label == "NP" {
            word.is_plural = Some(false);
        } else if label == "NNS" || label == "NPS" { 
            word.is_plural = Some(true);
        }

        word
    }
}


impl Default for Phrase {
    fn default() -> Phrase {
        Phrase {
            words: Vec::new(),
            action_verb: String::new()
        }
    }
}


