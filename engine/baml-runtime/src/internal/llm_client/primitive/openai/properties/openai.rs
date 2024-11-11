use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::{
    internal::llm_client::{properties_hander::PropertiesHandler, AllowedMetadata},
    RuntimeContext,
};

use super::PostRequestProperties;

pub fn resolve_properties(
    mut properties: PropertiesHandler,
    ctx: &RuntimeContext,
) -> Result<PostRequestProperties> {
    let default_role = properties.pull_default_role("system")?;
    let base_url = properties
        .pull_base_url()?
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

    let api_key = properties
        .pull_api_key()?
        .or_else(|| ctx.env.get("OPENAI_API_KEY").map(|s| s.to_string()));

    let allowed_metadata = properties.pull_allowed_role_metadata()?;
    let headers = properties.pull_headers()?;

    let supported_request_modes = properties.pull_supported_request_modes()?;

    Ok(PostRequestProperties {
        default_role,
        base_url,
        api_key,
        headers,
        properties: properties.finalize(),
        allowed_metadata,
        proxy_url: ctx
            .env
            .get("BOUNDARY_PROXY_URL")
            .map(|s| Some(s.to_string()))
            .unwrap_or(None),
        query_params: Default::default(),
        supported_request_modes,
    })
}
