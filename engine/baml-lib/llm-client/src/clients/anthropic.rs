use std::collections::HashSet;

use crate::{AllowedRoleMetadata, SupportedRequestModes, UnresolvedAllowedRoleMetadata};
use anyhow::Result;

use baml_types::{EvaluationContext, StringOr, UnresolvedValue};
use indexmap::IndexMap;

use super::helpers::{Error, PropertyHandler, UnresolvedUrl};

#[derive(Debug)]
pub struct UnresolvedAnthropic<Meta> {
    base_url: UnresolvedUrl,
    api_key: StringOr,
    allowed_roles: Vec<StringOr>,
    default_role: Option<StringOr>,
    allowed_metadata: UnresolvedAllowedRoleMetadata,
    supported_request_modes: SupportedRequestModes,
    headers: IndexMap<String, StringOr>,
    properties: IndexMap<String, (Meta, UnresolvedValue<Meta>)>,
}

impl<Meta> UnresolvedAnthropic<Meta> {
    pub fn without_meta(&self) -> UnresolvedAnthropic<()> {
        UnresolvedAnthropic {
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
            allowed_roles: self.allowed_roles.clone(),
            default_role: self.default_role.clone(),
            allowed_metadata: self.allowed_metadata.clone(),
            supported_request_modes: self.supported_request_modes.clone(),
            headers: self.headers.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            properties: self.properties.iter().map(|(k, (_, v))| (k.clone(), ((), v.without_meta()))).collect(),
        }
    }
}

pub struct ResolvedAnthropic {
    pub base_url: String,
    pub api_key: String,
    pub allowed_roles: Vec<String>,
    pub default_role: String,
    pub allowed_metadata: AllowedRoleMetadata,
    pub supported_request_modes: SupportedRequestModes,
    pub headers: IndexMap<String, String>,
    pub properties: IndexMap<String, serde_json::Value>,
    pub proxy_url: Option<String>,
}

impl<Meta: Clone> UnresolvedAnthropic<Meta> {
    pub fn required_env_vars(&self) -> HashSet<String> {
        let mut env_vars = HashSet::new();
        env_vars.extend(self.base_url.required_env_vars());
        env_vars.extend(self.api_key.required_env_vars());
        env_vars.extend(self.allowed_roles.iter().map(|r| r.required_env_vars()).flatten());
        self.default_role.as_ref().map(|r| env_vars.extend(r.required_env_vars()));
        env_vars.extend(self.allowed_metadata.required_env_vars());
        env_vars.extend(self.supported_request_modes.required_env_vars());
        env_vars.extend(self.headers.values().map(|v| v.required_env_vars()).flatten());
        env_vars.extend(self.properties.values().map(|(_, v)| v.required_env_vars()).flatten());

        env_vars
    }

    pub fn resolve(&self, ctx: &EvaluationContext<'_>) -> Result<ResolvedAnthropic> {
        let allowed_roles = self
            .allowed_roles
            .iter()
            .map(|role| role.resolve(ctx))
            .collect::<Result<Vec<_>>>()?;

        let Some(default_role) = self.default_role.as_ref() else {
            return Err(anyhow::anyhow!("default_role must be provided"));
        };
        let default_role = default_role.resolve(ctx)?;

        if !allowed_roles.contains(&default_role) {
            return Err(anyhow::anyhow!(
                "default_role must be in allowed_roles: {} not in {:?}",
                default_role,
                allowed_roles
            ));
        }

        let base_url = self.base_url.resolve(ctx)?;

        let mut headers = self
            .headers
            .iter()
            .map(|(k, v)| Ok((k.clone(), v.resolve(ctx)?)))
            .collect::<Result<IndexMap<_, _>>>()?;

        // Add default Anthropic version header if not present
        headers
            .entry("anthropic-version".to_string())
            .or_insert_with(|| "2023-06-01".to_string());

        let properties = {
            let mut properties = self
                .properties
                .iter()
                .map(|(k, (_, v))| Ok((k.clone(), v.resolve_serde::<serde_json::Value>(ctx)?)))
                .collect::<Result<IndexMap<_, _>>>()?;

            properties
                .entry("max_tokens".to_string())
                .or_insert(serde_json::json!(4096));

            properties
        };

        Ok(ResolvedAnthropic {
            base_url,
            api_key: self.api_key.resolve(ctx)?,
            allowed_roles,
            default_role,
            allowed_metadata: self.allowed_metadata.resolve(ctx)?,
            supported_request_modes: self.supported_request_modes.clone(),
            headers,
            properties,
            proxy_url: super::helpers::get_proxy_url(ctx),
        })
    }

    pub fn create_from(
        mut properties: PropertyHandler<Meta>,
    ) -> Result<Self, Vec<Error<Meta>>> {
        let base_url = properties.ensure_base_url_with_default(UnresolvedUrl::new_static("https://api.anthropic.com"));
        let api_key = properties
            .ensure_string("api_key", false)
            .map(|(_, v, _)| v.clone())
            .unwrap_or(StringOr::EnvVar("ANTHROPIC_API_KEY".to_string()));

        let allowed_roles = properties.ensure_allowed_roles().unwrap_or(vec![
            StringOr::Value("system".to_string()),
            StringOr::Value("user".to_string()),
            StringOr::Value("assistant".to_string()),
        ]);

        let default_role = properties.ensure_default_role(allowed_roles.as_slice(), 1);
        let allowed_metadata = properties.ensure_allowed_metadata();
        let supported_request_modes = properties.ensure_supported_request_modes();
        let headers = properties.ensure_headers().unwrap_or_default();

        let (properties, errors) = properties.finalize();
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            base_url,
            api_key,
            allowed_roles,
            default_role,
            allowed_metadata,
            supported_request_modes,
            headers,
            properties,
        })
    }
}
