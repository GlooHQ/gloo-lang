use baml_types::BamlMap;
use bstd::dedent;

use crate::jsonish::Value;

#[derive(Debug)]
pub enum JsonCollection {
    // Key, Value
    Object(Vec<String>, Vec<Value>),
    Array(Vec<Value>),
    QuotedString(String),
    TripleQuotedString(String),
    SingleQuotedString(String),
    // edge cases that need handling:
    // - triple backticks in a triple backtick string
    // - will the LLM terminate a triple backtick with a single backtick? probably not
    // - do we give the language specifier out? no
    // - what if the triple backtick block contains both a lang and path specifier? e.g. ```tsx path/to/file.tsx
    //   should we hand back the path?
    // - do we dedent the output?
    // - is it an acceptable heuristic to discard the first line of a triple backtick block?
    TripleBacktickString {
        lang: Option<String>,
        path: Option<String>,
        content: String,
    },
    BacktickString(String),
    // Handles numbers, booleans, null, and unquoted strings
    UnquotedString(String),
    // Starting with // or #
    TrailingComment(String),
    // Content between /* and */
    BlockComment(String),
}

impl JsonCollection {
    pub fn name(&self) -> &'static str {
        match self {
            JsonCollection::Object(_, _) => "Object",
            JsonCollection::Array(_) => "Array",
            JsonCollection::QuotedString(_) => "String",
            JsonCollection::SingleQuotedString(_) => "String",
            JsonCollection::TripleBacktickString { .. } => "TripleBacktickString",
            JsonCollection::BacktickString(_) => "String",
            JsonCollection::TripleQuotedString(_) => "TripleQuotedString",
            JsonCollection::UnquotedString(_) => "UnquotedString",
            JsonCollection::TrailingComment(_) => "Comment",
            JsonCollection::BlockComment(_) => "Comment",
        }
    }
}

impl From<JsonCollection> for Option<Value> {
    fn from(collection: JsonCollection) -> Option<Value> {
        Some(match collection {
            JsonCollection::TrailingComment(_) | JsonCollection::BlockComment(_) => return None,
            JsonCollection::Object(keys, values) => {
                // log::debug!("keys: {:?}", keys);
                let mut object = Vec::new();
                for (key, value) in keys.into_iter().zip(values.into_iter()) {
                    object.push((key, value));
                }
                Value::Object(object)
            }
            JsonCollection::Array(values) => Value::Array(values),
            JsonCollection::QuotedString(s) => Value::String(s),
            JsonCollection::TripleQuotedString(s) => Value::String(s),
            JsonCollection::SingleQuotedString(s) => Value::String(s),
            JsonCollection::TripleBacktickString { content, .. } => {
                let Some((fenced_codeblock_info, codeblock_contents)) = content.split_once("\n")
                else {
                    return Some(Value::String(content));
                };

                Value::String(dedent(codeblock_contents).content)
            }
            JsonCollection::BacktickString(s) => Value::String(s),
            JsonCollection::UnquotedString(s) => {
                let s = s.trim();
                if s == "true" {
                    Value::Boolean(true)
                } else if s == "false" {
                    Value::Boolean(false)
                } else if s == "null" {
                    Value::Null
                } else if let Ok(n) = s.parse::<i64>() {
                    Value::Number(n.into())
                } else if let Ok(n) = s.parse::<u64>() {
                    Value::Number(n.into())
                } else if let Ok(n) = s.parse::<f64>() {
                    match serde_json::Number::from_f64(n) {
                        Some(n) => Value::Number(n),
                        None => Value::String(s.into()),
                    }
                } else {
                    Value::String(s.into())
                }
            }
        })
    }
}
