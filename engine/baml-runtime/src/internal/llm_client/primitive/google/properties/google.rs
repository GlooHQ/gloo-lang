use std::collections::HashMap;
use anyhow::Result;
use crate::{
    internal::llm_client::{properties_hander::PropertiesHandler, SharedProperties},
    RuntimeContext,
};
use super::PostRequestProperties;

pub fn resolve_properties(
    mut properties: PropertiesHandler,
    ctx: &RuntimeContext,
) -> Result<PostRequestProperties> {
    let shared = properties.pull_shared_properties("user");
    
    // Override defaults in shared
    let shared = SharedProperties {
        base_url: shared.base_url
            .map(|url| url.unwrap_or_else(|| "https://generativelanguage.googleapis.com/v1beta".to_string())),
        api_key: shared.api_key
            .map(|key| key.or_else(|| ctx.env.get("GOOGLE_API_KEY").map(|s| s.to_string()))),
        ..shared
    };

    let model_id = properties.remove_str("model")?
        .unwrap_or_else(|| "gemini-1.5-flash".to_string());

    Ok(PostRequestProperties {
        shared,
        proxy_url: ctx.env.get("BOUNDARY_PROXY_URL").map(|s| s.to_string()),
        model_id: Some(model_id),
    })
} 