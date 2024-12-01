use std::collections::HashSet;

use crate::{AllowedRoleMetadata, SupportedRequestModes, UnresolvedAllowedRoleMetadata};
use anyhow::{Context, Result};

use baml_types::{GetEnvVar, StringOr, UnresolvedValue};
use indexmap::IndexMap;
use serde::Deserialize;

use super::helpers::{Error, PropertyHandler, UnresolvedUrl};

#[derive(Debug)]
enum UnresolvedServiceAccountDetails<Meta> {
    RawAuthorizationHeader(StringOr),
    MaybeFilePathOrContent(StringOr),
    Object(IndexMap<String, (Meta, UnresolvedValue<Meta>)>),
    Json(StringOr),
}

#[derive(Debug, Deserialize)]
pub struct ServiceAccount {
    pub token_uri: String,
    pub project_id: String,
    pub client_email: String,
    pub private_key: String,
}

pub enum ResolvedServiceAccountDetails {
    RawAuthorizationHeader(String),
    Json(ServiceAccount),
}

impl<Meta> UnresolvedServiceAccountDetails<Meta> {
    fn without_meta(&self) -> UnresolvedServiceAccountDetails<()> {
        match self {
            UnresolvedServiceAccountDetails::RawAuthorizationHeader(s) => {
                UnresolvedServiceAccountDetails::RawAuthorizationHeader(s.clone())
            }
            UnresolvedServiceAccountDetails::MaybeFilePathOrContent(s) => {
                UnresolvedServiceAccountDetails::MaybeFilePathOrContent(s.clone())
            }
            UnresolvedServiceAccountDetails::Object(s) => UnresolvedServiceAccountDetails::Object(
                s.iter()
                    .map(|(k, v)| (k.clone(), ((), v.1.without_meta())))
                    .collect(),
            ),
            UnresolvedServiceAccountDetails::Json(s) => {
                UnresolvedServiceAccountDetails::Json(s.clone())
            }
        }
    }

    fn required_env_vars(&self) -> HashSet<String> {
        match self {
            UnresolvedServiceAccountDetails::RawAuthorizationHeader(s) => s.required_env_vars(),
            UnresolvedServiceAccountDetails::MaybeFilePathOrContent(s) => s.required_env_vars(),
            UnresolvedServiceAccountDetails::Object(s) => s
                .values()
                .flat_map(|(_, v)| v.required_env_vars())
                .collect(),
            UnresolvedServiceAccountDetails::Json(s) => s.required_env_vars(),
        }
    }

    fn resolve(&self, ctx: &impl GetEnvVar) -> Result<ResolvedServiceAccountDetails> {
        match self {
            UnresolvedServiceAccountDetails::RawAuthorizationHeader(s) => Ok(
                ResolvedServiceAccountDetails::RawAuthorizationHeader(s.resolve(ctx)?),
            ),
            UnresolvedServiceAccountDetails::MaybeFilePathOrContent(s) => {
                let value = s.resolve(ctx)?;
                match serde_json::from_str(&value) {
                    Ok(json) => Ok(ResolvedServiceAccountDetails::Json(json)),
                    Err(_) => {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            // Not a valid JSON, so we assume it's a file path
                            // Load the file and parse it as JSON
                            let file = std::fs::read_to_string(&value).context(format!(
                                "Failed to read service account file: {}",
                                value
                            ))?;
                            let json = serde_json::from_str(&file)
                                .context("Failed to parse service account file as JSON")?;
                            Ok(ResolvedServiceAccountDetails::Json(json))
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            anyhow::bail!(
                                format!("Reading from files not supported in BAML playground. For the playground, pass in the contents of your credentials file as a string to the same environment variable you used for 'credentials'.\nFile: {}", value)
                            );
                        }
                    }
                }
            }
            UnresolvedServiceAccountDetails::Object(s) => {
                let raw = s
                    .iter()
                    .map(|(k, v)| Ok((k, v.1.resolve_serde::<serde_json::Value>(ctx)?)))
                    .collect::<Result<IndexMap<_, _>>>()?;
                Ok(ResolvedServiceAccountDetails::Json(
                    serde_json::from_value(serde_json::json!(raw))
                        .context("Failed to parse service account JSON")?,
                ))
            }
            UnresolvedServiceAccountDetails::Json(s) => {
                let raw = s.resolve(ctx)?;
                Ok(ResolvedServiceAccountDetails::Json(
                    serde_json::from_str(&raw).context("Failed to parse service account JSON")?,
                ))
            }
        }
    }
}

