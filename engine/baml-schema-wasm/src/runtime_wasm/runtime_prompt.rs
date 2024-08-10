use baml_runtime::{
    internal::llm_client::orchestrator::{ExecutionScope, OrchestrationScope},
    ChatMessagePart, RenderedPrompt,
};

use crate::runtime_wasm::ToJsValue;
use baml_types::{BamlMedia, BamlMediaContent, BamlMediaType, MediaBase64};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct WasmScope {
    scope: OrchestrationScope,
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmPrompt {
    prompt: RenderedPrompt,
    pub client_name: String,
}

impl From<OrchestrationScope> for WasmScope {
    fn from(scope: OrchestrationScope) -> Self {
        WasmScope { scope }
    }
}

impl From<(&RenderedPrompt, &OrchestrationScope)> for WasmPrompt {
    fn from((prompt, client_name): (&RenderedPrompt, &OrchestrationScope)) -> Self {
        WasmPrompt {
            prompt: prompt.clone(),
            client_name: client_name.name(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmChatMessage {
    #[wasm_bindgen(readonly)]
    pub role: String,
    #[wasm_bindgen(readonly)]
    pub parts: Vec<WasmChatMessagePart>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmChatMessagePart {
    part: ChatMessagePart,
}

impl From<ChatMessagePart> for WasmChatMessagePart {
    fn from(part: ChatMessagePart) -> Self {
        WasmChatMessagePart { part }
    }
}

#[wasm_bindgen]
impl WasmChatMessagePart {
    #[wasm_bindgen]
    pub fn is_text(&self) -> bool {
        matches!(self.part, ChatMessagePart::Text(_))
    }

    #[wasm_bindgen]
    pub fn is_image(&self) -> bool {
        if let ChatMessagePart::Media(m) = &self.part {
            m.media_type == BamlMediaType::Image
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn is_audio(&self) -> bool {
        if let ChatMessagePart::Media(m) = &self.part {
            m.media_type == BamlMediaType::Audio
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn as_text(&self) -> Option<String> {
        if let ChatMessagePart::Text(s) = &self.part {
            Some(s.clone())
        } else {
            None
        }
    }

    #[wasm_bindgen]
    // TODO: this needs to signal to TS how it should be rendered
    // currently we're only rendering file paths, but also need to support url & b64
    pub fn as_image(&self) -> Option<String> {
        let ChatMessagePart::Media(m) = &self.part else {
            return None;
        };
        Some(match &m.content {
            BamlMediaContent::Url(u) => u.url.clone(),
            BamlMediaContent::Base64(b) => b.base64.clone(),
            BamlMediaContent::File(f) => match f.baml_path.parent() {
                // TODO: this doesn't handle non-UTF8 paths
                Some(parent) => parent.join(&f.relpath).to_string_lossy().into_owned(),
                None => "<error: unresolveable path>".to_string(),
            },
        })
    }

    #[wasm_bindgen]
    pub fn as_audio(&self) -> Option<String> {
        let ChatMessagePart::Media(m) = &self.part else {
            return None;
        };
        Some(match &m.content {
            BamlMediaContent::Url(u) => u.url.clone(),
            BamlMediaContent::Base64(MediaBase64 { base64, mime_type }) => {
                format!("data:{};base64,{}", mime_type, base64.clone())
            }
            BamlMediaContent::File(f) => match f.baml_path.parent() {
                // TODO: this doesn't handle non-UTF8 paths
                Some(parent) => parent.join(&f.relpath).to_string_lossy().into_owned(),
                None => "<error: unresolveable path>".to_string(),
            },
        })
    }
}

#[wasm_bindgen]
impl WasmScope {
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.scope.name()
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_orchestration_scope_info(&self) -> JsValue {
        self.scope.to_js_value()
    }

    #[wasm_bindgen]
    pub fn iter_scopes(&self) -> ScopeIterator {
        ScopeIterator {
            scopes: self.scope.scope.clone(),
            index: 0,
        }
    }
}

#[wasm_bindgen]
pub struct ScopeIterator {
    scopes: Vec<ExecutionScope>,
    index: usize,
}

#[wasm_bindgen]
impl ScopeIterator {
    #[wasm_bindgen]
    pub fn next(&mut self) -> JsValue {
        if self.index < self.scopes.len() {
            let scope = &self.scopes[self.index];
            self.index += 1;
            match to_value(scope) {
                Ok(value) => value,
                Err(_) => JsValue::NULL,
            }
        } else {
            JsValue::NULL
        }
    }
}

#[wasm_bindgen]
impl WasmPrompt {
    #[wasm_bindgen]
    pub fn is_chat(&self) -> bool {
        matches!(self.prompt, RenderedPrompt::Chat(_))
    }

    #[wasm_bindgen]
    pub fn is_completion(&self) -> bool {
        matches!(self.prompt, RenderedPrompt::Completion(_))
    }

    #[wasm_bindgen]
    pub fn as_chat(&self) -> Option<Vec<WasmChatMessage>> {
        if let RenderedPrompt::Chat(s) = &self.prompt {
            Some(
                s.iter()
                    .map(|m| WasmChatMessage {
                        role: m.role.clone(),
                        parts: m.parts.iter().map(|p| p.clone().into()).collect(),
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}
