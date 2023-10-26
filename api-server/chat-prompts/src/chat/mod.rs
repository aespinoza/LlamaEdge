pub mod belle;
pub mod llama;
pub mod mistral;
pub mod vicuna;

use crate::error::Result;
use belle::*;
use endpoints::chat::ChatCompletionRequestMessage;
use llama::*;
use mistral::*;
use vicuna::*;

#[enum_dispatch::enum_dispatch]
pub trait BuildChatPrompt: Send {
    fn build(&self, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String>;
}

#[enum_dispatch::enum_dispatch(BuildChatPrompt)]
pub enum ChatPrompt {
    Llama2ChatPrompt,
    MistralInstructPrompt,
    CodeLlamaInstructPrompt,
    BelleLlama2ChatPrompt,
    VicunaChatPrompt,
}
