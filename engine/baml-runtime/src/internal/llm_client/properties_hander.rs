use anyhow::{Context, Result};
use std::collections::HashMap;

use super::{AllowedMetadata, SupportedRequestModes};

pub(super) struct PropertiesHandler {
    properties: HashMap<String, serde_json::Value>,
}

impl PropertiesHandler {
    pub fn new(properties: HashMap<String, serde_json::Value>) -> Self {
        Self { properties }
    }

    pub fn finalize(self) -> HashMap<String, serde_json::Value> {
        self.properties
    }

    fn get(&mut self, key: &str) -> Option<serde_json::Value> {
        self.properties.remove(key)
    }

    fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        // Ban certain keys
        match key {
            "allowed_role_metadata"
            | "supports_streaming"
            | "base_url"
            | "api_key"
            | "headers"
            | "default_role" => {
                unreachable!("{} is a reserved key in options", key)
            }
            _ => self.properties.remove(key),
        }
    }

    pub fn remove_serde<T: serde::de::DeserializeOwned>(&mut self, key: &str) -> Result<Option<T>> {
        match self.remove(key) {
            Some(value) => Ok(Some(
                serde_json::from_value(value).context(format!("Failed to parse: {key}"))?,
            )),
            None => Ok(None),
        }
    }

    pub fn remove_str(&mut self, key: &str) -> Result<Option<String>> {
        match self.remove(key) {
            Some(value) => match value.as_str() {
                Some(s) => Ok(Some(s.to_string())),
                None => anyhow::bail!("{} must be a string", key),
            },
            None => Ok(None),
        }
    }

    pub fn pull_headers(&mut self) -> Result<HashMap<String, String>> {
        let headers = self.get("headers").map(|v| {
            if let Some(v) = v.as_object() {
                v.iter()
                    .map(|(k, v)| {
                        Ok((
                            k.to_string(),
                            match v {
                                serde_json::Value::String(s) => s.to_string(),
                                _ => anyhow::bail!("Header '{k}' must be a string"),
                            },
                        ))
                    })
                    .collect::<Result<HashMap<String, String>>>()
            } else {
                Ok(Default::default())
            }
        });
        let headers = match headers {
            Some(h) => h?,
            None => Default::default(),
        };

        Ok(headers)
    }

    pub fn pull_allowed_role_metadata(&mut self) -> Result<AllowedMetadata> {
        let allowed_metadata = match self.get("allowed_role_metadata") {
            Some(allowed_metadata) => serde_json::from_value(allowed_metadata).context(
                "allowed_role_metadata must be an array of keys. For example: ['key1', 'key2']",
            )?,
            None => AllowedMetadata::None,
        };

        Ok(allowed_metadata)
    }

    pub fn pull_base_url(&mut self) -> Result<Option<String>> {
        self.get("base_url").map_or(Ok(None), |v| {
            match v
                .as_str()
                .map(|s| Some(s))
                .ok_or_else(|| anyhow::anyhow!("base_url must be a string"))?
            {
                Some(s) if s.is_empty() => {
                    anyhow::bail!("base_url must be a non-empty string")
                }
                Some(s) => Ok(Some(s.to_string())),
                None => Ok(None),
            }
        })
    }

    pub fn pull_default_role(&mut self, default: &str) -> Result<String> {
        let default_role = self.get("default_role").map(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| anyhow::anyhow!("default_role must be a string"))
        });
        match default_role {
            Some(Ok(role)) => Ok(role),
            Some(Err(e)) => Err(e),
            None => Ok(default.to_string()),
        }
    }

    pub fn pull_api_key(&mut self) -> Result<Option<String>> {
        let api_key = self.get("api_key").map(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| anyhow::anyhow!("api_key must be a string"))
        });
        match api_key {
            Some(Ok(key)) => Ok(Some(key)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    pub fn pull_supported_request_modes(&mut self) -> Result<SupportedRequestModes> {
        let supports_streaming = match self.get("supports_streaming") {
            Some(v) => match v {
                serde_json::Value::Bool(s) => Some(s),
                _ => {
                    return Err(anyhow::anyhow!(
                        "supports_streaming must be a bool: Got {:?}",
                        v
                    ))
                }
            },
            None => None,
        };

        Ok(SupportedRequestModes {
            stream: supports_streaming,
        })
    }
}

impl crate::client_registry::ClientProperty {
    pub(super) fn property_handler(&self) -> Result<PropertiesHandler> {
        Ok(PropertiesHandler::new(
            self.options
                .iter()
                .map(|(k, v)| Ok((k.clone(), serde_json::json!(v))))
                .collect::<Result<HashMap<_, _>>>()?,
        ))
    }
}
