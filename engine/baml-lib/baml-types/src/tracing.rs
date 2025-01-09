use serde::{Deserialize, Serialize};

pub type TraceTags = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug, Serialize, Deserialize)]
// TODO: use a prefixed UUID type for this
pub struct SpanId(pub Vec<String>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceEventBatch {
    pub events: Vec<TraceEvent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TraceEvent {
    SpanStart(TraceSpanStart),
    SpanEnd(TraceSpanEnd),
    Log(TraceLog),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceMetadata {
    /// human-readable callsite identifier, e.g. "ExtractResume" or "openai/gpt-4o/chat"
    pub callsite: String,
    /// verbosity level
    #[serde(with = "level_serde")]
    pub verbosity: tracing_core::Level,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceSpanStart {
    pub span_id: SpanId,
    pub meta: TraceMetadata,
    #[serde(with = "timestamp_serde")]
    pub start_time: web_time::Instant,
    pub fields: serde_json::Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceSpanEnd {
    pub span_id: SpanId,
    pub meta: TraceMetadata,
    #[serde(with = "timestamp_serde")]
    pub start_time: web_time::Instant,
    pub duration: web_time::Duration,
    pub fields: serde_json::Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceLog {
    pub span_id: SpanId,
    pub meta: TraceMetadata,
    #[serde(with = "timestamp_serde")]
    pub start_time: web_time::Instant,
    pub msg: String,
    pub tags: serde_json::Map<String, serde_json::Value>,
}

// Add this helper module for serialization
mod timestamp_serde {
    use serde::{Deserializer, Serializer};
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    // Serialize Instant as Unix timestamp (milliseconds since epoch)
    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let now = Instant::now();
        let system_now = SystemTime::now();
        let duration_since_epoch = system_now
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        let instant_duration = instant.duration_since(now);
        let timestamp = duration_since_epoch - instant_duration;

        serializer.serialize_u128(timestamp.as_millis())
    }

    // Deserialize Unix timestamp back to Instant
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp_ms: u128 = serde::Deserialize::deserialize(deserializer)?;
        let now = Instant::now();
        let system_now = SystemTime::now();
        let duration_since_epoch = system_now
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        let timestamp_duration = Duration::from_millis(timestamp_ms as u64);
        let instant = now - (duration_since_epoch - timestamp_duration);

        Ok(instant)
    }
}

// Add this helper module for tracing::Level serialization
mod level_serde {
    use serde::{Deserializer, Serializer};
    use tracing_core::Level;

    pub fn serialize<S>(level: &Level, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&level.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Level, D::Error>
    where
        D: Deserializer<'de>,
    {
        let level_str: String = serde::Deserialize::deserialize(deserializer)?;
        level_str.parse().map_err(serde::de::Error::custom)
    }
}
