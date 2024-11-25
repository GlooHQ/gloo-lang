use std::collections::HashSet;

use crate::{AllowedRoleMetadata, SupportedRequestModes, UnresolvedAllowedRoleMetadata};
use anyhow::Result;

use baml_types::{GetEnvVar, StringOr, UnresolvedValue};
use indexmap::IndexMap;

use super::helpers::{Error, PropertyHandler, UnresolvedUrl};

#[derive(Debug)]
pub struct UnresolvedOpenAI<Meta> {
    base_url: Option<either::Either<UnresolvedUrl, (StringOr, StringOr)>>,
    api_key: Option<StringOr>,
    allowed_roles: Vec<StringOr>,
    default_role: Option<StringOr>,
    allowed_role_metadata: UnresolvedAllowedRoleMetadata,
    supported_request_modes: SupportedRequestModes,
    headers: IndexMap<String, StringOr>,
    properties: IndexMap<String, (Meta, UnresolvedValue<Meta>)>,
    query_params: IndexMap<String, StringOr>,
}

impl<Meta> UnresolvedOpenAI<Meta> {
    pub fn without_meta(&self) -> UnresolvedOpenAI<()> {
        UnresolvedOpenAI {
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
            allowed_roles: self.allowed_roles.clone(),
            default_role: self.default_role.clone(),
            allowed_role_metadata: self.allowed_role_metadata.clone(),
            supported_request_modes: self.supported_request_modes.clone(),
            headers: self
                .headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            properties: self
                .properties
                .iter()
                .map(|(k, (_, v))| (k.clone(), ((), v.without_meta())))
                .collect::<IndexMap<_, _>>(),
            query_params: self
                .query_params
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }
}

pub struct ResolvedOpenAI {
    pub base_url: String,
    pub api_key: Option<String>,
    pub allowed_roles: Vec<String>,
    pub default_role: String,
    pub allowed_metadata: AllowedRoleMetadata,
    pub supported_request_modes: SupportedRequestModes,
    pub headers: IndexMap<String, String>,
    pub properties: IndexMap<String, serde_json::Value>,
    pub query_params: IndexMap<String, String>,
    pub proxy_url: Option<String>,
}

impl<Meta: Clone> UnresolvedOpenAI<Meta> {
    pub fn required_env_vars(&self) -> HashSet<String> {
        let mut env_vars = HashSet::new();

        self.base_url.as_ref().map(|url| match url {
            either::Either::Left(url) => {
                env_vars.extend(url.required_env_vars());
            }
            either::Either::Right((resource_name, deployment_id)) => {
                env_vars.extend(resource_name.required_env_vars());
                env_vars.extend(deployment_id.required_env_vars());
            }
        });
        self.api_key
            .as_ref()
            .map(|key| env_vars.extend(key.required_env_vars()));
        self.allowed_roles
            .iter()
            .for_each(|role| env_vars.extend(role.required_env_vars()));
        self.default_role
            .as_ref()
            .map(|role| env_vars.extend(role.required_env_vars()));
        env_vars.extend(self.allowed_role_metadata.required_env_vars());
        env_vars.extend(self.supported_request_modes.required_env_vars());
        self.headers
            .iter()
            .for_each(|(_, v)| env_vars.extend(v.required_env_vars()));
        self.properties
            .iter()
            .for_each(|(_, (_, v))| env_vars.extend(v.required_env_vars()));
        self.query_params
            .iter()
            .for_each(|(_, v)| env_vars.extend(v.required_env_vars()));

        env_vars
    }

    pub fn resolve(&self, provider: &crate::ClientProvider, ctx: &impl GetEnvVar) -> Result<ResolvedOpenAI> {
        let base_url = self
            .base_url
            .as_ref()
            .map(|url| match url {
                either::Either::Left(url) => url.resolve(ctx),
                either::Either::Right((resource_name, deployment_id)) => {
                    let resource_name = resource_name.resolve(ctx)?;
                    let deployment_id = deployment_id.resolve(ctx)?;
                    Ok(format!(
                        "https://{}.openai.azure.com/openai/deployments/{}",
                        resource_name, deployment_id
                    ))
                }
            })
            .transpose()?;

        let Some(base_url) = base_url else {
            return Err(anyhow::anyhow!("base_url is required"));
        };

        let api_key = self
            .api_key
            .as_ref()
            .map(|key| key.resolve(ctx))
            .transpose()?;

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

        let headers = self
            .headers
            .iter()
            .map(|(k, v)| Ok((k.clone(), v.resolve(ctx)?)))
            .collect::<Result<IndexMap<_, _>>>()?;

        let properties = {
            let mut properties = self
                .properties
                .iter()
                .map(|(k, (_, v))| Ok((k.clone(), v.resolve_serde::<serde_json::Value>(ctx)?)))
                .collect::<Result<IndexMap<_, _>>>()?;
            
            // TODO(vbv): Only do this for azure
            if matches!(provider, crate::ClientProvider::OpenAI(crate::OpenAIClientProviderVariant::Azure)) {
                properties
                    .entry("max_tokens".into())
                    .or_insert(serde_json::json!(4096));
            }
            properties
        };

        let query_params = self
            .query_params
            .iter()
            .map(|(k, v)| Ok((k.clone(), v.resolve(ctx)?)))
            .collect::<Result<IndexMap<_, _>>>()?;

        Ok(ResolvedOpenAI {
            base_url,
            api_key,
            allowed_roles,
            default_role,
            allowed_metadata: self.allowed_role_metadata.resolve(ctx)?,
            supported_request_modes: self.supported_request_modes.clone(),
            headers,
            properties,
            query_params,
            proxy_url: super::helpers::get_proxy_url(ctx),
        })
    }

