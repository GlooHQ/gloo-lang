use crate::internal::llm_client::properties_hander::SharedProperties;

pub struct PostRequestProperties {
    pub shared: SharedProperties,
    pub proxy_url: Option<String>,
} 