#[derive(Debug)]
pub struct UnresolvedVertex<Meta> {
    // Either base_url or location
    base_url: either::Either<UnresolvedUrl, StringOr>,
    project_id: Option<StringOr>,
    authorization: UnresolvedServiceAccountDetails<Meta>,
    model: StringOr,
    headers: IndexMap<String, StringOr>,
    allowed_roles: Vec<StringOr>,
    default_role: Option<StringOr>,
    allowed_role_metadata: UnresolvedAllowedRoleMetadata,
    supported_request_modes: SupportedRequestModes,
    properties: IndexMap<String, (Meta, UnresolvedValue<Meta>)>,
}

pub struct ResolvedVertex {
    pub base_url: String,
    pub authorization: ResolvedServiceAccountDetails,
    pub model: String,
    pub headers: IndexMap<String, String>,
    pub allowed_roles: Vec<String>,
    pub default_role: String,
    pub allowed_metadata: AllowedRoleMetadata,
    pub supported_request_modes: SupportedRequestModes,
    pub properties: IndexMap<String, serde_json::Value>,
    pub proxy_url: Option<String>,
}

impl<Meta: Clone> UnresolvedVertex<Meta> {
    pub fn required_env_vars(&self) -> HashSet<String> {
        let mut env_vars = HashSet::new();
        match self.base_url {
            either::Either::Left(ref base_url) => env_vars.extend(base_url.required_env_vars()),
            either::Either::Right(ref location) => env_vars.extend(location.required_env_vars()),
        }
        if let Some(ref project_id) = self.project_id {
            env_vars.extend(project_id.required_env_vars());
        }
        env_vars.extend(self.authorization.required_env_vars());
        env_vars.extend(self.model.required_env_vars());
        env_vars.extend(self.headers.values().flat_map(|v| v.required_env_vars()));
        env_vars.extend(
            self.allowed_roles
                .iter()
                .flat_map(|r| r.required_env_vars()),
        );
        if let Some(r) = self.default_role.as_ref() {
            env_vars.extend(r.required_env_vars())
        }
        env_vars.extend(self.allowed_role_metadata.required_env_vars());
        env_vars.extend(self.supported_request_modes.required_env_vars());
        env_vars.extend(
            self.properties
                .values()
                .flat_map(|(_, v)| v.required_env_vars()),
        );

        env_vars
    }

    pub fn without_meta(&self) -> UnresolvedVertex<()> {
        UnresolvedVertex {
            base_url: self.base_url.clone(),
            project_id: self.project_id.clone(),
            authorization: self.authorization.without_meta(),
            model: self.model.clone(),
            headers: self.headers.clone(),
            allowed_roles: self.allowed_roles.clone(),
            default_role: self.default_role.clone(),
            allowed_role_metadata: self.allowed_role_metadata.clone(),
            supported_request_modes: self.supported_request_modes.clone(),
            properties: self
                .properties
                .iter()
                .map(|(k, (_, v))| (k.clone(), ((), v.without_meta())))
                .collect(),
        }
    }

