use std::collections::HashSet;

use crate::{AllowedRoleMetadata, FinishReasonFilter, RolesSelection, SupportedRequestModes, UnresolvedAllowedRoleMetadata, UnresolvedFinishReasonFilter, UnresolvedRolesSelection};
use anyhow::Result;

use baml_types::{EvaluationContext, StringOr};

use super::helpers::{Error, PropertyHandler};

#[derive(Debug, Clone)]
pub struct UnresolvedAwsBedrock {
    model: Option<StringOr>,
    region: StringOr,
    access_key_id: StringOr,
    secret_access_key: StringOr,
    role_selection: UnresolvedRolesSelection,
    allowed_role_metadata: UnresolvedAllowedRoleMetadata,
    supported_request_modes: SupportedRequestModes,
    inference_config: Option<UnresolvedInferenceConfiguration>,
    finish_reason_filter: UnresolvedFinishReasonFilter,
}

#[derive(Debug, Clone)]
struct UnresolvedInferenceConfiguration {
    max_tokens: Option<i32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    stop_sequences: Option<Vec<StringOr>>,
}

impl UnresolvedInferenceConfiguration {
    pub fn resolve(&self, ctx: &EvaluationContext<'_>) -> Result<InferenceConfiguration> {
        Ok(InferenceConfiguration {
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            stop_sequences: self
                .stop_sequences
                .as_ref()
                .map(|s| s.iter().map(|s| s.resolve(ctx)).collect::<Result<Vec<_>>>())
                .transpose()?,
        })
    }

