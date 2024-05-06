use anyhow::Result;
use internal_baml_jinja::{ChatOptions, RenderedChatMessage};

use crate::{internal::llm_client::LLMResponse, RuntimeContext};

#[cfg(feature = "wasm")]
type ResponseType = Result<LLMResponse, wasm_bindgen::JsValue>;
#[cfg(not(feature = "wasm"))]
type ResponseType = Result<LLMResponse>;

pub trait WithChat: Sync + Send {
    fn chat_options(&self, ctx: &RuntimeContext) -> Result<ChatOptions>;

    async fn chat(&self, ctx: &RuntimeContext, prompt: &Vec<RenderedChatMessage>) -> ResponseType;
}

// pub trait WithChatStream: WithChat {
//     fn stream_chat(
//         &mut self,
//         ctx: &RuntimeContext,
//         prompt: &Vec<RenderedChatMessage>,
//     ) -> impl Stream<Item = Result<LLMStreamResponse>> {
//         stream! {
//             let response = self.chat(ctx, prompt).await?;
//             yield Ok(LLMStreamResponse {
//                 delta: response.content(),
//                 start_time_unix_ms: response.start_time_unix_ms,
//                 latency_ms: response.latency_ms,
//                 metadata: response.metadata,
//             });
//         }
//     }
// }

pub trait WithNoChat {}

impl<T> WithChat for T
where
    T: WithNoChat + Send + Sync,
{
    fn chat_options(&self, _ctx: &RuntimeContext) -> Result<ChatOptions> {
        anyhow::bail!("Chat prompts are not supported by this provider")
    }

    #[cfg(feature = "wasm")]
    async fn chat(&self, _: &RuntimeContext, _: &Vec<RenderedChatMessage>) -> ResponseType {
        Err(wasm_bindgen::JsValue::from_str(
            "Chat prompts are not supported by this provider",
        ))
    }

    #[cfg(not(feature = "wasm"))]
    async fn chat(&self, _: &RuntimeContext, _: &Vec<RenderedChatMessage>) -> ResponseType {
        anyhow::bail!("Chat prompts are not supported by this provider")
    }
}
