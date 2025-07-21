
use serde_derive::{Serialize, Deserialize};
use crate::task::TaskName;
use super::{ChatNode, ChatUserInput, RequestedItem};

pub trait ChatPipeline {
    fn required_info(&self) -> Vec<RequestedItem>;
    fn begin(&self) -> ChatNode;
    fn handle(&self, input: &ChatUserInput, prev_node: ChatNode) -> Option<ChatNode>;
}

pub struct ChatKit {
    pub task_name: TaskName,
    pub pipeline: Box<dyn ChatPipeline>,
    pub collected_items: Vec<RequestedItem>,
    //pub   previous_interactions: Vec<PreviousInteractions>,    /// Will do later
    pub current_node: ChatNode
}

impl ChatKit {

    pub fn new(pipeline: Box<dyn ChatPipeline>) -> Self {
        let current_node = pipeline.begin().to_owned();
        Self {
            task_name: TaskName::none,
            pipeline,
            collected_items: Vec::new(),
            current_node
        }
    }

}




