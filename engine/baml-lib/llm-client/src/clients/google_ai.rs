use std::collections::HashSet;

use anyhow::Result;
use crate::{
    AllowedRoleMetadata, SupportedRequestModes, UnresolvedAllowedRoleMetadata,
};

use baml_types::{EvaluationContext, StringOr, UnresolvedValue};
use indexmap::IndexMap;

use super::helpers::{Error, PropertyHandler, UnresolvedUrl};

#[derive(Debug)]
pub struct UnresolvedGoogleAI<Meta> {
    api_key: StringOr,
    base_url: UnresolvedUrl,
    headers: IndexMap<String, StringOr>,
    allowed_roles: Vec<StringOr>,
    default_role: Option<StringOr>,
    model: Option<StringOr>,
    allowed_metadata: UnresolvedAllowedRoleMetadata,
    supported_request_modes: SupportedRequestModes,
    properties: IndexMap<String, (Meta, UnresolvedValue<Meta>)>,
}


impl<Meta> UnresolvedGoogleAI<Meta> {
    pub fn without_meta(&self) -> UnresolvedGoogleAI<()> {
        UnresolvedGoogleAI {
            allowed_roles: self.allowed_roles.clone(),
            default_role: self.default_role.clone(),
            api_key: self.api_key.clone(),
            model: self.model.clone(),
            base_url: self.base_url.clone(),
            headers: self.headers.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            allowed_metadata: self.allowed_metadata.clone(),
            supported_request_modes: self.supported_request_modes.clone(),
            properties: self.properties.iter().map(|(k, (_, v))| (k.clone(), ((), v.without_meta()))).collect::<IndexMap<_, _>>(),
        }
    }
}

pub struct ResolvedGoogleAI {
    pub allowed_roles: Vec<String>,
    pub default_role: String,
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub headers: IndexMap<String, String>,
    pub allowed_metadata: AllowedRoleMetadata,
    pub supported_request_modes: SupportedRequestModes,
    pub properties: IndexMap<String, serde_json::Value>,
    pub proxy_url: Option<String>,
}

impl<Meta: Clone> UnresolvedGoogleAI<Meta> {
    pub fn required_env_vars(&self) -> HashSet<String> {
        let mut env_vars = HashSet::new();
        env_vars.extend(self.api_key.required_env_vars());
        env_vars.extend(self.base_url.required_env_vars());
        env_vars.extend(self.headers.values().map(|v| v.required_env_vars()).flatten());
        env_vars.extend(self.allowed_roles.iter().map(|r| r.required_env_vars()).flatten());
        self.default_role.as_ref().map(|r| env_vars.extend(r.required_env_vars()));
        self.model.as_ref().map(|m| env_vars.extend(m.required_env_vars()));
        env_vars.extend(self.allowed_metadata.required_env_vars());
        env_vars.extend(self.supported_request_modes.required_env_vars());
        env_vars.extend(self.properties.values().map(|(_, v)| v.required_env_vars()).flatten());
        env_vars
    }

    pub fn resolve(&self, ctx: &EvaluationContext<'_>) -> Result<ResolvedGoogleAI> {
        let api_key = self.api_key.resolve(ctx)?;
        let Some(default_role) = self.default_role.as_ref() else {
            return Err(anyhow::anyhow!("default_role must be provided"));
        };
        let default_role = default_role.resolve(ctx)?;

        let allowed_roles = self.allowed_roles.iter().map(|r| r.resolve(ctx)).collect::<Result<Vec<_>>>()?;
        if !allowed_roles.contains(&default_role) {
            return Err(anyhow::anyhow!(
                "default_role must be in allowed_roles: {} not in {:?}",
                default_role,
                allowed_roles
            ));
        }


        let model = self
            .model
            .as_ref()
            .map(|m| m.resolve(ctx))
            .transpose()?
            .unwrap_or_else(|| "gemini-1.5-flash".to_string());

        let base_url = self.base_url.resolve(ctx)?;

        let headers = self
            .headers
            .iter()
            .map(|(k, v)| Ok((k.clone(), v.resolve(ctx)?)))
            .collect::<Result<IndexMap<_, _>>>()?;

        Ok(ResolvedGoogleAI {
            default_role,
            api_key,
            model,
            base_url,
            headers,
            allowed_roles,
            allowed_metadata: self.allowed_metadata.resolve(ctx)?,
            supported_request_modes: self.supported_request_modes.clone(),
            properties: self
                .properties
                .iter()
                .map(|(k, (_, v))| Ok((k.clone(), v.resolve_serde::<serde_json::Value>(ctx)?)))
                .collect::<Result<IndexMap<_, _>>>()?,
            proxy_url: super::helpers::get_proxy_url(ctx),
        })
    }

    pub fn create_from(mut properties: PropertyHandler<Meta>) -> Result<Self, Vec<Error<Meta>>> {
        let allowed_roles = properties.ensure_allowed_roles().unwrap_or(vec![
            StringOr::Value("system".to_string()),
            StringOr::Value("user".to_string()),
            StringOr::Value("assistant".to_string()),
        ]);
        let default_role = properties.ensure_default_role(allowed_roles.as_slice(), 1);

        let api_key = properties.ensure_api_key().map(|v| v.clone()).unwrap_or(StringOr::EnvVar("GOOGLE_API_KEY".to_string()));

        let model = properties.ensure_string("model", false).map(|(_, v, _)| v.clone());

        let base_url = properties.ensure_base_url_with_default(UnresolvedUrl::new_static("https://generativelanguage.googleapis.com/v1beta"));

        let allowed_metadata = properties.ensure_allowed_metadata();
        let supported_request_modes = properties.ensure_supported_request_modes();
        let headers = properties.ensure_headers().unwrap_or_default();

        let (properties, errors) = properties.finalize();
        
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            allowed_roles,
            default_role,
            api_key,
            model,
            base_url,
            headers,
            allowed_metadata,
            supported_request_modes,
            properties,
        })
    }
}
