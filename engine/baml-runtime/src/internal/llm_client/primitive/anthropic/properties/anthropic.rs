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
    let shared = properties.pull_shared_properties("system");
    
    // Override defaults in shared
    let shared = SharedProperties {
        base_url: shared.base_url
            .map(|url| url.unwrap_or_else(|| "https://api.anthropic.com".into())),
        api_key: shared.api_key
            .map(|key| key.or_else(|| ctx.env.get("ANTHROPIC_API_KEY").map(|s| s.to_string()))),
        headers: shared.headers.map(|mut h| {
            h.entry("anthropic-version".to_string())
                .or_insert("2023-06-01".to_string());
            h
        }),
        ..shared
    };

    let mut properties = properties.finalize();
    properties.entry("max_tokens".into())
        .or_insert_with(|| 4096.into());

    Ok(PostRequestProperties {
        shared,
        proxy_url: ctx.env.get("BOUNDARY_PROXY_URL").map(|s| s.to_string()),
    })
} 