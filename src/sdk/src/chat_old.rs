
pub struct ChatPipelineUserInputRequest {
    request_type: RequestType,

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

pub enum TriggerType {
    Question,
    Answer,
    Statement,
    Clarification
}


