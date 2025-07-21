
use std::collections::HashMap;


pub struct ChatPipelineUserInputPhrase {
    phrase_type: TriggerType,
    phrase: String,
    verbs: Vec<Word>,
    nouns: vec<Word>,
    sentiment_analysis: Sentiment
}
pub struct ChatPipelineUserInputRequest {
    request_type: RequestType,

pub struct Word {
    word: String,
    is_negative: bool.
    is_plural: Option<bool>,
    tense: Option<Tense>
}

pub enum Tone {
    Neutral,
    Relaxed,
    Happy,
    Excited,
    Sad,
    Lonely,
    Angry,
    Frustrated,
    Disappointed,
    Desparate,
    LaidBak,
    Curious
}
pub enum Urgency,
    Neutral,
    Low,
    Medium
    High,
    Urgent
}

pub struct ChatPipelineTrigger {
    trigger_type: TriggerType,
    slug: String     // Used by developer to identify trigger
    verb: Option<Word>,
    noun: option<Word>,
    entity: Option<Word>,
    plural: Option<bool>,
    tense: Option<Tense>
}

pub enum Tense {
    Past,
    Present,
    FutureNearTerm,
    FutureLongTerm
}

pub enum TriggerType {
    Question,
    Answer,
    Statement,
    Clarification
}