    pub fn create_standard(
        mut properties: PropertyHandler<Meta>,
    ) -> Result<Self, Vec<Error<Meta>>> {
        let base_url = properties.ensure_base_url_with_default(UnresolvedUrl::new_static("https://api.openai.com/v1"));

        let api_key = Some(
            properties
                .ensure_api_key()
                .map(|v| v.clone())
                .unwrap_or_else(|| StringOr::EnvVar("OPENAI_API_KEY".to_string())),
        );

        Self::create_common(properties, Some(either::Either::Left(base_url)), api_key)
    }

    pub fn create_azure(mut properties: PropertyHandler<Meta>) -> Result<Self, Vec<Error<Meta>>> {
        let base_url = {
            let base_url = properties.ensure_base_url(false);
            let resource_name = properties
                .ensure_string("resource_name", false)
                .map(|(key_span, v, _)| (key_span, v.clone()));
            let deployment_id = properties
                .ensure_string("deployment_id", false)
                .map(|(key_span, v, _)| (key_span, v.clone()));

            match (base_url, resource_name, deployment_id) {
                (Some(url), None, None) => Some(either::Either::Left(url.1)),
                (None, Some(name), Some(id)) => Some(either::Either::Right((name.1, id.1))),
                (_, None, Some((key_span, _))) => {
                    properties.push_error(
                        "resource_name must be provided when deployment_id is provided",
                        key_span,
                    );
                    None
                }
                (_, Some((key_span, _)), None) => {
                    properties.push_error(
                        "deployment_id must be provided when resource_name is provided",
                        key_span,
                    );
                    None
                }
                (Some((key_1_span, ..)), Some((key_2_span, _)), Some((key_3_span, _))) => {
                    for key in [key_1_span, key_2_span, key_3_span] {
                        properties.push_error(
                            "Only one of base_url or both (resource_name, deployment_id) must be provided",
                            key
                        );
                    }
                    None
                }
                (None, None, None) => {
                    properties.push_option_error(
                        "Missing either base_url or both (resource_name, deployment_id)",
                    );
                    None
                }
            }
        };

        let api_key = Some(
            properties
                .ensure_api_key()
                .map(|v| v.clone())
                .unwrap_or_else(|| StringOr::EnvVar("AZURE_OPENAI_API_KEY".to_string())),
        );

        let mut query_params = IndexMap::new();
        if let Some((_, v, _)) = properties.ensure_string("api_version", false) {
            query_params.insert("api-version".to_string(), v.clone());
        }

        let mut instance = Self::create_common(properties, base_url, api_key)?;
        instance.query_params = query_params;

        Ok(instance)
    }

    pub fn create_generic(mut properties: PropertyHandler<Meta>) -> Result<Self, Vec<Error<Meta>>> {
        let base_url = properties.ensure_base_url(true);

        let api_key = properties.ensure_api_key().map(|v| v.clone());

        Self::create_common(
            properties,
            base_url.map(|url| either::Either::Left(url.1)),
            api_key,
        )
    }

    pub fn create_ollama(mut properties: PropertyHandler<Meta>) -> Result<Self, Vec<Error<Meta>>> {
        let base_url = properties.ensure_base_url_with_default(UnresolvedUrl::new_static("http://localhost:11434/v1"));

        let api_key = properties.ensure_api_key().map(|v| v.clone());

        Self::create_common(properties, Some(either::Either::Left(base_url)), api_key)
    }

    fn create_common(
        mut properties: PropertyHandler<Meta>,
        base_url: Option<either::Either<UnresolvedUrl, (StringOr, StringOr)>>,
        api_key: Option<StringOr>,
    ) -> Result<Self, Vec<Error<Meta>>> {
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
            allowed_role_metadata: allowed_metadata,
            supported_request_modes,
            headers,
            properties,
            query_params: IndexMap::new(),
        })
    }
}
