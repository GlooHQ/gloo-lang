use internal_baml_jinja::types::{Builder, OutputFormatContent};
use jsonish::from_str;
use baml_types::{FieldType, LiteralValue};

pub const CLASS_SCHEMA: &str = r#"
class Book {
    title string
    author string
    year int
    tags string[]
    ratings Rating[]
}

class Rating {
    score int
    reviewer string
    date string
}
"#;

pub const UNION_SCHEMA: &str = r#"
class TextContent {
    text string
}

class ImageContent {
    url string
    width int
    height int
}

class VideoContent {
    url string
    duration int
}
"#; 