    pub fn resolve(&self, ctx: &impl GetEnvVar) -> Result<ResolvedVertex> {
        // Validate auth options - only one should be provided
        let authorization = self.authorization.resolve(ctx)?;

        let base_url = match self.base_url.as_ref() {
            either::Either::Left(url) => url.resolve(ctx),
            either::Either::Right(location) => {
                let project_id = match self.project_id.as_ref() {
                    Some(project_id) => project_id.resolve(ctx)?,
                    None => match &authorization {
                        ResolvedServiceAccountDetails::Json(service_account) => {
                            service_account.project_id.clone()
                        }
                        ResolvedServiceAccountDetails::RawAuthorizationHeader(_) => {
                            return Err(anyhow::anyhow!(
                                "project_id is required when using location + authorization"
                            ))
                        }
                    },
                };

                let location = location.resolve(ctx)?;
                Ok(format!(
                    "https://{location}-aiplatform.googleapis.com/v1/projects/{project_id}/locations/{location}/publishers/google/models"
                ))
            }
        }?;

        let model = self.model.resolve(ctx)?;

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

        Ok(ResolvedVertex {
            base_url,
            authorization,
            model,
            headers,
            allowed_roles,
            default_role,
            allowed_metadata: self.allowed_role_metadata.resolve(ctx)?,
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
        let authorization = {
            let credentials = properties
                .ensure_any("credentials")
                .map(|(_, v)| v)
                .and_then(|v| match v {
                    UnresolvedValue::String(s, ..) => {
                        Some(UnresolvedServiceAccountDetails::MaybeFilePathOrContent(s))
                    }
                    UnresolvedValue::Map(m, ..) => Some(UnresolvedServiceAccountDetails::Object(m)),
                    other => {
                        properties.push_error(
                            format!(
                                "credentials must be a string or an object. Got: {}",
                                other.r#type()
                            ),
                            other.meta().clone(),
                        );
                        None
                    }
                });

            let credentials_content = properties
                .ensure_string("credentials_content", false)
                .map(|(_, v, _)| UnresolvedServiceAccountDetails::Json(v));

            let authz = properties
                .ensure_string("authorization", false)
                .map(|(_, v, _)| UnresolvedServiceAccountDetails::RawAuthorizationHeader(v));

            match (authz, credentials, credentials_content) {
                (Some(authz), _, _) => Some(authz),
                (None, Some(credentials), Some(credentials_content)) => {
                    if cfg!(target_arch = "wasm32") {
                        Some(credentials_content)
                    } else {
                        Some(credentials)
                    }
                }
                (None, Some(credentials), None) => Some(credentials),
                (None, None, Some(credentials_content)) => Some(credentials_content),
                (None, None, None) => {
                    if cfg!(target_arch = "wasm32") {
                        Some(UnresolvedServiceAccountDetails::Json(StringOr::EnvVar(
                            "GOOGLE_APPLICATION_CREDENTIALS_CONTENT".to_string(),
                        )))
                    } else {
                        Some(UnresolvedServiceAccountDetails::MaybeFilePathOrContent(
                            StringOr::EnvVar("GOOGLE_APPLICATION_CREDENTIALS".to_string()),
                        ))
                    }
                }
            }
        };
        let model = properties.ensure_string("model", true).map(|(_, v, _)| v);

        let base_url = {
            let base_url = properties.ensure_base_url(false);
            let location = properties
                .ensure_string("location", false)
                .map(|(key_span, v, _)| (key_span, v.clone()));

            match (base_url, location) {
                (Some(url), None) => Some(either::Either::Left(url.1)),
                (None, Some(name)) => Some(either::Either::Right(name.1)),
                (Some((key_1_span, ..)), Some((key_2_span, _))) => {
                    for key in [key_1_span, key_2_span] {
                        properties
                            .push_error("Only one of base_url or location must be provided", key);
                    }
                    None
                }
                (None, None) => {
                    // Its possible this will come in from credentials later
                    properties.push_option_error("Missing either base_url or location");
                    None
                }
            }
        };

        let project_id = properties
            .ensure_string("project_id", false)
            .map(|(_, v, _)| v);

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

        let model = model.expect("model is required");
        let base_url = base_url.expect("base_url is required");
        let authorization = authorization.expect("authorization is required");

        Ok(Self {
            base_url,
            project_id,
            authorization,
            model,
            headers,
            allowed_roles,
            default_role,
            allowed_role_metadata: allowed_metadata,
            supported_request_modes,
            properties,
        })
    }
}
