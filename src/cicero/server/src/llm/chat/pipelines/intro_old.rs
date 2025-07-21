

use cicero_sdk::chat::{ChatNodeCategory, ChatUserInput};
use cicero_sdk::chat::{ChatFacilitator, ChatPipelineNode};

pub struct IntroductionChat { }

impl IntroductionChat {

    pub fn new() -> Self {
        Self { }
    }
}

impl ChatFacilitator for IntroductionChat {

    /// Begin the conversation
    pub fn begin(&self) -> Option<ChatPipelineNode> {
        let mut node = ChatPipelineNode::new(ChatNodeCategory::Confirm);
        node.instruct("You notice a newcomer who looks confused yet curious.  Greet them, introduce yourself as their assistant, and ask if they have a few minutes for an introductory chat so you can get to know them, understand their expectations and how you can help them.");
        node
    }

    /// Handle incoming input
    fn handle(&self, input: &ChatUserInput, prev_node: ChatPipelineNode) -> Option<ChatPipelineNode> {
        None
    }

}