    pub fn required_env_vars(&self) -> HashSet<String> {
        self.stop_sequences
            .as_ref()
            .map(|s| s.iter().flat_map(|s| s.required_env_vars()).collect())
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct InferenceConfiguration {
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub stop_sequences: Option<Vec<String>>,
}

pub struct ResolvedAwsBedrock {
    pub model: String,
    pub region: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub inference_config: Option<InferenceConfiguration>,
    role_selection: RolesSelection,
    pub allowed_role_metadata: AllowedRoleMetadata,
    pub supported_request_modes: SupportedRequestModes,
    pub finish_reason_filter: FinishReasonFilter,
}

impl ResolvedAwsBedrock {
    pub fn allowed_roles(&self) -> Vec<String> {
        self.role_selection.allowed_or_else(|| {
            vec!["system".to_string(), "user".to_string(), "assistant".to_string()]
        })
    }

    pub fn default_role(&self) -> String {
        self.role_selection
            .default_or_else(|| {
                let allowed_roles = self.allowed_roles();
                if allowed_roles.contains(&"user".to_string()) {
                    "user".to_string()
                } else {
                    allowed_roles.first().cloned().unwrap_or_else(|| "user".to_string())
                }
            })
    }
}

impl UnresolvedAwsBedrock {
    pub fn required_env_vars(&self) -> HashSet<String> {
        let mut env_vars = HashSet::new();
        if let Some(m) = self.model.as_ref() {
            env_vars.extend(m.required_env_vars())
        }
        env_vars.extend(self.region.required_env_vars());
        env_vars.extend(self.access_key_id.required_env_vars());
        env_vars.extend(self.secret_access_key.required_env_vars());
        env_vars.extend(self.role_selection.required_env_vars());
        env_vars.extend(self.allowed_role_metadata.required_env_vars());
        env_vars.extend(self.supported_request_modes.required_env_vars());
        if let Some(c) = self.inference_config.as_ref() {
            env_vars.extend(c.required_env_vars())
        }
        env_vars
    }

    pub fn resolve(&self, ctx: &EvaluationContext<'_>) -> Result<ResolvedAwsBedrock> {
        let Some(model) = self.model.as_ref() else {
            return Err(anyhow::anyhow!("model must be provided"));
        };

        let role_selection = self.role_selection.resolve(ctx)?;

        Ok(ResolvedAwsBedrock {
            model: model.resolve(ctx)?,
            region: self.region.resolve(ctx).ok(),
            access_key_id: self.access_key_id.resolve(ctx).ok(),
            secret_access_key: self.secret_access_key.resolve(ctx).ok(),
            role_selection,
            allowed_role_metadata: self.allowed_role_metadata.resolve(ctx)?,
            supported_request_modes: self.supported_request_modes.clone(),
            inference_config: self
                .inference_config
                .as_ref()
                .map(|c| c.resolve(ctx))
                .transpose()?,
            finish_reason_filter: self.finish_reason_filter.resolve(ctx)?,
        })
    }

    pub fn create_from<Meta: Clone>(
        mut properties: PropertyHandler<Meta>,
    ) -> Result<Self, Vec<Error<Meta>>> {
        let model = {
            // Add AWS Bedrock-specific validation logic here
            let model_id = properties.ensure_string("model_id", false);
            let model = properties.ensure_string("model", false);

            match (model_id, model) {
                (Some((model_id_key_meta, _, _)), Some((model_key_meta, _, _))) => {
                    properties.push_error(
                        "model_id and model cannot both be provided",
                        model_id_key_meta,
                    );
                    properties
                        .push_error("model_id and model cannot both be provided", model_key_meta);
                    None
                }
                (Some((_, model, _)), None) | (None, Some((_, model, _))) => Some(model),
                (None, None) => {
                    properties.push_option_error("model_id is required");
                    None
                }
            }
        };

        let region = properties
            .ensure_string("region", false)
            .map(|(_, v, _)| v.clone())
            .unwrap_or_else(|| baml_types::StringOr::EnvVar("AWS_REGION".to_string()));
        let access_key_id = properties
            .ensure_string("access_key_id", false)
            .map(|(_, v, _)| v.clone())
            .unwrap_or_else(|| baml_types::StringOr::EnvVar("AWS_ACCESS_KEY_ID".to_string()));
        let secret_access_key = properties
            .ensure_string("secret_access_key", false)
            .map(|(_, v, _)| v.clone())
            .unwrap_or_else(|| baml_types::StringOr::EnvVar("AWS_SECRET_ACCESS_KEY".to_string()));

        let role_selection = properties.ensure_roles_selection();
        let allowed_metadata = properties.ensure_allowed_metadata();
        let supported_request_modes = properties.ensure_supported_request_modes();

        let inference_config = {
            let mut inference_config = UnresolvedInferenceConfiguration {
                max_tokens: None,
                temperature: None,
                top_p: None,
                stop_sequences: None,
            };
            let raw = properties.ensure_map("inference_configuration", false);
            if let Some((_, map, _)) = raw {
                for (k, (key_span, v)) in map.into_iter() {
                    match k.as_str() {
                        "max_tokens" => inference_config.max_tokens = v.as_numeric().and_then(|val| match val.parse() {
                            Ok(v) => Some(v),
                            Err(e) => {
                                properties.push_error(format!("max_tokens must be a number: {e}"), v.meta().clone());
                                None
                            }
                        }),
                        "temperature" => inference_config.temperature = v.as_numeric().and_then(|val| match val.parse() {
                            Ok(v) => Some(v),
                            Err(e) => {
                                properties.push_error(format!("temperature must be a number: {e}"), v.meta().clone());
                                None
                            }
                        }),
                        "top_p" => inference_config.top_p = v.as_numeric().and_then(|val| match val.parse() {
                            Ok(v) => Some(v),
                            Err(e) => {
                                properties.push_error(format!("top_p must be a number: {e}"), v.meta().clone());
                                None
                            }
                        }),
                        "stop_sequences" => inference_config.stop_sequences = match v.into_array() {
                            Ok((stop_sequences, _)) => Some(stop_sequences.into_iter().filter_map(|s| match s.into_str() {
                                    Ok((s, _)) => Some(s),
                                    Err(e) => {
                                        properties.push_error(format!("stop_sequences values must be a string: got {}", e.r#type()), e.meta().clone());
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()),
                            Err(e) => {
                                properties.push_error(
                                    format!("stop_sequences must be an array: {}", e.r#type()),
                                    e.meta().clone(),
                                );
                                None
                            }
                        },
                        _ => {
                            properties.push_error(format!("unknown inference_config key: {k}"), key_span.clone());
                        },
                    }
                }
            }
            Some(inference_config)
        };
        let finish_reason_filter = properties.ensure_finish_reason_filter();

        // TODO: Handle inference_configuration
        let errors = properties.finalize_empty();
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            model,
            region,
            access_key_id,
            secret_access_key,
            role_selection,
            allowed_role_metadata: allowed_metadata,
            supported_request_modes,
            inference_config,
            finish_reason_filter,
        })
    }
}
