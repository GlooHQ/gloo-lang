use anyhow::Result;
use std::collections::HashSet;

use baml_types::{GetEnvVar, StringOr};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub enum ClientSpec {
    Named(String),
    /// Shorthand for "<provider>/<model>"
    Shorthand(ClientProvider, String),
}

impl ClientSpec {
    pub fn as_str(&self) -> String {
        match self {
            ClientSpec::Named(n) => n.clone(),
            ClientSpec::Shorthand(provider, model) => format!("{provider}/{model}"),
        }
    }

    pub fn new_from_id(arg: &str) -> Result<Self, anyhow::Error> {
        if arg.contains("/") {
            let (provider, model) = arg.split_once("/").unwrap();
            Ok(ClientSpec::Shorthand(provider.parse()?, model.to_string()))
        } else {
            Ok(ClientSpec::Named(arg.into()))
        }
    }
}

/// The provider for the client, e.g. baml-openai-chat
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ClientProvider {
    /// The OpenAI client provider variant
    OpenAI(OpenAIClientProviderVariant),
    /// The Anthropic client provider variant
    Anthropic,
    /// The AWS Bedrock client provider variant
    AwsBedrock,
    /// The Google AI client provider variant
    GoogleAi,
    /// The Vertex client provider variant
    Vertex,
    /// The strategy client provider variant
    Strategy(StrategyClientProvider),
}

/// The OpenAI client provider variant
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OpenAIClientProviderVariant {
    /// The base OpenAI client provider variant
    Base,
    /// The Ollama client provider variant
    Ollama,
    /// The Azure client provider variant
    Azure,
    /// The generic client provider variant
    Generic,
}

/// The strategy client provider variant
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StrategyClientProvider {
    /// The round-robin strategy client provider variant
    RoundRobin,
    /// The fallback strategy client provider variant
    Fallback,
}

impl std::fmt::Display for ClientProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientProvider::OpenAI(variant) => write!(f, "{}", variant),
            ClientProvider::Anthropic => write!(f, "anthropic"),
            ClientProvider::AwsBedrock => write!(f, "aws-bedrock"),
            ClientProvider::GoogleAi => write!(f, "google-ai"),
            ClientProvider::Vertex => write!(f, "vertex-ai"),
            ClientProvider::Strategy(variant) => write!(f, "{}", variant),
        }
    }
}

impl std::fmt::Display for OpenAIClientProviderVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAIClientProviderVariant::Base => write!(f, "openai"),
            OpenAIClientProviderVariant::Ollama => write!(f, "ollama"),
            OpenAIClientProviderVariant::Azure => write!(f, "azure-openai"),
            OpenAIClientProviderVariant::Generic => write!(f, "openai-generic"),
        }
    }
}

impl std::fmt::Display for StrategyClientProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrategyClientProvider::RoundRobin => write!(f, "round-robin"),
            StrategyClientProvider::Fallback => write!(f, "fallback"),
        }
    }
}

impl std::str::FromStr for ClientProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "openai" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Base)),
            "baml-openai-chat" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Base)),
            "openai-generic" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Generic)),
            "azure-openai" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Azure)),
            "baml-azure-chat" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Azure)),
            "baml-ollama-chat" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Ollama)),
            "ollama" => Ok(ClientProvider::OpenAI(OpenAIClientProviderVariant::Ollama)),
            "anthropic" => Ok(ClientProvider::Anthropic),
            "baml-anthropic-chat" => Ok(ClientProvider::Anthropic),
            "aws-bedrock" => Ok(ClientProvider::AwsBedrock),
            "google-ai" => Ok(ClientProvider::GoogleAi),
            "vertex-ai" => Ok(ClientProvider::Vertex),
            "fallback" => Ok(ClientProvider::Strategy(StrategyClientProvider::Fallback)),
            "baml-fallback" => Ok(ClientProvider::Strategy(StrategyClientProvider::Fallback)),
            "round-robin" => Ok(ClientProvider::Strategy(StrategyClientProvider::RoundRobin)),
            "baml-round-robin" => Ok(ClientProvider::Strategy(StrategyClientProvider::RoundRobin)),
            _ => Err(anyhow::anyhow!("Invalid client provider: {}", s)),
        }
    }
}

impl std::str::FromStr for OpenAIClientProviderVariant {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "openai" => Ok(OpenAIClientProviderVariant::Base),
            "ollama" => Ok(OpenAIClientProviderVariant::Ollama),
            "azure-openai" => Ok(OpenAIClientProviderVariant::Azure),
            "openai-generic" => Ok(OpenAIClientProviderVariant::Generic),
            _ => Err(anyhow::anyhow!(
                "Invalid OpenAI client provider variant: {}",
                s
            )),
        }
    }
}

impl std::str::FromStr for StrategyClientProvider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "round-robin" => Ok(StrategyClientProvider::RoundRobin),
            "fallback" => Ok(StrategyClientProvider::Fallback),
            _ => Err(anyhow::anyhow!(
                "Invalid strategy client provider variant: {}",
                s
            )),
        }
    }
}

impl ClientProvider {
    pub fn allowed_providers() -> &'static [&'static str] {
        &[
            "openai",
            "openai-generic",
            "azure-openai",
            "anthropic",
            "ollama",
            "round-robin",
            "fallback",
            "google-ai",
            "vertex-ai",
            "aws-bedrock",
        ]
    }
}

impl std::fmt::Display for ClientSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientSpec::Named(n) => write!(f, "{}", n),
            ClientSpec::Shorthand(provider, model) => write!(f, "{}/{}", provider, model),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportedRequestModes {
    // If unset, treat as auto
    pub stream: Option<bool>,
}

impl SupportedRequestModes {
    pub fn required_env_vars(&self) -> HashSet<String> {
        HashSet::new()
    }
}

#[derive(Clone, Debug)]
pub enum UnresolvedAllowedRoleMetadata {
    Value(StringOr),
    All,
    None,
    Only(HashSet<StringOr>),
}

#[derive(Clone, Debug)]
pub enum AllowedRoleMetadata {
    All,
    None,
    Only(HashSet<String>),
}

impl UnresolvedAllowedRoleMetadata {
    pub fn required_env_vars(&self) -> HashSet<String> {
        match self {
            Self::Value(role) => role.required_env_vars(),
            Self::Only(roles) => roles
                .iter()
                .flat_map(|role| role.required_env_vars())
                .collect(),
            _ => HashSet::new(),
        }
    }

    pub fn resolve(&self, ctx: &impl GetEnvVar) -> Result<AllowedRoleMetadata> {
        match self {
            Self::Value(role) => {
                let role = role.resolve(ctx)?;
                match role.as_str() {
                    "all" => Ok(AllowedRoleMetadata::All),
                    "none" => Ok(AllowedRoleMetadata::None),
                    _ => Err(anyhow::anyhow!("Invalid allowed role metadata: {}. Allowed values are 'all' or 'none' or an array of roles.", role)),
                }
            }
            Self::All => Ok(AllowedRoleMetadata::All),
            Self::None => Ok(AllowedRoleMetadata::None),
            Self::Only(roles) => Ok(AllowedRoleMetadata::Only(
                roles
                    .iter()
                    .map(|role| role.resolve(ctx))
                    .collect::<Result<HashSet<_>>>()?,
            )),
        }
    }
}

impl AllowedRoleMetadata {
    pub fn is_allowed(&self, key: &str) -> bool {
        match self {
            Self::All => true,
            Self::None => false,
            Self::Only(allowed) => allowed.contains(&key.to_string()),
        }
    }
}
