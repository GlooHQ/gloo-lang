use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_base_url")]
    pub base_url: String,
    pub secret: Option<String>,
    pub project_id: Option<String>,
    #[serde(default = "default_sessions_id")]
    pub sessions_id: String,
    #[serde(default = "default_stage")]
    pub stage: String,
    #[serde(default = "default_host_name")]
    pub host_name: String,
    #[serde(default)] // default is false
    pub log_redaction_enabled: bool,
    #[serde(default = "default_redaction_placeholder")]
    pub log_redaction_placeholder: String,
    #[serde(default = "default_max_log_chunk_chars")]
    pub max_log_chunk_chars: usize,
    #[serde(default)] // default is false
    pub log_json: bool,
}

fn default_base_url() -> String {
    "https://app.boundaryml.com/api".to_string()
}

fn default_redaction_placeholder() -> String {
    "<BAML_LOG_REDACTED>".to_string()
}

fn default_sessions_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn default_stage() -> String {
    "development".to_string()
}

fn default_host_name() -> String {
    #[cfg(target_arch = "wasm32")]
    return "<browser>".to_string();

    #[cfg(not(target_arch = "wasm32"))]
    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or("unknown".to_string())
}

fn default_max_log_chunk_chars() -> usize {
    64_000
}

impl Config {
    pub fn from_env_vars<T: AsRef<str>>(env_vars: impl Iterator<Item = (T, T)>) -> Result<Self> {
        let env_map: HashMap<String, String> = env_vars
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect();

        // Extract and remove the BAML_LOG_JSON value
        let log_json_key = "BAML_LOG_JSON".to_string();
        let log_json_value = env_map.get(&log_json_key).cloned();

        // Collect remaining environment variables with the BOUNDARY_ prefix
        let boundary_vars = env_map
            .into_iter()
            .filter(|(k, _)| k != &log_json_key)
            .collect::<HashMap<String, String>>();

        // Parse the Config from BOUNDARY_ prefixed variables
        let mut config: Config = envy::prefixed("BOUNDARY_")
            .from_iter(boundary_vars.into_iter())
            .map_err(|e| anyhow::anyhow!("Failed to parse BOUNDARY_ config: {}", e))?;

        // Override the log_json field if BAML_LOG_JSON is present
        if let Some(value) = log_json_value {
            config.log_json = value
                .parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse BAML_LOG_JSON into bool: {}", e))?;
        }

        Ok(config)
    }
}
