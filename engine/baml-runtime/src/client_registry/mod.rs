// This is designed to build any type of client, not just primitives
use anyhow::{Context, Result};
pub use internal_llm_client::ClientProvider;
use internal_llm_client::{ClientSpec, PropertyHandler, UnresolvedClientProperty};
use std::collections::HashMap;
use std::sync::Arc;

use baml_types::{BamlMap, BamlValue};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{internal::llm_client::llm_provider::LLMProvider, RuntimeContext};

#[derive(Clone)]
pub enum PrimitiveClient {
    OpenAI,
    Anthropic,
    Google,
    Vertex,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ClientProperty {
    pub name: String,
    pub provider: ClientProvider,
    pub retry_policy: Option<String>,
    options: BamlMap<String, BamlValue>,
}

impl ClientProperty {
    pub fn new(name: String, provider: ClientProvider, retry_policy: Option<String>, options: BamlMap<String, BamlValue>) -> Self {
        Self {
            name,
            provider,
            retry_policy,
            options,
        }
    }

    pub fn from_shorthand(provider: &ClientProvider, model: &str) -> Self {
        Self {
            name: format!("{}/{}", provider, model),
            provider: provider.clone(),
            retry_policy: None,
            options: vec![("model".to_string(), BamlValue::String(model.to_string()))]
                .into_iter()
                .collect(),
        }
    }

    pub fn unresolved_options(&self) -> Result<UnresolvedClientProperty<()>> {
        let property = PropertyHandler::new(
            self.options
                .iter()
                .map(|(k, v)| Ok((k.clone(), ((), v.to_resolvable()?))))
                .collect::<Result<_>>()?,
            (),
        );
        self.provider.parse_client_property(property).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse client options for {}:\n{}",
                self.name,
                e.into_iter()
                    .map(|e| e.message)
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct ClientRegistry {
    #[serde(deserialize_with = "deserialize_clients")]
    clients: HashMap<String, ClientProperty>,
    primary: Option<String>,
}

impl ClientRegistry {
    pub fn new() -> Self {
        Self {
            clients: Default::default(),
            primary: None,
        }
    }

    pub fn add_client(&mut self, client: ClientProperty) {
        self.clients.insert(client.name.clone(), client);
    }

    pub fn set_primary(&mut self, primary: String) {
        self.primary = Some(primary);
    }

    pub fn to_clients(
        &self,
        ctx: &RuntimeContext,
    ) -> Result<(Option<String>, HashMap<String, Arc<LLMProvider>>)> {
        let mut clients = HashMap::new();
        for (name, client) in &self.clients {
            let provider = LLMProvider::try_from((client, ctx))
                .context(format!("Failed to parse client: {}", name))?;
            clients.insert(name.into(), Arc::new(provider));
        }
        // TODO: Also do validation here
        Ok((self.primary.clone(), clients))
    }
}

fn deserialize_clients<'de, D>(deserializer: D) -> Result<HashMap<String, ClientProperty>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Vec::deserialize(deserializer)?
        .into_iter()
        .map(|client: ClientProperty| (client.name.clone(), client))
        .collect())
}
