
use cicero_sdk::chat::{ChatPipeline, ChatNode, ChatUserInput, ChatNodeCategory, RequestedItem};

pub struct IntroductionChat { }

impl IntroductionChat {

    pub fn new() -> Self {
        Self { }
    }

}

impl ChatPipeline for IntroductionChat {

    fn begin(&self) -> ChatNode {
        let mut node = ChatNode::new();
        node.instruct("You notice a newcomer who looks confused yet curious.  Greet them, introduce yourself as their assistant, and ask if they have a few minutes for an introductory chat so you can get to know them, understand their expectations and how you can help them.");
        node
    }

    fn required_info(&self) -> Vec<RequestedItem> {
        Vec::new()
    }

    fn handle(&self, input: &ChatUserInput, prev_node: ChatNode) -> Option<ChatNode> {
        None
    }

}


